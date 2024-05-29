use crate::crossbreeder::Crossbreeder;
use crate::traits::PlantImpl;
use itertools::Itertools;
use std::fmt::Debug;
use std::iter::once;
use std::time::Instant;
use crate::plant::Plant;

pub mod crossbreeder;
mod gene;
pub mod plant;
mod plant16;
mod tests;
pub mod traits;
mod crossbreeding_results;

pub fn breed<const PERMUTATIONS: usize, T: PlantImpl + Clone + Sized + Debug + Copy>(
    plants: impl Iterator<Item = T>,
) -> impl Iterator<Item = Plant> {
    let permutations = plants
        .into_iter()
        .permutations(PERMUTATIONS)
        .map(|e| TryInto::<[T; PERMUTATIONS]>::try_into(e).unwrap());

    let start = Instant::now();
    todo!();
    // let new: Vec<T> = permutations
    //     .into_iter()
    //     .map(|permutation| {
    //         let breeder = Crossbreeder::from_iter(permutation.iter());
    //         breeder.winners()
    //     })
    //     .flatten()
    //     .collect();
    dbg!(start.elapsed());

    once(make_plant!("WWWWWW")) // TODO
}
