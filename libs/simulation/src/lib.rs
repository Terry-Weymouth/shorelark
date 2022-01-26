#![feature(crate_visibility_modifier)]
pub use self::{animal::*, brain::*, eye::*, food::*, world::*};

mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod world;

use self::animal_individual::*;
use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::FRAC_PI_2;

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;
const GENERATION_LENGTH: usize = 2500;

pub struct Simulation{
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::new(),
            ga::UniformCrossover::new(),
            ga::GaussianMutation::new(0.01, 0.3),
        );

        Self { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    /// Performs a single step - a single second, so to say - of our
    /// simulation.
    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            // start new generation
            self.evolve(rng);
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;

        // Transforms `Vec<Animal>` to `Vec<AnimalIndividual>`
        let current_population: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        // Evolves this `Vec<AnimalIndividual>`
        let evolved_population = self.ga.evolve(
            rng,
            &current_population,
        );

        // Transforms `Vec<AnimalIndividual>` back into `Vec<Animal>`
        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();


        // Step 4: Restart foods
        //
        // (this is not strictly necessary, but it allows to easily spot
        // when the evolution happens - so it's more of a UI thing.)
        for food in &mut self.world.foods {
            food.position = rng.gen();
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(
                    &animal.position,
                    &food.position,
                );

                if distance <= 0.01 {
                    animal.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision = animal.eye.process_vision(
                animal.position,
                animal.rotation,
                &self.world.foods,
            );

            let response = animal.brain.nn.propagate(vision);

            let speed = response[0].clamp(
                -SPEED_ACCEL,
                SPEED_ACCEL,
            );

            let rotation = response[1].clamp(
                -ROTATION_ACCEL,
                ROTATION_ACCEL,
            );

            animal.speed =
                (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);

            animal.rotation = na::Rotation2::new(
                animal.rotation.angle() + rotation,
            );
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(animal.speed, 0.0);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }
}
