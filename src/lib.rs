extern crate core;

use crate::crossbreeder::Crossbreeder;
use crate::traits::PlantImpl;
use itertools::{chain, Itertools};
use std::array;
use std::fmt::Debug;
use crate::plant::Plant;

pub mod crossbreeder;
mod gene;
pub mod plant;
pub mod plant16;
mod tests;
pub mod traits;

pub fn breed<
    const PERMUTATIONS: usize,
    const PARENTS_LIMIT: usize,
    T: PlantImpl + Clone + Sized + Debug + Copy,
>(
    plants: impl Iterator<Item = T>,
) -> impl Iterator<Item = (T, u8, [Option<T>; PARENTS_LIMIT])> {
    let permutations = plants
        .into_iter()
        .permutations(PERMUTATIONS)
        .map(|e| TryInto::<[T; PERMUTATIONS]>::try_into(e).unwrap());

    permutations
        .into_iter()
        .map(|permutation| {
            let breeder = Crossbreeder::from_iter(permutation.iter());
            let mut parents = permutation.iter().copied();
            (
                breeder.winners(),
                array::from_fn(|_| parents.next()),
            )
        })
        .map(|(probabilities, parents)| {
            let size = probabilities.size_hint().1.unwrap() as u8;
            probabilities.map(move |e| (e, size, parents))
        })
        .flatten()
}

pub fn breed_plants(input: String) -> String {
    let mut output = String::new();
    let mut plants = Plant::from_strings(&input);

    plants.sort_unstable_by_key(|e| e.score());
    plants.reverse();

    let new = chain!(
        breed::<4, 4, _>(plants.iter().copied()),
        breed::<3, 4, _>(plants.iter().copied()),
        breed::<2, 4, _>(plants.iter().copied()),
    );

    for (plant, count, parents) in new
        .filter(|(p, _, _)| p.avg_score() >= 5.5)
        .sorted_by_key(|e|e.0.score())
        .rev()
        .take(10)
    {
        output.push_str(&format!(
            "Score: {} {} {:.1}% Parents: {} \n",
            plant.avg_score(),
            plant,
            100.0 / count as f64,
            parents.iter().filter_map(|&e| e).join(" ")
        ));
    }
    output
}