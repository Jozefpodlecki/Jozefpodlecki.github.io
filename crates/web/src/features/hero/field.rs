use std::{cell::RefCell, f64::consts::TAU, rc::Rc};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum FlowMode {
    #[default]
    Calm,
    Dynamic,
}

impl FlowMode {
    pub fn speed(self) -> f64 {
        match self {
            Self::Calm => 1.0,
            Self::Dynamic => 2.3,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Calm => "calm",
            Self::Dynamic => "dynamic",
        }
    }
}


use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

struct Particle {
    x: f64,
    y: f64,
    speed: f64,
}

pub struct FlowField {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    pointer: Rc<RefCell<Option<(f64, f64)>>>,
    mode: Rc<RefCell<FlowMode>>,
    particles: Vec<Particle>,
    seed: u32,
    width: f64,
    height: f64,
    last_frame: Option<f64>,
    phase_time: f64,
    current_speed: f64,
    reduced_motion: bool,
    frame_index: usize,
}

impl FlowField {
    pub fn new(
        canvas: HtmlCanvasElement,
        pointer: Rc<RefCell<Option<(f64, f64)>>>,
        mode: Rc<RefCell<FlowMode>>,
        reduced_motion: bool,
    ) -> Option<Self> {
        let context = canvas
            .get_context("2d")
            .ok()
            .flatten()?
            .dyn_into::<CanvasRenderingContext2d>()
            .ok()?;

        let seed = (js_sys::Math::random() * f64::from(u32::MAX)) as u32;
        let _ = canvas.set_attribute("data-seed", &seed.to_string());

        let particles = (0..105)
            .map(|index| Particle {
                x: unit_random(index, 11, seed),
                y: unit_random(index, 23, seed),
                speed: 0.55 + unit_random(index, 67, seed) * 0.75,
            })
            .collect();

        Some(Self {
            canvas,
            context,
            pointer,
            mode,
            particles,
            seed,
            width: 0.0,
            height: 0.0,
            last_frame: None,
            phase_time: 0.0,
            current_speed: 1.0,
            reduced_motion,
            frame_index: 0,
        })
    }

    pub fn render(&mut self, timestamp: f64) {
        self.resize();

        if self.width <= 0.0 || self.height <= 0.0 {
            return;
        }

        if !self.reduced_motion && self.canvas.owner_document().is_some_and(|document| document.hidden()) {
            self.last_frame = None;
            return;
        }

        let target_speed = self.mode.borrow().speed();
        let dt = self
            .last_frame
            .map(|last| ((timestamp - last) / 1000.0).clamp(0.0, 0.05))
            .unwrap_or(0.0);

        self.last_frame = Some(timestamp);
        self.frame_index = self.frame_index.wrapping_add(1);

        if !self.reduced_motion {
            self.current_speed = eased_speed(self.current_speed, target_speed, dt);
            self.phase_time += dt * 0.16 * self.current_speed;
        }

        let phase = (f64::from(self.seed) / (f64::from(u32::MAX) + 1.0)) * TAU;
        let time = phase + self.phase_time;
        let is_dark = self.canvas.owner_document()
            .and_then(|document| document.document_element())
            .and_then(|root| root.get_attribute("data-theme"))
            .as_deref() == Some("dark");

        let navy = self.canvas.owner_document()
            .and_then(|document| document.document_element())
            .and_then(|root| root.get_attribute("data-palette"))
            .as_deref() == Some("navy");

        self.paint_background(is_dark, navy);
        self.paint_streamlines(time, is_dark, navy);
        self.paint_particles(dt * self.current_speed, time, is_dark, navy);
    }

