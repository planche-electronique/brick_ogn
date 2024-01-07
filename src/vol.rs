//! Tout ce qui attrait aux vols que nous enregistrons.

use crate::ogn::vols_ogn;
use crate::{creer_chemin_jour, data_dir, nom_fichier_date, ActifServeur};
use async_trait::async_trait;
use chrono::{Datelike, NaiveDate, NaiveTime};
use json::JsonValue;
use std::fs;

/// Représentation en mémoire d'un vol. Se référer à infos.json pour les différents codes.
#[derive(Clone, PartialEq, Debug)]
pub struct Vol {
    /// Le numéro de son vol venant d'ogn.
    pub numero_ogn: i32,
    /// Le code de décollage (T: treuillée, R: remorqué)
    pub code_decollage: String,
    /// La machine qui a fait le décollage (voir dans infos.json/remrorqueurs & infos.json/treuils).
    pub machine_decollage: String,
    /// La personne qui était dans la machine de décollage (voir infos.json/pilotes_tr & infos.json/pilote_rq).
    pub decolleur: String,
    /// L'immatriculation du planeur.
    pub aeronef: String,
    /// Le code du vol : Mutuel, Ecole etc.
    pub code_vol: String,
    /// Le nom du commandant de bord ou de l'élève.
    pub pilote1: String,
    /// Le nom de l'éventiuel passager ou de l'instructeur.
    pub pilote2: String,
    /// L'heure de décollage au format hh:mm.
    pub decollage: NaiveTime,
    /// L'heure d'atterissage au format hh:mm.
    pub atterissage: NaiveTime,
}

impl Default for Vol {
    fn default() -> Self {
        Vol {
            numero_ogn: 1,
            code_decollage: String::from("T"),
            machine_decollage: String::from("F-REMA"),
            decolleur: String::from("YDL"),
            aeronef: String::from("F-CERJ"),
            code_vol: String::from("S"),
            pilote1: String::from("Walt Disney"),
            pilote2: String::default(),
            decollage: NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
            atterissage: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        }
    }
}

impl Vol {
    fn _new() -> Self {
        Vol {
            numero_ogn: i32::default(),
            code_decollage: String::default(),
            machine_decollage: String::default(),
            decolleur: String::default(),
            aeronef: String::default(),
            code_vol: String::default(),
            pilote1: String::default(),
            pilote2: String::default(),
            decollage: NaiveTime::default(),
            atterissage: NaiveTime::default(),
        }
    }

    /// Encode le vol en JSON.
    pub fn vers_json(&self) -> String {
        let vol = json::object! {
            numero_ogn: self.numero_ogn,
            code_decollage: *self.code_decollage,
            machine_decollage: *self.machine_decollage,
            decolleur: *self.decolleur,
            aeronef: *self.aeronef,
            code_vol: *self.code_vol,
            pilote1: *self.pilote1,
            pilote2: *self.pilote2,
            decollage: *self.decollage.format("%H:%M").to_string(),
            atterissage: *self.atterissage.format("%H:%M").to_string(),
        };
        vol.dump()
    }
    /// Décode un vol depuis un JsonValue, qui peut être lui-même parsé en utilisant
    /// json::parse!(string).
    pub fn depuis_json(mut json_parse: JsonValue) -> Self {
        Vol {
            numero_ogn: json_parse["numero_ogn"].as_i32().unwrap_or_default(),
            code_decollage: json_parse["code_decollage"]
                .take_string()
                .unwrap_or_else(|| String::from("")),
            machine_decollage: json_parse["machine_decollage"]
                .take_string()
                .unwrap_or_else(|| String::from("")),
            decolleur: json_parse["decolleur"]
                .take_string()
                .unwrap_or_else(|| String::from("")),
            aeronef: json_parse["aeronef"]
                .take_string()
                .unwrap_or_else(|| String::from("")),
            code_vol: json_parse["code_vol"]
                .take_string()
                .unwrap_or_else(|| String::from("")),
            pilote1: json_parse["pilote1"]
                .take_string()
                .unwrap_or_else(|| String::from("")),
            pilote2: json_parse["pilote2"]
                .take_string()
                .unwrap_or_else(|| String::from("")),
            decollage: NaiveTime::parse_from_str(
                json_parse["decollage"].take_string().unwrap().as_str(),
                "%H:%M",
            )
            .unwrap(),
            atterissage: NaiveTime::parse_from_str(
                json_parse["atterissage"].take_string().unwrap().as_str(),
                "%H:%M",
            )
            .unwrap(),
        }
    }
}

/// Un trait qui permet d'encoder/décoder des vols en JSON.
pub trait VolJson {
    /// Permet d'encoder un vol en JSON.
    fn vers_json(self) -> String;
    /// Décode un vol depuis un JsonValue, qui peut être lui-même parsé en utilisant
    /// json::parse!(string).
    fn depuis_json(&mut self, json: JsonValue);
}

impl VolJson for Vec<Vol> {
    fn vers_json(self) -> String {
        //on crée une string qui sera la json final et on lui rajoute le dbut d'un tableau
        let mut vols_str = String::new();
        vols_str.push_str("[\n");

        //pour chaque vol on ajoute sa version json a vols_str et on rajoute une virgule
        for vol in self {
            vols_str.push_str(vol.vers_json().as_str());
            vols_str.push(',');
        }
        vols_str = vols_str[0..(vols_str.len() - 1)].to_string(); // on enleve la virgule de trop
        vols_str.push_str("\n]");
        vols_str
    }

    fn depuis_json(&mut self, json: JsonValue) {
        let mut vols = Vec::new();
        for vol in json.members() {
            vols.push(Vol::depuis_json(vol.clone()));
        }
        (*self) = vols;
    }
}
