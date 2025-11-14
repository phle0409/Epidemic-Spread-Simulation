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
