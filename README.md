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

## How it works
- User uses "cargo run" to run the program.
- With default settings, there are a total of 80 people, of which 3 are infected. All of them walk around randomly in the community. When a normal person is close enough to infected people (within the same radius), they have a 30% probability of getting infected. The infected people will become recovered after a specific time (default RECOVERED_DAY = 8.0). All of the default settings can be found in the file [settings.rs](src/settings.rs).
- There is a graph that collects how many people are in each state (susceptible, infected, and recovered) and displays it in real-time (in second).
- Users can adjust settings directly in the UI, including community size, initial infected count, and infected radius. After modifying the settings, click the "Apply and Reset" button to restart the simulation with the new parameters. This allows users to model different diseases with varying parameters.
- **Prevention Methods** (click "Apply and Reset" after changing parameters in the UI):
    - **Social Distancing**: When enabled, all individuals in the community maintain a safe distance from each other. Users can adjust the social distancing radius in the UI to control the minimum separation distance, helping to reduce disease transmission.
    - **Quarantine**: When enabled, infected individuals are moved to a quarantine zone after a specified period from when they got the disease (default: 5 days). This provides an additional method to reduce disease transmission by isolating infected people from the healthy population.
## Known Issues and Future Improvements
- In general, everything is working correctly. However, there are some features from my original plan that I haven't implemented yet, and some areas that I want to improve:
    - **Social Distancing**: Initially, this feature worked correctly with my integrated Intel GPU. However, when I tested it on another computer with a dedicated NVIDIA GPU at the same monitor refresh rate, people in the community stopped moving. To fix this issue, I had to pass the `time_frame_per_second` variable through all related code, which resolved the frame rate problem. Since I only tested on a 60Hz monitor, I'm not sure if it will work with other refresh rates. Additionally, I implemented a simple "repulsive force" algorithm for this feature, and sometimes I notice collisions between people when the radius is too low (possibly due to UI constraints and the small community area). With higher radius values, the feature works correctly. Given more time, I would like to explore and test different algorithms to improve this feature. 
    - **Quarantine**: The current code is working correctly, but in the UI, users will see that infected people are moved immediately to the quarantine zone without a smooth transition from the community area. I wanted to implement a feature to move infected people smoothly, but it's a UI enhancement that would require extra time and testing, so I didn't implement it.
    - **Travel Between Communities**: In my original plan, I mentioned implementing multiple communities where people could travel between them. I also planned to add a restriction method to limit the number of people traveling to the center area of each community to prevent disease spread. However, this feature would require significant development and testing time, so I haven't implemented it yet.

## Lessons Learned
I learned many things after finishing this project. During the development process, I encountered numerous problems with my code and figured out how to fix them, partly with the assistance of AI. I mainly used Claude AI to help me fix problems, find algorithms, test my code, and get recommendations on refactoring to make it easier to write unit tests. Below is my development journey:

1. In [person.rs](src/person.rs), I had difficulty finding a simple algorithm for the `update_position` method to handle wall collisions. I used AI to search for and evaluate several algorithms, and found one that works exactly as I wanted.

2. I wanted to show people randomly walking around in a 2D area. I used AI to recommend which library would be best for this and decided to use the `eframe` library. I implemented the basic `eframe::App` structure with only the update method. During this process, I asked AI to provide the basic structure for drawing the 2D space, rendering dots, and changing font sizes. This helped me quickly find the features I needed and saved me a lot of time reading through the documentation.

3. After successfully showing people walking around in the community area, I implemented the infection system with 3 infected people by default (shown in red in the UI). I created the `spread_infection` method, which uses a basic algorithm to find all vulnerable people in the community, then generates a random number and compares it to the `INFECTION_PROBABILITY` constant to determine who gets infected. I had difficulty writing unit tests for `spread_infection`, so I used AI to help me refactor the code into smaller methods like `find_vulnerable_people` and `is_within_infected_radius` to make them easier to test.

4. Next, I created a Basic Settings section in the UI that includes sliders for community size, initial infected count, and infected radius, allowing users to adjust these parameters.

5. I implemented a feature to make infected people become recovered (shown in gray in the UI) after a specific time using the `infection_duration` variable in the `Person` struct.