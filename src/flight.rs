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
            takeoff_code: String::from("T"),
            takeoff_machine: String::from("F-REMA"),
            takeoff_machine_pilot: String::from("YDL"),
            glider: String::from("F-CERJ"),
            flight_code: String::from("S"),
            pilot1: String::from("Walt Disney"),
            pilot2: String::default(),
            takeoff: NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
            landing: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        }
    }
}

impl Flight {
    fn _new() -> Self {
        Flight {
            ogn_nb: i32::default(),
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
    }/*

    /// Encode the flight in JSON string.
    pub fn to_json(&self) -> String {
        let vol = json::object! {
            ogn_nb: self.ogn_nb,
            takeoff_code: *self.takeoff_code,
            takeoff_machine: *self.takeoff_machine,
            takeoff_machine_pilot: *self.takeoff_machine_pilot,
            glider: *self.glider,
            flight_code: *self.flight_code,
            pilot1: *self.pilot1,
            pilot2: *self.pilot2,
            takeoff: *self.takeoff.format("%H:%M").to_string(),
            landing: *self.landing.format("%H:%M").to_string(),
        };
        vol.dump()
    }
    /// Parse a flight from a JsonValue. Seems that it can be replaced using serde.
    pub fn depuis_json(mut json_parse: JsonValue) -> Self {
        Flight {
            ogn_nb: json_parse["ogn_nb"].as_i32().unwrap_or_default(),
            takeoff_code: json_parse["takeoff_code"]
                .take_string()
                .unwrap_or_else(|| String::from("")),
            takeoff_machine: json_parse["takeoff_machine"]
                .take_string()
                .unwrap_or_else(|| String::from("")),
            takeoff_machine_pilot: json_parse["takeoff_machine_pilot"]
                .take_string()
                .unwrap_or_else(|| String::from("")),
            glider: json_parse["glider"]
                .take_string()
                .unwrap_or_else(|| String::from("")),
            flight_code: json_parse["flight_code"]
                .take_string()
                .unwrap_or_else(|| String::from("")),
            pilot1: json_parse["pilot1"]
                .take_string()
                .unwrap_or_else(|| String::from("")),
            pilot2: json_parse["pilot2"]
                .take_string()
                .unwrap_or_else(|| String::from("")),
            takeoff: NaiveTime::parse_from_str(
                json_parse["takeoff"].take_string().unwrap().as_str(),
                "%H:%M",
            )
            .unwrap(),
            landing: NaiveTime::parse_from_str(
                json_parse["landing"].take_string().unwrap().as_str(),
                "%H:%M",
            )
            .unwrap(),
        }
    }*/
}
/*
/// Un trait qui permet d'encoder/décoder des vols en JSON.
pub trait FlightJson {
    /// Permet d'encoder un vol en JSON.
    fn vers_json(self) -> String;
    /// Décode un vol depuis un JsonValue, qui peut être lui-même parsé en utilisant
    /// json::parse!(string).
    fn depuis_json(&mut self, json: JsonValue);
}

impl FlightJson for Vec<Flight> {
    fn vers_json(self) -> String {
        //on crée une string qui sera la json final et on lui rajoute le dbut d'un tableau
        let mut vols_str = String::new();
        vols_str.push_str("[\n");

        //pour chaque vol on ajoute sa version json a vols_str et on rajoute une virgule
        for vol in self {
            vols_str.push_str(vol.to_json().as_str());
            vols_str.push(',');
        }
        vols_str = vols_str[0..(vols_str.len() - 1)].to_string(); // on enleve la virgule de trop
        vols_str.push_str("\n]");
        vols_str
    }

    fn depuis_json(&mut self, json: JsonValue) {
        let mut vols = Vec::new();
        for vol in json.members() {
            vols.push(Flight::depuis_json(vol.clone()));
        }
        (*self) = vols;
    }
}
*/