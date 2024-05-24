use std::fmt::Debug;
use std::time::Instant;
use itertools::Itertools;
use crate::crossbreeder::Crossbreeder;
use crate::plant::Plant;
use crate::traits::PlantImpl;

pub mod crossbreeder;
mod gene;
pub mod plant;
mod tests;
pub mod traits;

wai_bindgen_rust::export!("rust_pow2.wai");

pub struct RustPow2 {}

impl rust_pow2::RustPow2 for RustPow2 {
	fn crossbreed(input: String) -> Vec<String> {
		let mut plants = Plant::from_strings(&input);
		plants.sort_unstable_by_key(|e| e.score());
		plants.reverse();

		let mut new = vec![];
		new.extend(breed::<4, _>(plants.clone().into_iter().take(15)));
		new.extend(breed::<3, _>(plants.clone().into_iter()));
		new.extend(breed::<2, _>(plants.clone().into_iter()));
		new.sort_unstable_by_key(|e| e.score());
		new.into_iter().rev().dedup().take(10).map(|e|e.to_string()).collect()
	}
}

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