#[cfg(test)]
use crate::flightlog::{update::ObsoleteUpdates, update::Update};

#[test]
fn t_enlever_maj_obsoletes() {
    let mut vec_majs = vec![Update::new(), Update::new()];

    vec_majs.remove_obsolete_updates(chrono::Duration::minutes(5));
    let empty_vec: Vec<Update> = Vec::new();
    assert_eq!(empty_vec, vec_majs)
}
