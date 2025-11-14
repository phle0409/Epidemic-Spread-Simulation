use rand::Rng;

use crate::setting::*;

pub enum PersonState {
    Susceptible,
    Infected,
    Recovered,
}

pub struct Person {
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub state: PersonState,
    pub infection_duration: f32,
}

impl Person {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);

        Self {
            x: rng.gen_range(0.0..SIMULATION_AREA_SIZE),
            y: rng.gen_range(0.0..SIMULATION_AREA_SIZE),
            vx: angle.cos() * MOVING_SPEED,
            vy: angle.sin() * MOVING_SPEED,
            state: ParticleState::Susceptible,
            infection_duration: 0.0,
        }
    }
}