    fn resize(&mut self) {
        let width = f64::from(self.canvas.client_width());
        let height = f64::from(self.canvas.client_height());

        if width <= 0.0 || height <= 0.0 {
            return;
        }

        if (width - self.width).abs() < 1.0 && (height - self.height).abs() < 1.0 {
            return;
        }

        self.width = width;
        self.height = height;

        let scale = self.canvas.owner_document()
            .and_then(|document| document.default_view())
            .map(|window| window.device_pixel_ratio().min(2.0))
            .unwrap_or(1.0);

        self.canvas.set_width((width * scale) as u32);
        self.canvas.set_height((height * scale) as u32);
        let _ = self.context.set_transform(scale, 0.0, 0.0, scale, 0.0, 0.0);
    }

    fn paint_background(&self, is_dark: bool, navy: bool) {
        let background = match (navy, is_dark) {
            (true, true) => "#091e39",
            (true, false) => "#eaf1fc",
            (false, true) => "#071b1e",
            (false, false) => "#f2f8f5",
        };
        self.context.set_fill_style_str(background);
        self.context.fill_rect(0.0, 0.0, self.width, self.height);

        if let Ok(glow) = self.context.create_radial_gradient(
            self.width * 0.58,
            self.height * 0.52,
            10.0,
            self.width * 0.58,
            self.height * 0.52,
            self.width * 0.67,
        ) {
            let glow_color = match (navy, is_dark) {
                (true, true) => "rgba(62,125,240,0.25)",
                (true, false) => "rgba(56,117,205,0.08)",
                (false, true) => "rgba(20,156,145,0.22)",
                (false, false) => "rgba(23,151,139,0.075)",
            };
            let _ = glow.add_color_stop(0.0, glow_color);
            let _ = glow.add_color_stop(1.0, "rgba(0,0,0,0)");
            self.context.set_fill_style_canvas_gradient(&glow);
            self.context.fill_rect(0.0, 0.0, self.width, self.height);
        }
    }

    fn paint_streamlines(&self, time: f64, is_dark: bool, navy: bool) {
        self.context.set_line_width(0.72);

        for index in 0..72 {
            let x = -0.16 + unit_random(index, 3, self.seed) * 0.3;
            let y = -0.12 + (index as f64 / 71.0) * 1.24;
            let mut position = (x, y);

            self.context.begin_path();
            self.context.move_to(x * self.width, y * self.height);

            for _ in 0..145 {
                let vector = flow_vector(position.0, position.1, time, None);
                position.0 += vector.0 * 0.008;
                position.1 += vector.1 * 0.008 * self.width / self.height;
                self.context.line_to(position.0 * self.width, position.1 * self.height);

                if position.0 > 1.15 || position.1 < -0.25 || position.1 > 1.25 {
                    break;
                }
            }

            let highlight = index % 13 == 0;
            let intensity = if highlight { 0.48 } else { 0.15 };
            self.context.set_line_width(if highlight { 1.15 } else { 0.72 });
            self.context.set_shadow_blur(if is_dark && highlight { 7.0 } else { 0.0 });
            self.context.set_shadow_color(if navy { "rgba(120,180,255,0.55)" } else { "rgba(36,230,211,0.55)" });
            let alpha = if is_dark { intensity } else if highlight { 0.38 } else { 0.12 };
            let color = match (navy, is_dark) {
                (true, true) => "105,171,255",
                (true, false) => "40,99,178",
                (false, true) => "50,198,188",
                (false, false) => "19,114,105",
            };
            self.context.set_stroke_style_str(&format!(
                "rgba({color},{alpha:.3})"
            ));
            self.context.stroke();
        }

        self.context.set_shadow_blur(0.0);
    }

