#[cfg(test)]
use {
    crate::flightlog::{update::UpdateObsoletes, Update, FlightLog},
    json,
};

#[test]
fn t_enlever_maj_obsoletes() {
    let mut vec_majs = vec![Update::new(), Update::new()];

    vec_majs.enlever_majs_obsoletes(chrono::Duration::minutes(5));
    let empty_vec: Vec<Update> = Vec::new();
    assert_eq!(empty_vec, vec_majs)
}
