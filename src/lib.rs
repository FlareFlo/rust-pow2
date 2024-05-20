use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Gene {
    G,
    Y,
    H,
    X,
    W,
}

impl Gene {
    pub const fn breed_weight(self) -> u8 {
        match self {
            Gene::G => 4,
            Gene::Y => 4,
            Gene::H => 4,
            Gene::X => 6,
            Gene::W => 6,
        }
    }

    pub const fn score(self) -> i8 {
        match self {
            Gene::G => 16,
            Gene::Y => 16,
            Gene::H => 8,
            Gene::X => 0,
            Gene::W => -16,
        }
    }

    pub fn iter_all() -> impl Iterator<Item = Self> {
        [Self::G, Self::Y, Self::H, Self::X, Self::W].into_iter()
    }

    pub const fn from_char(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'G' => Self::G,
            'Y' => Self::Y,
            'H' => Self::H,
            'X' => Self::X,
            'W' => Self::W,
            _ => {
                unreachable!()
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Plant {
    genes: [Gene; 6],
}

impl Plant {
    pub fn score(self) -> i8 {
        self.genes.into_iter().map(|e| e.score()).sum()
    }
    pub fn genes(&self) -> &[Gene] {
        &self.genes
    }
}

impl FromStr for Plant {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 6);
        let mut iter = s.chars().map(|g| Gene::from_char(g));
        let genes = [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ];
        Ok(Self { genes })
    }
}

#[macro_export]
macro_rules! plant {
    ($input:literal) => {
        std::str::FromStr::from_str($input).unwrap()
    };
}

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
    pub fn add(&mut self, plant: Plant) {
        for (index, weight) in self.acum.iter_mut().enumerate() {
            let gene = plant.genes()[index];
            weight.add(gene);
        }
    }
    pub fn winner(&self) -> Plant {
        let mut iter = self.acum.iter().map(|e|e.most_dominant());
        Plant {
            genes: [
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ],
        }
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

#[cfg(test)]
mod tests {
    mod breed_gene_weights {
        use crate::{BreedWeights, Gene};

        #[test]
        fn simple_singular() {
            for gene in Gene::iter_all() {
                let mut breeder = BreedWeights::new();
                breeder.add(gene);
                assert_eq!(breeder.most_dominant(), gene);
            }
        }

        #[test]
        fn dominance_bad_over_good() {
            let mut breeder = BreedWeights::new();
            breeder.add(Gene::H);
            breeder.add(Gene::Y);
            assert_eq!(breeder.most_dominant(), Gene::H);
        }

        #[test]
        fn dominance_good_over_bad() {
            let mut breeder = BreedWeights::new();
            breeder.add(Gene::H);
            breeder.add(Gene::Y);
            breeder.add(Gene::Y);
            assert_eq!(breeder.most_dominant(), Gene::Y);
        }

        #[test]
        fn rock_paper_scissors() {
            let mut breeder = BreedWeights::new();
            breeder.add(Gene::H);
            breeder.add(Gene::Y);
            breeder.add(Gene::G);
            assert_eq!(breeder.most_dominant(), Gene::H);
        }
    }

    mod breed_plants {
        use crate::Crossbreeder;

        #[test]
        fn simple_singular() {
                let mut breeder = Crossbreeder::new();
                breeder.add(plant!("YYYWWW"));
                assert_eq!(breeder.winner(), plant!("YYYWWW"));
        }

        #[test]
        fn simple_pure() {
            let mut breeder = Crossbreeder::new();
            breeder.add(plant!("YYYYYY"));
            assert_eq!(breeder.winner(), plant!("YYYYYY"));
        }

        #[test]
        fn simple_dominated() {
            let mut breeder = Crossbreeder::new();
            breeder.add(plant!("YYYYYY"));
            breeder.add(plant!("WWWWWW"));
            assert_eq!(breeder.winner(), plant!("WWWWWW"));
        }

        #[test]
        fn simple_striped() {
            let mut breeder = Crossbreeder::new();
            breeder.add(plant!("YWYWYW"));
            breeder.add(plant!("YYYYYY"));
            assert_eq!(breeder.winner(), plant!("YWYWYW"));
        }

        #[test]
        fn trio() {
            let mut breeder = Crossbreeder::new();
            breeder.add(plant!("YHGWWW"));
            breeder.add(plant!("GWGHWY"));
            breeder.add(plant!("WXGHYY"));
            assert_eq!(breeder.winner(), plant!("WWGHWY"));
        }
    }
}
