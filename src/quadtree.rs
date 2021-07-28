use crate::entities::Entity;
use macroquad::prelude::*;

pub struct QuadTree {
    level: u8,
    objects: Vec<Entity>,
    bounds: [f32; 4],
    nodes: Vec<QuadTree>,
}

/* Anti Clockwise alignment of nodes
┌───┬───┐
│ 2 │ 1 │
├───┼───┤
│ 3 │ 4 │
└───┴───┘
*/

impl QuadTree {
    pub fn new(level: u8, bounds: [f32; 4]) -> QuadTree {
        QuadTree {
            level,
            objects: Vec::new(),
            bounds,
            nodes: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.objects = Vec::new();
        for node in self.nodes.iter_mut() {
            node.clear();
        }
    }
    fn split(&mut self) {
        let width_half = self.bounds[2] / 2.;
        let height_half = self.bounds[3] / 2.;
        let x = self.bounds[0];
        let y = self.bounds[1];
        let lev = self.level + 1;
        self.nodes = vec![
            QuadTree::new(lev, [x + width_half, y, width_half, height_half]),
            QuadTree::new(lev, [x, y, width_half, height_half]),
            QuadTree::new(lev, [x, y + height_half, width_half, height_half]),
            QuadTree::new(
                lev,
                [x + width_half, y + height_half, width_half, height_half],
            ),
        ];
    }
    fn get_index(&self, circobj: &Entity) -> Option<usize> {
        let mut index: Option<usize> = None;
        let midx = self.bounds[0] + (self.bounds[2] / 2.);
        let midy = self.bounds[1] + (self.bounds[3] / 2.);

        match circobj {
            Entity::Food { rad: r, pos: p, .. } | Entity::Organism { rad: r, pos: p, .. } | Entity::Predator { size: r, pos: p, .. } => {
                let rad = *r;
                let pos = *p;
                let top_quad = pos.y - rad < midy && pos.y + rad < midy;
                let bottom_quad = pos.y - rad > midy;

                if pos.x - rad < midx && pos.x + rad < midx {
                    if top_quad {
                        index = Some(1);
                    } else if bottom_quad {
                        index = Some(2);
                    }
                } else if pos.x - rad > midx {
                    if top_quad {
                        index = Some(0);
                    } else if bottom_quad {
                        index = Some(3);
                    }
                }
            } // _ => panic!("Unsupported Entity for quadtree"),
        }
        index
    }
    pub fn insert(&mut self, circobj: &Entity) {
        if !self.nodes.is_empty() {
            let index = self.get_index(circobj);
            if let Some(i) = index {
                self.nodes[i].insert(circobj);
                return;
            }
        }
        self.objects.push(*circobj);
        if self.objects.len() > 10 && self.level < 5 {
            if self.nodes.is_empty() {
                self.split();
            }
            let mut i: usize = 0;
            while i < self.objects.len() {
                let ind = self.get_index(&self.objects[i]);
                match ind {
                    Some(inde) => {
                        self.nodes[inde].insert(&self.objects.remove(i));
                    }
                    None => {
                        i += 1;
                    }
                }
            }
        }
    }
    pub fn retrieve(&self, return_objects: &mut Vec<Entity>, obj: &Entity) {
        let index = self.get_index(obj);
        if let Some(index) = index {
            if !self.nodes.is_empty() {
                self.nodes[index].retrieve(return_objects, obj);
            }
        }
        for obj in &self.objects {
            return_objects.push(*obj);
        }
    }
}
