use std::str::FromStr;
use crate::gene::Gene;

impl Plant {
    pub fn from_genes(genes: [Gene; 6]) -> Self {
        Self {
            genes,
        }
    }

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
macro_rules! make_plant {
    ($input:literal) => {
        std::str::FromStr::from_str($input).unwrap()
    };
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Plant {
    genes: [Gene; 6],
}
