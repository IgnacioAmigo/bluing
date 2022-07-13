use egui::Checkbox;
use egui_backend::{egui, gl, sdl2};
use egui_backend::{sdl2::event::Event, DpiScaling, ShaderVersion};
use render::renderer::SpriteRenderer;
use render::subtexture::Subtexture;
use resources::Resources;
use std::path::Path;
use std::time::Instant;
// Alias the backend to something less mouthful
use egui_sdl2_gl as egui_backend;
use sdl2::video::SwapInterval;
use render::{data};
use render::data::AttributedVertex;
use glm;

use render::buffer;

#[macro_use] extern crate render_derive;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

mod render;
mod resources;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::f32_f32_f32,
}

fn main() {
    let mut sprite_pos = (0,0);

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
    }
    let mut i = 0.0;

    let window = video
        .window(
            "Blue",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .opengl()
        .build()
        .unwrap();

    // Create a window context
    let _ctx = window.gl_create_context().unwrap();
    // Init egui stuff
    let shader_ver = ShaderVersion::Default;
    // On linux use GLES SL 100+, like so:
    // let shader_ver = ShaderVersion::Adaptive;
    
    let (mut painter, mut egui_state) =
        egui_backend::with_sdl2(&window, shader_ver, DpiScaling::Custom(1.25));
    
    let mut egui_ctx = egui::CtxRef::default();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut test_str: String =
        "".to_owned();

    let mut enable_vsync = false;
    let mut quit = false;
    let mut slider = 0.0;

    window
        .subsystem()
        .gl_set_swap_interval(SwapInterval::VSync)
        .unwrap();

    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let shader_program = render::GlProgram::from_res(&res, "shaders/triangle").expect("Failed to load triangle shader asset");

    let sprite_renderer = SpriteRenderer::from_res(&res).expect("error creating sprite renderer");

    let texture = res.load_texture("sprites/test.png").expect("error loading test.png to texture");
    let map = res.load_texture("tiles/grass.png").expect("error loading test.png to texture");

    let first_tile = Subtexture::from_tiles(&map, 8,0, glm::vec2(16.0,16.0));

    let vertices: Vec<Vertex> = vec![
        Vertex { pos: (0.5, -0.5, 0.0).into(),  clr: (1.0, 0.0, 0.0).into() }, // bottom right
        Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0).into() }, // bottom left
        Vertex { pos: (0.0,  0.5, 0.0).into(),  clr: (0.0, 0.0, 1.0).into() }  // top
    ];
    
    let vbo = buffer::VertexBuffer::new();
    let vao = buffer::VertexArray::new();  // changed
    vbo.bind();
    vbo.upload_data_static_draw(&vertices);
    vbo.unbind();

    // set up vertex array object

    vao.bind();                               
    vbo.bind();                               
    Vertex::vertex_attrib_pointers();
    vbo.unbind();                             
    vao.unbind();                             

    // set up shared state for window

    unsafe {
        gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let start_time = Instant::now();
    let mut frame_time = Instant::now();
    'running: loop {

        egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_state.input.take());

        egui::CentralPanel::default().show(&egui_ctx, |ui| {
            ui.label(" ");
            ui.text_edit_multiline(&mut test_str);
            ui.label(format!("frame time:{}",frame_time.elapsed().as_millis()));
            ui.add(egui::Slider::new(&mut slider, 0.0..=50.0).text("Slider"));
            ui.label(" ");
            ui.add(Checkbox::new(&mut enable_vsync, "Enable vsync?"));
            ui.separator();
            if ui.button("Quit?").clicked() {
                quit = true;
            }
        });

        frame_time = Instant::now();

        let (egui_output, paint_cmds) = egui_ctx.end_frame();
        // Process ouput
        egui_state.process_output(&window, &egui_output);

        let paint_jobs = egui_ctx.tessellate(paint_cmds);

        {
            painter.paint_jobs(None, paint_jobs, &egui_ctx.font_image());
            
            for event in event_pump.poll_iter() {
                match event {
                    Event::MouseMotion { x, y , mousestate ,  ..} => {
                        sprite_pos.0 = x;
                        sprite_pos.1 = y;
                        egui_state.process_input(&window, event, &mut painter);
                    },
                    Event::Quit { .. } => break 'running,
                    _ => {
                        // Process input event
                        egui_state.process_input(&window, event, &mut painter);
                    }
                }
            }
        }

        // test triangle vbo

        shader_program.set_used();
        vao.bind();
        unsafe {
             gl::DrawArrays(
                 gl::TRIANGLES, // mode
                 0,             // starting index in the enabled arrays
                 3,             // number of indices to be rendered
             );
        }
        vao.unbind();

        // todo: this is all hacky and hardcoded, interface needs to be decided still
        i = i + 0.3;
        let tile = slider as i32;
        sprite_renderer.draw_quad(&map, sprite_pos.0 as f32, sprite_pos.1 as f32, 0.0, glm::vec3(1.0,1.0,1.0), 10.0, glm::vec4((16.0/map.width_f()) * tile as f32,0.0,(1.0/16.0),(16.0/map.height_f())));
        sprite_renderer.draw_subtexture(&first_tile, glm::vec2(200.0, 200.0));
        sprite_renderer.draw_quad(&texture, 650.0 as f32, 30 as f32, i, glm::vec3(1.0,1.0,1.0), 0.3, glm::vec4(i/100.0,0.0,1.0,1.0));

        window.gl_swap_window();
        if quit {
            break 'running;
        }
    }
}

