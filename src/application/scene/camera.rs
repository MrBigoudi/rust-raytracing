use glam::{Mat4, Vec3, Vec4};
// use log::error;

#[derive(Debug)]
#[repr(C)]
pub struct CameraGPU {
    pub view_matrix_inverse: Mat4,
    pub position: Vec4,
    pub plane_width: f32,
    pub plane_height: f32,
    pub plane_near: f32,
}

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Camera {
    // camera Attributes
    pub eye: Vec3,
    pub at: Vec3,
    pub world_up: Vec3,

    pub up: Vec3,
    pub right: Vec3,

    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,

    pub movement_acceleration: f32,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,

    pub yaw: f32,
    pub pitch: f32,

    pub is_accelerating: bool,
}

impl Camera {
    pub fn new(
        position: Vec3,
        aspect_ratio: f32,
        fov: f32,
        near: f32,
        world_up: Vec3,
    ) -> Self {
        let mut camera = Camera {
            eye: position,
            at: Vec3::ZERO,
            world_up,
            up: Vec3::ZERO,
            right: Vec3::ZERO,
            fov,
            aspect_ratio,
            near,
            movement_acceleration: 5.,
            movement_speed: 20.,
            mouse_sensitivity: 0.1,
            yaw: -90.,
            pitch: 0.,
            is_accelerating: false,
        };

        camera.update_vectors();

        camera
    }

    fn update_vectors(&mut self) {
        // calculate the new at vector
        let mut front = Vec3::ZERO;
        front.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        front.y = self.pitch.to_radians().sin();
        front.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.at = front.normalize();
        self.right = Vec3::cross(self.at, self.world_up).normalize();
        self.up = Vec3::cross(self.right, self.at).normalize();
    }

    fn get_view(&self) -> Mat4 {
        Mat4::look_at_rh(self.eye, self.eye + self.at, self.up).transpose()
        // Mat4{
        //     x_axis: Vec4::new(1., 0., 0., 0.),
        //     y_axis: Vec4::new(0., -1., 0., 0.),
        //     z_axis: Vec4::new(0., 0., 1., 5.),
        //     w_axis: Vec4::new(0., 0., 0., 1.),
        // }
    }

    fn get_plane_height(&self) -> f32 {
        2. * self.near * (0.5 * self.fov.to_radians()).tan()
    }

    fn get_plane_width(&self, plane_height: f32) -> f32 {
        plane_height * self.aspect_ratio
    }

    pub fn on_keyboard_input(&mut self, direction: CameraMovement, delta_time: f64) {
        let mut velocity = self.movement_speed * (delta_time as f32);
        if self.is_accelerating {
            velocity *= self.movement_acceleration;
        }

        match direction {
            CameraMovement::Forward => self.eye -= self.at * velocity,
            CameraMovement::Backward => self.eye += self.at * velocity,
            CameraMovement::Left => self.eye -= self.right * velocity,
            CameraMovement::Right => self.eye += self.right * velocity,
            CameraMovement::Up => self.eye += self.world_up * velocity,
            CameraMovement::Down => self.eye -= self.world_up * velocity,
        };
    }

    pub fn on_mouse_moved(&mut self, x_offset: f32, y_offset: f32, should_constrain_pitch: bool) {
        let x_offset = x_offset * self.mouse_sensitivity;
        let y_offset = y_offset * self.mouse_sensitivity;
        self.yaw -= x_offset;
        self.pitch += y_offset;
        // make sure that when pitch is out of bounds, screen doesn't get flipped
        if should_constrain_pitch {
            self.pitch = self.pitch.clamp(-89.0, 89.0);
        }
        // update Front, Right and Up Vectors using the updated Euler angles
        self.update_vectors();
    }

    pub fn on_resize(&mut self, new_width: u16, new_height: u16) {
        self.aspect_ratio = (new_width as f32) / (new_height as f32);
    }

    pub fn get_gpu_data(&self) -> CameraGPU {
        let view_matrix = self.get_view();
        let plane_height = self.get_plane_height();
        let plane_width = self.get_plane_width(plane_height);
        let view_matrix_inverse = Mat4::inverse(&Mat4::transpose(&view_matrix));
        let position = Vec4::new(self.eye.x, self.eye.y, self.eye.z, 1.);

        CameraGPU {
            view_matrix_inverse,
            position,
            plane_width,
            plane_height,
            plane_near: self.near,
        }
    }
}
