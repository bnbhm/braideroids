use crate::Shape;
use crate::{Mat2, Vec2, TAU};

pub fn collision(
    object1: &impl Shape,
    object2: &impl Shape,
) -> Option<[Vec<VerticeCollisionInfo>; 2]> {
    let mut info1 = Vec::new();
    let mut info2 = Vec::new();

    for (vertice_index, vertice) in object1.shape().iter().enumerate() {
        if let Some(mtv) = inside(&vertice, object2) {
            info1.push(VerticeCollisionInfo { vertice_index, mtv });
        }
    }

    for (vertice_index, vertice) in object2.shape().iter().enumerate() {
        if let Some(mtv) = inside(&vertice, object1) {
            info2.push(VerticeCollisionInfo { vertice_index, mtv });
        }
    }

    if info1.is_empty() && info2.is_empty() {
        return None;
    } else {
        return Some([info1, info2]);
    }
}

fn inside(vertice: &Vec2, object: &impl Shape) -> Option<MTV> {
    let vertices = object.shape();
    let mut surface_perps = vec![];

    for it in 0..vertices.len() {
        let b = vertices[(it + 1) % vertices.len()];
        let a = vertices[it % vertices.len()];
        let normal = b - a;
        let orthogonal = rotate(&normal, TAU / 4.0).normalize();

        surface_perps.push(orthogonal);
    }

    let mut min_overlap_magnitude = f32::MAX;
    let mut min_overlap_direction = surface_perps[0];
    for axis in surface_perps {
        let projections: Vec<f32> = vertices.iter().map(|vertice| vertice.dot(axis)).collect();
        let min_proj = (&projections)
            .into_iter()
            .min_by(|a, b| a.total_cmp(b))
            .unwrap();
        let max_proj = (&projections)
            .into_iter()
            .max_by(|a, b| a.total_cmp(b))
            .unwrap();
        let vertice_proj = vertice.dot(axis);

        if vertice_proj < *min_proj {
            return None;
        }

        if vertice_proj > *max_proj {
            return None;
        }

        let overlap = (*max_proj - vertice_proj).min(vertice_proj - *min_proj);

        if overlap < min_overlap_magnitude {
            min_overlap_magnitude = overlap;
            min_overlap_direction = axis;
        }
    }

    return Some(MTV {
        min_overlap_magnitude,
        min_overlap_direction,
    });
}

struct MTV {
    min_overlap_magnitude: f32,
    min_overlap_direction: Vec2,
}

pub struct VerticeCollisionInfo {
    vertice_index: usize,
    mtv: MTV,
}

fn rotate(vector: &Vec2, theta: f32) -> Vec2 {
    Mat2 {
        x_axis: Vec2 {
            x: theta.cos(),
            y: -1.0 * theta.sin(),
        },
        y_axis: Vec2 {
            x: theta.sin(),
            y: theta.cos(),
        },
    } * *vector
}
