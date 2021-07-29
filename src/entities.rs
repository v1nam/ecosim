use macroquad::{
    math::{vec2, Vec2},
    rand::gen_range,
    window::{screen_height, screen_width},
};

#[derive(Clone, Copy)]
pub enum Entity {
    Food {
        energy: f32,
        rad: f32,
        pos: Vec2,
    },
    Organism {
        // todo: genes and other characteristics
        energy: f32,
        rad: f32,
        pos: Vec2,
        velocity: Vec2,
        target: Vec2,
        reproductive_urge: f32,
    },
    Predator {
        energy: f32,
        rot: f32,
        pos: Vec2,
        sides: u8,
        size: f32,
        velocity: Vec2,
        target: Vec2,
        reproductive_urge: f32,
    },
}

impl Entity {
    pub fn update(&mut self) {
        match self {
            Entity::Organism {
                pos,
                rad,
                target,
                velocity,
                energy,
                ..
            } => {
                let w = screen_width() - *rad;
                let h = screen_height() - *rad;
                if pos.x >= w {
                    velocity.x = -(velocity.x.abs());
                }
                if pos.x <= *rad {
                    velocity.x = velocity.x.abs();
                }
                if pos.y >= h {
                    velocity.y = -(velocity.y.abs());
                }
                if pos.y <= *rad {
                    velocity.y = velocity.y.abs();
                }
                if target.x <= *rad {
                    target.x += *rad;
                }
                if target.x >= w {
                    target.x = w - (target.x - w);
                }
                if target.y <= *rad {
                    target.y += *rad;
                }
                if target.y >= h {
                    target.y = h - (target.y - h);
                }
                if target.distance(*pos) <= 1.0 {
                    let r = gen_range(40., 80.);
                    let p = *velocity + *pos;
                    let theta = (p.y - pos.y).atan2(p.x - pos.x);
                    let angle = gen_range(
                        theta - std::f32::consts::PI / 6.,
                        theta + std::f32::consts::PI / 6.,
                    );
                    *target = vec2(pos.x + r * angle.cos(), pos.y + r * angle.sin());
                }
                let direc = (*target - *pos).normalize();
                let dv = direc * 1.6;
                let accel = ((dv - *velocity) * 1.6).clamp_length_max(1.6);
                *velocity = (*velocity + accel).clamp_length_max(1.6);
                *pos += *velocity;
                *energy -= 0.0006;
            }
            Entity::Predator {
                energy,
                pos,
                velocity,
                rot,
                target,
                size,
                ..
            } => {
                let w = screen_width() - *size;
                let h = screen_height() - *size;
                if pos.x >= w {
                    velocity.x = -(velocity.x.abs());
                }
                if pos.x <= *size {
                    velocity.x = velocity.x.abs();
                }
                if pos.y >= h {
                    velocity.y = -(velocity.y.abs());
                }
                if pos.y <= *size {
                    velocity.y = velocity.x.abs();
                }
                if target.x <= *size {
                    target.x += *size;
                }
                if target.x >= w {
                    target.x = w - (target.x - w);
                }
                if target.y <= *size {
                    target.y += *size;
                }
                if target.y >= h {
                    target.y = h - (target.y - h);
                }
                if target.distance(*pos) <= 2.6 {
                    let r = gen_range(40., 80.);
                    let p = *velocity + *pos;
                    let theta = (p.y - pos.y).atan2(p.x - pos.x);
                    let angle = gen_range(
                        theta - std::f32::consts::PI / 6.,
                        theta + std::f32::consts::PI / 6.,
                    );
                    *target = vec2(pos.x + r * angle.cos(), pos.y + r * angle.sin());
                }
                let direc = (*target - *pos).normalize();
                let dv = direc * 2.6;
                let accel = ((dv - *velocity) * 2.6).clamp_length_max(2.6);
                *velocity = (*velocity + accel).clamp_length_max(2.6);
                *pos += *velocity;
                *energy -= 0.0025;
                *rot = (*rot + 1.) % 360.;
            }
            _ => panic!("Method only for Organism and Predator variant"),
        }
    }
}
