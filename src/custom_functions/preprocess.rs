use fancy_regex::Regex;

pub fn preprocess(s: &String) -> String {
    let mut res = s.clone();

    let replacements: [(&'static str, &'static str); 24] = [
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
        (" ", ""),
        ("_", ""),
        ("{", "("),
        ("[", "("),
        ("}", ")"),
        ("]", ")"),
        (r"&&", "&"),
        (r"||", "|"),
        (")(", ")*("),
        ("E", "*10"),
        // constants
        ("pi", "3.141592653589793"),
        ("π", "3.141592653589793"),
        ("e", "2.718281828459045"),
        // Eq to 1-symbol denomination
        ("!=", "≠"),
        (">=", "⪖"),
        ("<=", "⪕"),
    ];
    for rep in replacements {
        res = res.replace(rep.0, rep.1);
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
                res.insert(0,'(');
            }
        }
    };

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
        .replace_all(&res, "0-")
        .into_owned();
    // res = Regex::new(r"(?<=[^\d])-(?=[\d\(])")
    //     .unwrap()
    //     .replace_all(&res, "~")
    //     .into_owned();
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
