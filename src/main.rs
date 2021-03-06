extern crate glfw;
extern crate gl;
extern crate tga;

mod file_util;
mod gl_util;
mod vec_util;
mod meshes;

use glfw::{Glfw, Action, Context, Key};
use gl::types::*;
use std::os::raw;
use std::mem;
use std::ptr;
use std::env;
use std::f32;
use std::ffi::CString;


struct Uniforms {
    texture: GLint,
    p_matrix: GLint,
    mv_matrix: GLint,
}

struct Attributes {
    position: GLint,
    normal: GLint,
    texcoord: GLint,
    shininess: GLint,
    specular: GLint,
}

struct FlagProgram {
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    program: GLuint,
    uniforms: Uniforms,
    attributes: Attributes,
}

struct GResources {
    flag: meshes::FlagMesh,
    background: meshes::FlagMesh,
    flag_vertex_array: Vec<meshes::FlagVertex>,
    flag_program: FlagProgram,
    p_matrix: [GLfloat; 16],
    mv_matrix: [GLfloat; 16],
    eye_offset: [GLfloat; 2],
    window_size: [GLfloat; 2],
}

impl GResources {
    fn new() -> GResources {
        GResources {
            flag: meshes::FlagMesh {
                vertex_buffer: 0,
                element_buffer: 0,
                element_count: 0,
                texture: 0,
            },
            background: meshes::FlagMesh {
                vertex_buffer: 0,
                element_buffer: 0,
                element_count: 0,
                texture: 0,
            },
            flag_vertex_array: vec![],
            flag_program: FlagProgram {
                vertex_shader: 0,
                fragment_shader: 0,
                program: 0,
                uniforms: Uniforms {
                    texture: 0,
                    p_matrix: 0,
                    mv_matrix: 0,
                },
                attributes: Attributes {
                    position: 0,
                    normal: 0,
                    texcoord: 0,
                    shininess: 0,
                    specular: 0,
                },
            },
            p_matrix: [0.0; 16],
            mv_matrix: [0.0; 16],
            eye_offset: [0.0; 2],
            window_size: [0.0; 2],
        }
    }

    fn cleanup(&self) {

    }
}


fn init_gl_state() {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }
}

const PROJECTION_FOV_RATIO: GLfloat = 0.7;
const PROJECTION_NEAR_PLANE: GLfloat = 0.0625;
const PROJECTION_FAR_PLANE: GLfloat = 256.0;

fn update_p_matrix(matrix: &mut [GLfloat], w: i32, h: i32) {
    let wf: GLfloat = w as GLfloat;
    let hf: GLfloat = h as GLfloat;
    let r_xy_factor: GLfloat = f32::min(wf, hf) * 1.0 / PROJECTION_FOV_RATIO;
    let r_x: GLfloat = r_xy_factor / wf;
    let r_y: GLfloat = r_xy_factor / hf;
    let r_zw_factor: GLfloat = 1.0 / (PROJECTION_FAR_PLANE - PROJECTION_NEAR_PLANE);
    let r_z: GLfloat = (PROJECTION_NEAR_PLANE + PROJECTION_FAR_PLANE) * r_zw_factor;
    let r_w: GLfloat = -2.0 * PROJECTION_NEAR_PLANE * PROJECTION_FAR_PLANE * r_zw_factor;

    matrix[ 0] = r_x; matrix[ 1] = 0.0; matrix[ 2] = 0.0; matrix[ 3] = 0.0;
    matrix[ 4] = 0.0; matrix[ 5] = r_y; matrix[ 6] = 0.0; matrix[ 7] = 0.0;
    matrix[ 8] = 0.0; matrix[ 9] = 0.0; matrix[10] = r_z; matrix[11] = 1.0;
    matrix[12] = 0.0; matrix[13] = 0.0; matrix[14] = r_w; matrix[15] = 0.0;
}

const BASE_EYE_POSITION: [GLfloat ; 3]  = [0.5, -0.25, -1.25];

