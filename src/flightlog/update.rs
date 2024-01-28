//! Updates: little piece of information sent to change just a part of a FlightLog.
//! Useful when changing just the takeoff or the pilot of a flight without reloading 
//! entire flightlog's informations.

use chrono::{NaiveDate, NaiveTime};
use serde::{Serialize, Deserialize};

/// Memory object of a FlightLog update.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Update {
    /// The flight number in OGN day's fligths.
    pub ogn_nb: i32,
    /// The name of the field that was changed.
    pub updated_field: String,
    /// The new value of this champs.
    pub new_value: String,
    /// The date on which the modified flights happened.
    pub date: NaiveDate,
    /// The time the update has been made: useful for tracking the latests ones.
    pub time: NaiveTime,
}

impl Default for Update {
    fn default() -> Self {
        Self::new()
    }
}

impl Update {
    /// A new update using type's default values.
    pub fn new() -> Self {
        Update {
            ogn_nb: i32::default(), 
            updated_field: String::default(),
            new_value: String::default(),
            date: NaiveDate::default(),
            time: NaiveTime::default(),
        }
    }
}

/// Take care of too in the passt updates. Mainly to remove ones that are
/// for more than some time ago.
pub trait ObsoleteUpdates {
    /// To remove updates that dates from more than time.
    fn remove_obsolete_updates(&mut self, time: chrono::Duration);
}

impl ObsoleteUpdates for Vec<Update> {
    fn remove_obsolete_updates(&mut self, time: chrono::Duration) {
        let current_time = chrono::Local::now().time();
        let mut i = 0;
        while i < self.len() {
            if (current_time - self[i].time) > time {
                self.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
