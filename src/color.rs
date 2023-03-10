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
    