fn update_mv_matrix(matrix: &mut [GLfloat], eye_offset: &[GLfloat]) {
    matrix[ 0] = 1.0; matrix[ 1] = 0.0; matrix[ 2] = 0.0; matrix[ 3] = 0.0;
    matrix[ 4] = 0.0; matrix[ 5] = 1.0; matrix[ 6] = 0.0; matrix[ 7] = 0.0;
    matrix[ 8] = 0.0; matrix[ 9] = 0.0; matrix[10] = 1.0; matrix[11] = 0.0;
    matrix[12] = -BASE_EYE_POSITION[0] - eye_offset[0];
    matrix[13] = -BASE_EYE_POSITION[1] - eye_offset[1];
    matrix[14] = -BASE_EYE_POSITION[2];
    matrix[15] = 1.0;
}

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(0 as *const $ty)).$field as *const _ as usize
    }
}

fn render_mesh(g_resources: &GResources, mesh: &meshes::FlagMesh) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, mesh.texture);

        gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vertex_buffer);
        gl::VertexAttribPointer(
            g_resources.flag_program.attributes.position as GLuint,
            3, gl::FLOAT, gl::FALSE, mem::size_of::<meshes::FlagVertex>() as GLint,
            offset_of!(meshes::FlagVertex, position) as *const raw::c_void
        );
        gl::VertexAttribPointer(
            g_resources.flag_program.attributes.normal as GLuint,
            3, gl::FLOAT, gl::FALSE, mem::size_of::<meshes::FlagVertex>() as GLint,
            offset_of!(meshes::FlagVertex, normal) as *const raw::c_void
        );
        gl::VertexAttribPointer(
            g_resources.flag_program.attributes.texcoord as GLuint,
            2, gl::FLOAT, gl::FALSE, mem::size_of::<meshes::FlagVertex>() as GLint,
            offset_of!(meshes::FlagVertex, texcoord) as *const raw::c_void
        );
        gl::VertexAttribPointer(
            g_resources.flag_program.attributes.shininess as GLuint,
            1, gl::FLOAT, gl::FALSE, mem::size_of::<meshes::FlagVertex>() as GLint,
            offset_of!(meshes::FlagVertex, shininess) as *const raw::c_void
        );
        gl::VertexAttribPointer(
            g_resources.flag_program.attributes.specular as GLuint,
            4, gl::UNSIGNED_BYTE, gl::TRUE, mem::size_of::<meshes::FlagVertex>() as GLint,
            offset_of!(meshes::FlagVertex, specular) as *const raw::c_void
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.element_buffer);
        gl::DrawElements(
            gl::TRIANGLES,
            mesh.element_count,
            gl::UNSIGNED_SHORT,
            ptr::null()
        );
    }
}

const INITIAL_WINDOW_WIDTH: u32 = 640;
const INITIAL_WINDOW_HEIGHT: u32 = 480;

fn enact_flag_program(
    g_resources: &mut GResources,
    vertex_shader: GLuint, fragment_shader: GLuint, program: GLuint
) {
    g_resources.flag_program.vertex_shader = vertex_shader;
    g_resources.flag_program.fragment_shader = fragment_shader;

    g_resources.flag_program.program = program;

    unsafe {
        let texture_cstr = CString::new("texture").unwrap();
        g_resources.flag_program.uniforms.texture
            = gl::GetUniformLocation(program, texture_cstr.as_ptr());
        let p_matrix_cstr = CString::new("p_matrix").unwrap();
        g_resources.flag_program.uniforms.p_matrix
            = gl::GetUniformLocation(program, p_matrix_cstr.as_ptr());
        let mv_matrix_cstr = CString::new("mv_matrix").unwrap();
        g_resources.flag_program.uniforms.mv_matrix
            = gl::GetUniformLocation(program, mv_matrix_cstr.as_ptr());
        let position_cstr = CString::new("position").unwrap();
        g_resources.flag_program.attributes.position
            = gl::GetAttribLocation(program, position_cstr.as_ptr());
        let normal_cstr = CString::new("normal").unwrap();
        g_resources.flag_program.attributes.normal
            = gl::GetAttribLocation(program, normal_cstr.as_ptr());
        let texcoord_cstr = CString::new("texcoord").unwrap();
        g_resources.flag_program.attributes.texcoord
            = gl::GetAttribLocation(program, texcoord_cstr.as_ptr());
        let shininess_cstr = CString::new("shininess").unwrap();
        g_resources.flag_program.attributes.shininess
            = gl::GetAttribLocation(program, shininess_cstr.as_ptr());
        let specular_cstr = CString::new("specular").unwrap();
        g_resources.flag_program.attributes.specular
            = gl::GetAttribLocation(program, specular_cstr.as_ptr());
        }
}

