use macroquad::miniquad::date;
use macroquad::rand::{gen_range, srand};
use std::collections::VecDeque;

use crate::controls::Command;

const MAX_KNOB_DETENTS: u32 = 10;
const ZOOMS_PER_EFFECT: u32 = 2;
const ZOOM_SECONDS: f64 = 2.;

struct Glide {
    start: f64,
    detents: f32,
    // Eased fraction of the distance already emitted
    done: f32,
}

/// Touches the pad once nobody has touched it for a while: zoom, zoom, effect, repeat
pub struct Autopilot {
    idle: f64,
    period: f64,
    next_walk: f64,
    step: u32,
    queue: VecDeque<Command>,
    glide: Option<Glide>,
}

impl Autopilot {
    pub fn new(idle: f64, period: f64, now: f64) -> Self {
        srand(date::now() as u64);
        Autopilot {
            idle,
            period,
            next_walk: now + idle,
            step: 0,
            queue: VecDeque::new(),
            glide: None,
        }
    }
    pub fn touched(&mut self, now: f64) {
        self.queue.clear();
        self.glide = None;
        self.step = 0;
        self.next_walk = now + self.idle;
    }
    pub fn next(&mut self, now: f64) -> Option<Command> {
        if now >= self.next_walk {
            if self.step < ZOOMS_PER_EFFECT {
                self.zoom(now);
            } else {
                self.effect();
            }
            self.step = (self.step + 1) % (ZOOMS_PER_EFFECT + 1);
            self.next_walk = now + self.period;
        }
        self.queue.pop_front().or_else(|| self.glide(now))
    }
    // One of the 3 macropad keys
    fn effect(&mut self) {
        self.queue.push_back(match gen_range(0, 3) {
            0 => Command::NextMesh,
            1 => Command::NextVertexShader,
            _ => Command::NextFragmentShader,
        });
    }
    // A knob twist, eased over ZOOM_SECONDS instead of jumping detent by detent
    fn zoom(&mut self, now: f64) {
        let detents = gen_range(1, MAX_KNOB_DETENTS + 1) as f32;
        self.glide = Some(Glide {
            start: now,
            detents: if gen_range(0, 2) == 0 {
                detents
            } else {
                -detents
            },
            done: 0.,
        });
    }
    fn glide(&mut self, now: f64) -> Option<Command> {
        let glide = self.glide.as_mut()?;
        let t = ((now - glide.start) / ZOOM_SECONDS).min(1.) as f32;
        let eased = t * t * (3. - 2. * t);
        let shift = (eased - glide.done) * glide.detents;
        glide.done = eased;
        if t >= 1. {
            self.glide = None;
        }
        Some(Command::CameraShift(shift))
    }
}
