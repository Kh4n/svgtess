#[macro_use]
extern crate gfx;
extern crate gfx_window_sdl;
extern crate sdl2;
extern crate cgmath;
extern crate svgtess;

use gfx::Device;
use gfx::traits::FactoryExt;
use cgmath::prelude::*;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    constant Transform {
        transform: [[f32; 4];4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut builder = video.window("Example", 800, 800);

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 2);
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);


    let (mut window, mut gl_context, mut device, mut factory, color_view, depth_view) =
        gfx_window_sdl::init::<ColorFormat, DepthFormat>(&video, builder).unwrap();
    
    let pso = factory.create_pipeline_simple(
        include_bytes!("../shaders/vertex.glslv"),
        include_bytes!("../shaders/frag.glslf"),
        pipe::new()
    ).unwrap();

    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    /*let mut rect: [Vertex; 4] = [
        Vertex { pos: [ -0.5, -0.5, 0.0, 1.0 ], color: [1.0, 0.0, 0.0] },
        Vertex { pos: [  0.5, -0.5, 0.0, 1.0 ], color: [0.0, 1.0, 0.0] },
        Vertex { pos: [  0.0,  0.5, 0.0, 1.0 ], color: [0.0, 0.0, 1.0] },
        Vertex { pos: [  1.0,  1.0, 0.0, 1.0 ], color: [0.0, 0.0, 1.0] },
        ];
    const INDICES: &[u16] = &[0, 1, 2, 2, 1, 3];*/

    let line1 = vec![
        cgmath::Vector2::<f32>::new(0.0, 0.0), cgmath::Vector2::<f32>::new(0.8, 0.0), cgmath::Vector2::<f32>::new(0.8, -0.8),
        cgmath::Vector2::<f32>::new(0.4, -0.8), cgmath::Vector2::<f32>::new(0.4, -0.4)
        ];
    let (path, pathindices) = path_tesselate(&line1, 0.05);

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&path[..], &pathindices[..]);
    let transform_buffer = factory.create_constant_buffer(1);
    let data = pipe::Data {
        vbuf: vertex_buffer,
        transform: transform_buffer,
        out: color_view.clone(),
    };
    //Identity Matrix
    const TRANSFORM: Transform = Transform {
            transform: [[1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0]]
    };

    'main: loop {
        let mut event_pump = sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'main;
                }
                _ => {}
            }
        }
        
        encoder.clear(&color_view, BLACK); //clear the framebuffer with a color(color needs to be an array of 4 f32s, RGBa)
        encoder.update_buffer(&data.transform, &[TRANSFORM], 0); //update buffers
        //encoder.update_buffer(&data.vbuf, &rect, 0);
        encoder.draw(&slice, &pso, &data); // draw commands with buffer data and attached pso
        encoder.flush(&mut device); // execute draw commands

        //rect[0].pos[0] += 0.01;

        window.gl_swap_window();
        device.cleanup();
    }
    let vector = cgmath::Vector2::<f32>::new(3.0, 4.0);
    let mut vector2 = cgmath::Vector2::<f32>::new(1.0, 4.0);
    println!("{:?}\n", vector.normalize() - vector2);

    let linea0 = cgmath::Vector2::<f32>::new(1.0, 1.0);
    let linea1 = cgmath::Vector2::<f32>::new(-1.0, -1.0);
    let lineb0 = cgmath::Vector2::<f32>::new(-1.0, 0.1);
    let lineb1 = cgmath::Vector2::<f32>::new(-0.5, -0.9);

    let inters = intersection(linea0, linea1, lineb0, lineb1);
    let _tmp = cgmath::Vector2::new(-lineb1.y, lineb1.x);
    println!("{:?}\n", inters);
}