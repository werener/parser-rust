use fancy_regex::Regex;

// Подумать как реализовать унарное бинарное отрицание (по аналогии с костылем для унарного -)
pub fn preprocess(s: &String) -> String {
    let mut res = s.clone();
    res.retain(|c| !c.is_whitespace());

    let replacements: = [
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
        

        // Eq to 1-symbol denomination
        ("!=", "~"),
        (">=", "@"),
        ("<=", "#"),
    ];


    for (i, c) in s.chars().enumerate() {

    }
    return res;
}

pub fn run() {
    println!("{}", preprocess(&"[12] ** {12} + -12 - 14 - -2".to_string()).replace("/", ""));
}

// println!("{}",res);
//     // parenthesis
//     res = Regex::new(r"[\[\{]")
//         .unwrap()
//         .replace_all(&res, "(")
//         .into_owned();
//     res = Regex::new(r"[\]\}]")
//         .unwrap()
//         .replace_all(&res, ")")
//         .into_owned();

//     // symbol unification
//     res = res.replace("**", "*");
//     res = res.replace("==", "=");
//     res = res.replace("--", "+");
//     res = Regex::new(r"(/{2}|:)")
//         .unwrap()
//         .replace_all(&res, "/")
//         .into_owned();
//     res = Regex::new(r"").unwrap().replace_all(s, "/").into_owned();
//     println!("{}", res.replace("/", ""));
//     // unary minus
    
//     res = Regex::new(r"([^\d])-(?=\d)")
//         .unwrap()
//         .replace_all(&res, "0-")
//         .into_owned();