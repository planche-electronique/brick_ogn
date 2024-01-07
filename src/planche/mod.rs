//! Module des planche, i.e. un ensemble de plusieurs [`Vol`] et d'affectation.

pub mod mise_a_jour;

use crate::vol::{Vol, VolJson};
use chrono::{NaiveDate, NaiveTime};
use json;
use log;
pub use mise_a_jour::MiseAJour;

/// Représentation des données de vol d'une journée, en cours.
#[derive(PartialEq, Debug, Clone)]
pub struct Planche {
    /// Tous les vols d'un jour.
    pub vols: Vec<Vol>,
    /// La date de ce jour.
    pub date: NaiveDate,
    /// le pilote de treuil.
    pub pilote_tr: String, // parmi pilotes_tr
    /// Le treuil en service.
    pub treuil: String, // parmi treuils
    /// Le pilote de remorqueur en service.
    pub pilote_rq: String, // parmi pilotes_rq
    /// Le remorqueur en service.
    pub remorqueur: String, // parmi remorqueurs
    /// Le chef de piste en service.
    pub chef_piste: String, // parmi pilotes
}

impl Default for Planche {
    fn default() -> Self {
        Self::new()
    }
}

impl Planche {
    /// Une nouvelle planche.
    pub fn new() -> Self {
        Planche {
            vols: Vec::new(),
            date: NaiveDate::default(),
            pilote_tr: String::new(),
            treuil: String::new(),
            pilote_rq: String::new(),
            remorqueur: String::new(),
            chef_piste: String::new(),
        }
    }

    /// Encodage de la planche en Json.
    pub fn vers_json(self) -> String {
        let vols_json = self.vols.vers_json();
        let date_json = self.date.format("%Y/%m/%d").to_string();
        let reste_json = json::stringify(json::object! {
            pilote_tr: self.pilote_tr,
            treuil: self.treuil,
            pilote_rq: self.pilote_rq,
            remorqueur: self.remorqueur,
            chef_piste: self.chef_piste,
        });
        let mut json = String::new();
        json.push_str("{ \"date\": \"");
        json.push_str(&date_json);
        json.push_str("\",\n\"vols\" : ");
        json.push_str(&vols_json);
        json.push_str(", \n \"affectations\": ");
        json.push_str(&reste_json);
        json.push('\n');
        json.push('}');
        json
    }
}

/// Mise à jour d'une planche à l'aide d'une [`MiseAJour`].
pub trait MettreAJour {
    /// Mise à jour d'une planche à l'aide d'une [`MiseAJour`].
    fn mettre_a_jour(&mut self, mise_a_jour: MiseAJour);
}

impl MettreAJour for Planche {
    // on crée une fonction pour mettre la mise à jour dans le vecteur Vols du jour
    fn mettre_a_jour(&mut self, mise_a_jour: MiseAJour) {
        let mut vols = self.vols.clone();
        if mise_a_jour.date != self.date {
            log::error!("Mise a jour impossible: les dates ne sont pas les mêmes !");
        } else if mise_a_jour.champ_mis_a_jour.clone() == "nouveau" {
            vols.push(Vol {
                numero_ogn: mise_a_jour.numero_ogn,
                aeronef: mise_a_jour.nouvelle_valeur.clone(),
                code_vol: String::new(),
                code_decollage: String::new(),
                machine_decollage: String::new(),
                decolleur: String::new(),
                pilote1: String::new(),
                pilote2: String::new(),
                decollage: NaiveTime::default(),
                atterissage: NaiveTime::default(),
            });
        } else if mise_a_jour.champ_mis_a_jour.clone() == "supprimer" {
            vols.retain(|vol| vol.numero_ogn != mise_a_jour.numero_ogn);
        } else {
            for vol in &mut vols {
                if vol.numero_ogn == mise_a_jour.numero_ogn {
                    match mise_a_jour.champ_mis_a_jour.clone().as_str() {
                        "code_decollage" => {
                            vol.code_decollage = mise_a_jour.nouvelle_valeur.clone()
                        }
                        "machine_decollage" => {
                            vol.machine_decollage = mise_a_jour.nouvelle_valeur.clone()
                        }
                        "decolleur" => vol.decolleur = mise_a_jour.nouvelle_valeur.clone(),
                        "aeronef" => vol.aeronef = mise_a_jour.nouvelle_valeur.clone(),
                        "code_vol" => vol.code_vol = mise_a_jour.nouvelle_valeur.clone(),
                        "pilote1" => vol.pilote1 = mise_a_jour.nouvelle_valeur.clone(),
                        "pilote2" => vol.pilote2 = mise_a_jour.nouvelle_valeur.clone(),
                        "decollage" => {
                            vol.decollage = NaiveTime::parse_from_str(
                                &mise_a_jour.nouvelle_valeur.clone(),
                                "%H:%M",
                            )
                            .unwrap();
                        }
                        "atterissage" => {
                            vol.atterissage = NaiveTime::parse_from_str(
                                &mise_a_jour.nouvelle_valeur.clone(),
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
            if mise_a_jour.numero_ogn == 0 {
                match mise_a_jour.champ_mis_a_jour.as_str() {
                    "pilote_tr" => self.pilote_tr = mise_a_jour.nouvelle_valeur,
                    "treuil" => self.treuil = mise_a_jour.nouvelle_valeur,
                    "pilote_rq" => self.pilote_rq = mise_a_jour.nouvelle_valeur,
                    "remorqueur" => self.remorqueur = mise_a_jour.nouvelle_valeur,
                    "chef_piste" => self.chef_piste = mise_a_jour.nouvelle_valeur,
                    _ => log::warn!(
                        "la mise a jour pour le {} à {} ne contient pas le bon champ",
                        mise_a_jour.date.format("%Y/%m/%d"),
                        mise_a_jour.heure.format("%H:%M")
                    ),
                }
            }
        }
        self.vols = vols.clone();
    }
}
