use crate::geometry::*;
use rand::prelude::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
#[derive(Debug)]
pub enum EntityType {
    Tree,
    Terrain,
}
pub struct Entity {
    pub mesh: Mesh,
    pub entity_type: EntityType,
    pub lifetime: u64,
}
impl Entity {
    pub fn new(entity_type: EntityType, mesh: Mesh) -> Entity {
        Entity {
            mesh: mesh,
            entity_type: entity_type,
            lifetime: 0,
        }
    }
    pub fn tick(&mut self, delta: u64) {
        self.lifetime += delta;
        for line in &mut self.mesh.lines {
            line.tick(delta);
        }
    }
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        for line in &self.mesh.lines {
            canvas.set_draw_color(line.color);
            line.draw(canvas);
        }

    }
}
