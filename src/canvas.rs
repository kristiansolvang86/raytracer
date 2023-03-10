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