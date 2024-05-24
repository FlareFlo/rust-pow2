use std::fmt::Debug;
use std::time::Instant;
use itertools::Itertools;
use crate::crossbreeder::Crossbreeder;
use crate::traits::PlantImpl;

pub mod crossbreeder;
mod gene;
pub mod plant;
mod tests;
pub mod traits;

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
			breeder.winner()
		})
		.collect();
	dbg!(start.elapsed());

	new.into_iter()
}