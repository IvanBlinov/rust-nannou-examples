extern crate nannou;

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Particle {
    pub position: Point2,
    pub velocity: Vec2,
    mass: f32,
}

impl Particle {
    fn new(x: f32, y: f32) -> Self {
        let position: Point2 = Point2::new(x, y);
        let velocity: Vec2 = vec2(random_f32() - 0.5, random_f32() - 0.5);
        let mass = 200.0 + random_f32() * 300.0;
        Particle {
            position,
            velocity,
            mass,
        }
    }

    fn new_particle(x: f32, y: f32, mass: f32, velocity: Vec2) -> Self {
        let position: Point2 = Point2::new(x, y);
        Particle {
            position,
            velocity,
            mass,
        }
    }

    fn display(&self, draw: &Draw) {
        let mut blue = 0.0;
        let mut red = 0.0;
        let mut alpha = 1.0;
        let mut radius = 20.0;
        if self.mass < 1000.0 {
            blue = map_range(self.mass, 200.0, 500.0, 0.0, 1.0);
            red = 1.0 - blue;
            alpha = 0.8;
            radius = self.mass / 100.0;
        }
        draw.ellipse()
            .xy(self.position)
            .radius(radius)
            .rgba(red, 0.0, blue, alpha);
        // .stroke(rgba(0.5, 0.5, 0.5, 0.5))
        // .stroke_weight(1.0);
    }

    fn apply_gravity_to_particle(&mut self, position: Vec2, mass: f32) {
        let g = 1.0;

        let abs_x = 1.0.max(abs(self.position.x - position.x));
        let abs_y = 1.0.max(abs(self.position.y - position.y));

        let x_mult = if self.position.x > position.x {
            -1.0
        } else {
            1.0
        };
        let y_mult = if self.position.y > position.y {
            -1.0
        } else {
            1.0
        };

        let distance = 1.0.max(f32::sqrt(pow(abs_x, 2) + pow(abs_y, 2)));

        let gravity = (g * mass) / pow(distance, 2);
        let gravity_x = x_mult * gravity * (abs_x / distance);
        let gravity_y = y_mult * gravity * (abs_y / distance);

        self.velocity += vec2(gravity_x / self.mass, gravity_y / self.mass);
    }

    fn apply_gravity(&mut self, particles: &Vec<Particle>) {
        particles
            .iter()
            .for_each(|p| self.apply_gravity_to_particle(p.position, p.mass));
    }

    fn update(&mut self, window: Rect) {
        let bottom = window.bottom();
        let right = window.right();
        let left = window.left();
        let top = window.top();
        if self.position.x + self.velocity.x > right {
            self.velocity = vec2(self.velocity.x * -1.0, self.velocity.y);
        }
        if self.position.y + self.velocity.y < bottom {
            self.velocity = vec2(self.velocity.x, -1.0 * self.velocity.y);
        }
        if self.position.x + self.velocity.x < left {
            self.velocity = vec2(self.velocity.x * -1.0, self.velocity.y);
        }
        if self.position.y + self.velocity.y > top {
            self.velocity = vec2(self.velocity.x, -1.0 * self.velocity.y);
        }
        self.position += self.velocity;
    }
}

struct Model {
    particles: Vec<Particle>,
}

fn model(_app: &App) -> Model {
    let boundary = _app.window_rect();
    let y_max = boundary.top();
    let y_min = boundary.bottom();
    let x_max = boundary.right();
    let x_min = boundary.left();

    let mut particles: Vec<Particle> = Vec::new();
    // Stars
    for _ in 1..50 {
        particles.push(Particle::new(
            random_f32() * (x_max - x_min) + x_min,
            random_f32() * (y_max - y_min) + y_min,
        ));
    }
    // Sun
    let sun_mass = 10000.0;
    particles.push(Particle::new_particle(0.0, 0.0, sun_mass, vec2(0.0, 0.0)));
    Model { particles }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    for i in 0.._model.particles.len() {
        for j in 0.._model.particles.len() {
            if i != j {
                let position = _model.particles[j].position;
                let mass = _model.particles[j].mass;
                _model
                    .particles
                    .get_mut(i)
                    .unwrap()
                    .apply_gravity_to_particle(position, mass);
            }
        }
    }
    for p in _model.particles.iter_mut() {
        p.update(_app.window_rect());
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();

    // Clear the background to purple.
    draw.background().color(PLUM);

    for p in _model.particles.iter() {
        p.display(&draw);
    }

    draw.to_frame(_app, &frame).unwrap()
}
