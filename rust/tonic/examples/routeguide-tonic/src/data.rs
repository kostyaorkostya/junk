use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Point {
    latitude: i32,
    longitude: i32,
}

#[derive(Debug, Deserialize)]
struct Feature {
    name: String,
    location: Point,
}

pub fn load() -> Result<Vec<crate::route_guide::Feature>, Box<dyn std::error::Error>> {
    let contents: Vec<Feature> = {
        let data_dir = std::path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "data"]);
        let file = File::open(data_dir.join("route_guide_db.json"))?;
        serde_json::from_reader(&file)?
    };

    Ok(contents
        .into_iter()
        .map(|feature| crate::route_guide::Feature {
            name: feature.name,
            location: Some(crate::route_guide::Point {
                latitude: feature.location.latitude,
                longitude: feature.location.longitude,
            }),
        })
        .collect())
}
