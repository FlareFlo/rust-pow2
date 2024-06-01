use itertools::{chain, Itertools};
use rust_pow2::breed;
use rust_pow2::plant::Plant;
use rust_pow2::traits::PlantImpl;
use std::time::Instant;

fn main() {
    let mut plants = Plant::from_file("plants.txt");

    let start = Instant::now();
    plants.sort_unstable_by_key(|e| e.score());
    plants.reverse();

    let new = chain!(
        breed::<4, 4, _>(plants.iter().copied()),
        breed::<3, 4, _>(plants.iter().copied()),
        breed::<2, 4, _>(plants.iter().copied()),
    );

    println!("Top 10 plants:");
    for (plant, count, parents) in new
        .filter(|(p, _, _)| p.avg_score() >= 5.5)
        .take(10)
    {
        println!(
            "Score: {} {} {:.1}% Parents: {}",
            plant.avg_score(),
            plant,
            100.0 / count as f64,
            parents.iter().filter_map(|&e| e).join(" ")
        );
    }
    println!("Runtime: {:?}", start.elapsed());
}
