use copperfin_compatibility_corpus::geodesy::{
    GeoPoint, MEAN_EARTH_RADIUS_METERS, destination_point, great_circle_distance, initial_bearing,
};
use copperfin_compatibility_corpus::graph::{WeightedGraph, travelling_salesperson};
use std::process::ExitCode;

fn number(text: Option<String>, name: &str) -> Result<f64, String> {
    text.ok_or_else(|| format!("missing {name}"))?
        .parse::<f64>()
        .map_err(|_| format!("invalid {name}"))
}

fn point(lat: f64, lon: f64) -> Result<GeoPoint, String> {
    GeoPoint::try_new(lat, lon).map_err(|error| format!("invalid coordinate: {error:?}"))
}

fn run() -> Result<(), String> {
    let mut arguments = std::env::args().skip(1);
    match arguments.next().as_deref() {
        Some("distance") => {
            let first = point(
                number(arguments.next(), "latitude 1")?,
                number(arguments.next(), "longitude 1")?,
            )?;
            let second = point(
                number(arguments.next(), "latitude 2")?,
                number(arguments.next(), "longitude 2")?,
            )?;
            let distance = great_circle_distance(first, second, MEAN_EARTH_RADIUS_METERS)
                .map_err(|error| format!("distance failed: {error:?}"))?;
            println!("{distance:.6}");
        }
        Some("destination") => {
            let start = point(
                number(arguments.next(), "latitude")?,
                number(arguments.next(), "longitude")?,
            )?;
            let distance = number(arguments.next(), "distance metres")?;
            let bearing = number(arguments.next(), "bearing degrees")?;
            let destination = destination_point(start, distance, bearing, MEAN_EARTH_RADIUS_METERS)
                .map_err(|error| format!("destination failed: {error:?}"))?;
            println!(
                "{:.10},{:.10}",
                destination.latitude_degrees, destination.longitude_degrees
            );
        }
        Some("bearing") => {
            let first = point(
                number(arguments.next(), "latitude 1")?,
                number(arguments.next(), "longitude 1")?,
            )?;
            let second = point(
                number(arguments.next(), "latitude 2")?,
                number(arguments.next(), "longitude 2")?,
            )?;
            let bearing = initial_bearing(first, second)
                .map_err(|error| format!("bearing failed: {error:?}"))?;
            println!("{bearing:.10}");
        }
        Some("demo-route") => {
            let mut graph = WeightedGraph::new(4);
            graph.add_undirected_edge(0, 1, 4.0).unwrap();
            graph.add_undirected_edge(0, 2, 2.0).unwrap();
            graph.add_undirected_edge(2, 1, 1.0).unwrap();
            graph.add_undirected_edge(1, 3, 5.0).unwrap();
            graph.add_undirected_edge(2, 3, 8.0).unwrap();
            let path = graph.shortest_path(0, 3).unwrap();
            println!("shortest={:?}; cost={}", path.nodes, path.total_cost);

            let matrix = vec![
                vec![0.0, 1.0, 2.0_f64.sqrt(), 1.0],
                vec![1.0, 0.0, 1.0, 2.0_f64.sqrt()],
                vec![2.0_f64.sqrt(), 1.0, 0.0, 1.0],
                vec![1.0, 2.0_f64.sqrt(), 1.0, 0.0],
            ];
            let tour = travelling_salesperson(&matrix, 0, 18).unwrap();
            println!(
                "tour={:?}; cost={}; proven_optimal={}",
                tour.nodes, tour.total_cost, tour.proven_optimal
            );
        }
        _ => {
            return Err(
                "usage: corpus-runner <distance|destination|bearing|demo-route> [arguments]"
                    .to_string(),
            );
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
