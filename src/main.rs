use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand};
mod entities;
mod quadtree;

use entities::{
    Entity::{self, Food, Organism},
    NewCell,
};
use quadtree::QuadTree;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ecosystem".to_owned(),
        window_width: 1260,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut all_objects: Vec<Entity> = Vec::new();
    let mut spawn_food = 0.0;
    let mut add_to_pop = 0.0;
    let mut population: Vec<u32> = Vec::new();
    let mut food: Vec<u32> = Vec::new();
    let mut average_speed: Vec<f32> = Vec::new();
    srand(macroquad::miniquad::date::now() as u64);
    for _ in 0..3 {
        let angle = gen_range(0.0, std::f32::consts::PI * 2.);
        let pos = vec2(gen_range(10.0, 1250.0), gen_range(10.0, 710.0));
        all_objects.push(Organism {
            energy: gen_range(0.6, 0.85),
            rad: gen_range(6.0, 11.0),
            pos,
            target: vec2(pos.x + angle.cos(), pos.y + angle.sin()),
            velocity: Vec2::ZERO,
            max_speed: gen_range(1.0, 1.6),
        });
    }
    population.push(3);
    average_speed.push(
        all_objects
            .iter()
            .map(|e| {
                if let Organism { max_speed, .. } = e {
                    *max_speed
                } else {
                    0.0
                }
            })
            .sum::<f32>()
            / 3.,
    );

    for _ in 0..50 {
        all_objects.push(Food {
            energy: gen_range(0.3, 1.0),
            rad: 3.0,
            pos: vec2(gen_range(3.0, 1257.0), gen_range(3.0, 717.0)),
        });
    }
    food.push(50);

    let mut qtree = QuadTree::new(0, [0.0, 0.0, 1260.0, 720.0]);
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(BLACK);
        qtree.clear();
        spawn_food += get_frame_time();
        add_to_pop += get_frame_time();

        if spawn_food >= 3.0 {
            spawn_food = 0.0;
            for _ in 0..25 {
                all_objects.push(Food {
                    energy: gen_range(0.3, 1.0),
                    rad: 3.0,
                    pos: vec2(gen_range(3.0, 1257.0), gen_range(3.0, 717.0)),
                });
            }
        }

        for org in all_objects.iter_mut() {
            match org {
                Organism { .. } => org.update(),
                Food { pos, rad, energy } => {
                    draw_circle(pos.x, pos.y, *rad, Color::new(0.0, 1.0, 0.0, *energy));
                    qtree.insert(org);
                }
            }
        }

        let mut return_objects: Vec<Entity>;
        let mut food_to_remove: Vec<Vec2> = Vec::new();
        let mut organisms_to_remove: Vec<Vec2> = Vec::new();
        let mut organisms_to_add: Vec<NewCell> = Vec::new();
        let mut organims_count = 0;
        let mut food_count = 0;
        for obj in all_objects.iter_mut() {
            if let Organism {
                pos: objpos,
                rad: objrad,
                energy: objen,
                max_speed: omsp,
                ..
            } = obj
            {
                organims_count += 1;
                draw_circle_lines(
                    objpos.x,
                    objpos.y,
                    *objrad,
                    2.0,
                    Color::new(1.0, (255. - *omsp * 30.) / 255., 0.0, *objen),
                );
                return_objects = Vec::new();
                qtree.retrieve(&mut return_objects, obj);

                if let Organism {
                    pos: op,
                    rad: or,
                    energy: oen,
                    target: otar,
                    max_speed: msp,
                    ..
                } = obj
                {
                    for other_obj in return_objects {
                        match other_obj {
                            Food { pos, rad, energy } => {
                                if pos.distance(*op) <= *or + rad {
                                    *oen += energy;
                                    *or += 0.4;
                                    food_to_remove.push(pos);
                                } else if pos.distance(*op) <= *or + rad + 20. {
                                    *otar = pos;
                                }
                            }
                            _ => panic!("Unsupported Entity for quadtree"),
                        }
                    }
                    if *oen <= 0.0 {
                        organisms_to_remove.push(*op);
                    } else if *oen >= 0.9 {
                        let new_en = gen_range(*oen / 4., *oen / 2.);
                        *oen -= new_en;
                        let sp = if gen_range(0.0, 1.0) <= 1. / 10. {
                            gen_range(*msp - 1., *msp + 2.)
                        } else {
                            *msp
                        };
                        organisms_to_add.push(NewCell {
                            energy: new_en,
                            size: gen_range(*or / 1.3, *or),
                            speed: sp,
                            pos: *op,
                        });
                    }
                }
            } else {
                food_count += 1;
            }
        }

        if add_to_pop >= 1.0 {
            population.push(organims_count);
            food.push(food_count);
            average_speed.push(
                all_objects
                    .iter()
                    .map(|e| {
                        if let Organism { max_speed, .. } = e {
                            *max_speed
                        } else {
                            0.0
                        }
                    })
                    .sum::<f32>()
                    / organims_count as f32,
            );
            add_to_pop = 0.0;
        }

        if !food_to_remove.is_empty() {
            all_objects.retain(|o| match o {
                Food { pos, .. } => !food_to_remove.contains(pos),
                _ => true,
            });
        }
        if !organisms_to_remove.is_empty() {
            all_objects.retain(|o| match o {
                Organism { pos, .. } => !organisms_to_remove.contains(pos),
                _ => true,
            });
        }
        for new_cell in organisms_to_add {
            let angle = gen_range(0.0, std::f32::consts::PI * 2.);
            all_objects.push(Organism {
                energy: new_cell.energy,
                rad: new_cell.size,
                pos: new_cell.pos,
                target: vec2(new_cell.pos.x + angle.cos(), new_cell.pos.y + angle.sin()),
                velocity: Vec2::ZERO,
                max_speed: new_cell.speed,
            });
        }
        // draw_text(&format!("FPS: {}", get_fps() as u32, 10., 20., 20., WHITE);
        draw_text(
            &format!("Population: {}", organims_count),
            10.,
            20.,
            20.,
            WHITE,
        );
        next_frame().await
    }
    println!("Population: {:?}", population);
    println!("Food: {:?}", food);
    println!("Average Speed {:?}", average_speed);
}
