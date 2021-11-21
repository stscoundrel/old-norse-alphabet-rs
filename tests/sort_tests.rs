use old_norse_alphabet::old_norse_sort;

fn vecs_are_equal<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

#[test]
fn sorts_by_old_norse_alphabet() {
    let mut words = vec![
        "aðild",
        "AÐAL",
        "abbast",
        "aðal-vellir",
        "öðli",
        "ógnan",
        "æðrask",
        "aðili",
        "þakkan",
        "áfir",
        "AÐA",
        "abbindi",
        "á-auki",
        "él-ligr",
        "Maðka",
        "ef-lauss",
        "œgir",
        "áðr",
        "maðr",
        "mæðr",
        "Gerzkr",
        "ger",
        "madr",
        "hal-dreki",
        "ÆZLI",
        "gervi",
        "eyxn",
        "halzi",
        "æxling"
    ];
    let expected = vec![
        "abbast",
        "abbindi",
        "AÐA",
        "AÐAL",
        "aðal-vellir",
        "aðild",
        "aðili",
        "á-auki",
        "áðr",
        "áfir",
        "ef-lauss",
        "eyxn",
        "él-ligr",
        "ger",
        "gervi",
        "Gerzkr",
        "hal-dreki",
        "halzi",
        "madr",
        "Maðka",
        "maðr",
        "mæðr",
        "ógnan",
        "þakkan",
        "æðrask",
        "æxling",
        "ÆZLI",
        "œgir",
        "öðli"
    ];
    
    words.sort_by(|a, b| old_norse_sort(a, b));

    assert!(vecs_are_equal(&words, &expected));  
}