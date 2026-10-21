use macroquad::miniquad::date;
use macroquad::rand::{gen_range, srand};
use std::collections::VecDeque;

use crate::controls::Command;

const MAX_KNOB_DETENTS: u32 = 10;
const ZOOMS_PER_EFFECT: u32 = 2;

/// Touches the pad once nobody has touched it for a while: zoom, zoom, effect, repeat
pub struct Autopilot {
    idle: f64,
    period: f64,
    next_walk: f64,
    step: u32,
    queue: VecDeque<Command>,
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
        }
    }
    pub fn touched(&mut self, now: f64) {
        self.queue.clear();
        self.step = 0;
        self.next_walk = now + self.idle;
    }
    pub fn next(&mut self, now: f64) -> Option<Command> {
        if now >= self.next_walk {
            if self.step < ZOOMS_PER_EFFECT {
                self.zoom();
            } else {
                self.effect();
            }
            self.step = (self.step + 1) % (ZOOMS_PER_EFFECT + 1);
            self.next_walk = now + self.period;
        }
        self.queue.pop_front()
    }
    // One of the 3 macropad keys
    fn effect(&mut self) {
        self.queue.push_back(match gen_range(0, 3) {
            0 => Command::NextMesh,
            1 => Command::NextVertexShader,
            _ => Command::NextFragmentShader,
        });
    }
    // A knob twist, one detent per frame
    fn zoom(&mut self) {
        let up = gen_range(0, 2) == 0;
        for _ in 0..gen_range(1, MAX_KNOB_DETENTS + 1) {
            self.queue.push_back(if up {
                Command::CameraUp
            } else {
                Command::CameraDown
            });
        }
    }
}
