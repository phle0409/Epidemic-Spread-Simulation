# Epidemic-Spread-Simulation

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Agent-based SIR disease spread simulation in Rust

## Description

This is a simulation that shows how diseases spread through a population. It uses the SIR model, which stands for Susceptible-Infected-Recovered.

In this project, each person is like a little dot that moves around in a 2D space. When a susceptible person gets close to a infected person, they have the probabillity set by user that can get infected too. User can change different settings to see how it affects the spread. There's also a visual display so you can watch what happens in real-time.

## Features (What was built)
- A 2D space that shows people as dots to simulate the spreading disease
- The graph displays the number and percentage of susceptible, infected, and recovered people throughout the simulation
- There are several intervention methods to reduce the spread of disease, such as social distancing and quarantine.
- User can adjust settings like community size, initial infected, infected radius, social distancing radius and time before quarantine.

## How it works.
- User uses "cargo run" to run the program.
- With default settings, there are a total of 80 people, of which 3 are infected. All of them walk around randomly in the community. When a normal person is close enough to infected people (within the same radius), they have a 30% probability of getting infected. The infected people will become recovered after a specific time (default RECOVERED_DAY = 8.0). All of the default settings can be found in the file [settings.rs](src/settings.rs).
- There is a graph that collects how many people are in each state (susceptible, infected, and recovered) and displays it in real-time (in second).
- Users can adjust settings directly in the UI, including community size, initial infected count, and infected radius. After modifying the settings, click the "Apply and Reset" button to restart the simulation with the new parameters. This allows users to model different diseases with varying information.
- **Prevention Methods** (click "Apply and Reset" after changing parameters in the UI):
    - **Social Distancing**: When enabled, all individuals in the community maintain a safe distance from each other. Users can adjust the social distancing radius in the UI to control the minimum separation distance, helping to reduce disease transmission.
    - **Quarantine**: When enabled, infected individuals are moved to a quarantine zone after a specified period from when they got the disease (default: 5 days). This provides an additional method to reduce disease transmission by isolating infected people from the healthy population.
