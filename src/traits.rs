use crate::from_into_impl;
use crate::gene::Gene;
use crate::plant::Plant;
use crate::plant16::Plant16;
use std::array::from_fn;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub trait PlantImpl {
    fn from_genes(genes: [Gene; 6]) -> Self;
    fn genes(&self) -> impl Iterator<Item = Gene>;

    fn is_one_of_many(&self) -> bool;
    fn set_one_of_many(&mut self, is_one_of_many: bool);

    fn with_one_of_many(mut self, is_one_of_many: bool) -> Self
    where
        Self: Sized,
    {
        self.set_one_of_many(is_one_of_many);
        self
    }

    fn from_iter(mut iter: impl Iterator<Item = Gene>) -> Self
    where
        Self: Sized,
    {
        PlantImpl::from_genes(from_fn(|_| iter.next().unwrap()))
    }

    fn genes_array(&self) -> [Gene; 6] {
        let mut iter = self.genes();
        from_fn(|_| iter.next().unwrap())
    }

    fn score(&self) -> i8 {
        self.genes().into_iter().map(|e| e.score()).sum()
    }

    fn avg_score(&self) -> f64 {
        self.score() as f64 / 16.0
    }

    fn is_useless(&self, red_threshold: u8) -> bool {
        self.count_red() >= red_threshold
    }

    fn from_file(path: impl AsRef<Path>) -> Vec<Self> where Self: Sized + FromStr + Debug, <Self as FromStr>::Err: Debug  {
        let read = fs::read_to_string(path).unwrap();
        Self::from_strings(&read)
    }

    fn from_strings(s: &str) -> Vec<Self> where Self: Sized + FromStr, <Self as FromStr>::Err: Debug  {
        s.split("\n").map(|e| Self::from_str(e).unwrap()).collect()
    }

    crate::impl_gene_match!(Gene::G, count_g);
    crate::impl_gene_match!(Gene::Y, count_y);
    crate::impl_gene_match!(Gene::H, count_h);
    crate::impl_gene_match!(Gene::X, count_x);
    crate::impl_gene_match!(Gene::W, count_w);

    crate::impl_gene_match!(Gene::W | Gene::X, count_red);
    crate::impl_gene_match!(Gene::Y | Gene::G | Gene::H, count_green);
}

#[macro_export]
macro_rules! make_plant {
    ($input:literal) => {
        <crate::plant::Plant as std::str::FromStr>::from_str($input).unwrap()
    };
}

#[macro_export]
macro_rules! impl_gene_match {
    ($pat: pat, $fn_name: ident) => {
        fn $fn_name(&self) -> u8 {
            self.genes().filter(|&e| matches!(e, $pat)).count() as u8
        }
    };
}

from_into_impl!(Plant, Plant16);

#[macro_export]
macro_rules! from_into_impl {
    ($from:ident, $to:ident) => {
        impl From<$from> for $to {
            fn from(value: Plant) -> Self {
                Self::from_genes(value.genes_array())
            }
        }
    };
}
