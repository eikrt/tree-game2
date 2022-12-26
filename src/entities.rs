use crate::geometry::*;
use rand::prelude::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub enum EntityType {
    Tree,
    Terrain,
    Ufo,
    Seed,
}
#[derive(Clone)]
pub struct EntityGroup {
    pub entities: HashMap<(i32,i32), HashMap<u32,Entity>>
}
impl EntityGroup {
    pub fn insert_entity(&mut self, coords: (i32,i32), entity: Entity) {
        let len: u32 = match self.entities.clone().get(&coords) {
            Some(e) => {
                e.len() as u32
            }
            None => {
                self.entities.insert(coords, HashMap::new());
                0
            }
        };
        self.entities.get_mut(&coords).unwrap().insert(len+1, entity);
    }
}

#[derive(Clone)]
pub struct Entity {
    pub mesh: Mesh,
    pub entity_type: EntityType,
    pub lifetime: u64,
    pub vel: (f32, f32),
    pub is_collidable: bool,
}
impl Entity {
    pub fn new(entity_type: EntityType, mesh: Mesh, is_collidable: bool) -> Entity {
        Entity {
            mesh: mesh,
            entity_type: entity_type,
            lifetime: 0,
            vel: (0.0, 0.0),
            is_collidable: is_collidable,
        }
    }
    pub fn tick(&mut self, delta: u64) {
        self.lifetime += delta;
        for line in &mut self.mesh.lines {
            line.vel = self.vel;
            line.tick(delta);
        }
    }
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        for line in &self.mesh.lines {
            canvas.set_draw_color(line.color);
            line.draw(canvas);
        }
    }
    pub fn collide(&mut self, other_entity: &Entity) {
        for mut l in &mut self.mesh.lines {
            for other_l in &other_entity.mesh.lines {
                l.collide(&other_l);
            }
        }
    }
}
