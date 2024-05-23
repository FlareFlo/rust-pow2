use std::time::Instant;
use itertools::Itertools;
use rust_pow2::crossbreeder::Crossbreeder;
use rust_pow2::plant::Plant;
use rust_pow2::traits::PlantImpl;
use rayon::prelude::*;


fn main() {
    let mut plants = Plant::from_file("plants.txt");
    plants.sort_unstable_by_key(|e| e.score());
    plants.reverse();

   let new = breed(plants.into_iter().take(15));

    println!("Top 10 plants:");
    for result in new.into_iter().rev().dedup().take(10) {
        println!("Score: {} {}", result.avg_score(), result);
    }
}

fn breed(plants: impl Iterator<Item = Plant>) -> impl Iterator<Item = Plant> + DoubleEndedIterator {
    let start = Instant::now();
    let permutations: Vec<Vec<Plant>> = plants.into_iter().permutations(4).collect();
    dbg!("Compute useful permutations", start.elapsed());

    let start = Instant::now();
    let mut new: Vec<Plant> = permutations
        .into_par_iter()
        .map(|permutation| {
            let breeder = Crossbreeder::from_iter(permutation.iter());
            breeder.winner()
        })
        .collect();
    dbg!("Crossbreed", start.elapsed());

    new.sort_unstable_by_key(|lhs| lhs.score());
    new.into_iter()
}