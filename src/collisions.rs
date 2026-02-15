use crate::Shape;
use crate::{Mat2, Vec2, TAU};

pub fn collision(object1: &impl Shape, object2: &impl Shape) -> bool {
    for vertice in object1.shape() {
        if inside(&vertice, object2) {
            return true;
        }
    }
    for vertice in object2.shape() {
        if inside(&vertice, object1) {
            return true;
        }
    }
    false
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

fn inside(vertice: &Vec2, object: &impl Shape) -> bool {
    let vertices = object.shape();
    let mut surface_perps = vec![];

    for it in 0..vertices.len() {
        let b = vertices[(it + 1) % vertices.len()];
        let a = vertices[it % vertices.len()];
        let normal = b - a;
        let orthogonal = rotate(&normal, TAU / 4.0);

        surface_perps.push(orthogonal);
    }

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
            return false;
        }

        if vertice_proj > *max_proj {
            return false;
        }
    }

    return true;
}
