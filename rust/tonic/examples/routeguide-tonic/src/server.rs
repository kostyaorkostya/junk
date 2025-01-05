#![allow(unused)]

use route_guide::route_guide_server::{RouteGuide, RouteGuideServer};
use route_guide::{Feature, Point, Rectangle, RouteNote, RouteSummary};

pub mod route_guide {
    tonic::include_proto!("routeguide");
}

use futures_core::stream::BoxStream;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};
use tracing::{error, info};

mod data;

#[derive(Debug)]
pub struct RouteGuideService {
    features: Arc<HashMap<Point, Feature>>,
}

impl RouteGuideService {
    fn new(features: Vec<Feature>) -> Self {
        let features: HashMap<Point, Feature> = features
            .into_iter()
            .map(|x| (x.location.clone().unwrap(), x))
            .collect();
        Self {
            features: Arc::new(features),
        }
    }
}

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(&self, req: Request<Point>) -> Result<Response<Feature>, Status> {
        info!("GetFeature: {:?}", req.get_ref());
        if let Some(x) = self.features.get(req.get_ref()) {
            Ok(Response::new(x.clone()))
        } else {
            Err(Status::not_found(""))
        }
    }

    type ListFeaturesStream = ReceiverStream<Result<Feature, Status>>;

    async fn list_features(
        &self,
        req: Request<Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        info!("ListFeatures: {:?}", req.get_ref());
        let (tx, rx) = mpsc::channel(4);
        let features = self.features.clone();
        tokio::spawn(async move {
            for (p, f) in (&features).iter() {
                if in_range(f.location.as_ref().unwrap(), req.get_ref()) {
                    // TODO(kostya): What happens if the connection is busted or if a receiver
                    // is closed before we're done? Will [unwrap] result in panic?
                    tx.send(Ok(f.clone())).await.unwrap();
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn record_route(
        &self,
        req: Request<Streaming<Point>>,
    ) -> Result<Response<RouteSummary>, Status> {
        use tokio_stream::StreamExt;
        info!("RecordRoute");

        let mut stream = req.into_inner();
        let mut summary = RouteSummary::default();
        let mut last_point = None;
        let now = Instant::now();

        while let Some(point) = stream.next().await {
            let point = point?;
            summary.point_count += 1;

            if self.features.contains_key(&point) {
                summary.feature_count += 1;
            }

            if let Some(ref last_point) = last_point {
                summary.distance += calc_distance(last_point, &point);
            }

            last_point = Some(point);
        }

        summary.elapsed_time = now.elapsed().as_secs() as i32;

        Ok(Response::new(summary))
    }

    type RouteChatStream = BoxStream<'static, Result<RouteNote, Status>>;

    async fn route_chat(
        &self,
        req: Request<Streaming<RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        use tokio_stream::StreamExt;

        info!("RouteChat");

        let mut notes = HashMap::new();
        let mut stream = req.into_inner();

        let output = async_stream::try_stream! {
            while let Some(note) = stream.next().await {
                let note = note?;

                let location = note.location.unwrap();
                let location_notes = notes.entry(location).or_insert(vec![]);
                location_notes.push(note);

                for note in location_notes {
                    yield note.clone();
                }
            }
        };

        Ok(Response::new(Box::pin(output) as Self::RouteChatStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = "[::1]:10000".parse()?;
    info!("listening on {}", addr);

    let route_guide = RouteGuideServer::new(RouteGuideService::new(data::load()?));

    Server::builder()
        .add_service(route_guide)
        .serve(addr)
        .await?;

    Ok(())
}

impl Hash for Point {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.latitude.hash(state);
        self.longitude.hash(state);
    }
}

impl Eq for Point {}

fn in_range(p: &Point, rect: &Rectangle) -> bool {
    use std::cmp;

    let lo = rect.lo.as_ref().unwrap();
    let hi = rect.hi.as_ref().unwrap();

    let left = cmp::min(lo.longitude, hi.longitude);
    let right = cmp::max(lo.longitude, hi.longitude);
    let bottom = cmp::min(lo.latitude, hi.latitude);
    let top = cmp::max(lo.latitude, hi.latitude);

    p.longitude >= left && p.longitude <= right && p.latitude >= bottom && p.latitude <= top
}

/// Calculates the distance between two points using the "haversine" formula.
/// This code was taken from http://www.movable-type.co.uk/scripts/latlong.html.
fn calc_distance(p1: &Point, p2: &Point) -> i32 {
    const CORD_FACTOR: f64 = 1e7;
    const R: f64 = 6_371_000.0; // meters

    let lat1 = p1.latitude as f64 / CORD_FACTOR;
    let lat2 = p2.latitude as f64 / CORD_FACTOR;
    let lng1 = p1.longitude as f64 / CORD_FACTOR;
    let lng2 = p2.longitude as f64 / CORD_FACTOR;

    let lat_rad1 = lat1.to_radians();
    let lat_rad2 = lat2.to_radians();

    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lng = (lng2 - lng1).to_radians();

    let a = (delta_lat / 2f64).sin() * (delta_lat / 2f64).sin()
        + (lat_rad1).cos() * (lat_rad2).cos() * (delta_lng / 2f64).sin() * (delta_lng / 2f64).sin();

    let c = 2f64 * a.sqrt().atan2((1f64 - a).sqrt());

    (R * c) as i32
}
