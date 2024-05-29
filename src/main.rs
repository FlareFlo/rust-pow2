use itertools::Itertools;
use rust_pow2::breed;
use rust_pow2::plant::Plant;
use rust_pow2::traits::PlantImpl;

fn main() {
    let mut plants = Plant::from_file("plants.txt");
    plants.sort_unstable_by_key(|e| e.score());
    plants.reverse();

    let mut new = vec![];
    new.extend(breed::<4, _>(plants.clone().into_iter().take(15)));
    new.extend(breed::<3, _>(plants.clone().into_iter()));
    new.extend(breed::<2, _>(plants.clone().into_iter()));
    new.sort_unstable_by_key(|e| e.score());

    println!("Top 10 plants:");
    for result in new.into_iter().rev().dedup().take(10) {
        println!("Score: {} {}", result.avg_score(), result);
    }
}
