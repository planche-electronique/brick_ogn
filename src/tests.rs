#[cfg(test)]
use {crate::planche::MiseAJour, crate::planche::Planche, json};

#[test]
fn test_miseajour_parse() {
    let mut maj_def = MiseAJour::new();
    maj_def.numero_ogn = 42;
    let maj_def_json = json::parse(maj_def.vers_json().as_str()).unwrap();

    let mut maj_mod = MiseAJour::new();
    maj_mod.parse(maj_def_json).unwrap();
    assert_eq!(maj_def, maj_mod)
}
