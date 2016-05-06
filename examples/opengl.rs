extern crate openvr;
extern crate nalgebra;

#[macro_use]
extern crate glium;

use std::convert::From;
use nalgebra::Inverse;
use glium::framebuffer::ToColorAttachment;
use glium::framebuffer::ToDepthAttachment;
use glium::GlObject;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texcoord: [f32; 2]
}
implement_vertex!(Vertex, position, normal, texcoord);

pub fn main() {
    {
        // init vr system
        let system = match openvr::init() {
            Ok(ivr) => ivr,
            Err(err) => {
                println!("Failed to create IVR subsystem {:?}", err);
                return;
            }
        };

        // init render model subsystem
        let models = match openvr::render_models() {
            Ok(ext) => ext,
            Err(err) => {
                println!("Failed to create IVRRenderModels subsystem {:?}", err);
                return;
            }
        };

        for device in system.tracked_devices(0.0).connected_iter() {
            println!("device found :) -> {}",
                device.get_property_string(openvr::ETrackedDeviceProperty_Prop_RenderModelName_String).unwrap_or_else(|_| { panic!("No render model")} ));

            println!("\t{:?}", device);
            println!("\t{:?}", device.device_class());
        }

        // init compositor subsystem
        let comp = match openvr::compositor() {
            Ok(ext) => ext,
            Err(err) => {
                println!("Failed to create IVRCompositor subsystem {:?}", err);
                return;
            }
        };

        // create glium window and context
        use glium::{DisplayBuild, Surface};
        let display = glium::glutin::WindowBuilder::new()
                .with_depth_buffer(24)
                .build_glium()
                .unwrap();

        // create frame buffer for hmd
        let texture_size = system.recommended_render_target_size();

        let left_eye_depth = glium::framebuffer::DepthRenderBuffer::new(
            &display,
            glium::texture::DepthFormat::I24,
            texture_size.width,
            texture_size.height).unwrap();

        let left_eye_texture = glium::framebuffer::RenderBuffer::new(
            &display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            texture_size.width,
            texture_size.height).unwrap();

        let mut left_eye_framebuffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer
            (
                &display, left_eye_texture.to_color_attachment(), left_eye_depth.to_depth_attachment()
            ).unwrap();

        let right_eye_depth = glium::framebuffer::DepthRenderBuffer::new(
            &display,
            glium::texture::DepthFormat::I24,
            texture_size.width,
            texture_size.height).unwrap();

        let right_eye_texture = glium::framebuffer::RenderBuffer::new(
            &display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            texture_size.width,
            texture_size.height).unwrap();

        let mut right_eye_framebuffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer
            (
                &display, right_eye_texture.to_color_attachment(), right_eye_depth.to_depth_attachment()
            ).unwrap();

        // prepare shader
        let vertex_shader_src = r#"
            #version 140

            in vec3 position;
            in vec3 normal;
            in vec2 texcoord;

            out vec3 v_normal;
            out vec2 v_texcoord;

            uniform mat4 matrix;

            void main() {
                v_normal = normal;
                v_texcoord = texcoord;
                gl_Position = matrix * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            uniform sampler2D diffuse;

            in vec3 v_normal;
            in vec2 v_texcoord;

            out vec4 color;

            void main() {
                color = texture(diffuse, v_texcoord);
            }
        "#;

        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        // load controller models
        let controller = models.load(String::from("lh_basestation_vive")).unwrap_or_else(|err| {
            openvr::shutdown(); panic!("controller render model not found: {:?}", err) });

        let mut controller_vertices: Vec<Vertex> = Vec::new();
        let mut controller_indices: Vec<u16> = Vec::new();
        for vertex in controller.vertex_iter() {
            controller_vertices.push(Vertex {
                position: [vertex.vPosition.v[0] as f32, vertex.vPosition.v[1] as f32, vertex.vPosition.v[2] as f32],
                normal: [vertex.vNormal.v[0] as f32, vertex.vNormal.v[1] as f32, vertex.vNormal.v[2] as f32],
                texcoord: [vertex.rfTextureCoord[0] as f32, vertex.rfTextureCoord[1] as f32],
            });
        }
        for index in controller.index_iter() {
            controller_indices.push(*index);
        }

        let controller_vertex_buffer = glium::VertexBuffer::new(&display, &controller_vertices).unwrap();
        let controller_index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &controller_indices).unwrap();
        let controller_texture_response = controller.load_texture().unwrap();
        let dimension = (controller_texture_response.dimension().0 as u32, controller_texture_response.dimension().1 as u32);
        let image = glium::texture::RawImage2d::from_raw_rgba(controller_texture_response.to_vec(), dimension);
        let controller_texture = glium::texture::Texture2d::new(&display, image).unwrap();

        // get static jmatrices
        let left_projection = {
            let raw = system.projection_matrix(openvr::Eye::Left, 0.01, 1000.0);
            let mat = nalgebra::Matrix4::new(
                raw[0][0], raw[0][1], raw[0][2], raw[0][3],
                raw[1][0], raw[1][1], raw[1][2], raw[1][3],
                raw[2][0], raw[2][1], raw[2][2], raw[2][3],
                raw[3][0], raw[3][1], raw[3][2], raw[3][3]);
            mat
        };

        let left_eye_transform = {
            let raw = system.eye_to_head_transform(openvr::Eye::Left);
            let mat = nalgebra::Matrix4::new(
                raw[0][0], raw[1][0], raw[2][0], 0.0,
                raw[0][1], raw[1][1], raw[2][1], 0.0,
                raw[0][2], raw[1][2], raw[2][2], 0.0,
                raw[0][3], raw[1][3], raw[2][3], 1.0);
            mat.inverse().unwrap()
        };
        let right_projection = {
            let raw = system.projection_matrix(openvr::Eye::Right, 0.01, 1000.0);
            let mat = nalgebra::Matrix4::new(
                raw[0][0], raw[0][1], raw[0][2], raw[0][3],
                raw[1][0], raw[1][1], raw[1][2], raw[1][3],
                raw[2][0], raw[2][1], raw[2][2], raw[2][3],
                raw[3][0], raw[3][1], raw[3][2], raw[3][3]);
            mat
        };

        let right_eye_transform = {
            let raw = system.eye_to_head_transform(openvr::Eye::Right);
            let mat = nalgebra::Matrix4::new(
                raw[0][0], raw[1][0], raw[2][0], 0.0,
                raw[0][1], raw[1][1], raw[2][1], 0.0,
                raw[0][2], raw[1][2], raw[2][2], 0.0,
                raw[0][3], raw[1][3], raw[2][3], 1.0);
            mat.inverse().unwrap()
        };

        'render: loop {
            // this is important to make sure frames are synced correctly
            let tracked_devices = comp.wait_get_poses();

            let mut left_matrix = left_projection * left_eye_transform;
            let mut right_matrix = right_projection * right_eye_transform;
            let mut once = false;

            for device in tracked_devices.connected_iter() {
                match device.device_class() {
                    openvr::ETrackedDeviceClass_TrackedDeviceClass_HMD => {
                        let matrix = {
                            let raw = device.to_device;
                            let mat = nalgebra::Matrix4::new(
                                raw[0][0], raw[0][1], raw[0][2], raw[0][3],
                                raw[1][0], raw[1][1], raw[1][2], raw[1][3],
                                raw[2][0], raw[2][1], raw[2][2], raw[2][3],
                                0.0, 0.0, 0.0, 1.0);
                            mat.inverse().unwrap()
                        };
                        left_matrix *= matrix;
                        right_matrix *= matrix;
                    },
                    openvr::ETrackedDeviceClass_TrackedDeviceClass_TrackingReference => {
                        if once { continue; }
                        once = true;

                        let matrix = {
                            let raw = device.to_device;
                            let mat = nalgebra::Matrix4::new(
                                raw[0][0], raw[0][1], raw[0][2], raw[0][3],
                                raw[1][0], raw[1][1], raw[1][2], raw[1][3],
                                raw[2][0], raw[2][1], raw[2][2], raw[2][3],
                                0.0, 0.0, 0.0, 1.0);
                            mat
                        };

                        left_matrix *= matrix;
                        right_matrix *= matrix;
                    },
                    _ => { }
                };
            }

            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

            let left_uniforms = uniform! {
                matrix: *left_matrix.as_ref(),
                diffuse: &controller_texture
            };

            let right_uniforms = uniform! {
                matrix: *right_matrix.as_ref(),
                diffuse: &controller_texture
            };

            let params = glium::DrawParameters {
               depth: glium::Depth {
                   test: glium::draw_parameters::DepthTest::IfLess,
                   write: true,
                   .. Default::default()
               },
               backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
               .. Default::default()
            };

            // render 2d display output
            target.draw(&controller_vertex_buffer, &controller_index_buffer, &program, &left_uniforms, &params).unwrap();

            // render hmd eye outputs
            left_eye_framebuffer.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
            right_eye_framebuffer.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

            left_eye_framebuffer.draw(&controller_vertex_buffer, &controller_index_buffer, &program, &left_uniforms, &params).unwrap();
            right_eye_framebuffer.draw(&controller_vertex_buffer, &controller_index_buffer, &program, &right_uniforms, &params).unwrap();

            // finish all rendering
            target.finish().unwrap();

            // submit to hmd
            comp.submit(openvr::Eye::Left, left_eye_texture.get_id() as usize, openvr::common::TextureBounds::new((0.0, 1.0), (0.0, 1.0)));
            comp.submit(openvr::Eye::Right, right_eye_texture.get_id() as usize, openvr::common::TextureBounds::new((0.0, 1.0), (0.0, 1.0)));

            // handle window events
            for ev in display.poll_events() {
                match ev {
                    glium::glutin::Event::Closed => break 'render,   // the window has been closed by the user
                    _ => ()
                }
            }
        }
    }

    // free openvr
    openvr::shutdown();
}
