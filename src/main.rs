extern crate nannou;

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Particle {
    position: Point2,
    velocity: Vec2,
    mass: f32,
}

impl Particle {
    fn new(x: f32, y: f32) -> Self {
        let position: Point2 = Point2::new(x, y);
        let velocity: Vec2 = vec2(random_f32() * 10.0 - 5.0, random_f32() * 10.0 - 5.0);
        let mass = 1.0;
        Particle {
            position,
            velocity,
            mass,
        }
    }

    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(2.0)
            .rgba(0.5, 0.5, 0.5, 0.5)
            .stroke(rgba(0.5, 0.5, 0.5, 0.5))
            .stroke_weight(2.0);
    }

    fn apply_gravity_to_particle(&mut self, particle: &Particle) {
        let distance_x = self.position.x - particle.position.x;
        let distance_y = self.position.y - particle.position.y;
        let g = 2.0;
        let distance = f32::sqrt(pow(distance_x, 2) + pow(distance_y, 2));
        self.velocity += vec2(g / distance_x, g / distance_y);
    }

    fn apply_gravity(&mut self, particles: &Vec<Particle>) {
        particles
            .iter()
            .for_each(|p| self.apply_gravity_to_particle(p));
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
    let mut particles: Vec<Particle> = Vec::new();
    for _ in 1..10 {
        particles.push(Particle::new(random_f32() * 100.0, random_f32() * 100.0));
    }
    Model { particles }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // _model
    //     .particles
    //     .iter_mut()
    //     .for_each(|x| x.apply_gravity(&_model.particles));
    for p in _model.particles.iter_mut() {
        p.update(_app.window_rect());
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    // frame.clear(PURPLE);

    let draw = _app.draw();

    // Generate sine wave data based on the time of the app
    // let sine = _app.time.sin();
    // let slowersine = (_app.time / 2.0).sin();

    // Get boundary of the window (to constrain the movements of our circle)
    let boundary = _app.window_rect();

    // Map the sine wave functions to ranges between the boundaries of the window
    // let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    // let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    // Clear the background to purple.
    draw.background().color(PLUM);

    for p in _model.particles.iter() {
        p.display(&draw);
    }

    // draw.ellipse().color(STEELBLUE).x_y(x, y);
    draw.to_frame(_app, &frame).unwrap()
}
