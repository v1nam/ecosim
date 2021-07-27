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
        max_speed: f32,
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
                max_speed
            } => {
                let w = screen_width() - *rad;
                let h = screen_height() - *rad;
                if pos.x >= w || pos.x <= *rad {
                    velocity.x *= -1.;
                }
                if pos.y >= h || pos.y <= *rad {
                    velocity.y *= -1.;
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
                if target.distance(*pos) <= *max_speed {
                    let r = gen_range(*max_speed * 20., *max_speed * 40.);
                    let p = *velocity + *pos;
                    let theta = (p.y - pos.y).atan2(p.x - pos.x);
                    let angle = gen_range(
                        theta - std::f32::consts::PI / 6.,
                        theta + std::f32::consts::PI / 6.,
                    );
                    *target = vec2(pos.x + r * angle.cos(), pos.y + r * angle.sin());
                }
                let direc = (*target - *pos).normalize();
                let dv = direc * *max_speed;
                let accel = ((dv - *velocity) * *max_speed).clamp_length_max(*max_speed);
                *velocity = (*velocity + accel).clamp_length_max(*max_speed);
                *pos += *velocity;
                *energy -= *max_speed / 2700.;
            }
            _ => panic!("Method only for Organism variant"),
        }
    }
}

pub struct NewCell {
    pub energy: f32,
    pub speed: f32,
    pub size: f32,
    pub pos: Vec2,
}
