extern crate core;

use crate::crossbreeder::Crossbreeder;
use crate::traits::PlantImpl;
use itertools::Itertools;
use std::array;
use std::fmt::Debug;

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
