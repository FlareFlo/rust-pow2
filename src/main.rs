use std::time::Instant;
use itertools::Itertools;
use rust_pow2::crossbreeder::Crossbreeder;
use rust_pow2::plant::Plant;

fn main() {
    let mut plants = Plant::from_file("plants.txt");
    plants.sort_unstable_by_key(|e|e.score());
    plants.reverse();

    let start = Instant::now();

    let mut new = vec![];
    for mut permutation in plants.iter().copied().permutations(4) {
        let breeder = Crossbreeder::from_iter(permutation.iter());
        new.push(breeder.winner());

        if start.elapsed().as_secs_f64() >= 20.0 {
            break;
        }
    }


    new.sort_unstable_by_key(|lhs| lhs.score());
    for result in new
        .iter()
        .rev()
        .take(10)
    {
        println!("{} {}", result.avg_score(), result);
    }
}
