use crate::geometry::*;
use noise::{NoiseFn, Perlin, Seedable};
use sdl2::pixels::Color;
use sdl2::rect::Point;
pub struct Generator {}
impl Generator {
    pub fn generate_terrain() -> Mesh {
        let perlin = Perlin::new(1);

        let mut lines = vec![];
        let mut p_val = 0.0;
        let planet_pos = (1000.0, 0.0);

        let r = 1024.0;
        let mut points = (
            (
                planet_pos.0 + 0.0_f32.cos() * r,
                planet_pos.1 + 0.0_f32.sin() * r,
            ),
            (
                planet_pos.0 + 0.0_f32.cos() * r,
                planet_pos.1 + 0.0_f32.sin() * r,
            ),
        );
        for i in 0..628 {
            p_val = (perlin.get([i as f64, 37.7, 2.8]) * 32.0) as f32;
            points = (
                (points.1 .0, points.1 .1),
                (
                    planet_pos.0 + ((i as f32) / 100.0).cos() * (r + p_val),
                    planet_pos.1 + ((i as f32) / 100.0).sin() * (r + p_val),
                ),
            );
            lines.push(Line::new(
                true,
                1,
                LineType::Dirt,
                Color::RGB(100, 100, 55),
                points,
                false,
            ));
        }
        Mesh { lines: lines }
    }
    pub fn generate_ufo(pos: (f32, f32)) -> Mesh {
        let mut lines = vec![];
        for i in 0..4 {
            let points = (pos, (pos.0 + i as f32 * 4.0, pos.1));
            lines.push(Line::new(
                true,
                1,
                LineType::Titanium,
                Color::RGB(100, 0, 0),
                points,
                false,
            ));
        }
        Mesh { lines: lines }
    }
}
