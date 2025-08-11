use fancy_regex::Regex;

use crate::program;

// Подумать как реализовать бинарное отрицание (по аналогии с костылем для унарного -)
pub fn preprocess(s: &String) -> String {
    let mut res = s.clone();

    let replacements: [(&'static str, &'static str); 18] = [
        ("**", "^"),
        ("//", "/"),
        (":", "/"),
        ("==", "="),
        (",", "."),
        ("--", "+"),
        (" ", ""),
        ("_", ""),
        ("{", "("),
        ("[", "("),
        ("}", ")"),
        ("]", ")"),
        (r"&&", "&"),
        (r"||", "|"),
        (")(", ")*("),
        // Eq to 1-symbol denomination
        ("!=", "`"),
        (">=", "@"),
        ("<=", "#"),
    ];
    for rep in replacements {
        res = res.replace(rep.0, rep.1);
    }
    // left-out zeroes
    res = Regex::new(r"(?<=[^\d])\.(?=[\d])")
        .unwrap()
        .replace_all(&res, "0.")
        .into_owned();
    res = Regex::new(r"(?<=[\d])\.(?=[^\d])")
        .unwrap()
        .replace_all(&res, ".0")
        .into_owned();
    // leading unary minus
    res = Regex::new(r"(?<=\A)-(?=[\d\(])")
        .unwrap()
        .replace_all(&res, "0-")
        .into_owned();

    // implicit multiplication
    res = Regex::new(r"(?<=[)])(?=[\d])")
        .unwrap()
        .replace_all(&res, "*")
        .into_owned();
    res = Regex::new(r"(?<=[\d])(?=[\(])")
        .unwrap()
        .replace_all(&res, "*")
        .into_owned();
    return res;
}

pub fn run() {
    let pr = program::Program::new();
    if pr.expression != "" {
        println!("preprocess:");
        println!("{}", preprocess(&pr.expression));
    }
}
