use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand};
mod entities;
mod plotdata;
mod quadtree;

use entities::Entity::{self, Food, Organism, Predator};
use plotdata::plot;
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
    let mut organism_population: Vec<u32> = vec![6];
    let mut predator_population: Vec<u32> = vec![2];
    let mut food: Vec<u32> = vec![50];
    srand(macroquad::miniquad::date::now() as u64);
    for _ in 0..6 {
        let angle = gen_range(0.0, std::f32::consts::PI * 2.);
        let pos = vec2(gen_range(10.0, 1250.0), gen_range(10.0, 710.0));
        all_objects.push(Organism {
            energy: gen_range(0.6, 0.85),
            rad: gen_range(6.0, 11.0),
            pos,
            target: vec2(pos.x + angle.cos(), pos.y + angle.sin()),
            velocity: Vec2::ZERO,
            reproductive_urge: 0.0,
        });
    }

    for _ in 0..2 {
        let pos = vec2(
            gen_range(10.0, screen_width() - 10.),
            gen_range(10.0, screen_height() - 10.),
        );
        let angle = gen_range(0.0, std::f32::consts::PI * 2.0);
        all_objects.push(Predator {
            energy: gen_range(1.5, 2.5),
            rot: angle * 180. / std::f32::consts::PI,
            sides: 5,
            size: gen_range(10., 13.),
            pos,
            velocity: Vec2::ZERO,
            target: vec2(pos.x + angle.cos(), pos.y + angle.sin()),
            reproductive_urge: 0.0,
        });
    }

    for _ in 0..50 {
        all_objects.push(Food {
            energy: gen_range(0.3, 1.0),
            rad: 3.0,
            pos: vec2(gen_range(3.0, 1257.0), gen_range(3.0, 717.0)),
        });
    }

    let mut org_tree = QuadTree::new(0, [0.0, 0.0, 1260.0, 720.0]);
    let mut pred_tree = QuadTree::new(0, [0.0, 0.0, 1260.0, 720.0]);
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(BLACK);
        org_tree.clear();
        pred_tree.clear();
        spawn_food += get_frame_time();
        add_to_pop += get_frame_time();

        if spawn_food >= 3.0 {
            spawn_food = 0.0;
            for _ in 0..gen_range(20, 30) {
                all_objects.push(Food {
                    energy: gen_range(0.3, 1.0),
                    rad: 3.0,
                    pos: vec2(gen_range(3.0, 1257.0), gen_range(3.0, 717.0)),
                });
            }
        }

        for org in all_objects.iter_mut() {
            match org {
                Organism { .. } => {
                    org.update();
                    pred_tree.insert(org);
                }
                Predator { .. } => {
                    org.update();
                    org_tree.insert(org);
                }
                Food { pos, rad, energy } => {
                    draw_circle(pos.x, pos.y, *rad, Color::new(0.0, 1.0, 0.0, *energy));
                    org_tree.insert(org);
                }
            }
        }

        let mut return_objects: Vec<Entity>;
        let mut food_to_remove: Vec<Vec2> = Vec::new();
        let mut organisms_to_remove: Vec<Vec2> = Vec::new();
        let mut organisms_to_add: Vec<Vec2> = Vec::new();
        let mut predator_to_remove: Vec<Vec2> = Vec::new();
        let mut predator_to_add: Vec<Vec2> = Vec::new();
        let mut organims_count = 0;
        let mut predator_count = 0;
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
                org_tree.retrieve(&mut return_objects, obj);

                if let Organism {
                    pos: op,
                    rad: or,
                    energy: oen,
                    target: otar,
                    reproductive_urge: repurge,
                    ..
                } = obj
                {
                    for other_obj in return_objects {
                        match other_obj {
                            Food { pos, rad, energy } => {
                                if pos.distance(*op) <= *or + rad {
                                    *oen += energy;
                                    *repurge += gen_range(0.0, energy);
                                    food_to_remove.push(pos);
                                } else if pos.distance(*op) <= *or + rad + 20. {
                                    *otar = pos;
                                }
                            }
                            Predator { pos, .. } => {
                                if pos.distance(*op) <= *or + 20. {
                                    *otar = *op + *op - pos;
                                }
                            }
                            _ => {}
                        }
                    }
                    if *oen <= 0.0 {
                        organisms_to_remove.push(*op);
                    } else if *oen >= 1.0 && *repurge >= 1.0 {
                        *oen -= 0.6;
                        *repurge = 0.0;
                        organisms_to_add.push(*op);
                    }
                }
            } else if let Predator {
                energy,
                size,
                pos,
                sides,
                rot,
                ..
            } = obj
            {
                predator_count += 1;
                draw_poly_lines(
                    pos.x,
                    pos.y,
                    *sides,
                    *size,
                    *rot,
                    2.,
                    Color::new(1.0, 0.0, 0.0, *energy),
                );
                return_objects = Vec::new();
                if *energy < 0.6 {
                    pred_tree.retrieve(&mut return_objects, obj);
                }

                if let Predator {
                    energy: pen,
                    pos: position,
                    size: psize,
                    target: ptar,
                    reproductive_urge: repurge,
                    ..
                } = obj
                {
                    for other_obj in return_objects {
                        match other_obj {
                            Organism {
                                pos, rad, energy, ..
                            } => {
                                if pos.distance(*position) <= *psize + rad {
                                    *pen += energy;
                                    *repurge += gen_range(0.0, energy);
                                    organisms_to_remove.push(pos);
                                } else if pos.distance(*position) <= *psize + rad + 20. {
                                    *ptar = pos;
                                }
                            }
                            _ => {}
                        }
                    }
                    if *pen <= 0.0 {
                        predator_to_remove.push(*position);
                    } else if *pen >= 1.0 && *repurge >= 1.5 {
                        *pen -= 0.8;
                        *repurge = 0.0;
                        predator_to_add.push(*position);
                    }
                }
            } else {
                food_count += 1;
            }
        }

        if add_to_pop >= 1.0 {
            organism_population.push(organims_count);
            predator_population.push(predator_count);
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
        if !predator_to_remove.is_empty() {
            all_objects.retain(|o| match o {
                Predator { pos, .. } => !predator_to_remove.contains(pos),
                _ => true,
            });
        }
        for new_cell in organisms_to_add {
            let angle = gen_range(0.0, std::f32::consts::PI * 2.);
            all_objects.push(Organism {
                energy: gen_range(0.6, 0.85),
                rad: gen_range(6.0, 11.0),
                pos: new_cell,
                reproductive_urge: 0.0,
                target: vec2(new_cell.x + angle.cos(), new_cell.y + angle.sin()),
                velocity: Vec2::ZERO,
            });
        }
        for new_pred in predator_to_add {
            let angle = gen_range(0.0, std::f32::consts::PI * 2.0);

            all_objects.push(Predator {
                energy: gen_range(0.6, 1.1),
                rot: angle * 180. / std::f32::consts::PI,
                sides: 5,
                size: gen_range(10., 13.),
                pos: new_pred,
                velocity: Vec2::ZERO,
                target: vec2(new_pred.x + angle.cos(), new_pred.y + angle.sin()),
                reproductive_urge: 0.0,
            });
        }
        // draw_text(&format!("FPS: {}", get_fps() as u32, 10., 20., 20., WHITE);
        draw_text(
            &format!("Population: {}", organims_count),
            10.,
            20.,
            20.,
            BLUE,
        );
        draw_text(
            &format!("Population: {}", predator_count),
            10.,
            50.,
            20.,
            RED,
        );
        draw_text(
            &format!("Time Elapsed: {}", get_time().round()),
            screen_width() - 150.,
            20.,
            20.,
            WHITE,
        );

        next_frame().await
    }
    plot(organism_population, predator_population, food);
}
