use crate::geometry::*;
use sdl2::rect::Point;
use sdl2::rect::Rect;
pub struct RayCast {}

impl RayCast {
    pub fn cast_ray(start_point: Point, length: u32, mesh: &Mesh) -> Option<(Point,Point)> {
        let mut i_p = None;
        for line in mesh.lines.iter() {
                match Rect::new(start_point.x(), start_point.y(), 1, length)
                    .intersect_line(line.get_points().0, line.get_points().1) {
                        Some(p) => i_p = Some(p),
                        None => {}
                    }
        }
        i_p
    }
}
