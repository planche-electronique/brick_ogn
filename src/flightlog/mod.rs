//! Module des planche, i.e. un ensemble de plusieurs [`Vol`] et d'affectation.

pub mod update;

use crate::flight::{Flight, FlightJson};
use chrono::{NaiveDate, NaiveTime};
use json;
use log;
pub use update::Update;

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
    /// Une nouvelle planche.
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

    /// Encodage de la planche en Json.
    pub fn to_json(self) -> String {
        let flights_json = self.flights.vers_json();
        let date_json = self.date.format("%Y/%m/%d").to_string();
        let rest_json = json::stringify(json::object! {
            winch_pilot: self.winch_pilot,
            winch: self.winch,
            tow_pilot: self.tow_pilot,
            tow_plane: self.tow_plane,
            field_chief: self.field_chief,
        });
        let mut json = String::new();
        json.push_str("{ \"date\": \"");
        json.push_str(&date_json);
        json.push_str("\",\n\"flights\" : ");
        json.push_str(&flights_json);
        json.push_str(", \n \"affectations\": ");
        json.push_str(&rest_json);
        json.push('\n');
        json.push('}');
        json
    }
}

/// Mise à jour d'une planche à l'aide d'une [`MiseAJour`].
pub trait MettreAJour {
    /// Mise à jour d'une planche à l'aide d'une [`MiseAJour`].
    fn mettre_a_jour(&mut self, mise_a_jour: Update);
}

impl MettreAJour for FlightLog {
    // on crée une fonction pour mettre la mise à jour dans le vecteur flights du jour
    fn mettre_a_jour(&mut self, mise_a_jour: Update) {
        let mut flights = self.flights.clone();
        if mise_a_jour.date != self.date {
            log::error!("Mise a jour impossible: les dates ne sont pas les mêmes !");
        } else if mise_a_jour.updated_field.clone() == "nouveau" {
            flights.push(Flight {
                ogn_nb: mise_a_jour.ogn_nb,
                glider: mise_a_jour.new_value.clone(),
                flight_code: String::new(),
                takeoff_code: String::new(),
                takeoff_machine: String::new(),
                takeoff_machine_pilot: String::new(),
                pilot1: String::new(),
                pilot2: String::new(),
                takeoff: NaiveTime::default(),
                landing: NaiveTime::default(),
            });
        } else if mise_a_jour.updated_field.clone() == "supprimer" {
            flights.retain(|vol| vol.ogn_nb != mise_a_jour.ogn_nb);
        } else {
            for vol in &mut flights {
                if vol.ogn_nb == mise_a_jour.ogn_nb {
                    match mise_a_jour.updated_field.clone().as_str() {
                        "takeoff_code" => {
                            vol.takeoff_code = mise_a_jour.new_value.clone()
                        }
                        "machine_decollage" => {
                            vol.takeoff_machine = mise_a_jour.new_value.clone()
                        }
                        "decolleur" => vol.takeoff_machine_pilot = mise_a_jour.new_value.clone(),
                        "aeronef" => vol.glider = mise_a_jour.new_value.clone(),
                        "code_vol" => vol.flight_code = mise_a_jour.new_value.clone(),
                        "pilote1" => vol.pilot1 = mise_a_jour.new_value.clone(),
                        "pilote2" => vol.pilot2 = mise_a_jour.new_value.clone(),
                        "decollage" => {
                            vol.takeoff = NaiveTime::parse_from_str(
                                &mise_a_jour.new_value.clone(),
                                "%H:%M",
                            )
                            .unwrap();
                        }
                        "atterissage" => {
                            vol.landing = NaiveTime::parse_from_str(
                                &mise_a_jour.new_value.clone(),
                                "%H:%M",
                            )
                            .unwrap();
                        }
                        _ => {
                            eprintln!("Requète de mise a jour mauvaise.");
                        }
                    }
                }
            }
            if mise_a_jour.ogn_nb == 0 {
                match mise_a_jour.updated_field.as_str() {
                    "pilote_tr" => self.winch_pilot = mise_a_jour.new_value,
                    "treuil" => self.winch = mise_a_jour.new_value,
                    "pilote_rq" => self.tow_pilot = mise_a_jour.new_value,
                    "remorqueur" => self.tow_plane = mise_a_jour.new_value,
                    "chef_piste" => self.field_chief = mise_a_jour.new_value,
                    _ => log::warn!(
                        "la mise a jour pour le {} à {} ne contient pas le bon champ",
                        mise_a_jour.date.format("%Y/%m/%d"),
                        mise_a_jour.time.format("%H:%M")
                    ),
                }
            }
        }
        self.flights = flights.clone();
    }
}
