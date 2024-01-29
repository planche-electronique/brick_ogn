//! This module contains every struct and functions that could be useful
//! about flight.



use chrono::NaiveTime;

/// Memory object of a flight. Different codes can be defined.
#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Flight {
    /// The flight number in ogn's list of today's flights.
    pub ogn_nb: i32,
    /// Takeoff code in order to separate winch and towing or other launch method.
    pub takeoff_code: String,
    /// The machine that helped to takeoff (either a towplane immatriculation 
    /// or a codename for a winch).
    pub takeoff_machine: String,
    /// The operator of the launch machine.
    pub takeoff_machine_pilot: String,
    /// Gider immatriculation in no specific length requirement.
    pub glider: String,
    /// To tell to your club if it was a school flight, a mutual (shared cost
    /// flight). Its main goal is for your club to claim you more efficiently
    /// the money you owe it.
    pub flight_code: String,
    /// The name of the pilot in command or the student (The guy in the front
    /// seat of the glider).
    pub pilot1: String,
    /// The name of the passenger or the flight instructor (back seat of the glider).
    pub pilot2: String,
    /// Takeoff time.
    pub takeoff: NaiveTime,
    /// The time of the landing.
    pub landing: NaiveTime,
}

impl Default for Flight {
    fn default() -> Self {
        Flight {
            ogn_nb: 1,
            takeoff_code: String::default(),
            takeoff_machine: String::default(),
            takeoff_machine_pilot: String::default(),
            glider: String::default(),
            flight_code: String::default(),
            pilot1: String::default(),
            pilot2: String::default(),
            takeoff: NaiveTime::default(),
            landing: NaiveTime::default(),
        }
    }
}

impl Flight {
    fn _new() -> Self {
        Self::default()
    }
}