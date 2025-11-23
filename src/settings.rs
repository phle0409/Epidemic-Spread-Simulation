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
/// `1.25` pixels per frame
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

/// Probability of infection
///
/// # Default Value
/// `0.3` (30% chance) range from 0.0 <= random < 0.1
pub const INFECTION_PROBABILITY: f32 = 0.3;

/// Maximum speed to prevent people from moving too fast when social distancing is enabled.
///
/// # Default Value
/// `2.5` pixels per frame
pub const SOCIAL_DISTANCING_MAX_SPEED: f32 = 0.75;

/// Size of the quarantine area in pixels for quarantine zone.
///
/// # Default Value
/// `200.0` pixels (creating a 200x200  area)
pub const QUARANTINE_AREA_SIZE: f32 = 200.0;

/// The gap between the community area and the quarantine zone
///
/// # Default Value
/// `40.0` pixels
pub const GAP_COMMUNITY_QUARANTINE: f32 = 40.0;



pub const RECOVERED_DAY: f32 = 8.0;