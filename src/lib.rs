extern crate cgmath;

use cgmath::prelude::*;
use std::f32;

pub enum JointType {
    Miter,
    Bevel,
    Round,
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

pub fn is_to_right(v1: cgmath::Vector2<f32>, v2: cgmath::Vector2<f32>) -> bool {
    let v2r90 = cgmath::Vector2::new(-v2.y,  v2.x);
    (cgmath::dot(v1, v2r90) > 0.0)
}

fn intersection(
    pa0: &cgmath::Vector2<f32>, pa1: &cgmath::Vector2<f32>, 
    pb0: &cgmath::Vector2<f32>, pb1: &cgmath::Vector2<f32>
    ) -> cgmath::Vector2<f32> {
    let a1 = pa1.y - pa0.y;
    let b1 = pa0.x - pa1.x;
    let c1 = (a1 * pa0.x) + (b1 * pa0.y);

    let a2 = pb1.y - pb0.y;
    let b2 = pb0.x - pb1.x;
    let c2 = (a2 * pb0.x) + (b2 * pb0.y);

    let determinant = (a1 * b2) - (a2 * b1);

    let x = (b2*c1 - b1*c2)/determinant;
    let y = (a1*c2 - a2*c1)/determinant;

    cgmath::Vector2::new(x, y)
}

//Takes a vector of points, a line thickness, and a joint type, and generates a series of vertices and indices which can be fed to OpenGL
pub fn path_tessellate(points: &Vec<cgmath::Vector2<f32>>, thickness: f32, jtype: JointType) -> (Vec<cgmath::Vector2<f32>>, Vec<u16>) {
    let mut vertices = Vec::<cgmath::Vector2<f32>>::with_capacity(points.len() * 3);
    let mut indices = Vec::<u16>::with_capacity(points.len() * 3);
    //v: short for vertices, keeps track of points.len() without repeatedly calling it
    let mut v: u16 = 0;
    let mut prev_dir: Direction;
    let mut curr_dir: Direction;

    let mut vector_dir1 = points[1] - points[0];
    let mut vector_dir2;
    if points.len() > 2 {
        vector_dir2 = points[2] - points[1];
    } else {
        vector_dir2 = cgmath::Vector2::<f32>::unit_x();
    }
    let mut perpendicular1: cgmath::Vector2<f32>;
    let mut perpendicular2: cgmath::Vector2<f32>;
    let mut pc1: cgmath::Vector2<f32> = cgmath::Vector2::new(0.0, 0.0);
    let mut pd1: cgmath::Vector2<f32> = cgmath::Vector2::new(0.0, 0.0);
    let mut p_ij;
    let mut p_oas;
    if is_to_right(vector_dir1, vector_dir2) {
        perpendicular1 = cgmath::Vector2::<f32>::new(-vector_dir1.y, vector_dir1.x).normalize();
        prev_dir = Direction::Right;
    } else {
        perpendicular1 = cgmath::Vector2::<f32>::new(vector_dir1.y, -vector_dir1.x).normalize();
        prev_dir = Direction::Left;
    }

    let pa0 = (perpendicular1 * 0.5 * thickness) + points[0];
    let pb0 = (perpendicular1 * -0.5 * thickness) + points[0];
    vertices.push(pa0);
    vertices.push(pb0);
    v += 2;
    p_ij = v - 1;
    p_oas = v - 2;
    for i in 0..(points.len() - 2) {
        vector_dir1 = points[i + 1] - points[i + 0];
        vector_dir2 = points[i + 2] - points[i + 1];
        if is_to_right(vector_dir1, vector_dir2) {
            perpendicular1 = cgmath::Vector2::<f32>::new(-vector_dir1.y, vector_dir1.x).normalize();
            perpendicular2 = cgmath::Vector2::<f32>::new(-vector_dir2.y, vector_dir2.x).normalize();
            curr_dir = Direction::Right;
        } else {
            perpendicular1 = cgmath::Vector2::<f32>::new(vector_dir1.y, -vector_dir1.x).normalize();
            perpendicular2 = cgmath::Vector2::<f32>::new(vector_dir2.y, -vector_dir2.x).normalize();
            curr_dir = Direction::Left;
        }
        let pa0 = cgmath::Vector2::new(vertices[(v - 2) as usize].x, vertices[(v - 2) as usize].y);
        let pb0 = cgmath::Vector2::new(vertices[(v - 1) as usize].x, vertices[(v - 1) as usize].y);
        let pa1 = (perpendicular1 *  0.5 * thickness) + points[i + 1];
        let pb1 = (perpendicular1 * -0.5 * thickness) + points[i + 1];

        let pc0 = (perpendicular2 *  0.5 * thickness) + points[i + 1];
            pc1 = (perpendicular2 *  0.5 * thickness) + points[i + 2];
        let pd0 = (perpendicular2 * -0.5 * thickness) + points[i + 1];
            pd1 = (perpendicular2 * -0.5 * thickness) + points[i + 2];

        //ij: innerjoint, oj: outerjoint, ojf: outeranchor first, ojs: outeranchor second
        let ij = intersection(&pb0, &pb1, &pd0, &pd1);
        let oaf = (perpendicular1 * thickness) + ij;
        let oas = (perpendicular2 * thickness) + ij;
        vertices.push(oaf);
        vertices.push(ij);
        v += 2;
        if curr_dir == prev_dir {
            indices.extend(&[
                p_oas,  p_ij, v - 1, 
                v - 1, v - 2, p_oas,
            ]);
        } else {
            indices.extend(&[
                p_oas,  p_ij, v - 1, 
                v - 1, v - 2, p_ij,
            ]);
        }
        p_ij = v - 1;
        match jtype {
            JointType::Miter => {
                let oj = intersection(&pa0, &pa1, &pc0, &pc1);
                vertices.push(oj);
                vertices.push(oas);
                v += 2;
                indices.extend(&[
                     p_ij, v - 1, v - 4,
                    v - 1, v - 2, v - 4,
                ]);
                p_oas = v - 1;
            }
            JointType::Bevel => {
                vertices.push(oas);
                v += 1;
                indices.extend(&[ 
                     p_ij, v - 1, v - 3,
                ]);
                p_oas = v - 1;
            }
            JointType::Round => {
                vertices.push(oaf);
                vertices.push(ij);
                v += 2;
                indices.extend(&[
                    p_oas, p_ij,  v - 1, 
                    v - 1, v - 2, p_oas,
                ]);
                p_ij = v - 1;
                let rot: cgmath::Basis2<f32>;
                let i2r: bool;
                if let Direction::Right = curr_dir {
                    rot = Rotation2::from_angle(cgmath::Deg(-0.5));
                    i2r = true;
                } else {
                    rot = Rotation2::from_angle(cgmath::Deg(0.5));
                    i2r = false;
                }
                let mut curr_direction = perpendicular1 * thickness;
                let end_direction = perpendicular2 * thickness;
                curr_direction = rot.rotate_vector(curr_direction);
                vertices.push(curr_direction + ij);
                v += 1;
                indices.extend(&[p_ij, v - 1, v - 3]);
                while is_to_right(curr_direction, end_direction) == i2r {
                    curr_direction = rot.rotate_vector(curr_direction);
                    vertices.push(curr_direction + ij);
                    v += 1;
                    indices.extend(&[p_ij, v - 1, v - 2]);
                }
                vertices.push(oas);
                v += 1;
                indices.extend(&[p_ij, v - 1, v - 2]);
                p_oas = v - 1;
            }
        }
    }
    vertices.push(pc1);
    vertices.push(pd1);
    v += 2;
    indices.extend(&[
         p_ij, v - 1, v - 2, 
        v - 2, p_oas, p_ij, 
    ]);
    
    println!("{:?}\n", vertices);
    (vertices, indices)
}