    fn paint_particles(&mut self, dt: f64, time: f64, is_dark: bool, navy: bool) {
        let pointer = if self.reduced_motion { None } else { *self.pointer.borrow() };
        let alpha = if is_dark { 0.8 } else { 0.52 };
        let color = match (navy, is_dark) {
            (true, true) => "155,203,255",
            (true, false) => "38,98,177",
            (false, true) => "115,245,225",
            (false, false) => "15,113,102",
        };
        self.context.set_fill_style_str(&format!(
            "rgba({color},{alpha})"
        ));

        for (index, particle) in self.particles.iter_mut().enumerate() {
            if !self.reduced_motion {
                let vector = flow_vector(particle.x, particle.y, time, pointer);
                particle.x += vector.0 * dt * 0.12 * particle.speed;
                particle.y += vector.1 * dt * 0.12 * particle.speed * self.width / self.height;

                if particle.x > 1.06 || particle.x < -0.12 || particle.y < -0.12 || particle.y > 1.12 {
                    particle.x = -0.05;
                    particle.y = unit_random(index + self.frame_index, 81, self.seed);
                }
            }

            let radius = if index % 9 == 0 { 1.55 } else { 0.85 };
            self.context.begin_path();
            let _ = self.context.arc(
                particle.x * self.width,
                particle.y * self.height,
                radius,
                0.0,
                TAU,
            );
            self.context.fill();
        }
    }
}

fn eased_speed(current: f64, target: f64, dt: f64) -> f64 {
    current + (target - current) * (1.0 - (-dt * 7.0).exp())
}

fn flow_vector(x: f64, y: f64, time: f64, pointer: Option<(f64, f64)>) -> (f64, f64) {
    let mut vx = 1.0 + (y * 8.5 + time).sin() * 0.16;
    let mut vy = (x * 6.3 - time).sin() * 0.24 + (y * 8.0).cos() * 0.08;

    for (cx, cy, strength) in [(0.26, 0.34, 1.1), (0.8, 0.73, -1.3)] {
        let dx = x - cx;
        let dy = y - cy;
        let radius = dx * dx + dy * dy;
        let influence = (-radius / 0.037).exp() * strength;
        vx -= dy * influence * 11.0;
        vy += dx * influence * 11.0;
    }

    if let Some((px, py)) = pointer {
        let dx = x - px;
        let dy = y - py;
        let distance = dx * dx + dy * dy;
        let influence = (-distance / 0.022).exp();
        vx -= dy * influence * 1.8;
        vy += dx * influence * 1.8;
    }

    let magnitude = vx.hypot(vy).max(0.0001);
    (vx / magnitude, vy / magnitude)
}

fn unit_random(index: usize, salt: usize, seed: u32) -> f64 {
    let mut value = (index as u32)
        .wrapping_mul(0x9e3779b9)
        .wrapping_add((salt as u32).wrapping_mul(0x85ebca6b))
        .wrapping_add(seed);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846ca68b);
    value ^= value >> 16;
    f64::from(value) / (f64::from(u32::MAX) + 1.0)
}

#[cfg(test)]
mod tests {
    use super::{eased_speed, flow_vector, unit_random, FlowMode};

    #[test]
    fn dynamic_mode_moves_faster_than_calm() {
        assert!(FlowMode::Dynamic.speed() > FlowMode::Calm.speed());
    }

    #[test]
    fn speed_change_eases_toward_target() {
        let next = eased_speed(1.0, 2.3, 1.0 / 60.0);
        assert!(next > 1.0);
        assert!(next < 2.3);
        let back = eased_speed(next, 1.0, 1.0 / 60.0);
        assert!(back < next);
    }

    #[test]
    fn field_direction_is_normalized() {
        let (x, y) = flow_vector(0.5, 0.5, 0.0, None);
        assert!((x.hypot(y) - 1.0).abs() < 0.00001);
    }

    #[test]
    fn seeded_positions_stay_within_bounds() {
        for index in 0..200 {
            assert!((0.0..1.0).contains(&unit_random(index, 11, 42)));
        }
    }

    #[test]
    fn different_seeds_change_particle_positions() {
        assert_ne!(unit_random(12, 23, 42), unit_random(12, 23, 43));
        assert_eq!(unit_random(12, 23, 42), unit_random(12, 23, 42));
    }
}