fn make_flag_program(
    vertex_shader: &mut GLuint, 
    fragment_shader: &mut GLuint, program: &mut GLuint) -> isize {

    *vertex_shader = gl_util::make_shader(gl::VERTEX_SHADER, "shaders/flag.v.glsl");
    if *vertex_shader == 0 {
        return 0;
    }

    *fragment_shader = gl_util::make_shader(gl::FRAGMENT_SHADER, "shaders/flag.f.glsl");
    if *fragment_shader == 0 {
        return 0;
    }

    *program = gl_util::make_program(*vertex_shader, *fragment_shader);
    if *program == 0 {
        return 0;
    }

    return 1;
}

fn delete_flag_program(g_resources: &GResources) {
    unsafe {
        gl::DetachShader(
            g_resources.flag_program.program,
            g_resources.flag_program.vertex_shader
        );
        gl::DetachShader(
            g_resources.flag_program.program,
            g_resources.flag_program.fragment_shader
        );
        gl::DeleteProgram(g_resources.flag_program.program);
        gl::DeleteShader(g_resources.flag_program.vertex_shader);
        gl::DeleteShader(g_resources.flag_program.fragment_shader);
    }
}

fn update_flag_program(g_resources: &mut GResources) {
    println!("reloading program\n");
    let mut vertex_shader: GLuint = 0; 
    let mut fragment_shader: GLuint = 0; 
    let mut program: GLuint = 0;

    if make_flag_program(&mut vertex_shader, &mut fragment_shader, &mut program) != 0 {
        delete_flag_program(g_resources);
        enact_flag_program(g_resources, vertex_shader, fragment_shader, program);
    }
}

fn make_resources() -> Option<GResources> {
    let mut vertex_shader: GLuint = 0;
    let mut fragment_shader: GLuint = 0;
    let mut program: GLuint = 0;

    let mut g_resources: GResources = GResources::new();

    // Load meshes.
    g_resources.flag_vertex_array = meshes::init_flag_mesh(&mut g_resources.flag);
    meshes::init_background_mesh(&mut g_resources.background);

    // Create textures.
    g_resources.flag.texture = gl_util::make_texture("assets/flag.tga");
    g_resources.background.texture = gl_util::make_texture("assets/background.tga");

    if g_resources.flag.texture == 0 || g_resources.background.texture == 0 {
        return None;
    }

    if make_flag_program(&mut vertex_shader, &mut fragment_shader, &mut program) == 0 {
        return None;
    }

    enact_flag_program(&mut g_resources, vertex_shader, fragment_shader, program);

    g_resources.eye_offset[0] = 0.0;
    g_resources.eye_offset[1] = 0.0;
    g_resources.window_size[0] = INITIAL_WINDOW_WIDTH as GLfloat;
    g_resources.window_size[1] = INITIAL_WINDOW_HEIGHT as GLfloat;

    update_p_matrix(
        &mut g_resources.p_matrix,
        INITIAL_WINDOW_WIDTH as GLint,
        INITIAL_WINDOW_HEIGHT as GLint
    );
    update_mv_matrix(&mut g_resources.mv_matrix, &g_resources.eye_offset);

    return Some(g_resources);
}

fn update(g_resources: &mut GResources, glfw: &mut Glfw, window: &mut glfw::Window) {
    let seconds = glfw.get_time() as GLfloat;
    meshes::update_flag_mesh(&g_resources.flag, &mut g_resources.flag_vertex_array, seconds);

    // Poll events.
    glfw.poll_events();
}

