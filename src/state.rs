use chrono::{DateTime, Utc};
use macroquad::models::Mesh;
use macroquad::prelude::*;

use crate::camera3d::CameraState;
use crate::material::{Shader, Style};
use crate::meshes::Figure;

/// The previous effect, kept animating while it fades out over the new one
pub struct Fade {
    pub material: Material,
    pub mesh_ind: usize,
    pub start: f64,
    pub duration: f64,
}

impl Fade {
    pub fn alpha(&self, now: f64) -> f32 {
        let t = ((now - self.start) / self.duration).min(1.) as f32;
        1. - t * t * (3. - 2. * t)
    }
}

pub struct State {
    pub style: Style,
    pub figure: Figure,
    pub camera: CameraState,
    pub cycle: u32,
    pub max_cycle: u32,
    pub default_cycle: u32,
    pub cycle_mult: f32,
    pub phase: f32,
    pub dt: DateTime<Utc>,
    pub fade: Option<Fade>,
}

impl State {
    pub fn get_material(&self) -> Material {
        self.style.get_material()
    }
    pub fn get_mesh(&self) -> &Vec<Mesh> {
        self.figure.get_mesh()
    }
    pub fn effect_key(&self) -> (usize, usize, usize) {
        (
            self.style.v_shader_ind,
            self.style.f_shader_ind,
            self.figure.mesh_ind,
        )
    }
    pub fn increase_height(&mut self) {
        self.camera.increase_height();
    }
    pub fn decrease_height(&mut self) {
        self.camera.decrease_height();
    }
    pub fn reset_camera_heigth(&mut self) {
        self.camera.reset_heigth();
    }
    pub fn increase_cycle(&mut self) {
        self.cycle = u32::min(
            self.max_cycle,
            ((self.cycle as f32) * self.cycle_mult) as u32,
        );
    }
    pub fn decrease_cycle(&mut self) {
        self.cycle = u32::max(1, ((self.cycle as f32) / self.cycle_mult) as u32);
    }
    pub fn reset_cycle(&mut self) {
        self.cycle = self.default_cycle;
    }
    pub fn next_phase(&mut self) {
        let dt = Utc::now();
        let td = (dt - self.dt).num_milliseconds();
        self.dt = dt;
        self.phase += (td as f32) / (self.cycle as f32);
        if self.phase > 1. {
            self.phase -= 1.;
        }
    }
    pub fn new(
        camera: CameraState,
        vertex_shaders: Vec<Shader>,
        fragment_shaders: Vec<Shader>,
        meshes: Vec<Vec<Mesh>>,
        default_cycle: u32,
        max_cycle: u32,
        cycle_mult: f32,
    ) -> Self {
        State {
            style: Style::new(vertex_shaders, fragment_shaders),
            figure: Figure::new(meshes),
            camera,
            cycle: default_cycle,
            max_cycle,
            default_cycle,
            cycle_mult,
            phase: 0.,
            dt: Utc::now(),
            fade: None,
        }
    }
}
