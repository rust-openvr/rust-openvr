extern crate openvr;
extern crate nalgebra;

#[macro_use]
extern crate glium;

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
        }

        // init compositor subsystem
        /*let comp = match openvr::compositor() {
            Ok(ext) => ext,
            Err(err) => {
                println!("Failed to create IVRCompositor subsystem {:?}", err);
                return;
            }
        };*/


        // create glium window and context
        use glium::{DisplayBuild, Surface};
        let display = glium::glutin::WindowBuilder::new()
                .with_depth_buffer(24)
                .build_glium()
                .unwrap();

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
        let controller = models.load(String::from("generic_hmd")).unwrap_or_else(|err| {
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

        'render: loop {
            // this is important to make sure frames are synced correctly
            //let _ = comp.wait_get_poses();

            // render 2d display output
            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

            let uniforms = uniform! {
                matrix: [
                    [5.0, 0.0, 0.0, 0.0],
                    [0.0, 5.0, 0.0, 0.0],
                    [0.0, 0.0, 5.0, 0.0],
                    [0.0 , 0.0, 0.0, 1.0f32],
                ],
                diffuse: &controller_texture
            };

            let params = glium::DrawParameters {
               depth: glium::Depth {
                   test: glium::draw_parameters::DepthTest::IfLess,
                   write: true,
                   .. Default::default()
               },
               backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
               .. Default::default()
            };

            target.draw(&controller_vertex_buffer, &controller_index_buffer, &program, &uniforms, &params).unwrap();

            // render hmd eye outputs

            // finish all rendering
            target.finish().unwrap();

            // submit to hmd

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
