use crate::geometry::*;
use sdl2::render::WindowCanvas;
pub struct Entity {
    pub mesh: Mesh,
}
impl Entity {
    pub fn new(mesh: Mesh) -> Entity {
        Entity {
            mesh: mesh,
        }
    }
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        for line in self.mesh.lines.iter() {
            canvas.set_draw_color(line.color);
            canvas.draw_line(line.points.0, line.points.1);
        }
    }
}
