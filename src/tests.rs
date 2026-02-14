use crate::custom_functions::shunting_yard::eval;
use test_case::test_case;


#[cfg(test)]
#[test_case("2", 2. ; "basic numbers")]
#[test_case("3 * 4", 12.0; "basic arithmetic")]
#[test_case("2+3*4", 14.; "basic arithmetic 2")]
#[test_case("12*sin(23)", -10.15464485; "basic trigonometry")]
#[test_case("sin ( max ( 2, 3 ) ÷ 3 × π )", 0.; "complex trigonometry")]
#[test_case("cos(pi/2)sin(2pi)", 0.; "multiplication omission")]
#[test_case("-2 * (3 + 4)", -14.; "unary minus multiplicaiton")]
#[test_case("17 * -2 + 21**2", 407.; "unary minus multiplication 2")]
#[test_case("2 / 1", 2.; "floating point rounding")]
#[test_case("-2^3 * 3 + (-2)**2", -20.; "powers of negatives")]
#[test_case("2^-2 * 8", 2.; "negative powers")]
#[test_case("2^2^2", 16.; "power stacking")]
#[test_case("cos(pi) ^ 2 + sin(pi) ^ 2", 1.; "main trigonometric equality")]
#[test_case("2E5", 200000.; "scientific notation")]
#[test_case("sin2cos2", -0.378401247; "low-effort trigonometry")]
#[test_case("fact(4)", 24.; "factorial")]
#[test_case("fact(0)", 1.; "factorial lower edge case")]
#[test_case("fact(20)", 2432902008176640000.; "factorial higher edge case")]
pub fn evaluation(expression: &str, real_value: f64) {
    let (result, is_bool): (f64, bool) = eval(expression); //  evaluation

    if is_bool {
        let result = result == 1.;
    } 
    assert!((result - real_value).abs() <= crate::DELTA)
}
