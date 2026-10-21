use config::{Config, File};
use glob::glob;
use lazy_static::lazy_static;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use std::fs;
use std::sync::mpsc::{self, TryRecvError, TrySendError};
use std::thread;
use v4l::buffer::Type;
use v4l::control::{Control, Value};
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::v4l_sys::{V4L2_CID_EXPOSURE_AUTO_PRIORITY, V4L2_CID_SATURATION};
use v4l::video::Capture;
use v4l::Device;
use v4l::FourCC;
use webcam::controls::full::FullKeyboard;
use webcam::controls::mini::MiniKeyboard;
use webcam::controls::Keyboard;

use webcam::autopilot::Autopilot;
use webcam::camera3d::CameraState;
use webcam::config::Settings;
use webcam::controls::{Command, KeyboardType};
use webcam::decoder::*;
use webcam::material::Shader;
use webcam::meshes::{get_hexagons, get_triangles};
use webcam::state::{Fade, State};

const BUFFER_COUNT: u32 = 4;

fn get_config() -> Settings {
    let settings = Config::builder()
        .add_source(File::with_name("config/default.json").required(false))
        .add_source(File::with_name("config/local.json").required(false))
        .build()
        .unwrap();
    settings.try_deserialize().unwrap()
}

lazy_static! {
    static ref SETTINGS: Settings = get_config();
}

