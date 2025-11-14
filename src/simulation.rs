use eframe::egui;
use rand::Rng;

use crate::person::{Person, PersonState};
use crate::settings::*;

pub struct App {
    pub community: Vec<Person>,
    pub total_time: f32,
    pub community_size: usize,
}

impl App {
    pub fn new() -> Self {
        let community_size = 100;
        let community: Vec<Person> = (0..community_size).map(|_| Person::new()).collect();

        Self {
            community,
            total_time: 0.0,
            community_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::person::{Person, PersonState};
    use crate::settings::*;

    #[test]
    fn test_create_new_app() {
        let app = App::new();
        assert_eq!(app.total_time, 0.0);
        assert_eq!(app.community_size, 100);
        assert_eq!(app.community.len(), 100);
        for person in &app.community {
            assert!(matches!(person.state, PersonState::Susceptible));
            assert_eq!(person.infection_duration, 0.0);
        }
    }
}
