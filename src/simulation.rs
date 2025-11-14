use eframe::egui;
use rand::Rng;

use crate::person::{Person, PersonState};

pub struct App {
    community: Vec<Person>,
    width: f32,
    height: f32,
}


impl App {
    pub fn new() -> Self {
        let mut community: Vec<Person> = (0..100)
        .map(|| Person::)
    }
}