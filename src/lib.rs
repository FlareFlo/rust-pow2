use crate::crossbreeder::Crossbreeder;
use crate::traits::PlantImpl;
use itertools::Itertools;
use std::fmt::Debug;
use std::time::Instant;

pub mod crossbreeder;
mod gene;
pub mod plant;
mod tests;
pub mod traits;
mod plant16;

pub fn breed<const PERMUTATIONS: usize, T: PlantImpl + Clone + Sized + Debug>(
    plants: impl Iterator<Item = T>,
) -> impl Iterator<Item = T> + DoubleEndedIterator {
    let permutations = plants
        .into_iter()
        .permutations(PERMUTATIONS)
        .map(|e| TryInto::<[T; PERMUTATIONS]>::try_into(e).unwrap());

    let start = Instant::now();
    let new: Vec<T> = permutations
        .into_iter()
        .map(|permutation| {
            let breeder = Crossbreeder::from_iter(permutation.iter());
            breeder.winners()
        })
        .flatten()
        .collect();
    dbg!(start.elapsed());

    new.into_iter()
}
