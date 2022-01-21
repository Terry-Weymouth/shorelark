use rand::RngCore;
// use rand::Rng;
use rand::seq::SliceRandom;

pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

pub struct GeneticAlgorithm;

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, I>(
       &self,
       rng: &mut dyn RngCore,
       population: &'a [I],
    ) -> &'a I
    where
        I: Individual;
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(
       &self,
       rng: &mut dyn RngCore,
       population: &'a [I],
    ) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self
    }

    pub fn evolve<I>(&self, population: &[I]) -> Vec<I> {
        assert!(!population.is_empty());
        (0..population.len())
        .map(|_| {
            // TODO selection
            // TODO crossover
            // TODO mutation
            todo!()
        });
//        .collect();
        todo!()
    }
}




#[cfg(test)]
mod tests {
    use crate::GeneticAlgorithm;
    use crate::Individual;
    use crate::SelectionMethod;
    use crate::RouletteWheelSelection;
    use rand::SeedableRng;
    use std::iter::FromIterator;
    use std::collections::BTreeMap;
    use rand_chacha::ChaCha8Rng;

    #[derive(Clone, Debug)]
    pub struct TestIndividual {
        fitness: f32,
    }

    impl TestIndividual {
        pub fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            self.fitness
        }
    }

    #[test]
    fn test() {
        let method = RouletteWheelSelection::new();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let mut actual_histogram = BTreeMap::new();

        //               there is nothing special about this thousand;
        //          v--v a number as low as fifty might do the trick, too
        for _ in 0..1000 {
            let fitness = method
                .select(&mut rng, &population)
                .fitness() as i32;

            *actual_histogram
                .entry(fitness)
                .or_insert(0) += 1;
        }

        let expected_histogram = BTreeMap::from_iter(vec![
            // (fitness, how many times this fitness has been chosen)
            (1, 98),  // 'near' 100
            (2, 202), // 'near' 200
            (3, 278), // 'near' 300
            (4, 422), // 'near' 400
        ]);

        assert_eq!(actual_histogram, expected_histogram);
    }
}