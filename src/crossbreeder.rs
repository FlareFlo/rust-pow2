use crate::gene::Gene;
use crate::traits::PlantImpl;
use itertools::{iproduct, Itertools};
use std::array::from_fn;
use std::iter::once;
use crate::crossbreeding_results::CrossbreedingResults;

#[derive(Debug)]
pub struct Crossbreeder {
    // In order: G Y H X W
    acum: [BreedWeights; 6],
}

impl Crossbreeder {
    pub fn new() -> Self {
        Self {
            acum: [BreedWeights::new(); 6],
        }
    }
    pub fn add(&mut self, plant: impl PlantImpl) {
        for (index, weight) in self.acum.iter_mut().enumerate() {
            let gene = plant.genes().nth(index).unwrap();
            weight.add(gene);
        }
    }
    pub fn winners<T: PlantImpl + Copy>(&self) -> impl Iterator<Item = T> {
        let mut iter = self
            .acum
            .into_iter()
            .map(|e| e.most_dominant().collect_vec());
        iter.multi_cartesian_product().map(|e|T::from_iter(e.into_iter()))
    }

    pub fn from_iter<'a, T: PlantImpl + Clone + 'a>(iter: impl Iterator<Item = &'a T>) -> Self {
        let mut breeder = Self::new();
        for plant in iter {
            breeder.add(plant.clone())
        }
        breeder
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BreedWeights {
    // G Y H X W with their weights
    genes: [u8; 5],
}

impl BreedWeights {
    pub fn new() -> Self {
        Self { genes: [0; 5] }
    }

    pub fn most_dominant(self) -> impl Iterator<Item = Gene> {
        let &max = self
            .genes
            .iter()
            .max_by(|&&lhs, &rhs| lhs.cmp(rhs))
            .unwrap();

        self.genes
            .into_iter()
            .enumerate()
            .filter(move |&e| e.1 == max)
            .map(|e| Gene::from_digit(e.0 as u8))
    }

    pub fn add(&mut self, gene: Gene) {
        let val = self.genes.get_mut(gene.to_digit() as usize).unwrap();
        *val = val.saturating_add(gene.breed_weight());
    }
}
