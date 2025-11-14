use rand::Rng;

use crate::settings::*;

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
        let direction = rng.gen_range(0.0..std::f32::consts::TAU);

        Self {
            x: rng.gen_range(0.0..SIMULATION_AREA_SIZE),
            y: rng.gen_range(0.0..SIMULATION_AREA_SIZE),
            velocity_x: direction.cos() * MOVING_SPEED,
            velocity_y: direction.sin() * MOVING_SPEED,
            state: PersonState::Susceptible,
            infection_duration: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::*;

    #[test]
    fn test_create_new_person() {
        let person = Person::new();
        assert!(matches!(person.state, PersonState::Susceptible));
        assert_eq!(person.infection_duration, 0.0);
        assert!(person.x >= 0.0 && person.x <= SIMULATION_AREA_SIZE);
        assert!(person.y >= 0.0 && person.y <= SIMULATION_AREA_SIZE);
        assert!(person.velocity_x >= -MOVING_SPEED && person.velocity_x <= MOVING_SPEED);
        assert!(person.velocity_y >= -MOVING_SPEED && person.velocity_y <= MOVING_SPEED);
    }
}
