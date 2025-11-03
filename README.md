# Epidemic-Spread-Simulation

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Agent-based SIR disease spread simulation in Rust

## Description

This is a simulation that shows how diseases spread through a population. It uses the SIR model, which stands for Susceptible-Infected-Recovered.

In this project, each person is like a little dot that moves around in a 2D space. When a susceptible person gets close to a infected person, they have the probabillity set by user that can get infected too. User can change different settings to see how it affects the spread. There's also a visual display so you can watch what happens in real-time.

## Features
- A 2D space that shows people as dots to simulate the spreading disease
- There are several intervention methods to reduce the spread of disease, such as social distancing, quarantine, and restricted travel between communities
- The graph displays the number and percentage of susceptible, infected, and recovered people throughout the simulation
