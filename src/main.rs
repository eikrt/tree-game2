use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseButton, MouseState};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use lerp::Lerp;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use tree::entities::*;
use tree::generator::*;
use tree::geometry::*;
use tree::sound::*;
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

    let audio_subsystem = sdl_context.audio().unwrap();
    let mut entities: HashMap<u32, Entity> = HashMap::new();
    let mut selection_index = 0;
    let player = Entity::new(
        EntityType::Ufo,
        Generator::generate_ufo((300.0, 300.0)),
        true,
    );
    entities.insert(0, player);
    entities.insert(
        entities.values().len() as u32 + 1,
        Entity::new(EntityType::Terrain, Generator::generate_terrain(), false),
    );
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1), // mono
        samples: None,     // default sample size
    };
    let device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.05,
        })
        .unwrap();

    // Start playback
    let mut event_pump = sdl_context.event_pump()?;
    let mut lifetime = 0;

    let mut compare_time = SystemTime::now();
    let mut wasd = (false, false, false, false);
    'running: loop {
        let delta_o = SystemTime::now().duration_since(compare_time).unwrap();
        compare_time = SystemTime::now();

        let delta = delta_o.as_millis() as u64;
        lifetime += delta;
        canvas.clear();
        let mouse_state = event_pump.mouse_state();
        match handle_input(&mouse_state, &mut event_pump, &mut entities, &mut canvas, &mut wasd) {
            true => {}
            false => break 'running,
        }
        for (id, e) in entities.iter_mut() {
            e.draw(&mut canvas);
            e.tick(delta);
        }

        let collide_entities = entities.clone();
        for (id, e) in entities.iter_mut() {
            if !e.is_collidable {
                continue;
            }
            for (other_id, other_e) in collide_entities.iter() {
                if id == other_id {
                    continue;
                }
                e.collide(other_e);
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.present();
        if lifetime % 10 == 0 {
            device.pause();
        }
        ::std::thread::sleep(Duration::new(0, 10));
    }

    Ok(())
}
fn handle_input(
    mouse_state: &sdl2::mouse::MouseState,
    event_pump: &mut EventPump,
    entities: &mut HashMap<u32, Entity>,
    canvas: &mut WindowCanvas,
    wasd: &mut (bool,bool,bool,bool)
) -> bool {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    let mut intersect_point = Point::new(0, 0);
    /*for (id, e) in entities.iter() {
        intersect_point =
            match RayCast::cast_ray(Point::new(mouse_state.x(), mouse_state.y()), 500, &e.mesh) {
                Some(p) => p.1,
                None => intersect_point,
            }
    }*/
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                wasd.0 = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                wasd.1 = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                wasd.2 = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                wasd.3 = true;
            }

            Event::KeyUp {
                keycode: Some(Keycode::W),
                ..
            } => {
                wasd.0 = false;
            }
            Event::KeyUp {
                keycode: Some(Keycode::A),
                ..
            } => {
                wasd.1 = false;
            }
            Event::KeyUp {
                keycode: Some(Keycode::S),
                ..
            } => {
                wasd.2 = false;
            }
            Event::KeyUp {
                keycode: Some(Keycode::D),
                ..
            } => {
                wasd.3 = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                let middle_point = (
                    entities.get_mut(&0).unwrap().mesh.lines[0].points.0 .0,
                    entities.get_mut(&0).unwrap().mesh.lines[0].points.0 .1,
                );
                entities.insert(
                    entities.values().len() as u32 + 1,
                    Entity::new(
                        EntityType::Seed,
                        Mesh {
                            lines: vec![Line::new(
                                true,
                                0,
                                LineType::Seed,
                                Color::RGB(80, 65, 40),
                                (middle_point, (middle_point.0 + 0.0, middle_point.1 - 4.0)),
                                true,
                            )],
                        },
                        true,
                    ),
                );
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => {
            }
            _ => {}
        }
    }
    if wasd.0 {
        entities.get_mut(&0).unwrap().vel.1 = -50.4;
    }
    if wasd.1 {
        entities.get_mut(&0).unwrap().vel.0 = -50.4;
    }
    if wasd.2 {
        entities.get_mut(&0).unwrap().vel.1 = 50.4;
    }
    if wasd.3 {
        entities.get_mut(&0).unwrap().vel.0 = 50.4;
    }
    if !wasd.0 && !wasd.2 {
        entities.get_mut(&0).unwrap().vel.1 = entities.get_mut(&0).unwrap().vel.1.lerp(0.0, 0.01);
    }
    if !wasd.1 && !wasd.3 {
        entities.get_mut(&0).unwrap().vel.0 = entities.get_mut(&0).unwrap().vel.0.lerp(0.0, 0.01);
    }
    true
}
fn play_sound(lines: &Vec<Line>, sound_device: &AudioDevice<SquareWave>) {
    for line in lines {
        for p_q in &line.sound_queue {
            sound_device.resume();
        }
        play_sound(&line.leafs, sound_device);
    }
}
fn terminate_sound_if_empty(lines: &Vec<Line>, sound_device: &AudioDevice<SquareWave>) -> bool {
    let mut empty = true;
    for line in lines {
        for p_q in &line.sound_queue {
            if *p_q {
                empty = false;
            }
            empty = terminate_sound_if_empty(&line.leafs, sound_device);
        }
    }
    empty
}
