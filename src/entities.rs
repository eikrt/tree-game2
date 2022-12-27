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
pub struct Camera {
    pub pos: (f32,f32),
    pub vel: (f32,f32)
}
impl Camera {
    pub fn new() -> Camera{
        Camera {
            pos: (0.0,0.0),
            vel: (0.0,0.0),
        } 
    }
}
#[derive(Clone)]
pub struct Entity {
    pub mesh: Mesh,
    pub entity_type: EntityType,
    pub lifetime: u64,
    pub vel: (f32, f32),
    pub is_collidable: bool,
    pub is_collide_agent: bool,
}
impl Entity {
    pub fn new(entity_type: EntityType, mesh: Mesh, is_collidable: bool, is_collide_agent: bool) -> Entity {
        Entity {
            mesh: mesh,
            entity_type: entity_type,
            lifetime: 0,
            vel: (0.0, 0.0),
            is_collidable: is_collidable,
            is_collide_agent: is_collide_agent,
        }
    }
    pub fn tick(&mut self, delta: u64) {
        self.lifetime += delta;
        for line in &mut self.mesh.lines {
            match line.signal {
                SignalType::ConvertToTree => {
                    self.is_collidable = false;
                    self.is_collide_agent = false;
                }
                _ => {}
            }
            line.vel = self.vel;
            line.tick(delta);
        }
    }
    pub fn draw(&self, canvas: &mut WindowCanvas, camera: &Camera) {
        for line in &self.mesh.lines {
            canvas.set_draw_color(line.color);
            line.draw(canvas, camera);
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
