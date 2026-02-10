pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

pub fn square_root(a: f64) -> Result<f64, String> {
    if a < 0.0 {
        Err("Cannot take square root of negative number".to_string())
    } else {
        Ok(a.sqrt())
    }
}
