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
    /*/// Pour parser une mise à jour depuis un texte json, préalablement parsé à l'aide de [`json::parse()`].
    pub fn parse(&mut self, texte_json: json::JsonValue) -> Result<(), String> {
        match texte_json {
            json::JsonValue::Object(objet) => {
                self.ogn_nb = objet["ogn_nb"].as_i32().unwrap_or_else(|| {
                    log::error!("pas de numero de vol dans la requete");
                    0
                });

                self.updated_field = objet["updated_field"]
                    .as_str()
                    .unwrap_or_else(|| {
                        log::error!("pas le bon champ pour la nouvelle valeur");
                        ""
                    })
                    .to_string();

                self.new_value = objet["new_value"]
                    .as_str()
                    .unwrap_or_else(|| {
                        log::error!("pas la bonne valeur pour la nouvelle valeur");
                        ""
                    })
                    .to_string();

                self.date = NaiveDate::parse_from_str(
                    objet["date"].as_str().unwrap_or_else(|| {
                        log::error!("pas la bonne valeur pour la nouvelle valeur");
                        ""
                    }),
                    "%Y/%m/%d",
                )
                .unwrap();

                self.time = NaiveTime::parse_from_str(
                    objet["time"].as_str().unwrap_or_else(|| {
                        log::error!("pas la bonne valeur pour la nouvelle valeur de l'time");
                        ""
                    }),
                    "%H:%M",
                )
                .unwrap();
            }
            _ => {
                eprintln!("pas un objet json");
            }
        };
        Ok(())
    }*/

    /*/// Pour encoder en Json.
    pub fn vers_json(&self) -> String {
        json::object! {
            ogn_nb: self.ogn_nb,
            date: *self.date.format("%Y/%m/%d").to_string(),
            updated_field: *self.updated_field,
            new_value: *self.new_value,
            time: *self.time.format("%H:%M").to_string(),
        }
        .dump()
    }*/
}

/* S'occupe des relations entre plusieurs mises à jour et Json.
pub trait UpdateJson {
    /// Encode plusieurs mises à jour en Json.
    fn vers_json(&self) -> String;
}

impl UpdateJson for Vec<Update> {
    fn vers_json(&self) -> String {
        let mut string = String::new();
        string.push('[');
        for maj in self {
            string.push_str(maj.vers_json().as_str());
            string.push(',')
        }
        if string != *"[" {
            string.pop();
        }
        string.push(']');
        string
    }
}*/

/// Take care of too in the passt updates. Mainly to remove ones that are
/// for more than some time ago.
pub trait ObsoleteUpdates {
    /// To remove updates that dates from more than time.
    fn remove_obsolete_updates(&mut self, time: chrono::Duration);
}

impl ObsoleteUpdates for Vec<Update> {
    fn remove_obsolete_updates(&mut self, time: chrono::Duration) {
        let time_actuelle = chrono::Local::now().time();
        let mut i = 0;
        while i < self.len() {
            if (time_actuelle - self[i].time) > time {
                self.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
