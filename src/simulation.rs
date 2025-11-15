use eframe::egui;
use rand::Rng;

use crate::person::{Person, PersonState};
use crate::settings::*;

pub struct Simulation {
    pub community: Vec<Person>,
    pub total_time: f32,
    pub community_size: usize,
}

impl Simulation {
    pub fn new() -> Self {
        let community_size = 100;
        let mut community: Vec<Person> = (0..community_size).map(|_| Person::new()).collect();

        for i in 0..INITIAL_INFECTED_PEOPLE as usize {
            community[i].state = PersonState::Infected;
        }

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

impl eframe::App for Simulation {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_community();

        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(
                egui::vec2(SIMULATION_AREA_SIZE + 80.0, SIMULATION_AREA_SIZE + 80.0),
                egui::Sense::hover(),
            );

            let rect = response.rect;
            let border_offset_x = rect.left() + 40.0;
            let border_offset_y = rect.top() + 40.0;
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
                painter.circle_filled(particle_pos, 4.0, egui::Color32::BLUE);
            }
        });

        ctx.request_repaint();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::person::{Person, PersonState};
    use crate::settings::*;

    // check create new app with default INITIAL_INFECTED_PEOPLE
    #[test]
    fn test_create_new_app() {
        let app = Simulation::new();
        assert_eq!(app.total_time, 0.0);
        assert_eq!(app.community_size, 100);
        assert_eq!(app.community.len(), 100);

        let infected = app.community
            .iter()
            .filter(|p| matches!(p.state, PersonState::Infected))
            .count();

        assert_eq!(infected, INITIAL_INFECTED_PEOPLE);
        
        for person in &app.community {
            assert_eq!(person.infection_duration, 0.0);
        }
    }

    /// Check that position should change. If position does not change, the velocity must change due to a corner case.
    #[test]
    fn test_update_community_first_person() {
        let mut app = Simulation::new();
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
