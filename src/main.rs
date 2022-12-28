use lerp::Lerp;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
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
use tree::consts_and_vars::*;
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
    let mut player = Entity::new(
        EntityType::Ufo,
        Generator::generate_ufo((300.0, 300.0)),
        true,
        true,
    );
    let mut entities = EntityGroup {
        entities: HashMap::new(),
    };
    entities.insert_entity((0, 0), player);

    //entities.insert(0, player);
    //
    let mut gravity_points = Vec::new();
    let mut home_planet = Entity::new(
        EntityType::Terrain,
        Generator::generate_terrain(),
        true,
        false,
    );

    gravity_points.push(home_planet.mesh.center_point);
    entities.insert_entity((0, 0), home_planet);
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
    let mut camera = Camera::new();
    'running: loop {
        let delta_o = SystemTime::now().duration_since(compare_time).unwrap();
        compare_time = SystemTime::now();

        let delta = delta_o.as_millis() as u64;
        lifetime += delta;

        //let player_chunk_pos = ((player_pos.0 / CHUNK_SIZE as f32).floor() as i32, (player_pos.1 / CHUNK_SIZE as f32).floor() as i32);
        let player_chunk_pos = (0, 0);
        let mut entities_hm = entities.entities.get_mut(&player_chunk_pos).unwrap();

        let player_clone = entities_hm.get_mut(&1).unwrap().clone();
        let player_pos = (
            player_clone.mesh.lines[0].points.0 .0,
            player_clone.mesh.lines[0].points.0 .1,
        );
        let player_relative_pos = (player_pos.0 - camera.pos.0, player_pos.1 - camera.pos.1);
        camera.tick(delta);
        canvas.clear();
        let mouse_state = event_pump.mouse_state();
        match handle_input(
            &mouse_state,
            &mut event_pump,
            &mut entities,
            &mut canvas,
            &mut camera,
            &mut wasd,
            player_chunk_pos,
            &gravity_points,
        ) {
            true => {}
            false => break 'running,
        }
        for (id, e) in entities
            .entities
            .get_mut(&player_chunk_pos)
            .unwrap()
            .iter_mut()
        {
            e.draw(&mut canvas, &camera);
            e.tick(delta);
        }

        let collide_entities = entities.entities.get(&player_chunk_pos).unwrap().clone();
        for (id, e) in entities
            .entities
            .get_mut(&player_chunk_pos)
            .unwrap()
            .iter_mut()
        {
            if !e.is_collide_agent {
                continue;
            }
            for (other_id, other_e) in collide_entities.iter() {
                if !other_e.is_collidable {
                    continue;
                }
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
    entities: &mut EntityGroup,
    canvas: &mut WindowCanvas,
    camera: &mut Camera,
    wasd: &mut (bool, bool, bool, bool),
    player_chunk_pos: (i32, i32),
    gravity_points: &Vec<(f32, f32)>,
) -> bool {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    let mut intersect_point = Point::new(0, 0);

    let entities_len = entities
        .entities
        .get(&player_chunk_pos)
        .unwrap()
        .values()
        .len() as u32;
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
                let mut entities_hm = entities.entities.get_mut(&player_chunk_pos).unwrap();

                let player = entities_hm.get_mut(&1).unwrap();
                let middle_point = (
                    player.mesh.lines[0].points.0 .0,
                    player.mesh.lines[0].points.0 .1,
                );

                let p = match gravity_points.last() {
                    Some(s) => s.clone(),
                    None => (0.0, 0.0),
                };

                let angle = (middle_point.1 - p.1).atan2(middle_point.0 - p.0);
                let mut seed = Entity::new(
                    EntityType::Seed,
                    Generator::generate_seed(middle_point, angle),
                    true,
                    true,
                );
                for g_p in gravity_points {
                    seed.add_gravity_point((g_p.0, g_p.1));
                }
                entities_hm.insert(entities_hm.len() as u32 + 1, seed);
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => {}
            _ => {}
        }
    }

    let mut entities_hm = entities.entities.get_mut(&player_chunk_pos).unwrap();
    let mut player = entities_hm.get_mut(&1).unwrap();

    let player_pos = (
        player.mesh.lines[0].points.0 .0,
        player.mesh.lines[0].points.0 .1,
    );
    let player_relative_pos = (player_pos.0 - camera.pos.0, player_pos.1 - camera.pos.1);
    if wasd.0 {
        player.vel.1 = -150.4;

        if player_relative_pos.1 < 200.0 {
            camera.vel.1 = player.vel.1;
        }
    }
    if wasd.1 {
        player.vel.0 = -150.4;

        if player_relative_pos.0 < 200.0 {
            camera.vel.0 = player.vel.0;
        }
    }
    if wasd.2 {
        player.vel.1 = 150.4;

        if player_relative_pos.1 > 400.0 {
            camera.vel.1 = player.vel.1;
        }
    }
    if wasd.3 {
        player.vel.0 = 150.4;

        if player_relative_pos.0 > 600.0 {
            camera.vel.0 = player.vel.0;
        }
    }
    if !wasd.0 && !wasd.2 {
        player.vel.1 = player.vel.1.lerp(0.0, 0.01);
        camera.vel.1 = player.vel.1;
    }
    if !wasd.1 && !wasd.3 {
        player.vel.0 = player.vel.0.lerp(0.0, 0.01);

        camera.vel.0 = player.vel.0;
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
