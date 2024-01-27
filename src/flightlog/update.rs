//! Gestion des mise à jour: envoi des modifications par champ de vol pour éviter de recharger
//! toute la planche à chaque fois

use chrono::{NaiveDate, NaiveTime};
use serde::{Serialize, Deserialize};

/// Représentation en mémoire d'une "planche".
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Update {
    /// Le numero du vol sur OGN.
    pub ogn_nb: i32,
    /// Le nom du champ qui a été changé.
    pub updated_field: String,
    /// La nouvelle valeur de ce champ.
    pub new_value: String,
    /// La date du vol sur lequel le changement est fait.
    pub date: NaiveDate,
    /// L'time à laquelle la requete est faite.
    pub time: NaiveTime,
}

impl Default for Update {
    fn default() -> Self {
        Self::new()
    }
}

impl Update {
    /// Nouvelle mise à jour.
    pub fn new() -> Self {
        Update {
            ogn_nb: i32::default(), //numero du vol **OGN**
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

/// S'occupe des mises a jour trop vieilles.
pub trait UpdateObsoletes {
    /// Pour supprimer les mises a jour de plus d'un certain temps.
    fn enlever_majs_obsoletes(&mut self, temps: chrono::Duration);
}

impl UpdateObsoletes for Vec<Update> {
    fn enlever_majs_obsoletes(&mut self, temps: chrono::Duration) {
        let time_actuelle = chrono::Local::now().time();
        let mut i = 0;
        while i < self.len() {
            if (time_actuelle - self[i].time) > temps {
                self.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
