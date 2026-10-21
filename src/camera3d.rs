use std::f32::consts::PI;

pub struct CameraState {
    pub angle: f32,
    pub height: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub start_height: f32,
    pub height_step: f32,
    pub angle_step: f32,
    pub max_angle_step: f32,
    pub angle_speed_change_step: f32,
    pub rotation: bool,
}

impl CameraState {
    pub fn increase_height(&mut self) {
        self.height = f32::min(self.max_height, self.height + self.height_step);
    }
    pub fn decrease_height(&mut self) {
        self.height = f32::max(self.min_height, self.height - self.height_step);
    }
    pub fn reset_heigth(&mut self) {
        self.height = self.start_height;
    }
    pub fn increase_angle_speed(&mut self) {
        self.angle_step = f32::min(
            self.max_angle_step,
            self.angle_step + self.angle_speed_change_step,
        );
    }
    pub fn decrease_angle_speed(&mut self) {
        self.angle_step = f32::max(
            -self.max_angle_step,
            self.angle_step - self.angle_speed_change_step,
        );
    }
    pub fn set_zero_angle_speed(&mut self) {
        self.angle_step = 0.
    }
    pub fn switch_rotation(&mut self) {
        self.rotation = !self.rotation;
    }
    // angle_step is per frame at the camera's 30 fps, the rate rendering used to be tied to
    pub fn rotate(&mut self, dt: f32) {
        if self.rotation {
            self.angle += self.angle_step * dt * 30.;
        }
        if self.angle > 2.0 * PI {
            self.angle -= 2.0 * PI;
        } else if self.angle < -2.0 * PI {
            self.angle += 2.0 * PI;
        }
    }
    pub fn new(
        height: f32,
        min_height: f32,
        max_height: f32,
        height_step: f32,
        angle_step: f32,
        angle_speed_change_step: f32,
        max_angle_step: f32,
    ) -> Self {
        CameraState {
            angle: 0.,
            height,
            min_height,
            max_height,
            angle_step,
            angle_speed_change_step,
            max_angle_step,
            height_step,
            rotation: true,
            start_height: height,
        }
    }
}
