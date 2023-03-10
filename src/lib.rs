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

mod helpers {
    pub fn equal(a: f64, b: f64) -> bool {
        let factor = 10.0f64.powi(6 as i32);
        let a = (a * factor).trunc();
        let b = (b * factor).trunc();
        a == b
    }
}

pub mod canvas {
    use std::fs::File;
    use std::io::prelude::*;

    use crate::color::Color;

    pub struct Canvas {
        pub width: i32,
        pub height: i32,
        pub img: Vec<Vec<Color>>,
    }

    impl Canvas {
        pub fn new(width: i32, height: i32) -> Canvas {
            let mut img = vec![];
            for _ in 0..height {
                img.push(vec![Color::new((0.0, 0.0, 0.0)); width as usize]);
            }

            Canvas { width, height, img }
        }

        pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {

            if x < (self.height as usize) && y < (self.width as usize) {
                self.img[x][y] = color;
            }
        }

        pub fn write_pixel_f64(&mut self, x:f64, y: f64, color: Color) {
            let x = x as i32;
            let x = self.height - x;
            let x = x as usize;
            let y = y as usize;

            self.write_pixel(x, y, color);
        }

        pub fn write_to_ppm(&self) {
            let header = format!("P3\n{} {}\n255", self.width, self.height);
            let body = self.to_string();

            let content = format!("{}\n{}", header, body);
            let mut file = File::create("img.ppm").unwrap();
            file.write_all(content.as_bytes()).unwrap();

        }

        fn to_string(&self) -> String{
            let mut content = "".to_owned(); 
            for i in 0..self.height {
                let row = &self.img[i as usize];
                    content.push_str(&self.row_to_string(row));
            }
            content.push_str("\n");
            content
        }

        fn row_to_string(&self, row: &Vec<Color>) -> String {
            let mut row_string = "".to_owned();
            let mut index = 0;
            for color in row {
                let c_str = &color.to_string();

                if index < 70 - c_str.len() -1 {
                    row_string.push_str(c_str);
                    index += c_str.len();
                } else {
                    let newline_str = format!("\n{}",c_str);
                    row_string.push_str(&newline_str);
                    index = 0;
                }
            }

            row_string
        }
    }
}

#[cfg(test)]
mod canvas_tests {
    use crate::{canvas::Canvas, color::Color};

    #[test]
    fn constructing_ppm_header() {
        let canvas = Canvas::new(5,3);
        canvas.write_to_ppm(); 
    }

    #[test]
    fn create_canvas() {
        let a = Canvas::new(2, 3);

        assert_eq!(a.width, 2);
        assert_eq!(a.height, 3);
        assert_eq!(a.img[1][0], Color::new((0.0, 0.0, 0.0)));
    }

    #[test]
    fn write_pixel_on_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new((1.0,0.0,0.0));

        canvas.write_pixel(2, 3, red);

        assert_eq!(canvas.img[2][3], Color::new((1.0, 0.0, 0.0)));
    }
}

pub mod color {
    use crate::helpers;

    #[derive(Debug, Copy, Clone)]
    pub struct Color {pub red: f64, pub green: f64, pub blue: f64 }

    impl Color {
        pub fn new(c: (f64, f64,f64)) -> Color {
            Color { red: c.0, green: c.1, blue: c.2 }
        }

        pub fn add(a: Color, b: Color) -> Color {
            Color {red: a.red + b.red, green: a.green + b.green, blue: a.blue + b.blue }
        }
        
        pub fn subtract(a: Color, b: Color) -> Color {
            Color { red: a.red - b.red, green: a.green - b.green, blue: a.blue - b.blue }
        }

        pub fn scalar_multiplication(a: Color, b: f64) -> Color {
            Color { red: a.red * b, green: a.green * b, blue: a.blue * b }
        }

        pub fn multiply(a: Color, b: Color) -> Color {
            Color { red: a.red * b.red, green: a.green * b.green, blue: a.blue * b.blue }
        }

        pub fn to_string(&self) -> String {
            let s = format!("{} {} {} ", self.red * 255.0, self.green * 255.0, self.blue * 255.0);

            s
        }
    }

    impl PartialEq for Color {
        fn eq(&self, other: &Self) -> bool {
            helpers::equal(self.red, other.red) && helpers::equal(self.green, other.green) && helpers::equal(self.blue, other.blue)
        }
    }
}

#[cfg(test)]
mod color_tests {
    use crate::color::*;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = Color::new((-0.5, 0.4, 1.7));

        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4,);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn multiply_colors() {
        let a = Color::new((1.0, 0.2, 0.4));
        let b = Color::new((0.9, 1.0, 0.1));

        let result = Color::multiply(a, b);
        let wish = Color::new((0.9, 0.2, 0.04));
        assert_eq!(result, wish);
    }
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