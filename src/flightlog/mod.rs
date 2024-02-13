//! FlightLog module i.e. the module in which we define how to store the
//! informations about flights of a day and also the ground operations. For 
//! operations, we store data on organisation on the ground.
pub mod update;

use crate::flight::Flight;
use chrono::{NaiveDate, NaiveTime};
use log;

/// It is the struct associated with the flights of a day and what is 
/// currently happenning for the ground organisation
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FlightLog {
    /// Every flight of a day
    pub flights: Vec<Flight>,
    pub date: NaiveDate,
    /// Winch pilot, comprising name and surname.
    pub winch_pilot: String, 
    /// The winch that is in-use
    pub winch: String,
    /// The pilot that is in the winch
    pub tow_pilot: String,
    /// The in-use tow-plane
    pub tow_plane: String,
    /// The chief of the ground operations
    pub field_chief: String,
}

impl Default for FlightLog {
    fn default() -> Self {
        Self::new()
    }
}

impl FlightLog {
    /// A new Flightlog using default and new.
    pub fn new() -> Self {
        FlightLog {
            flights: Vec::new(),
            date: NaiveDate::default(),
            winch_pilot: String::new(),
            winch: String::new(),
            tow_pilot: String::new(),
            tow_plane: String::new(),
            field_chief: String::new(),
        }
    }

    pub fn new_date(date: NaiveDate) -> Self {
        FlightLog {
            flights: Vec::new(),
            date,
            winch_pilot: String::new(),
            winch: String::new(),
            tow_pilot: String::new(),
            tow_plane: String::new(),
            field_chief: String::new(),
        }
    }

    pub fn update(&mut self, update: update::Update) {
        let mut flights = self.flights.clone();
        if update.date != self.date {
            log::error!("Impossible update: dates doesn't match between the \
                    proposed update ({}) and the FlightLog of {}.",
                update.date,
                self.date
            );
        } else if update.updated_field.clone() == "new" {
            flights.push(Flight {
                ogn_nb: update.ogn_nb,
                glider: update.new_value.clone(),
                flight_code: String::new(),
                takeoff_code: String::new(),
                takeoff_machine: String::new(),
                takeoff_machine_pilot: String::new(),
                pilot1: String::new(),
                pilot2: String::new(),
                takeoff: NaiveTime::default(),
                landing: NaiveTime::default(),
            });
        } else if update.updated_field.clone() == "delete" {
            flights.retain(|vol| vol.ogn_nb != update.ogn_nb);
        } else {
            for vol in &mut flights {
                if vol.ogn_nb == update.ogn_nb {
                    match update.updated_field.clone().as_str() {
                        "takeoff_code" => {
                            vol.takeoff_code = update.new_value.clone()
                        }
                        "machine_decollage" => {
                            vol.takeoff_machine = update.new_value.clone()
                        }
                        "decolleur" => vol.takeoff_machine_pilot = update.new_value.clone(),
                        "aeronef" => vol.glider = update.new_value.clone(),
                        "code_vol" => vol.flight_code = update.new_value.clone(),
                        "pilote1" => vol.pilot1 = update.new_value.clone(),
                        "pilote2" => vol.pilot2 = update.new_value.clone(),
                        "decollage" => {
                            vol.takeoff = NaiveTime::parse_from_str(
                                &update.new_value.clone(),
                                "%H:%M",
                            )
                            .unwrap();
                        }
                        "atterissage" => {
                            vol.landing = NaiveTime::parse_from_str(
                                &update.new_value.clone(),
                                "%H:%M",
                            )
                            .unwrap();
                        }
                        _ => {
                            eprintln!("Bad update request.");
                        }
                    }
                }
            }
            if update.ogn_nb == 0 {
                match update.updated_field.as_str() {
                    "pilote_tr" => self.winch_pilot = update.new_value,
                    "treuil" => self.winch = update.new_value,
                    "pilote_rq" => self.tow_pilot = update.new_value,
                    "remorqueur" => self.tow_plane = update.new_value,
                    "chef_piste" => self.field_chief = update.new_value,
                    _ => log::warn!(
                        "The update of the {} at {} does not contain the right field",
                        update.date.format("%Y/%m/%d"),
                        update.time.format("%H:%M")
                    ),
                }
            }
        }
        self.flights = flights.clone();
    }
}

