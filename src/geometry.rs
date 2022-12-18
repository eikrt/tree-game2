use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
#[derive(Clone, PartialEq)]
pub enum LineType {
    Branch,
    Leaf,
    Dirt,
}
#[derive(Clone)]
pub struct Line {
    pub points: ((f32, f32), (f32, f32)),
    pub color: Color,
    pub grows: u32,
    pub generation: u32,
    pub main_branch: bool,
    pub line_type: LineType,
    pub leafs: Vec<Line>,
    pub lifetime: u64,
}
impl Line {
    pub fn new(
        main_branch: bool,
        generation: u32,
        line_type: LineType,
        color: Color,
        points: ((f32, f32), (f32, f32)),
    ) -> Line {
        Line {
            color: color,
            points: points,
            grows: 0,
            line_type: line_type,
            main_branch: main_branch,
            leafs: Vec::new(),
            lifetime: 0,
            generation: generation,
        }
    }
    pub fn tick(&mut self, delta: u64) {
        self.lifetime += delta;

        for leaf in &mut self.leafs {
            leaf.tick(delta);
        }
        match &self.line_type {
            LineType::Branch => {
                if self.lifetime % 2000 == 0 {
                    self.grow();
                }
                if self.lifetime % 6000 == 0 {
                    self.branch();
                }
            }
            _ => {}
        }
    }
    pub fn grow(&mut self) {
        if self.leafs.len() > 2 {
            return;
        }
        self.grows += 1;
        let grow_dir = (self.points.0 .1 - self.points.1 .1)
            .atan2((self.points.1 .0 - self.points.0 .0))
            + 3.14 / 2.0;
        self.points.1 .0 += grow_dir.sin() * 2.0;
        self.points.1 .1 += grow_dir.cos() * 2.0;
    }
    pub fn branch(&mut self) {
        if self.generation > 3 {
            return;
        }
        if self.leafs.len() > 2 {
            return;
        }
        let mut color = Color::RGB(80, 65, 40);
        let mut line_type = LineType::Branch;
        if self.generation > 2 {
            color = Color::RGB(0, 255, 0);
            line_type = LineType::Leaf;
        }

        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(-4.0..4.0); // generates a float between 0 and 1
        let y: f32 = rng.gen_range(0.0..4.0); // generates a float between 0 and 1
        self.leafs.push(Line::new(
            false,
            self.generation + 1,
            line_type,
            color,
            (
                (self.points.1 .0, self.points.1 .1),
                (self.points.1 .0 - x, self.points.1 .1 - y),
            ),
        ));
    }
    pub fn get_points(&self) -> (Point, Point) {
        (
            Point::new(self.points.0 .0 as i32, self.points.0 .1 as i32),
            Point::new(self.points.1 .0 as i32, self.points.1 .1 as i32),
        )
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);
        canvas.draw_line(self.get_points().0, self.get_points().1);
        for leaf in &self.leafs {
            leaf.draw(canvas);
        }
    }
}
pub struct Mesh {
    pub lines: Vec<Line>,
}
