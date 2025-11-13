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
