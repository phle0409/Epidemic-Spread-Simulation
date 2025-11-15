// # Simulation Settings
//!
//! Global configuration constants for the epidemic spread simulation.

/// Size of the simulation area in pixels for a community.
///
/// It represents the height and width border of a community.
/// # Default Value
/// `350.0` pixels (creating a 350x350 simulation area)
pub const SIMULATION_AREA_SIZE: f32 = 350.0;

/// Movement speed for all people in the simulation.
///
/// it represents how many pixels per frame that all people move.
///
/// # Default Value
/// `1.5` pixels per frame
pub const MOVING_SPEED: f32 = 1.25;


/// Border padding around a community simulation area.
///
/// # Default Value
/// `40.0` pixels
pub const BORDER_PADDING: f32 = 40.0;

/// The radius of a person in the simulation.
///
/// # Default Value
/// `4.0` pixels
pub const PERSON_RADIUS: f32 = 4.0;

/// Creates a margin for better visibility by preventing people from getting too close to the border.
///
/// # Default Value
/// `10.0` pixels
pub const MARGIN_FROM_WALL: f32 = 10.0;

/// Number of people who are infected when starting the simulation in a community.
///
/// # Default Value
/// `3` infected people
pub const INITIAL_INFECTED_PEOPLE: usize = 3; 

