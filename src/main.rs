use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand};
mod entities;
mod quadtree;

use entities::Entity::{self, Food, Organism};
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
        });
    }
    population.push(3);

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
        let mut organisms_to_add: Vec<Vec2> = Vec::new();
        let mut organims_count = 0;
        let mut food_count = 0;
        for obj in all_objects.iter_mut() {
            if let Organism {
                pos: objpos,
                rad: objrad,
                energy: objen,
                ..
            } = obj
            {
                organims_count += 1;
                draw_circle_lines(
                    objpos.x,
                    objpos.y,
                    *objrad,
                    2.0,
                    Color::new(0.0, 0.0, 1.0, *objen),
                );
                return_objects = Vec::new();
                qtree.retrieve(&mut return_objects, obj);

                if let Organism {
                    pos: op,
                    rad: or,
                    energy: oen,
                    target: otar,
                    ..
                } = obj
                {
                    for other_obj in return_objects {
                        match other_obj {
                            Food { pos, rad, energy } => {
                                if pos.distance(*op) <= *or + rad {
                                    *oen += energy;
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
                        *oen -= 0.4;
                        organisms_to_add.push(*op);
                    }
                }
            } else {
                food_count += 1;
            }
        }

        if add_to_pop >= 1.0 {
             population.push(organims_count);
             food.push(food_count);
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
                energy: gen_range(0.6, 0.85),
                rad: gen_range(6.0, 11.0),
                pos: new_cell,
                target: vec2(new_cell.x + angle.cos(), new_cell.y + angle.sin()),
                velocity: Vec2::ZERO,
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
}
