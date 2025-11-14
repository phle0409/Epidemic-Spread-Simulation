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

    fn update_community(&mut self) {
        for person in &mut self.community {
            person.update_position();
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

    /// Check that position should change. If position does not change, the velocity must change due to a corner case.
    #[test]
    fn test_update_community_first_person() {
        let mut app = App::new();
        let initial_x = app.community[0].x;
        let initial_y = app.community[0].y;
        let initial_velocity_x = app.community[0].velocity_x;
        let initial_velocity_y = app.community[0].velocity_y;

        app.update_community();

        let new_position_x = app.community[0].x;
        let new_position_y = app.community[0].y;
        let new_velocity_x = app.community[0].velocity_x;
        let new_velocity_y = app.community[0].velocity_y;

        assert!(
            new_position_x != initial_x
                || new_position_y != initial_y
                || new_velocity_x != initial_velocity_x
                || new_velocity_y != initial_velocity_y
        );
    }
}
