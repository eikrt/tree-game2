use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseButton, MouseState};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use std::collections::HashSet;
use std::time::Duration;
use tree::entities::*;
use tree::generator::*;
use tree::geometry::*;
use tree::utils::*;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Tree Game", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
    let mut entities: Vec<Entity> = Vec::new();
    let mut selection_index = 0;

    entities.push(Entity::new(
        EntityType::Terrain,
        Generator::generate_terrain(),
    ));
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        let delta = 10;
        canvas.clear();
        let mouse_state = event_pump.mouse_state();
        match handle_input(&mouse_state, &mut event_pump, &mut entities, &mut canvas) {
            true => {}
            false => break 'running,
        }
        for e in &mut entities {
            e.draw(&mut canvas);
            e.tick(delta);
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 10));
    }

    Ok(())
}
fn handle_input(
    mouse_state: &sdl2::mouse::MouseState,
    event_pump: &mut EventPump,
    entities: &mut Vec<Entity>,
    canvas: &mut WindowCanvas,
) -> bool {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    let mut intersect_point = Point::new(0, 0);
    for e in entities.iter() {
        intersect_point =
            match RayCast::cast_ray(Point::new(mouse_state.x(), mouse_state.y()), 500, &e.mesh) {
                Some(p) => p.1,
                None => intersect_point,
            }
    }

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return false;
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => {
                entities.push(Entity::new(
                    EntityType::Tree,
                    Mesh {
                        lines: vec![Line::new(
                            true,
                            0,
                            LineType::Branch,
                            Color::RGB(80, 65, 40),
                            (
                                (intersect_point.x() as f32, intersect_point.y() as f32),
                                (
                                    intersect_point.x() as f32,
                                    intersect_point.y() as f32 - 12.0,
                                ),
                            ),
                        )],
                    },
                ));
            }
            _ => {}
        }
    }
    canvas.fill_rect(Rect::new(
        intersect_point.x(),
        intersect_point.y() - 2,
        4,
        4,
    ));
    true
}
