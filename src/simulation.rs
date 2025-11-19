use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use rand::Rng;

use crate::person::{Person, PersonState};
use crate::settings::*;

pub struct Simulation {
    pub community: Vec<Person>,
    pub total_time: Vec<f32>,
    pub community_size: usize,
    pub initial_infected_count: usize,
    pub recovered_days: f32,
    pub infected_radius: f32,
    pub ui_infected_radius: f32,
    pub infected_chart: Vec<f32>,
}

impl Simulation {
    pub fn new() -> Self {
        let community_size = 80;
        let mut community: Vec<Person> = (0..community_size).map(|_| Person::new()).collect();
        for person in community.iter_mut().take(INITIAL_INFECTED_PEOPLE) {
            person.state = PersonState::Infected;
            person.infection_duration = 0.0;
        }
        let mut total_time = Vec::new();
        let mut infected_chart = Vec::new();

        total_time.push(0.0);
        infected_chart.push((INITIAL_INFECTED_PEOPLE as f32 / community_size as f32) * 100.0);
        Self {
            community,
            total_time,
            community_size,
            initial_infected_count: INITIAL_INFECTED_PEOPLE,
            recovered_days: 7.0,
            infected_radius: 3.0,
            ui_infected_radius: 3.0,
            infected_chart,
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
                if distance <= self.infected_radius {
                    return true;
                }
            }
        }
        false
    }

    fn restart(&mut self) {
        self.community = (0..self.community_size).map(|_| Person::new()).collect();
        for i in 0..self.initial_infected_count {
            self.community[i].state = PersonState::Infected;
        }

        self.total_time.clear();
        self.total_time.push(0.0);
    }

    fn update_chart(&mut self, time_frame_per_second: f32) {
        let total_people = self.community.len() as f32;
        if let Some(&last) = self.total_time.last() {
            self.total_time.push(last + time_frame_per_second);
        }

        let current_infected = self
            .community
            .iter()
            .filter(|p| matches!(p.state, PersonState::Infected))
            .count() as f32;
        self.infected_chart
            .push((current_infected / total_people) * 100.0);
    }
}

impl eframe::App for Simulation {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let time_frame_per_second: f32 = ctx.input(|i| i.stable_dt);
        self.update_community(time_frame_per_second);
        self.update_chart(time_frame_per_second);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Community size:").size(15.0));
                        ui.add(egui::Slider::new(&mut self.community_size, 20..=150));
                    });

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Initial Infected:").size(15.0));
                        ui.add(egui::Slider::new(&mut self.initial_infected_count, 1..=30));
                    });

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Infected Radius:").size(15.0));
                        ui.add(egui::Slider::new(&mut self.ui_infected_radius, 1.0..=16.0));
                    });
                });

                let reset_button = ui.button(egui::RichText::new("Apply and Reset").size(15.0));
                if reset_button.clicked() {
                    self.restart();
                }
            });
            ui.separator();
            if !self.total_time.is_empty() {
                Plot::new("SIR chart")
                    .height(350.0)
                    .x_axis_label("Time")
                    .y_axis_label("Percentage")
                    .include_y(0.0)
                    .include_y(100.0)
                    .legend(
                        egui_plot::Legend::default()
                            .position(egui_plot::Corner::RightTop)
                            .background_alpha(0.8),
                    )
                    .show(ui, |plot_ui| {

                        // Infected chart
                        let last_infected_percentage = match self.infected_chart.last() {
                            Some(&value) => value,
                            None => 0.0,
                        };
                        let infected_points: PlotPoints = self
                            .total_time
                            .iter()
                            .zip(self.infected_chart.iter())
                            .map(|(t, i)| [*t as f64, *i as f64])
                            .collect();

                        plot_ui.line(
                            Line::new(infected_points)
                                .color(egui::Color32::RED)
                                .name(format!("{:.1}% infected", last_infected_percentage))
                                .fill(0.0),
                        );
                    });
            }

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
        assert_eq!(app.community_size, 80);
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
