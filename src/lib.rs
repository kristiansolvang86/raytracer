pub fn point(t: (f64, f64,f64)) ->  (f64, f64, f64, f64) {
    (t.0, t.1, t.2, 1.0)
}

pub fn vector(t: (f64, f64,f64)) -> (f64, f64, f64, f64) {
    (t.0, t.1, t.2, 0.0)
}

pub fn equal(a: f64, b: f64) -> bool {
    let factor = 10.0f64.powi(10 as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}

pub fn add(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
}

pub fn subtract(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3)
}

pub fn negate(a: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    (a.0 * -1.0, a.1 * -1.0, a.2 * -1.0, a.3 * -1.0)
}

pub fn scalar_multiplication(a: (f64, f64, f64, f64), b: f64) -> (f64, f64, f64, f64) {
    (a.0 * b, a.1 * b, a.2 * b, a.3 * b)
}

pub fn scalar_division(a: (f64, f64, f64, f64), b: f64) -> (f64, f64, f64, f64) {
    (a.0 / b, a.1 / b, a.2 / b, a.3 / b)
}

pub fn vector_magnitude(a: (f64, f64, f64, f64)) -> f64 {
    let result = a.0.powf(2.0) + a.1.powf(2.0) + a.2.powf(2.0);

    result.sqrt()
}

pub fn normalize_vector(a: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    let magnitude = vector_magnitude(a);
    (a.0 / magnitude, a.1 / magnitude, a.2 / magnitude, a.3)
}

pub fn vector_dot(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> f64 {
    a.0 * b.0 +
    a.1 * b.1 +
    a.2 * b.2 +
    a.3 * b.3
}

pub fn vector_cross(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) ->  (f64, f64, f64, f64) {
    (a.1 * b.2 - a.2 * b.1,
    a.2 * b.0 - a.0 * b.2,
    a.0 * b.1 - a.1 * b.0, 
    0.0) //vector
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

    #[test]
    fn negating_a_tuple() {
        let a = (1.0, -2.0, 3.0, -4.0);
        let result = negate(a);

        assert_eq!(result, (-1.0, 2.0, -3.0, 4.0))
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = (1.0, -2.0, 3.0, -4.0);
        let result = scalar_multiplication(a, 3.5);

        assert_eq!(result, (3.5, -7.0, 10.5, -14.0))
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = (1.0, -2.0, 3.0, -4.0);
        let result = scalar_multiplication(a, 0.5);

        assert_eq!(result, (0.5, -1.0, 1.5, -2.0))
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = (1.0, -2.0, 3.0, -4.0);
        let result = scalar_division(a, 2.0);

        assert_eq!(result, (0.5, -1.0, 1.5, -2.0))
    }

    #[test]
    fn compute_magnitude_of_vector_1_0_0() {
        let a = (1.0, 0.0, 0.0, 0.0);
        let result = vector_magnitude(a);

        assert_eq!(result, 1.0);
    }

    #[test]
    fn compute_magnitude_of_vector_0_1_0() {
        let a = (0.0, 1.0, 0.0, 0.0);
        let result = vector_magnitude(a);

        assert_eq!(result, 1.0);
    }

    #[test]
    fn compute_magnitude_of_vector_0_0_1() {
        let a = (0.0, 0.0, 1.0, 0.0);
        let result = vector_magnitude(a);

        assert_eq!(result, 1.0);
    }

    #[test]
    fn compute_magnitude_of_vector_1_2_3() {
        let a = (1.0, 2.0, 3.0, 0.0);
        let result = vector_magnitude(a);
        let answer = 14.0_f64;
        assert_eq!(result, answer.sqrt());
    }

    #[test]
    fn compute_magnitude_of_vector_negative_1_2_3() {
        let a = (-1.0, -2.0, -3.0, 0.0);
        let result = vector_magnitude(a);
        let answer = 14.0_f64;
        assert_eq!(result, answer.sqrt());
    }

    #[test]
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let a = (4.0, 0.0, 0.0, 0.0);
        let result = normalize_vector(a);

        assert_eq!(result, (1.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let a = (1.0, 2.0, 3.0, 0.0);
        let result = normalize_vector(a);

        assert_eq!(result, (1.0/14.0_f64.sqrt(), 2.0/14.0_f64.sqrt(), 3.0/14.0_f64.sqrt(), 0.0));
    }

    #[test]
    fn normalizing_vector_1_2_3_then_magnitude() {
        let a = (1.0, 2.0, 3.0, 0.0);
        let normalized = normalize_vector(a);
        let result = vector_magnitude(normalized);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let a = (1.0, 2.0, 3.0, 0.0);
        let b = (2.0, 3.0, 4.0, 0.0);
        let result = vector_dot(a,b);

        assert_eq!(result, 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = (1.0, 2.0, 3.0, 0.0);
        let b = (2.0, 3.0, 4.0, 0.0);

        let result_a = vector_cross(a, b);
        assert_eq!(result_a, (-1.0, 2.0, -1.0, 0.0));

        let result_b = vector_cross(b, a);
        assert_eq!(result_b, (1.0, -2.0, 1.0, 0.0))

    }

}