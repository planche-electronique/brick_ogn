#[cfg(test)]
use {
    crate::flightlog::{update::UpdateObsoletes, Update, FlightLog},
    json,
};

#[test]
fn test_update_parse() {
    let mut maj_def = Update::new();
    maj_def.ogn_nb = 42;
    let maj_def_json = json::parse(maj_def.vers_json().as_str()).unwrap();

    let mut maj_mod = Update::new();
    maj_mod.parse(maj_def_json).unwrap();
    assert_eq!(maj_def, maj_mod)
}

#[test]
fn t_enlever_maj_obsoletes() {
    let mut vec_majs = vec![Update::new(), Update::new()];

    vec_majs.enlever_majs_obsoletes(chrono::Duration::minutes(5));
    let empty_vec: Vec<Update> = Vec::new();
    assert_eq!(empty_vec, vec_majs)
}
