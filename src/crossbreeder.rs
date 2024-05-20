use std::array::from_fn;
use crate::gene::Gene;
use crate::plant::Plant;

impl Crossbreeder {
    pub fn new() -> Self {
        Self {
            acum: [BreedWeights::new(); 6],
        }
    }
    pub fn add(&mut self, plant: Plant) {
        for (index, weight) in self.acum.iter_mut().enumerate() {
            let gene = plant.genes()[index];
            weight.add(gene);
        }
    }
    pub fn winner(&self) -> Plant {
        let mut iter = self.acum.iter().map(|e|e.most_dominant());
       Plant::from_genes(from_fn(|_|iter.next().unwrap()))
    }
}

#[derive(Copy, Clone)]
pub struct BreedWeights {
    // G Y H X W with their weights
    genes: [u8; 5],
}

impl BreedWeights {
    pub fn new() -> Self {
        Self { genes: [0; 5] }
    }

    pub fn most_dominant(self) -> Gene {
        let gene_index = self
            .genes
            .iter()
            .enumerate()
            .max_by(|&lhs, rhs| lhs.1.cmp(&rhs.1))
            .map(|e| e.0);
        match &gene_index {
            Some(value) => match value {
                0 => Gene::G,
                1 => Gene::Y,
                2 => Gene::H,
                3 => Gene::X,
                4 => Gene::W,
                _ => unreachable!(),
            },
            None => {
                unreachable!()
            }
        }
    }

    pub fn add(&mut self, gene: Gene) {
        let idx = match gene {
            Gene::G => 0,
            Gene::Y => 1,
            Gene::H => 2,
            Gene::X => 3,
            Gene::W => 4,
        } as usize;
        let val = self.genes.get_mut(idx).unwrap();
        *val = val.saturating_add(gene.breed_weight());
    }
}

pub struct Crossbreeder {
    // In order: G Y H X W
    acum: [BreedWeights; 6],
}