use crate::*;

#[derive(Debug)]
pub struct Brain {
    crate nn: nn::Network,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(eye)),
        }
    }
    crate fn from_chromosome(
        chromosome: ga::Chromosome,
        eye: &Eye,
    ) -> Self {
        Self {
            nn: nn::Network::from_weights(
                &Self::topology(eye),
                chromosome,
            ),
        }
    }

    crate fn as_chromosome(&self) -> ga::Chromosome {
        let genes: Vec<f32> = self.nn.weights();
        ga::Chromosome::new(genes)
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: eye.cells(),
            },
            nn::LayerTopology {
                neurons: 2 * eye.cells(),
            },
            nn::LayerTopology { neurons: 2 },
        ]
    }
}