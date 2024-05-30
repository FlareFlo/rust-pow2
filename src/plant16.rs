use crate::gene::Gene;
use crate::traits::PlantImpl;
use nonmax::NonMaxU16;
use std::iter::once;
use std::ops::Shr;
use std::str::FromStr;
use crate::plant::Plant;

/// ID based plant set
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Plant16(NonMaxU16);

const NICHE_BIT: u16 = 1 << 14;

impl PlantImpl for Plant16 {
    fn from_genes(genes: [Gene; 6]) -> Self {
        Self(NonMaxU16::new(genes_to_index(genes)).unwrap())
    }

    fn genes(&self) -> impl Iterator<Item = Gene> {
        index_to_genes(self.0.get()).into_iter()
    }

    fn is_one_of_many(&self) -> bool {
        self.0.get() | NICHE_BIT != 0
    }

    fn set_one_of_many(&mut self, is_one_of_many: bool) {
        self.0 = NonMaxU16::new(self.0.get() | ((is_one_of_many as u16) << 13)).unwrap();
    }
}

fn index_to_genes(mut index: u16) -> [Gene; 6] {
    let mut genes = [Gene::G; 6];
    for i in (0..6).rev() {
        genes[i] = Gene::from_digit((index % 5) as u8);
        index /= 5;
    }
    genes
}

fn genes_to_index(genes: [Gene; 6]) -> u16 {
    let mut val: u16 = 0;

    for gene in genes {
        let gene_val = match gene {
            Gene::G => 0,
            Gene::Y => 1,
            Gene::H => 2,
            Gene::X => 3,
            Gene::W => 4,
        };
        val = val * 5 + gene_val as u16;
    }
    val & !NICHE_BIT
}

impl FromStr for Plant16 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 6);
        let iter = s.chars().map(|g| Gene::from_char(g));
        Ok(Self::from_iter(iter))
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::make_plant;

    #[test]
    fn simple() {
        assert_eq!(genes_to_index(make_plant!("WWWWWW").genes_array()), 15624);
        assert_eq!(genes_to_index(make_plant!("GGGGGG").genes_array()), 0);
    }

    #[test]
    fn bitflag_safety() {
        let original: Plant16 = make_plant!("WWWWWW").into();
        let mut modified = original.clone();
        modified.set_one_of_many(true);
        println!("{:b} {:b}", original.0.get(), modified.0.get());
        assert!(modified.is_one_of_many());
        assert_eq!(modified.genes_array(), original.genes_array());

        modified.set_one_of_many(false);
        assert!(modified.is_one_of_many());
        assert_eq!(modified.genes_array(), original.genes_array());
    }
}
