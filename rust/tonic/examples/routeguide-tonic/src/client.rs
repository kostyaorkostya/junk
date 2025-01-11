pub mod route_guide {
    tonic::include_proto!("routeguide");
}

use time::Duration;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use route_guide::route_guide_client::RouteGuideClient;
use route_guide::{Point, Rectangle, RouteNote};
use std::error::Error;
use tokio::time;
use tonic::transport::Channel;
use tonic::Request;
use tracing::{error, info};

async fn print_features(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let rectangle = Rectangle {
        lo: Some(Point {
            latitude: 400_000_000,
            longitude: -750_000_000,
        }),
        hi: Some(Point {
            latitude: 420_000_000,
            longitude: -730_000_000,
        }),
    };

    let mut stream = client
        .list_features(Request::new(rectangle))
        .await?
        .into_inner();

    while let Some(f) = stream.message().await? {
        info!("FEATURE = {:?}", f);
    }

    Ok(())
}

async fn run_record_route(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let req = {
        let (outbound, point_cnt) = {
            let mut rng = SmallRng::from_rng(rand::thread_rng()).unwrap();
            let point_cnt: usize = rng.gen_range(2..100);
            let outbound = async_stream::stream! {
                for _ in 0..=point_cnt {
                    yield random_point(&mut rng)
                }
            };
            (outbound, point_cnt)
        };

        info!("Traversing {} points", point_cnt);
        Request::new(outbound)
    };

    match client.record_route(req).await {
        Ok(resp) => info!("SUMMARY {:?}", resp.into_inner()),
        Err(err) => error!("RecordRoute failed: {}", err),
    }

    Ok(())
}

async fn run_route_chat(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let start = time::Instant::now();

    let mut inbound = {
        let outbound = async_stream::stream! {
            let mut interval = time::interval(Duration::from_secs(1));

            loop {
                let time = interval.tick().await;
                let elapsed = time.duration_since(start);
                let note = RouteNote {
                    location : Some(Point {
                        latitude: 409146138 + elapsed.as_secs() as i32,
                        longitude: -746188906,
                    }),
                    message : format!("at {:?}", elapsed)
                };

                yield note
            }
        };

        client
            .route_chat(Request::new(outbound))
            .await?
            .into_inner()
    };

    while let Some(note) = inbound.message().await? {
        info!("NOTE = {:?}", note);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let mut client = RouteGuideClient::connect("http://[::1]:10000").await?;

    info!("*** SIMPLE RPC ***");
    let resp = client
        .get_feature(Request::new(Point {
            latitude: 409_146_138,
            longitude: -746_188_906,
        }))
        .await?;
    info!("RESPONSE = {:?}", resp);

    info!("\n*** SERVER STREAMING ***");
    print_features(&mut client).await?;

    info!("\n*** CLIENT STREAMING ***");
    run_record_route(&mut client).await?;

    println!("\n*** BIDIRECTIONAL STREAMING ***");
    run_route_chat(&mut client).await?;

    Ok(())
}

fn random_point(rng: &mut impl Rng) -> Point {
    let latitude = (rng.gen_range(0..180) - 90) * 10_000_000;
    let longitude = (rng.gen_range(0..360) - 180) * 10_000_000;
    Point {
        latitude,
        longitude,
    }
}
