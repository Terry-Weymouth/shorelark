### Rust Language Tutorial Exercise - GA on Neural Network

Following Learning to Fly (https://pwy.io/en/posts/learning-to-fly-pt1/), a
tutorial that introduces Rust through the building of a project to evolve feeding Boids.
Based on a simple application of a Genetic Algorithm to a weights of a Neural Network.

Captured here, mostly, so that I can switch among on multiple computers (home, portable, etc.).

Note: To get the 'wasm-pack build' to work, I had to add the following to the lib.rs file of
the source for 'bumpalo-3.9.1' (in ~/.cargo/registry/src/...):

`#![feature(extended_key_value_attributes)]`

(noted here, as it is not in this GitHub projec)

Place holder for current section:
https://pwy.io/en/posts/learning-to-fly-pt4/#huggin-n-evolvin