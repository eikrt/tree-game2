use sdl2::rect::Point;
use sdl2::pixels::Color;
pub struct Line {
    pub points: (Point, Point),
    pub color: Color 
}
pub struct Mesh {
    pub lines: Vec<Line>,
}
