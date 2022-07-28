use egui_backend::{egui, gl, sdl2};
use egui_backend::{sdl2::event::Event, DpiScaling, ShaderVersion};
use render::renderer::SpriteRenderer;
use render::subtexture::Subtexture;
use resources::Resources;
use sdl2::keyboard::Keycode;
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

const SCREEN_WIDTH: u32 = 1800;
const SCREEN_HEIGHT: u32 = 900;

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
        gl_attr.set_context_version(3, 0);
    }
    let mut i = 0.0;

    let window = video
        .window(
            "Blue",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .opengl()
        .allow_highdpi()
        .build()
        .unwrap();

    // Create a window context
    let _ctx = window.gl_create_context().unwrap();
    gl::load_with(|s| video.gl_get_proc_address(s) as _);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut quit = false;

    window.subsystem()
          .gl_set_swap_interval(SwapInterval::VSync)
          .unwrap();

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);
    let mut last_frame = Instant::now();
      
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let shader_program = render::GlProgram::from_res(&res, "shaders/triangle").expect("Failed to load triangle shader asset");

    let sprite_renderer = SpriteRenderer::from_res(&res, glm::vec2(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)).expect("error creating sprite renderer");

    let texture = res.load_texture("sprites/test.png").expect("error loading test.png to texture");
    let map = res.load_texture("tiles/grass.png").expect("error loading test.png to texture");

    let first_tile = Subtexture::from_tiles(&map, 9,6, glm::vec2(16.0,16.0));

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

    'running: loop {
        
        for event in event_pump.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            if imgui_sdl2.ignore_event(&event) { continue; }
    
            match event {
            Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                break 'running
            },
            _ => {}
            }
        }
  
        imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;
        imgui.io_mut().delta_time = delta_s;

        let ui = imgui.frame();
        ui.show_demo_window(&mut true);

        // test triangle vbo
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
          }
      

       
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

        sprite_renderer.draw_quad(&map, sprite_pos.0 as f32, sprite_pos.1 as f32, 0.0, glm::vec3(1.0,1.0,1.0), 10.0, glm::vec4((16.0/map.width_f()) * 10 as f32,0.0,(1.0/16.0),(16.0/map.height_f())));
        sprite_renderer.draw_subtexture(&first_tile, glm::vec2(200.0, 200.0));
        sprite_renderer.draw_quad(&texture, 650.0 as f32, 30 as f32, i, glm::vec3(1.0,1.0,1.0), 0.3, glm::vec4(i/100.0,0.0,1.0,1.0));

        sprite_renderer.draw_rect(glm::vec4(20.0,300.0,100.0,100.0), glm::vec3(0.4,0.3,0.7));

        imgui_sdl2.prepare_render(&ui, &window);
        renderer.render(ui);

        window.gl_swap_window();

        if quit {
            break 'running;
        }
    }
}
