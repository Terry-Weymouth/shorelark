use rand::Rng;

pub struct Network {
    layers: Vec<Layer>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {

    pub fn random(layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);
        let layers = layers
        .windows(2)
        .map(|layers| {
            Layer::random(layers[0].neurons, layers[1].neurons)
        })
        .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
         self.layers
        .iter()
        .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

}

struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {

    pub fn random(
        input_neurons: usize,
        output_neurons: usize,
    ) -> Self {
        let neurons = (0..output_neurons)
        .map(|_| Neuron::random(input_neurons))
        .collect();

        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}


impl Neuron {
    pub fn random(output_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

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


// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     mod random {
//         use super::*;
//         use rand::SeedableRng;
//         use rand_chacha::ChaCha8Rng;
//         use approx;
//
//         #[test]
//         fn test() {
//             // Because we always use the same seed, our `rng` in here will
//             // always return the same set of values
//             let mut rng = ChaCha8Rng::from_seed(Default::default());
//             let neuron = Neuron::random(&mut rng, 4);
//
//             assert_eq!(neuron.bias, 0.0);
//             assert_eq!(neuron.weights, &[0.0, 0.0, 0.0, 0.0]);
//         }

//         #[test]
//         fn test() {
//             let mut rng = ChaCha8Rng::from_seed(Default::default());
//             let neuron = Neuron::random(&mut rng, 4);
//
//             assert_relative_eq!(neuron.bias, -0.6255188);
//
//             assert_relative_eq!(neuron.weights.as_slice(), [
//                 0.67383957,
//                 0.8181262,
//                 0.26284897,
//                 0.5238807,
//             ].as_ref());
//         }
//     }
// }