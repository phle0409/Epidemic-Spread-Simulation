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
    pub susceptible_chart: Vec<f32>,
    pub recovered_chart: Vec<f32>,
    pub social_distancing_radius: f32,
    pub social_distancing_enabled: bool,
    pub quarantine_enabled: bool,
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
        let mut susceptible_chart = Vec::new();
        let mut recovered_chart = Vec::new();

        total_time.push(0.0);
        infected_chart.push((INITIAL_INFECTED_PEOPLE as f32 / community_size as f32) * 100.0);
        susceptible_chart.push(
            ((community_size - INITIAL_INFECTED_PEOPLE) as f32 / community_size as f32) * 100.0,
        );
        recovered_chart.push(0.0);
        Self {
            community,
            total_time,
            community_size,
            initial_infected_count: INITIAL_INFECTED_PEOPLE,
            recovered_days: 7.0,
            infected_radius: 3.5,
            ui_infected_radius: 3.5,
            infected_chart,
            susceptible_chart,
            recovered_chart,
            social_distancing_radius: 20.0,
            social_distancing_enabled: false,
            quarantine_enabled: false,
        }
    }

    fn update_community(&mut self, time_frame_per_second: f32) {
        self.move_infected_to_quarantine();

        if self.social_distancing_enabled {
            let mut forces = Vec::new();
            for i in 0..self.community.len() {
                forces.push(self.calculate_social_distancing_force(i));
            }
            self.apply_forces(forces);
        }

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
        self.infected_radius = self.ui_infected_radius;
        self.community = (0..self.community_size).map(|_| Person::new()).collect();
        for i in 0..self.initial_infected_count {
            self.community[i].state = PersonState::Infected;
            self.community[i].infection_duration = 0.0;
        }

        self.total_time.clear();
        self.infected_chart.clear();
        self.susceptible_chart.clear();
        self.recovered_chart.clear();
        self.total_time.push(0.0);

        self.infected_chart
            .push((self.initial_infected_count as f32 / self.community_size as f32) * 100.0);
        self.susceptible_chart.push(
            ((self.community_size - self.initial_infected_count) as f32
                / self.community_size as f32)
                * 100.0,
        );
        self.recovered_chart.push(0.0);
    }

    fn update_chart(&mut self, time_frame_per_second: f32) {
        let total_people = self.community.len() as f32;
        if let Some(&last) = self.total_time.last() {
            self.total_time.push(last + time_frame_per_second);
        }

        let current_infected = self
            .community
            .iter()
            .filter(|person| matches!(person.state, PersonState::Infected))
            .count() as f32;
        self.infected_chart
            .push((current_infected / total_people) * 100.0);

        let current_susceptible = self
            .community
            .iter()
            .filter(|person| matches!(person.state, PersonState::Susceptible))
            .count() as f32;
        self.susceptible_chart
            .push((current_susceptible / total_people) * 100.0);

        let current_recovered = self
            .community
            .iter()
            .filter(|person| matches!(person.state, PersonState::Recovered))
            .count() as f32;
        self.recovered_chart
            .push((current_recovered / total_people) * 100.0);
    }

    fn calculate_social_distancing_force(&self, own_index: usize) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        let person = &self.community[own_index];
        for (index, other) in self.community.iter().enumerate() {
            if index == own_index {
                continue;
            }
            let distance = person.calculate_distance(other);
            if distance < self.social_distancing_radius && distance > 0.0 {
                let dx = person.x - other.x;
                let dy = person.y - other.y;
                let strength =
                    (self.social_distancing_radius - distance) / self.social_distancing_radius;
                x += (dx / distance) * strength;
                y += (dy / distance) * strength;
            }
        }
        (x, y)
    }

    fn apply_forces(&mut self, forces: Vec<(f32, f32)>) {
        for (person, (fx, fy)) in self.community.iter_mut().zip(forces.iter()) {
            person.velocity_x += fx * 0.25;
            person.velocity_y += fy * 0.25;
            let speed = (person.velocity_x * person.velocity_x
                + person.velocity_y * person.velocity_y)
                .sqrt();
            if speed > SOCIAL_DISTANCING_MAX_SPEED {
                person.velocity_x = (person.velocity_x / speed) * SOCIAL_DISTANCING_MAX_SPEED;
                person.velocity_y = (person.velocity_y / speed) * SOCIAL_DISTANCING_MAX_SPEED;
            }
        }
    }

    fn move_infected_to_quarantine(&mut self) {
        if !self.quarantine_enabled {
            return;
        }
        let mut rng = rand::thread_rng();

        for person in &mut self.community {
            if person.state == PersonState::Infected {
                person.x =
                    rng.gen_range(MARGIN_FROM_WALL..(QUARANTINE_AREA_SIZE - MARGIN_FROM_WALL));
                person.y =
                    rng.gen_range(MARGIN_FROM_WALL..(QUARANTINE_AREA_SIZE - MARGIN_FROM_WALL));
                person.move_to_quarantine();
            }
        }
    }
}

