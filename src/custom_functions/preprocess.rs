use fancy_regex::Regex;

pub fn preprocess(s: &String) -> String {
    let mut res: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    if res == "" {
        panic!("No expression given")
    }
    // closing missing parenthesis
    {
        let mut unclosed_paren: i32 = 0;
        for ch in res.chars() {
            if ch == '(' {
                unclosed_paren += 1;
            }
            if ch == ')' {
                unclosed_paren -= 1;
            }
        }
        if unclosed_paren > 0 {
            for _ in 0..unclosed_paren {
                res.push(')');
            }
        }
        if unclosed_paren < 0 {
            for _ in 0..-unclosed_paren {
                res.insert(0, '(');
            }
        }
    };

    // implicit multiplication
    res = Regex::new(r"(?<=[\d])(?=[a-zA-Z])")
        .unwrap()
        .replace_all(&res, "*")
        .into_owned();
    res = Regex::new(r"(?<=[\)])(?=[a-zA-Z])")
        .unwrap()
        .replace_all(&res, "*")
        .into_owned();

    let replacements: [(&'static str, &'static str); 22] = [
        // symbol unification
        ("**", "^"),
        ("//", "/"),
        (":", "/"),
        ("÷", "/"),
        ("×", "*"),
        ("−", "-"),
        ("==", "="),
        // костыли
        ("--", "+"),
        ("{", "("),
        ("[", "("),
        ("}", ")"),
        ("]", ")"),
        (")(", ")*("),
        ("E", "*10"),
        // constants
        ("pi", "3.141592653589793"),
        ("π", "3.141592653589793"),
        ("e", "2.718281828459045"),
        // Boolean
        ("!=", "≠"),
        (">=", "⪖"),
        ("<=", "⪕"),
        (r"&&", "&"),
        (r"||", "|"),
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
    // unary minuses
    res = Regex::new(r"(?<=\A)-(?=[\d\(])")
        .unwrap()
        .replace_all(&res, "~")
        .into_owned();
    res = Regex::new(r"(?<=[^0-9)])-(?=[\d\(])")
        .unwrap()
        .replace_all(&res, "~")
        .into_owned();
    // more implicit multiplication
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