fn window_conf() -> Conf {
    let settings = SETTINGS.clone();
    Conf {
        window_title: "Kaleidoscope".to_owned(),
        fullscreen: settings.fullscreen,
        window_height: settings.window_height,
        window_width: settings.window_width,
        // Without vsync the render loop spins a core the capture thread needs
        platform: miniquad::conf::Platform {
            swap_interval: Some(1),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn angle2vec(angle: f32) -> Vec3 {
    let x = angle.sin();
    let y = angle.cos();
    vec3(x, y, 0.)
}

#[macroquad::main(window_conf)]
async fn main() {
    let settings = SETTINGS.clone();

    // In kiosk mode the camera may be plugged in after boot
    let dev = loop {
        match Device::new(settings.webcamera.device_id) {
            Ok(dev) => break dev,
            Err(e) => {
                clear_background(BLACK);
                let msg = format!(
                    "Waiting for camera /dev/video{}: {}",
                    settings.webcamera.device_id, e
                );
                draw_text(&msg, 20., 40., 30., WHITE);
                next_frame().await
            }
        }
    };
    dev.set_control(Control {
        id: V4L2_CID_SATURATION,
        value: Value::Integer(settings.webcamera.saturation),
    })
    .expect("Failed to set saturation");
    // Otherwise auto exposure drops the camera to ~15 fps in a dim room
    dev.set_control(Control {
        id: V4L2_CID_EXPOSURE_AUTO_PRIORITY,
        value: Value::Boolean(false),
    })
    .expect("Failed to disable dynamic framerate");
    let mut fmt = dev.format().expect("Failed to read format");
    fmt.width = settings.webcamera.width;
    fmt.height = settings.webcamera.height;
    // MJPG: 30 fps, the camera gives YUYV only 25 fps at 640x480
    fmt.fourcc = FourCC::new(b"MJPG");
    println!("Format to set:\n{}", fmt);
    let fmt = dev.set_format(&fmt).expect("Failed to write format");
    println!("Format in use:\n{}", fmt);
    let (width, height) = (fmt.width as u16, fmt.height as u16);
    // Capture and decode off the render thread, so rendering runs at vsync, not camera rate
    let (frames_tx, frames) = mpsc::sync_channel::<Vec<u8>>(1);
    thread::spawn(move || {
        let decoder: Box<dyn Decoder> = if fmt.fourcc == FourCC::new(b"MJPG") {
            Box::new(MjpegDecoder {})
        } else {
            Box::new(YuyvDecoder {})
        };
        let mut stream = Stream::with_buffers(&dev, Type::VideoCapture, BUFFER_COUNT)
            .expect("Failed to create buffer stream");
        loop {
            let (buf, _meta) = stream.next().expect("Camera lost");
            let mut rgba = vec![0; width as usize * height as usize * 4];
            decoder.decode(&mut rgba, buf);
            // Full means the renderer hasn't taken the previous frame yet: drop this one
            if let Err(TrySendError::Disconnected(_)) = frames_tx.try_send(rgba) {
                break;
            }
        }
    });
    let mut image = Image::gen_image_color(width, height, WHITE);
    let texture = Texture2D::from_image(&image);

    let fragment_shaders: Vec<_> = glob("shaders/*.fs")
        .expect("Failed to read glob pattern")
        .flatten()
        .map(|x| {
            println!("{:?}", x.display());
            Shader {
                path: x.to_str().unwrap().to_string(),
                code: fs::read_to_string(x.to_str().unwrap()).unwrap(),
            }
        })
        .collect();
    let vertex_shaders: Vec<_> = glob("shaders/*.vs")
        .expect("Failed to read glob pattern")
        .flatten()
        .map(|x| {
            println!("{:?}", x.display());
            Shader {
                path: x.to_str().unwrap().to_string(),
                code: fs::read_to_string(x.to_str().unwrap()).unwrap(),
            }
        })
        .collect();

    let camera3d = CameraState::new(
        settings.camera3d.start_height,
        settings.camera3d.min_height,
        settings.camera3d.max_height,
        settings.camera3d.step,
        settings.camera3d.angle_step,
        settings.camera3d.angle_speed_change_step,
        settings.camera3d.max_angle_step,
    );
    let mut state = State::new(
        camera3d,
        vertex_shaders,
        fragment_shaders,
        vec![
            get_triangles(settings.mesh.triangles.levels, texture.clone()),
            get_hexagons(settings.mesh.hexagons.levels, texture.clone()),
        ],
        settings.default_cycle,
        settings.max_cycle,
        settings.cycle_mult,
    );
    state
        .style
        .set_shaders("default".to_string(), "default".to_string());
    let mut camera = Camera3D {
        position: vec3(0., 0., state.camera.height),
        up: angle2vec(state.camera.angle),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };

    let keyboard: Box<dyn Keyboard> = match settings.keyboard {
        KeyboardType::Full => Box::new(FullKeyboard {}),
        KeyboardType::Mini => Box::new(MiniKeyboard {}),
    };
    let mut autopilot = Autopilot::new(
        settings.autopilot.idle,
        settings.autopilot.period,
        get_time(),
    );
    let mut fade_rt: Option<RenderTarget> = None;
    loop {
        //
        // Webcam
        //
        // On unplug the capture thread dies: exit and let systemd restart us into the waiting loop
        match frames.try_recv() {
            Ok(rgba) => {
                image.bytes = rgba;
                texture.update(&image);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => panic!("Camera lost"),
        }

        //
        // Input
        //
        let command = keyboard.process_input();
        if command.is_some() {
            autopilot.touched(get_time());
        }
        let (old_key, old_material) = (state.effect_key(), state.get_material());
        match command.or_else(|| autopilot.next(get_time())) {
            Some(Command::Quit) => break,
            Some(Command::SwitchRotation) => state.camera.switch_rotation(),
            Some(Command::NextMesh) => {
                state.figure.next_mesh();
            }
            Some(Command::NextVertexShader) => {
                state.style.next_vertex_shader();
            }
            Some(Command::PrevVertexShader) => {
                state.style.prev_vertex_shader();
            }
            Some(Command::PrevFragmentShader) => {
                state.style.prev_fragment_shader();
            }
            Some(Command::NextFragmentShader) => {
                state.style.next_fragment_shader();
            }
            Some(Command::CameraDown) => {
                state.decrease_height();
            }
            Some(Command::CameraUp) => {
                state.increase_height();
            }
            Some(Command::CameraShift(detents)) => {
                state.camera.shift_height(detents);
            }
            Some(Command::IncreaseAngleSpeed) => {
                state.camera.increase_angle_speed();
            }
            Some(Command::DecreaseAngleSpeed) => {
                state.camera.decrease_angle_speed();
            }
            Some(Command::SetZeroAngleSpeed) => {
                state.camera.set_zero_angle_speed();
            }
            Some(Command::CameraReset) => {
                state.reset_camera_heigth();
            }
            Some(Command::IncreaseCycle) => {
                state.increase_cycle();
            }
            Some(Command::DecreaseCycle) => {
                state.decrease_cycle();
            }
            Some(Command::ResetCycle) => {
                state.reset_cycle();
            }
            Some(Command::Shaders(vshader, fshader)) => {
                state.style.set_shaders(vshader, fshader);
            }
            Some(Command::FShader(fshader)) => {
                state.style.set_fragment_shader(fshader);
            }
            Some(Command::VShader(vshader)) => {
                state.style.set_vetrex_shader(vshader);
            }
            _ => (),
        }
        if settings.crossfade > 0. && state.effect_key() != old_key {
            state.fade = Some(Fade {
                material: old_material,
                mesh_ind: old_key.2,
                start: get_time(),
                duration: settings.crossfade,
            });
        }

        //
        // GUI
        //
        // Current time (since last midnight, in milliseconds)
        let material = state.get_material();
        state.next_phase();
        material.set_uniform("phase", state.phase);
        state.camera.rotate(get_frame_time());
        camera.up = angle2vec(state.camera.angle);
        camera.position = vec3(0., 0., state.camera.height);
        clear_background(BLACK);
        set_camera(&camera);
        gl_use_material(&state.get_material());
        state.get_mesh().iter().for_each(draw_mesh);
        gl_use_default_material();

        //
        // Crossfade: render the previous effect offscreen and lay it over with fading alpha
        //
        if let Some(fade) = &state.fade {
            let alpha = fade.alpha(get_time());
            let size = vec2(screen_width(), screen_height());
            if fade_rt.as_ref().is_some_and(|rt| rt.texture.size() != size) {
                fade_rt = None;
            }
            let rt = fade_rt.get_or_insert_with(|| {
                let rt = render_target(size.x as u32, size.y as u32);
                rt.texture.set_filter(FilterMode::Linear);
                rt
            });
            set_camera(&Camera3D {
                position: camera.position,
                up: camera.up,
                target: camera.target,
                render_target: Some(rt.clone()),
                ..Default::default()
            });
            clear_background(BLACK);
            fade.material.set_uniform("phase", state.phase);
            gl_use_material(&fade.material);
            state.figure.meshes[fade.mesh_ind]
                .iter()
                .for_each(draw_mesh);
            gl_use_default_material();
            set_default_camera();
            draw_texture_ex(
                &rt.texture,
                0.,
                0.,
                Color::new(1., 1., 1., alpha),
                DrawTextureParams {
                    dest_size: Some(size),
                    flip_y: true,
                    ..Default::default()
                },
            );
            if alpha <= 0. {
                state.fade = None;
            }
        }

        next_frame().await
    }
}
