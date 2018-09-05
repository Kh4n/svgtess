extern crate cgmath;

use cgmath::prelude::*;

pub struct Vertex {
        pos: [f32; 4],
        color: [f32; 3]
}

pub fn is_to_right(v1: cgmath::Vector2<f32>, v2: cgmath::Vector2<f32>) -> bool {
    let v2r90 = cgmath::Vector2::new(-v2.y,  v2.x);
    (cgmath::dot(v1, v2r90) > 0.0)
}

fn intersection(
    pa0: cgmath::Vector2<f32>, pa1: cgmath::Vector2<f32>, 
    pb0: cgmath::Vector2<f32>, pb1: cgmath::Vector2<f32>
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

pub fn path_tesselate(points: &Vec<cgmath::Vector2<f32>>, thickness: f32) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::<Vertex>::new();
    let mut indices = Vec::<u16>::new();
    //t: short for triangles. need to keep track to offset the index buffer
    let mut v: u16 = 0;

    let mut vector_dir1 = points[1] - points[0];
    let mut vector_dir2 = points[2] - points[1];
    let mut perpendicular1: cgmath::Vector2<f32>;
    let mut perpendicular2: cgmath::Vector2<f32>;
    if is_to_right(vector_dir1, vector_dir2) {
        perpendicular1 = cgmath::Vector2::<f32>::new(-vector_dir1.y, vector_dir1.x).normalize();
    } else {
        perpendicular1 = cgmath::Vector2::<f32>::new(vector_dir1.y, -vector_dir1.x).normalize();
    }

    let pa0 = (perpendicular1 * 0.5 * thickness) + points[0];
    let pb0 = (perpendicular1 * -0.5 * thickness) + points[0];
    vertices.push(Vertex{pos: [pa0.x, pa0.y, 0.0, 1.0], color: [0.0, 0.0, 1.0]});
    vertices.push(Vertex{pos: [pb0.x, pb0.y, 0.0, 1.0], color: [0.0, 1.0, 1.0]});
    v += 2;
    let mut i = 0;
    for i in 0..(points.len() - 2) {
        vector_dir1 = points[i + 1] - points[i + 0];
        vector_dir2 = points[i + 2] - points[i + 1];
        if is_to_right(vector_dir1, vector_dir2) {
            perpendicular1 = cgmath::Vector2::<f32>::new(-vector_dir1.y, vector_dir1.x).normalize();
            perpendicular2 = cgmath::Vector2::<f32>::new(-vector_dir2.y, vector_dir2.x).normalize();
        } else {
            perpendicular1 = cgmath::Vector2::<f32>::new(vector_dir1.y, -vector_dir1.x).normalize();
            perpendicular2 = cgmath::Vector2::<f32>::new(vector_dir2.y, -vector_dir2.x).normalize();
        }
        let pa0 = cgmath::Vector2::new(vertices[(v - 2) as usize].pos[0], vertices[(v - 2) as usize].pos[1]);
        let pb0 = cgmath::Vector2::new(vertices[(v - 1) as usize].pos[0], vertices[(v - 1) as usize].pos[1]);
        let pa1 = (perpendicular1 * 0.5     * thickness) + points[i + 1];
        let pb1 = (perpendicular1 * -0.5 * thickness) + points[i + 1];

        let pc0 = (perpendicular2 * 0.5 * thickness) + points[i + 1];
        let pc1 = (perpendicular2 * 0.5 * thickness) + points[i + 2];
        let pd0 = (perpendicular2 * -0.5 * thickness) + points[i + 1];
        let pd1 = (perpendicular2 * -0.5 * thickness) + points[i + 2];

        //ij: innerjoint, oj: outerjoint, ojf: outeranchor first, ojs: outeranchor second
        let ij = intersection(pb0, pb1, pd0, pd1);
        let oj = intersection(pa0, pa1, pc0, pc1);
        let oaf = (perpendicular1 * thickness) + ij;
        let oas = (perpendicular2 * thickness) + ij;

        vertices.push(Vertex{pos: [oaf.x, oaf.y, 0.0, 1.0], color: [0.0, 1.0, 1.0]});
        vertices.push(Vertex{pos: [oj.x,  oj.y,  0.0, 1.0], color: [0.0, 1.0, 1.0]});
        vertices.push(Vertex{pos: [oas.x, oas.y, 0.0, 1.0], color: [0.0, 1.0, 1.0]});
        vertices.push(Vertex{pos: [ij.x,  ij.y,  0.0, 1.0], color: [0.0, 1.0, 1.0]});
        v += 4;

        indices.extend(&[
            v - 6, v - 5, v - 1, 
            v - 1, v - 4, v - 6, 
            v - 1, v - 2, v - 4,
            v - 2, v - 3, v - 4,
        ]);
    }
    
    println!("{:?}", i);
    (vertices, indices)
}