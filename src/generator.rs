use crate::geometry::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use noise::{NoiseFn, Perlin, Seedable};
pub struct Generator {}
impl Generator {
    pub fn generate_terrain() -> Mesh {
        let perlin = Perlin::new(1);

        let mut lines = vec![];
        let mut points = (Point::new(0,400), Point::new(0, 400));
        let mut p_val = 0;
        for i in 0..96 {
            p_val = (perlin.get([i as f64, 37.7, 2.8]) * 32.0) as i32;
            points = (points.1, Point::new(i * 12, 400 - p_val));
            lines.push(Line {
                color: Color::RGB(100, 100, 55),
                points: points,
            });

        }
        Mesh { lines: lines }
    }
    
}
