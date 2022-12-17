use sdl2::rect::Point;
use sdl2::pixels::Color;

#[derive(Clone, PartialEq)]
pub enum LineType {
    Branch,
    Leaf,
    Dirt,
}
#[derive(Clone)]
pub struct Line {
    pub points: ((f32,f32), (f32,f32)),
    pub color: Color, 
    pub grows: u32,
    pub branches: u32,
    pub main_branch: bool,
    pub line_type: LineType,
}
impl Line {
    pub fn new(main_branch: bool, line_type: LineType,color: Color, points: ((f32,f32), (f32,f32))) -> Line {
        Line {
            color: color, 
            points: points,
            grows: 0,
            branches: 0,
            line_type: line_type,
            main_branch: main_branch,
        }  
    }
    pub fn grow(&mut self) {
        self.grows += 1;
        let grow_dir = ((self.points.0.1 - self.points.1.1)).atan2((self.points.1.0 - self.points.0.0)) + 3.14 / 2.0;
        self.points.1.0 += grow_dir.sin() * 2.0;
        self.points.1.1 += grow_dir.cos() * 2.0;
    }
    pub fn branch(&mut self) {
        self.branches += 1;
    }
    pub fn get_points(&self) -> (Point, Point) {
        (Point::new(self.points.0.0 as i32, self.points.0.1 as i32), Point::new(self.points.1.0 as i32, self.points.1.1 as i32))
    }
}
pub struct Mesh {
    pub lines: Vec<Line>,
}
