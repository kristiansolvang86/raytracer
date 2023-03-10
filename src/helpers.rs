
pub fn equal(a: f64, b: f64) -> bool {
    let factor = 10.0f64.powi(6 as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}