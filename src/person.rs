use eframe::egui;
use rand::Rng;

use crate::settings::*;

#[derive(PartialEq)]
pub enum PersonState {
    Susceptible,
    Infected,
    Recovered,
}


impl PersonState {
    pub fn person_colors(&self) -> egui::Color32 {
        match self {
            PersonState::Infected => egui::Color32::RED,
            PersonState::Recovered => egui::Color32::GRAY,
            PersonState::Susceptible => egui::Color32::BLUE,
        }
    }
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

    pub fn is_susceptible(&self) -> bool {
        matches!(self.state, PersonState::Susceptible)
    }

    pub fn is_infected(&self) -> bool {
        matches!(self.state, PersonState::Infected)
    }

    pub fn calculate_distance(&self, other: &Person) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::*;

    /// Tests that creating a new person places them within the SIMULATION_AREA_SIZE.
    #[test]
    fn test_create_new_person() {
        let person = Person::new();
        assert!(matches!(person.state, PersonState::Susceptible));
        assert_eq!(person.infection_duration, 0.0);
        assert!(person.x >= 0.0 && person.x <= SIMULATION_AREA_SIZE);
        assert!(person.y >= 0.0 && person.y <= SIMULATION_AREA_SIZE);
    }

    /// Tests that the person's position is correct after moving in the community.
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

    /// Tests that a person bounces back and changes velocity x when moving to the left border.
    /// The right, top and bottom cases use the same principle as the left border case.
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

    /// Tests the distance calculation between two people in the community.
    #[test]
    fn test_calculate_distance() {
        let person1 = Person {
            x: 10.0,
            y: 20.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            state: PersonState::Susceptible,
            infection_duration: 0.0,
        };
        let person2 = Person {
            x: 10.0,
            y: 24.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            state: PersonState::Susceptible,
            infection_duration: 0.0,
        };
        let distance = person1.calculate_distance(&person2);
        assert_eq!(distance, 4.0);
    }

    /// Tests the function that checks if a person is susceptible.
    #[test]
    fn test_is_susceptible() {
        let person = Person {
            x: 17.0,
            y: 23.0,
            velocity_x: 2.0,
            velocity_y: 2.0,
            state: PersonState::Susceptible,
            infection_duration: 0.0,
        };
        assert!(person.is_susceptible());
    }

    /// Tests the function that checks if a person is infected.
    #[test]
    fn test_is_infected() {
        let person = Person {
            x: 17.0,
            y: 23.0,
            velocity_x: 2.0,
            velocity_y: 2.0,
            state: PersonState::Infected,
            infection_duration: 0.0,
        };
        assert!(person.is_infected());
    }

    /// Tests that the person state matches the corresponding color in the UI.
    #[test]
    fn test_person_colors() {
        use eframe::egui;
        assert_eq!(PersonState::Infected.person_colors(), egui::Color32::RED);
        assert_eq!(PersonState::Recovered.person_colors(), egui::Color32::GRAY);
        assert_eq!(
            PersonState::Susceptible.person_colors(),
            egui::Color32::BLUE
        );
    }
}
