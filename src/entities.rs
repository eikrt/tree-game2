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
        self.live(); 
    }
    pub fn live(&mut self) {
        if self.lifetime > 150000 {
            return;
        }
        match &self.entity_type {
            EntityType::Tree => {
                if self.lifetime % 4000 == 0 {
                    self.grow();
                }
                if self.lifetime % 10000 == 0 {
                    self.branch();
                }
            }
            _ => {}
        }
    }
    pub fn grow(&mut self) {
        for line in &mut self.mesh.lines {

            if line.branches > 1 && !line.main_branch {
                continue;
            }
            if line.line_type == LineType::Leaf && line.grows > 3 {
                continue;
            }
            line.grow();
        }
    }
    pub fn branch(&mut self) {
        let mut rng = rand::thread_rng();
        let mut push_lines = Vec::new();
        for line in &mut self.mesh.lines {
            let x: f32 = rng.gen_range(-4.0..4.0); // generates a float between 0 and 1
            let y: f32 = rng.gen_range(0.0..4.0); // generates a float between 0 and 1
            line.branch();
            let mut color = Color::RGB(80,65,40);
            let mut line_type = LineType::Branch;
            if line.branches > 1 && !line.main_branch {
                color = Color::RGB(0,255,0);
                line_type = LineType::Leaf;
            }
            if line.branches > 2 && !line.main_branch {
                continue;
            }
            push_lines.push(Line::new(
                false,
                line_type,
                color,
                (
                    (line.points.1 .0, line.points.1 .1),
                    (line.points.1 .0 - x, line.points.1 .1 - y),
                ),
            ));
        }
        for mut line in push_lines {
            self.mesh.lines.push(line.clone());
        }
    }
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        for line in self.mesh.lines.iter() {
            canvas.set_draw_color(line.color);
            canvas.draw_line(line.get_points().0, line.get_points().1);
        }
    }
}
