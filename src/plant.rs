use crate::gene::Gene;
use std::array::from_fn;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Plant {
    genes: [Gene; 6],
}

impl Plant {
    pub fn from_genes(genes: [Gene; 6]) -> Self {
        Self { genes }
    }

    pub fn score(self) -> i8 {
        self.genes.into_iter().map(|e| e.score()).sum()
    }

    pub fn avg_score(self) -> f64 {
        self.score() as f64 / 16.0
    }
    pub fn genes(&self) -> &[Gene] {
        &self.genes
    }

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

#[macro_export]
macro_rules! make_plant {
    ($input:literal) => {
        <crate::plant::Plant as std::str::FromStr>::from_str($input).unwrap()
    };
}

#[macro_export]
macro_rules! impl_gene_count {
    ($gene_type: ident, $fn_name: ident) => {
        pub fn $fn_name(self) -> u8 {
            self.genes
                .iter()
                .filter(|&&e| e == Gene::$gene_type)
                .count() as u8
        }
    };
}

impl Plant {
    impl_gene_count!(G, count_g);
    impl_gene_count!(Y, count_y);
    impl_gene_count!(H, count_h);
    impl_gene_count!(X, count_x);
    impl_gene_count!(W, count_w);
}