fn drag(g_resources: &mut GResources, x: i32, y: i32) {
    let w: f32 = g_resources.window_size[0] as f32;
    let h: f32 = g_resources.window_size[1] as f32;
    g_resources.eye_offset[0] = (x as f32) / w - 0.5;
    g_resources.eye_offset[1] = -(y as f32) / h + 0.5;
    update_mv_matrix(&mut g_resources.mv_matrix, &g_resources.eye_offset);
}

fn mouse(g_resources: &mut GResources, button: glfw::MouseButton, state: i32, x: i32, y: i32) {
    if button == glfw::MouseButton::Button1 && state == 1 /* && (state == GLUT_UP) */ {
        g_resources.eye_offset[0] = 0.0;
        g_resources.eye_offset[1] = 0.0;
        update_mv_matrix(&mut g_resources.mv_matrix, &g_resources.eye_offset);
    }
}

fn keyboard(g_resources: &mut GResources, key: Key, x: i32, y: i32) {
    if key == Key::R {
        update_flag_program(g_resources);
    }
}

fn reshape(g_resources: &mut GResources, w: i32, h: i32) {
    g_resources.window_size[0] = w as f32;
    g_resources.window_size[1] = h as f32;
    update_p_matrix(&mut g_resources.p_matrix, w, h);
    unsafe {
        gl::Viewport(0, 0, w, h);
    }
}

fn render(g_resources: &mut GResources, window: &mut glfw::Window) {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        gl::UseProgram(g_resources.flag_program.program);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::Uniform1i(g_resources.flag_program.uniforms.texture, 0);

        gl::UniformMatrix4fv(
            g_resources.flag_program.uniforms.p_matrix,
            1, gl::FALSE,
            g_resources.p_matrix.as_ptr()
        );

        gl::UniformMatrix4fv(
            g_resources.flag_program.uniforms.mv_matrix,
            1, gl::FALSE,
            g_resources.mv_matrix.as_ptr()
        );

        gl::EnableVertexAttribArray(g_resources.flag_program.attributes.position as GLuint);
        gl::EnableVertexAttribArray(g_resources.flag_program.attributes.normal as GLuint);
        gl::EnableVertexAttribArray(g_resources.flag_program.attributes.texcoord as GLuint);
        gl::EnableVertexAttribArray(g_resources.flag_program.attributes.shininess as GLuint);
        gl::EnableVertexAttribArray(g_resources.flag_program.attributes.specular as GLuint);

        render_mesh(g_resources, &g_resources.flag);
        render_mesh(g_resources, &g_resources.background);

        gl::DisableVertexAttribArray(g_resources.flag_program.attributes.position as GLuint);
        gl::DisableVertexAttribArray(g_resources.flag_program.attributes.normal as GLuint);
        gl::DisableVertexAttribArray(g_resources.flag_program.attributes.texcoord as GLuint);
        gl::DisableVertexAttribArray(g_resources.flag_program.attributes.shininess as GLuint);
        gl::DisableVertexAttribArray(g_resources.flag_program.attributes.specular as GLuint);
    }
    window.swap_buffers();
}

fn handle_window_event(g_resources: &mut GResources, window: &mut glfw::Window, (time, event): (f64, glfw::WindowEvent)) {
    println!("TIME: {:?}; EVENT: {:?}", time, event);
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::Key(key,_,_,_) => {
            keyboard(g_resources, key, 0, 0);
        },
        glfw::WindowEvent::MouseButton(button, Action::Press, _) => {
            mouse(g_resources, button, 1, 0, 0);
        },
        glfw::WindowEvent::Size(w, h) => {
            reshape(g_resources, w, h);
        }
        _ => {

        }

    }
}

fn main() {
    // Initialize our resources.
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT, "Flag", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current.
    window.make_current();
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_size_polling(true);
    window.set_refresh_polling(true);

    // Load the OpenGl function pointers.
    gl::load_with(|symbol| { window.get_proc_address(symbol) as *const _ });

    // Initialize GL.
    init_gl_state();

    let mut g_resources = make_resources().expect("Failed to load resources.");

    // Loop until the user closes the window
    while !window.should_close() {
        update(&mut g_resources, &mut glfw, &mut window);
        render(&mut g_resources, &mut window);

        for (time, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut g_resources, &mut window, (time, event));
        }
    }

    g_resources.cleanup();
}
