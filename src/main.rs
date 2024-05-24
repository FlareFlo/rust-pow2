use std::fmt::Debug;
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

    let mut new = vec![];
    new.extend(breed::<4, _>(plants.clone().into_iter().take(15)));
    new.extend(breed::<3, _>(plants.clone().into_iter().take(15)));
    new.extend(breed::<2, _>(plants.clone().into_iter().take(15)));
    new.sort_unstable_by_key(|e| e.score());

    println!("Top 10 plants:");
    for result in new.into_iter().rev().dedup().take(10) {
        println!("Score: {} {}", result.avg_score(), result);
    }
}

fn breed<const PERMUTATIONS: usize, T: PlantImpl + Clone + Sized + Debug, >(plants: impl Iterator<Item = T>) -> impl Iterator<Item = T> + DoubleEndedIterator {
    let start = Instant::now();
    let permutations: Vec<[T; PERMUTATIONS]> = plants.into_iter().permutations(PERMUTATIONS).map(|e|e.try_into().unwrap()).collect();
    dbg!(start.elapsed());

    let start = Instant::now();
    let mut new: Vec<T> = permutations
        .into_iter()
        .map(|permutation| {
            let breeder = Crossbreeder::from_iter(permutation.iter());
            breeder.winner()
        })
        .collect();
    dbg!(start.elapsed());

    new.into_iter()
}