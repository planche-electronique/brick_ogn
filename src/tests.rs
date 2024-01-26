#[cfg(test)]
use {
    crate::planche::{mise_a_jour::MiseAJourObsoletes, MiseAJour, Planche},
    json,
};

#[test]
fn test_miseajour_parse() {
    let mut maj_def = MiseAJour::new();
    maj_def.numero_ogn = 42;
    let maj_def_json = json::parse(maj_def.vers_json().as_str()).unwrap();

    let mut maj_mod = MiseAJour::new();
    maj_mod.parse(maj_def_json).unwrap();
    assert_eq!(maj_def, maj_mod)
}

#[test]
fn t_enlever_maj_obsoletes() {
    let mut vec_majs = vec![MiseAJour::new(), MiseAJour::new()];

    vec_majs.enlever_majs_obsoletes(chrono::Duration::minutes(5));
    let empty_vec: Vec<MiseAJour> = Vec::new();
    assert_eq!(empty_vec, vec_majs)
}
