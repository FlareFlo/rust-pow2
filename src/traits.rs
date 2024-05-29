use std::array::from_fn;
use crate::gene::Gene;

pub trait PlantImpl {
    fn from_genes(genes: [Gene; 6]) -> Self;
    fn genes(&self) -> impl Iterator<Item = Gene>;

    fn from_iter(mut iter: impl Iterator<Item = Gene>) -> Self where Self: Sized {
        PlantImpl::from_genes(from_fn(|_|iter.next().unwrap()))
    }

    fn genes_array(&self) -> [Gene; 6] {
        let mut iter = self.genes();
        from_fn(|_|iter.next().unwrap())
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
