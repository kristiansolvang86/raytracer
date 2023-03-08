fn main() {
    println!("Hello, world!");
}

fn point(t: (f64, f64,f64)) ->  (f64, f64, f64, f64) {
    (t.0, t.1, t.2, 1.0)
}

fn vector(t: (f64, f64,f64)) -> (f64, f64, f64, f64) {
    (t.0, t.1, t.2, 0.0)
}

fn equal(a: f64, b: f64) -> bool {
    let factor = 10.0f64.powi(10 as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}

fn add(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
}

fn subtract(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3)
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn a_tuple_with_w1_is_a_point() {
        let a = (4.3, -4.2, 3.1, 1.0);

        assert_eq!(a.3, 1.0);
    }

    #[test]
    fn a_tuple_with_w0_is_a_vector() {
        let a = (4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.3, 0.0);
    }
    
    #[test]
    fn point_creates_tupe_with_w1() {
        let a = (4.3, -4.2, 3.1);

        let result = point(a);
        assert_eq!(result.3, 1.0);
    }

    #[test]
    fn vector_creates_tupe_with_w0() {
        let a = (4.3, -4.2, 3.1);

        let result = vector(a);
        assert_eq!(result.3, 0.0);
    }

    #[test]
    fn adding_two_tuples() {
        let a = (3.0, -2.0, 5.0, 1.0);
        let b = (-2.0, 3.0, 1.0, 0.0);

        let result = add(a, b);
        let expected = (1.0, 1.0, 6.0, 1.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn subtracting_point_from_point() {
        let a = (3.0, 2.0, 1.0, 1.0);
        let b = (5.0, 6.0, 7.0, 1.0);

        let result = subtract(a, b);
        let expected = (-2.0, -4.0, -6.0, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn subtracting_vector_from_point() {
        let a = (3.0, 2.0, 1.0, 1.0);
        let b = (5.0, 6.0, 7.0, 0.0);

        let result = subtract(a, b);
        let expected = (-2.0, -4.0, -6.0, 1.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn subtracting_vector_from_vector() {
        let a = (3.0, 2.0, 1.0, 0.0);
        let b = (5.0, 6.0, 7.0, 0.0);

        let result = subtract(a, b);
        let expected = (-2.0, -4.0, -6.0, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let a = (0.0, 0.0, 0.0, 0.0);
        let b = (1.0, -2.0, 3.0, 0.0);

        let result = subtract(a, b);
        let expected = (-1.0, 2.0, -3.0, 0.0);
        assert_eq!(result, expected);
    }

}