use std::fmt::Error;

pub fn sin(args: &[f64]) -> Result<f64, &'static str> {
    for arg in args {
        return Ok(arg.sin())
    };
    return Err("sin calculation error")
}