use rand::Rng;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {

    pub fn random(
        layers: &[LayerTopology],
        rng: &mut dyn rand::RngCore,
    ) -> Self {
        assert!(layers.len() > 1);
        let layers = layers
        .windows(2)
        .map(|layers| {
            Layer::random(rng,layers[0].neurons, layers[1].neurons)
        })
        .collect();

        Self { layers }
    }

    fn new (layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
         self.layers
        .iter()
        .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn weights(&self) -> Vec<f32> {
        todo!()
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {

    pub fn random(
        rng: &mut dyn rand::RngCore,
        input_neurons: usize,
        output_neurons: usize,
    ) -> Self {
        let neurons = (0..output_neurons)
        .map(|_| Neuron::random(rng, input_neurons))
        .collect();

        Self { neurons }
    }

    fn new( neurons: Vec<Neuron> ) -> Self {
        Self {
            neurons,
        }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    pub fn random(
        rng: &mut dyn rand::RngCore,
        output_size: usize,
    ) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    fn new(bias: f32, weights: Vec<f32>) -> Self {
        Self { bias, weights }
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
        // assert_eq!(inputs.len(), self.weights.len());
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        (self.bias + output).max(0.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand_chacha::ChaCha8Rng;
    use rand::SeedableRng;
    use approx;

    mod neuron {
        use super::*;

        mod random {
            use super::*;

            #[test]
            fn test() {
                let mut rng = ChaCha8Rng::from_seed(Default::default());
                let neuron = Neuron::random(&mut rng, 4);

                approx::assert_relative_eq!(neuron.bias, -0.6255188);

                approx::assert_relative_eq!(neuron.weights.as_slice(), [
                    0.67383957,
                    0.8181262,
                    0.26284897,
                    0.5238807,
                ].as_ref());
            }
        }

        mod propagate{
            use super::*;

            #[test]
            fn test() {
                let neuron = Neuron {
                    bias: 0.5,
                    weights: vec![-0.3, 0.8],
                };

                // Ensures `.max()` (our ReLU) works:
                approx::assert_relative_eq!(
                    neuron.propagate(&[-10.0, -10.0]),
                    0.0,
                );

                // `0.5` and `1.0` chosen by a fair dice roll:
                approx::assert_relative_eq!(
                    neuron.propagate(&[0.5, 1.0]),
                    (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
                );

                // We could've written `1.15` right away, but showing the entire
                // formula makes our intentions clearer
            }

            #[test]
            fn test_with_new() {
                let neuron = Neuron::new(0.5,vec![-0.3, 0.8]);
                approx::assert_relative_eq!(
                    neuron.propagate(&[-10.0, -10.0]),
                    0.0,
                );
                approx::assert_relative_eq!(
                    neuron.propagate(&[0.5, 1.0]),
                    (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
                );
            }
        }
    }

    mod layer {
        use super::*;

        mod propagate {
            use super::*;
            #[test]
            fn test() {
                let layer = Layer::new(
                    vec![
                        Neuron::new(0.1, vec![0.2, 0.3, 0.4]),
                        Neuron::new(- 0.1, vec![0.2, 0.3, 0.4]),
                    ]
                );

                let results = layer.propagate(vec![0.5, 1.0]);
                let actual = vec![0.5 as f32, 0.3];
                approx::assert_relative_eq!(actual.as_slice(), results.as_slice())
            }
        }
    }

    mod network {
        use super::*;
        mod propagate {
            use super::*;

            #[test]
            fn test() {
                let network = Network::new(vec![
                    Layer::new(vec![Neuron::new(0.1, vec![0.2, 0.3, 0.4])]),
                    Layer::new(vec![Neuron::new(0.5, vec![0.6, 0.7, 0.8])]),
                ]);

                let results = network.propagate(vec![0.2, 0.3]);
                let actual = vec![0.638 as f32];
                approx::assert_relative_eq!(actual.as_slice(), results.as_slice());
            }
        }
    }
}