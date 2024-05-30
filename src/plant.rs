use crate::gene::Gene;
use crate::traits::PlantImpl;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Plant {
    genes: [Gene; 6],
    is_one_of_many: bool,
}

impl FromStr for Plant {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 6);
        let iter = s.chars().map(|g| Gene::from_char(g));
        Ok(Self::from_iter(iter))
    }
}

impl Display for Plant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.genes.iter().map(|e| e.to_char()).collect::<String>()
        )
    }
}

impl PlantImpl for Plant {
    fn from_genes(genes: [Gene; 6]) -> Self {
        Self {
            genes,
            is_one_of_many: false,
        }
    }

    fn genes(&self) -> impl Iterator<Item = Gene> {
        self.genes.into_iter()
    }

    fn is_one_of_many(&self) -> bool {
        self.is_one_of_many
    }

    fn set_one_of_many(&mut self, is_one_of_many: bool) {
        self.is_one_of_many = is_one_of_many;
    }
}
