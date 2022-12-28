use crate::consts_and_vars::*;
use crate::entities::*;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

#[derive(Clone, PartialEq)]
pub enum SignalType {
    ConvertToTree,
    Empty,
}
#[derive(Clone, PartialEq)]
pub enum LineType {
    Branch,
    Leaf,
    Dirt,
    Titanium,
    Seed,
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
    pub sound_queue: Vec<bool>,
    pub vel: (f32, f32),
    pub affected_by_gravity: bool,
    pub gravity_factor: f32,
    pub gravity_points: Vec<(f32, f32)>,
    pub deleted: bool,
    pub step: (f32, f32),
    pub grow_time: u64,
    pub grow_change: u64,
    pub branch_time: u64,
    pub branch_change: u64,
    pub signal: SignalType,
}
impl Line {
    pub fn new(
        main_branch: bool,
        generation: u32,
        line_type: LineType,
        color: Color,
        points: ((f32, f32), (f32, f32)),
        affected_by_gravity: bool,
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
            sound_queue: Vec::new(),
            vel: (0.0, 0.0),
            affected_by_gravity: affected_by_gravity,
            gravity_points: Vec::new(),
            gravity_factor: 0.0,
            deleted: false,
            step: (0.0, 0.0),
            grow_time: 200,
            grow_change: 0,
            branch_time: 500,
            branch_change: 0,
            signal: SignalType::Empty,
        }
    }
    pub fn tick(&mut self, delta: u64) {
        if self.deleted {
            return;
        }
        self.lifetime += delta;
        if self.affected_by_gravity {
            self.gravity_factor += GRAVITY;
            for p in &self.gravity_points {
                let angle = (self.points.0 .1 - p.1).atan2(self.points.0 .0 - p.0) + 3.14;
                self.vel.0 += angle.cos() * self.gravity_factor;
                self.vel.1 += angle.sin() * self.gravity_factor;
            }
        }
        self.trigger_move(delta);
        for leaf in &mut self.leafs {
            leaf.tick(delta);
        }
        match &self.line_type {
            LineType::Branch => {
                self.grow(delta);
                self.branch(delta);
            }
            _ => {}
        }
    }
    pub fn add_gravity_point(&mut self, point: (f32, f32)) {
        self.gravity_points.push(point);
        for leaf in &mut self.leafs {
            leaf.add_gravity_point(point);
        }
    }
    pub fn trigger_move(&mut self, delta: u64) {
        self.step = (
            (self.vel.0 * delta as f32) / 1000.0,
            (self.vel.1 * delta as f32) / 1000.0,
        );
        self.points.0 .0 += self.step.0;
        self.points.0 .1 += self.step.1;
        self.points.1 .0 += self.step.0;
        self.points.1 .1 += self.step.1;
    }
    pub fn grow(&mut self, delta: u64) {
        self.grow_change += delta;
        if self.grow_change > self.grow_time {
            self.grow_change = 0;
        } else {
            return;
        }
        if self.leafs.len() > 2 {
            return;
        }
        self.grows += 1;
        let grow_dir = (self.points.0 .1 - self.points.1 .1)
            .atan2(self.points.1 .0 - self.points.0 .0)
            + 3.14 / 2.0;
        self.points.1 .0 += grow_dir.sin() * 2.0;
        self.points.1 .1 += grow_dir.cos() * 2.0;
    }
    pub fn branch(&mut self, delta: u64) {
        self.branch_change += delta;
        if self.branch_change > self.branch_time {
            self.branch_change = 0;
        } else {
            return;
        }
        if self.generation > 3 {
            return;
        }
        if self.leafs.len() > 2 {
            return;
        }
        let mut color = Color::RGB(80, 65, 40);
        let mut line_type = LineType::Branch;
        self.sound_queue.push(true);
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
            self.affected_by_gravity,
        ));
    }
    pub fn get_points(&self) -> (Point, Point) {
        (
            Point::new(self.points.0 .0 as i32, self.points.0 .1 as i32),
            Point::new(self.points.1 .0 as i32, self.points.1 .1 as i32),
        )
    }
    pub fn get_points_c(&self, camera: &Camera) -> (Point, Point) {
        (
            Point::new(
                self.points.0 .0 as i32 - camera.pos.0 as i32,
                self.points.0 .1 as i32 - camera.pos.1 as i32,
            ),
            Point::new(
                self.points.1 .0 as i32 - camera.pos.0 as i32,
                self.points.1 .1 as i32 - camera.pos.1 as i32,
            ),
        )
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, camera: &Camera) {
        if self.deleted {
            return;
        }
        canvas.set_draw_color(self.color);
        canvas.draw_line(self.get_points_c(camera).0, self.get_points_c(camera).1);
        for leaf in &self.leafs {
            leaf.draw(canvas, camera);
        }
    }
    pub fn collide(&mut self, other_l: &Line) {
        for i in (0)..((self.step.1 * 10.0) as i32) {
            let intersects = intersect_line(
                (
                    self.points.0 .0,
                    self.points.0 .1 + self.step.1 as f32 / 10.0,
                ),
                (
                    self.points.1 .0,
                    self.points.1 .1 + self.step.1 as f32 / 10.0,
                ),
                other_l.points.0,
                other_l.points.1,
            );
            if intersects {
                self.trigger_collision(other_l);
            }
        }
        for leaf in &mut self.leafs {
            leaf.collide(other_l);
        }
    }
    pub fn trigger_collision(&mut self, other_l: &Line) {
        if other_l.line_type != LineType::Dirt {
            return;
        }
        match self.line_type {
            LineType::Titanium => {
                self.deleted = true;
            }
            LineType::Seed => {
                self.convert(LineType::Branch);
            }
            _ => {}
        }
    }
    pub fn convert(&mut self, l_t: LineType) {
        match l_t {
            LineType::Branch => {
                self.affected_by_gravity = false;
                self.line_type = LineType::Branch;
                self.signal = SignalType::ConvertToTree;
            }
            _ => {}
        }
    }
}
#[derive(Clone)]
pub struct Mesh {
    pub lines: Vec<Line>,
    pub center_point: (f32, f32),
}
impl Mesh {
    pub fn new(lines: Vec<Line>) -> Mesh {
        let mut xs = 0.0;
        let mut ys = 0.0; 
        let mut i = 0.0;
        for l in &lines {
            xs += l.points.0.0;
            ys += l.points.0.1;
            xs += l.points.1.0;
            ys += l.points.1.1;
            i += 2.0;
        }
        xs /= i;
        ys /= i;
        Mesh {
            lines: lines,
            center_point: (xs, ys),
        }
    }
}

/*fn ccw(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> bool {
    (c.1 - a.1) * (b.0 - a.0) > (b.1 - a.1) * (c.0 - a.0)
}

fn intersect_line(a: (f32, f32), b: (f32, f32), c: (f32, f32), d: (f32, f32)) -> bool {
    ccw(a, c, d) != ccw(b, c, d) && ccw(a, b, c) != ccw(a, b, d)
}*/
fn intersect_line(
    line1: (f32, f32),
    line2: (f32, f32),
    line3: (f32, f32),
    line4: (f32, f32),
) -> bool {
    let (x1, y1) = line1;
    let (x2, y2) = line2;
    let (x3, y3) = line3;
    let (x4, y4) = line4;

    let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    // If den == 0, it means the lines are colinear
    if den == 0.0 {
        return false;
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

    t > 0.0 && t < 1.0 && u > 0.0 && u < 1.0
}
