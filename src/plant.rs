use crate::gene::Gene;
use std::array::from_fn;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use crate::traits::PlantImpl;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Plant {
    genes: [Gene; 6],
}

impl Plant {
    pub fn from_file(path: impl AsRef<Path>) -> Vec<Self> {
        let read = fs::read_to_string(path).unwrap();
        read.split("\n")
            .map(|e| Self::from_str(e).unwrap())
            .collect()
    }
}

impl FromStr for Plant {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 6);
        let mut iter = s.chars().map(|g| Gene::from_char(g));
        Ok(Self {
            genes: from_fn(|_| iter.next().unwrap()),
        })
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
        Self { genes }
    }

    fn genes(&self) -> impl Iterator<Item=Gene> {
        self.genes.into_iter()
    }
}