impl eframe::App for Simulation {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let time_frame_per_second: f32 = ctx.input(|i| i.stable_dt);
        self.update_community(time_frame_per_second);
        self.update_chart(time_frame_per_second);
        egui::CentralPanel::default().show(ctx, |ui| {
            // Basic Settings section
            ui.label(egui::RichText::new("Basic Settings").size(18.0).strong());
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
            // Prevention section
            ui.label(
                egui::RichText::new("Prevention Methods")
                    .size(18.0)
                    .strong(),
            );

            // social distancing
            ui.label(
                egui::RichText::new("Social Distancing")
                    .size(16.0)
                    .underline(),
            );

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Enable:").size(15.0));
                ui.checkbox(&mut self.social_distancing_enabled, "");
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Radius:").size(15.0));
                ui.add_enabled(
                    self.social_distancing_enabled,
                    egui::Slider::new(&mut self.social_distancing_radius, 0.0..=50.0),
                );
            });

            // quarantine
            ui.label(egui::RichText::new("Quarantine").size(16.0).underline());

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Enable:").size(15.0));
                ui.checkbox(&mut self.quarantine_enabled, "");
            });

            ui.separator();
            // Chart
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
                            Some(&percentage) => percentage,
                            None => 0.0,
                        };
                        let infected_points: PlotPoints = (0..self.total_time.len())
                            .map(|i| {
                                let time = self.total_time[i];
                                let infected_percentage = self.infected_chart[i];
                                [time as f64, infected_percentage as f64]
                            })
                            .collect();

                        plot_ui.line(
                            Line::new(infected_points)
                                .color(egui::Color32::RED)
                                .name(format!("{:.1}% infected", last_infected_percentage))
                                .fill(0.0),
                        );

                        // Susceptible chart
                        let last_susceptible_percentage = match self.susceptible_chart.last() {
                            Some(&percentage) => percentage,
                            None => 0.0,
                        };
                        let susceptible_points: PlotPoints = (0..self.total_time.len())
                            .map(|i| {
                                let time = self.total_time[i];
                                let susceptible_percentage = self.susceptible_chart[i];
                                let infected_percentage = self.infected_chart[i];
                                [
                                    time as f64,
                                    (infected_percentage + susceptible_percentage) as f64,
                                ]
                            })
                            .collect();

                        plot_ui.line(
                            Line::new(susceptible_points)
                                .color(egui::Color32::BLUE)
                                .name(format!("{:.1}% Susceptible", last_susceptible_percentage))
                                .fill(0.0),
                        );

                        // Recovered chart
                        let last_recovered_percentage = match self.recovered_chart.last() {
                            Some(&percentage) => percentage,
                            None => 0.0,
                        };
                        let recovered_points: PlotPoints = (0..self.total_time.len())
                            .map(|i| {
                                let time = self.total_time[i];
                                let recovered_percentage = self.recovered_chart[i];
                                let susceptible_percentage = self.susceptible_chart[i];
                                let infected_percentage = self.infected_chart[i];
                                [
                                    time as f64,
                                    (infected_percentage
                                        + susceptible_percentage
                                        + recovered_percentage)
                                        as f64,
                                ]
                            })
                            .collect();

                        plot_ui.line(
                            Line::new(recovered_points)
                                .color(egui::Color32::GRAY)
                                .name(format!("{:.1}% Recovered", last_recovered_percentage))
                                .fill(0.0),
                        );
                    });
            }

            ui.separator();
            ui.heading("Community Simulation");
            let padding = 80.0;
            let width =
                SIMULATION_AREA_SIZE + GAP_COMMUNITY_QUARANTINE + QUARANTINE_AREA_SIZE + padding;
            let height = SIMULATION_AREA_SIZE + padding;

            let (response, painter) =
                ui.allocate_painter(egui::vec2(width, height), egui::Sense::hover());

            let rect = response.rect;
            let border_offset_x = rect.left() + BORDER_PADDING;
            let border_offset_y = rect.top() + BORDER_PADDING;

            ui.visuals_mut().panel_fill = egui::Color32::BLACK;

            // Community
            let main_rect = egui::Rect::from_min_size(
                egui::pos2(border_offset_x, border_offset_y),
                egui::vec2(SIMULATION_AREA_SIZE, SIMULATION_AREA_SIZE),
            );
            painter.rect_filled(main_rect, 0.0, egui::Color32::BLACK);
            painter.rect_stroke(main_rect, 0.0, egui::Stroke::new(3.0, egui::Color32::WHITE));

            painter.text(
                egui::pos2(border_offset_x, border_offset_y - 20.0),
                egui::Align2::LEFT_CENTER,
                "Community",
                egui::FontId::proportional(15.0),
                egui::Color32::WHITE,
            );

            // quarantine
            let quarantine_offset_x =
                border_offset_x + SIMULATION_AREA_SIZE + GAP_COMMUNITY_QUARANTINE;
            let quarantine_rect = egui::Rect::from_min_size(
                egui::pos2(quarantine_offset_x, border_offset_y),
                egui::vec2(QUARANTINE_AREA_SIZE, QUARANTINE_AREA_SIZE),
            );
            painter.rect_filled(quarantine_rect, 0.0, egui::Color32::BLACK);
            painter.rect_stroke(
                quarantine_rect,
                0.0,
                egui::Stroke::new(3.0, egui::Color32::WHITE),
            );

            painter.text(
                egui::pos2(quarantine_offset_x, border_offset_y - 20.0),
                egui::Align2::LEFT_CENTER,
                "Quarantine Zone",
                egui::FontId::proportional(15.0),
                egui::Color32::WHITE,
            );

            // people
            for person in &self.community {
                let dot_postion = if person.is_in_quarantine {
                    egui::pos2(quarantine_offset_x + person.x, border_offset_y + person.y)
                } else {
                    egui::pos2(border_offset_x + person.x, border_offset_y + person.y)
                };
                painter.circle_filled(dot_postion, PERSON_RADIUS, person.state.person_colors());
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
        assert_eq!(app.total_time[0], 0.0);
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
        app.community_size = 80;
        app.ui_infected_radius = 6.0;
        app.restart();
        let count = app
            .community
            .iter()
            .filter(|person| matches!(person.state, PersonState::Infected))
            .count();
        assert_eq!(count, 5);
        assert_eq!(app.community.len(), 80);
        assert_eq!(app.infected_radius, 6.0);
        assert_eq!(app.recovered_chart[0], 0.0);
        let first_infected_percentage = (5.0 / 80.0) * 100.0;
        let first_susceptible_percentage = (75.0 / 80.0) * 100.0;
        assert_eq!(app.infected_chart[0], first_infected_percentage);
        assert_eq!(app.susceptible_chart[0], first_susceptible_percentage);
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

    /// Tests that the social distancing force calculation pushes people away from each other.
    #[test]
    fn test_calculate_social_distancing_force() {
        let mut app = Simulation::new();
        app.community.clear();
        app.social_distancing_radius = 50.0;
        app.community.push(Person {
            x: 50.0,
            y: 50.0,
            velocity_x: 1.0,
            velocity_y: 1.0,
            state: PersonState::Susceptible,
            infection_duration: 0.0,
        });
        app.community.push(Person {
            x: 70.0,
            y: 30.0,
            velocity_x: 1.0,
            velocity_y: 1.0,
            state: PersonState::Susceptible,
            infection_duration: 0.0,
        });
        let (fx, fy) = app.calculate_social_distancing_force(0);
        assert!(fx < 0.0);
        assert!(fy > 0.0);
    }

    /// Tests that forces are correctly applied to person velocities.
    #[test]
    fn test_apply_forces() {
        let mut app = Simulation::new();
        app.community.clear();
        app.community.push(Person {
            x: 50.0,
            y: 50.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            state: PersonState::Susceptible,
            infection_duration: 0.0,
        });
        let forces = vec![(1.0, -1.0)];
        app.apply_forces(forces);
        assert_eq!(app.community[0].velocity_x, 0.25);
        assert_eq!(app.community[0].velocity_y, -0.25);
    }
}
