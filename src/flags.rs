pub enum Flag {
    Help,       //  show help menu
    UserInput,  //  continuous input with instant evaluation
    Null,       //  non-existing flag
}
/*
    consider adding features for working with previous results(i.e. {res} for previous result and {res2} one before it)
*/

// flags can only start with --, because of a leading "-" for negative numbers
impl Flag {
    pub fn get_flag(s: &String) -> Flag {
        match s.as_str() {
            "--h" | "--help" => Flag::Help,
            "--i" | "--input" => Flag::UserInput,
            _ => Flag::Null,
        }
    }
}