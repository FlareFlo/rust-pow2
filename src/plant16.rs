use std::iter::once;
use std::ops::Shr;
use crate::gene::Gene;
use crate::traits::PlantImpl;

/// ID based plant set
struct Plant16(u16);

impl PlantImpl for Plant16 {
	fn from_genes(genes: [Gene; 6]) -> Self {
		Self(genes_to_index(genes))
	}

	fn genes(&self) -> impl Iterator<Item=Gene> {
		index_to_genes(self.0).into_iter()
	}
}

fn index_to_genes(mut index: u16) -> [Gene; 6] {
	let mut genes = [Gene::G; 6];
	for i in (0..6).rev() {
		genes[i] = Gene::from_digit((index % 5) as u8);
		index /= 5;
	}
	genes
}

fn genes_to_index(genes: [Gene; 6]) -> u16 {
	let mut val: u16 = 0;

	for gene in genes {
		let gene_val = match gene {
			Gene::G => {0}
			Gene::Y => {1}
			Gene::H => {2}
			Gene::X => {3}
			Gene::W => {4}
		};
		val = val * 5 + gene_val as u16;
	}
	val
}

#[cfg(test)]
mod test {
	use crate::make_plant;
	use super::*;

	#[test]
	fn simple() {
		assert_eq!(genes_to_index(make_plant!("WWWWWW").genes_array()), 15624);
		assert_eq!(genes_to_index(make_plant!("GGGGGG").genes_array()), 0);
	}
}