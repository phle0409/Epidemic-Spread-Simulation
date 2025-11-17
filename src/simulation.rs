use eframe::egui;
use rand::Rng;

use crate::person::{Person, PersonState};
use crate::settings::*;

pub struct Simulation {
    pub community: Vec<Person>,
    pub total_time: f32,
    pub community_size: usize,
    pub initial_infected_count: usize,
    pub recovered_days: f32,
}

impl Simulation {
    pub fn new() -> Self {
        let community_size = 100;
        let mut community: Vec<Person> = (0..community_size).map(|_| Person::new()).collect();

        for index in 0..INITIAL_INFECTED_PEOPLE {
            community[index].state = PersonState::Infected;
            community[index].infection_duration = 0.0;
        }

        Self {
            community,
            total_time: 0.0,
            community_size,
            initial_infected_count: INITIAL_INFECTED_PEOPLE,
            recovered_days: 7.0,
        }
    }

    fn update_community(&mut self, time_frame_per_second: f32) {
        for person in &mut self.community {
            if person.state == PersonState::Infected {
                person.infection_duration += time_frame_per_second;
                if person.infection_duration >= self.recovered_days {
                    person.infection_duration = 0.0;
                    person.state = PersonState::Recovered;
                }
            }

            person.update_position();
        }
        self.spread_infection();
    }

    fn spread_infection(&mut self) {
        let mut rng = rand::thread_rng();
        let susceptibles = self.find_vulnerable_people();
        for index in susceptibles {
            let random = rng.gen_range(0.0..1.0);
            if random < INFECTION_PROBABILITY {
                self.community[index].state = PersonState::Infected;
                self.community[index].infection_duration = 0.0;
            }
        }
    }

    fn find_vulnerable_people(&self) -> Vec<usize> {
        let mut vulnerable_people = Vec::new();
        for (index, person) in self.community.iter().enumerate() {
            if !person.is_susceptible() {
                continue;
            }
            if self.is_within_infected_radius(person) {
                vulnerable_people.push(index);
            }
        }
        vulnerable_people
    }

    fn is_within_infected_radius(&self, person: &Person) -> bool {
        for member in &self.community {
            if member.is_infected() {
                let distance = person.calculate_distance(member);
                if distance <= INFECTION_RADIUS {
                    return true;
                }
            }
        }
        false
    }

    fn restart(&mut self) {
        let count = self.initial_infected_count;
        self.community = (0..self.community_size).map(|_| Person::new()).collect();
        for i in 0..count {
            self.community[i].state = PersonState::Infected;
        }
        self.total_time = 0.0;
    }
}

impl eframe::App for Simulation {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let time_frame_per_second: f32 = ctx.input(|i| i.stable_dt);
        self.update_community(time_frame_per_second);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Initial Infected:").size(15.0));
                ui.add(egui::Slider::new(&mut self.initial_infected_count, 3..=30));
                if ui
                    .button(egui::RichText::new("Apply and Reset").size(15.0))
                    .clicked()
                {
                    self.restart();
                }
            });
            ui.separator();
            let (response, painter) = ui.allocate_painter(
                egui::vec2(SIMULATION_AREA_SIZE + 80.0, SIMULATION_AREA_SIZE + 80.0),
                egui::Sense::hover(),
            );
            let rect = response.rect;
            let border_offset_x = rect.left() + BORDER_PADDING;
            let border_offset_y = rect.top() + BORDER_PADDING;
            ui.visuals_mut().panel_fill = egui::Color32::BLACK;
            let rect = egui::Rect::from_min_size(
                egui::pos2(border_offset_x, border_offset_y),
                egui::vec2(SIMULATION_AREA_SIZE, SIMULATION_AREA_SIZE),
            );
            painter.rect_filled(rect, 0.0, egui::Color32::BLACK);
            painter.rect_stroke(rect, 0.0, egui::Stroke::new(3.0, egui::Color32::WHITE));
            for person in &self.community {
                let particle_pos =
                    egui::pos2(border_offset_x + person.x, border_offset_y + person.y);
                painter.circle_filled(particle_pos, PERSON_RADIUS, person.state.person_colors());
            }
        });
        ctx.request_repaint();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::person::PersonState;

    /// Tests that a new app is created with default INITIAL_INFECTED_PEOPLE
    #[test]
    fn test_create_new_app() {
        let app = Simulation::new();
        assert_eq!(app.total_time, 0.0);
        assert_eq!(app.community_size, 100);
        assert_eq!(app.community.len(), 100);
        let infected = app
            .community
            .iter()
            .filter(|p| matches!(p.state, PersonState::Infected))
            .count();
        assert_eq!(infected, INITIAL_INFECTED_PEOPLE);
    }

    /// Tests that a person's position in the community should change after update.
    ///  If the position does not change, then the velocity must change due to a corner case.
    #[test]
    fn test_update_community_first_person_position() {
        let mut app = Simulation::new();
        let initial_x = app.community[0].x;
        let initial_y = app.community[0].y;
        let initial_velocity_x = app.community[0].velocity_x;
        let initial_velocity_y = app.community[0].velocity_y;
        app.update_community(1.0);
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

    /// Tests the restart method with new infected people
    /// In the UI, user can adjust how many initial people that are infected
    #[test]
    fn test_restart_with_new_infected_people() {
        let mut app = Simulation::new();
        app.initial_infected_count = 5;
        app.restart();
        let count = app
            .community
            .iter()
            .filter(|person| matches!(person.state, PersonState::Infected))
            .count();
        assert_eq!(count, 5);
    }

    /// Tests that true is returned when a normal person is within the radius of an infected person.
    /// Creates an infected person in the same radius as a normal person to ensure the function returns true.
    #[test]
    fn test_is_within_infected_radius_true() {
        let mut app = Simulation::new();
        app.community[0].x = 20.0;
        app.community[0].y = 20.0;
        app.community[1].state = PersonState::Susceptible;
        app.community[1].x = 21.0;
        app.community[1].y = 22.0;

        assert!(app.is_within_infected_radius(&app.community[1]));
    }

    /// Tests that false is returned when no one in the community is infected.
    /// Creates a new community with no infected people to ensure the function returns false.
    #[test]
    fn test_is_within_infected_radius_false() {
        let mut app = Simulation::new();
        app.community[0].x = 20.0;
        app.community[0].y = 20.0;
        for index in 1..3 {
            app.community[index].state = PersonState::Susceptible;
        }
        app.community[0].x = 80.0;
        app.community[0].y = 80.0;

        assert!(!app.is_within_infected_radius(&app.community[1]));
    }

    /// Tests that vulnerable people are correctly identified in the community.
    #[test]
    fn test_find_vulnerable_people() {
        let mut app = Simulation::new();
        app.community[0].x = 20.0;
        app.community[0].y = 20.0;
        app.community[1].state = PersonState::Susceptible;
        app.community[1].x = 21.0;
        app.community[1].y = 22.0;

        let vulnerable = app.find_vulnerable_people();
        assert!(vulnerable.contains(&1));
    }

    /// Tests that a infected person will be recorved after 7 days
    #[test]
    fn test_community_update_recovered_person() {
        let mut app = Simulation::new();
        app.update_community(6.0);
        assert!(matches!(app.community[0].state, PersonState::Infected));
        app.update_community(1.0);
        assert!(matches!(app.community[0].state, PersonState::Recovered));
    }
}
