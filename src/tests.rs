#[cfg(test)]
mod tests {
    mod breed_gene_weights {
        use crate::crossbreeder::BreedWeights;
        use crate::gene::Gene;

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
        fn order_independent_middle() {
            let mut breeder = BreedWeights::new();
            breeder.add(Gene::Y);
            breeder.add(Gene::H);
            breeder.add(Gene::Y);
            assert_eq!(breeder.most_dominant(), Gene::Y);
        }

        #[test]
        fn order_independent_last() {
            let mut breeder = BreedWeights::new();
            breeder.add(Gene::Y);
            breeder.add(Gene::Y);
            breeder.add(Gene::H);
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
        use crate::crossbreeder::Crossbreeder;
        use crate::make_plant;
        use crate::plant::Plant;

        #[test]
        fn simple_singular() {
            let mut breeder = Crossbreeder::new();
            breeder.add(make_plant!("YYYWWW"));
            assert_eq!(breeder.winner::<Plant>(), make_plant!("YYYWWW"));
        }

        #[test]
        fn simple_pure() {
            let mut breeder = Crossbreeder::new();
            breeder.add(make_plant!("YYYYYY"));
            assert_eq!(breeder.winner::<Plant>(), make_plant!("YYYYYY"));
        }

        #[test]
        fn simple_dominated() {
            let mut breeder = Crossbreeder::new();
            breeder.add(make_plant!("YYYYYY"));
            breeder.add(make_plant!("WWWWWW"));
            assert_eq!(breeder.winner::<Plant>(), make_plant!("WWWWWW"));
        }

        #[test]
        fn simple_striped() {
            let mut breeder = Crossbreeder::new();
            breeder.add(make_plant!("YWYWYW"));
            breeder.add(make_plant!("YYYYYY"));
            assert_eq!(breeder.winner::<Plant>(), make_plant!("YWYWYW"));
        }

        #[test]
        fn trio() {
            let mut breeder = Crossbreeder::new();
            breeder.add(make_plant!("YHGWWW"));
            breeder.add(make_plant!("GWGHWY"));
            breeder.add(make_plant!("WXGHYY"));
            assert_eq!(breeder.winner::<Plant>(), make_plant!("WWGHWY"));
        }
    }
}
