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

    pub fn update_position(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;

        if self.x <= MARGIN_FROM_WALL {
            self.velocity_x = -self.velocity_x;
            self.x = MARGIN_FROM_WALL;
        }

        if self.x >= SIMULATION_AREA_SIZE - MARGIN_FROM_WALL {
            self.velocity_x = -self.velocity_x;
            self.x = SIMULATION_AREA_SIZE - MARGIN_FROM_WALL;
        }

        if self.y <= MARGIN_FROM_WALL {
            self.velocity_y = -self.velocity_y;
            self.y = MARGIN_FROM_WALL;
        }

        if self.y >= SIMULATION_AREA_SIZE - MARGIN_FROM_WALL {
            self.velocity_y = -self.velocity_y;
            self.y = SIMULATION_AREA_SIZE - MARGIN_FROM_WALL;
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

    #[test]
    fn test_update_position_normal() {
        let mut person = Person {
            x: 150.0,
            y: 20.0,
            velocity_x: 2.0,
            velocity_y: 2.0,
            state: PersonState::Susceptible,
            infection_duration: 0.0,
        };

        person.update_position();

        assert_eq!(person.x, 152.0);
        assert_eq!(person.y, 22.0);
    }

    #[test]
    fn test_update_position_right_border() {
        let mut person = Person {
            x: SIMULATION_AREA_SIZE - MARGIN_FROM_WALL - 1.0,
            y: 10.0,
            velocity_x: 2.0,
            velocity_y: 2.0,
            state: PersonState::Susceptible,
            infection_duration: 0.0,
        };

        person.update_position();

        assert_eq!(person.x, SIMULATION_AREA_SIZE - MARGIN_FROM_WALL);
        assert_eq!(person.velocity_x, -2.0);
        assert_eq!(person.velocity_y, 2.0);
    }

    #[test]
    fn test_update_position_left_border() {
        let mut person = Person {
            x: 7.0,
            y: 100.0,
            velocity_x: -2.0,
            velocity_y: 2.0,
            state: PersonState::Susceptible,
            infection_duration: 0.0,
        };

        person.update_position();

        assert_eq!(person.x, MARGIN_FROM_WALL);
        assert_eq!(person.velocity_x, 2.0);
        assert_eq!(person.velocity_y, 2.0);
    }

    #[test]
    fn test_update_position_bottom_border() {
        let mut person = Person {
            x: 200.0,
            y: SIMULATION_AREA_SIZE - MARGIN_FROM_WALL - 1.0,
            velocity_x: 2.0,
            velocity_y: 2.0,
            state: PersonState::Susceptible,
            infection_duration: 0.0,
        };

        person.update_position();

        assert_eq!(person.y, SIMULATION_AREA_SIZE - MARGIN_FROM_WALL);
        assert_eq!(person.velocity_x, 2.0);
        assert_eq!(person.velocity_y, -2.0);
    }

    #[test]
    fn test_update_position_top_border() {
        let mut person = Person {
            x: 200.0,
            y: 7.0,
            velocity_x: 2.0,
            velocity_y: -2.0,
            state: PersonState::Susceptible,
            infection_duration: 0.0,
        };

        person.update_position();

        assert_eq!(person.y, MARGIN_FROM_WALL);
        assert_eq!(person.velocity_x, 2.0);
        assert_eq!(person.velocity_y, 2.0);
    }
}
