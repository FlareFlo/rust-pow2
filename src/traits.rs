use crate::gene::Gene;

pub trait PlantImpl {
    fn from_genes(genes: [Gene; 6]) -> Self;
    fn genes(&self) -> impl Iterator<Item = Gene>;

    fn score(&self) -> i8 {
        self.genes().into_iter().map(|e| e.score()).sum()
    }

    fn avg_score(&self) -> f64 {
        self.score() as f64 / 16.0
    }

    crate::impl_gene_count!(G, count_g);
    crate::impl_gene_count!(Y, count_y);
    crate::impl_gene_count!(H, count_h);
    crate::impl_gene_count!(X, count_x);
    crate::impl_gene_count!(W, count_w);
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
        fn $fn_name(&self) -> u8 {
            self.genes().filter(|&e| e == Gene::$gene_type).count() as u8
        }
    };
}
