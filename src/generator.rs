use crate::geometry::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use noise::{NoiseFn, Perlin, Seedable};
pub struct Generator {}
impl Generator {
    pub fn generate_terrain() -> Mesh {
        let perlin = Perlin::new(1);

        let mut lines = vec![];
        let mut points = ((0.0,400.0), (0.0,400.0));
        let mut p_val = 0.0;
        for i in 0..96 {
            p_val = (perlin.get([i as f64, 37.7, 2.8]) * 32.0) as f32;
            points = ((points.1.0, points.1.1), (i as f32 * 12.0, 400.0 - p_val));
            lines.push(Line::new(
                true,
                LineType::Dirt,
                Color::RGB(100, 100, 55),
                points,
            ));

        }
        Mesh { lines: lines }
    }
    
}
