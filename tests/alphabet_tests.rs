use old_norse_alphabet::{get_lowercase, get_uppercase};
use old_norse_alphabet::{ETH, THORN, O_CAUDATA, SLASHED_O, AE, OE, ASH};
use insta::assert_json_snapshot;

#[test]
fn exposes_special_characters() {
    let mut special_characters = "".to_string();

    special_characters.push_str(&ETH.to_string());
    special_characters.push_str(&THORN.to_string());
    special_characters.push_str(&O_CAUDATA.to_string());
    special_characters.push_str(&ASH.to_string());
    special_characters.push_str(&SLASHED_O.to_string());
    special_characters.push_str(&OE.to_string());
    special_characters.push_str(&AE.to_string());

    let expected = "ðþǫæøœæ".to_string();

    assert_eq!(special_characters, expected);
}

#[test]
fn exposes_lower_alphabet() {
    let lower = get_lowercase().to_vec();

    assert_json_snapshot!(lower);
}

#[test]
fn exposes_upper_alphabet() {
    let upper = get_uppercase().to_vec();

    assert_json_snapshot!(upper);
}