extern crate glfw;
extern crate gl;
extern crate tga;

mod file_util;
mod gl_util;
mod vec_util;
mod meshes;

use gl::types::*;
use std::os::raw;
use std::mem;
use std::ptr;


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
/*
static mut G_RESOURCES: GResources = GResources {
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
};
*/
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

fn render_mesh(g_resources: &mut GResources, mesh: &meshes::FlagMesh) {
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

const INITIAL_WINDOW_WIDTH: usize = 640;
const INITIAL_WINDOW_HEIGHT: usize = 480;

fn enact_flag_program(
    g_resources: &mut GResources,
    vertex_shader: GLuint, fragment_shader: GLuint, program: GLuint
) {
    g_resources.flag_program.vertex_shader = vertex_shader;
    g_resources.flag_program.fragment_shader = fragment_shader;

    g_resources.flag_program.program = program;

    unsafe {
        //let texture_cstr = CString::new("texture").unwrap();
        g_resources.flag_program.uniforms.texture
            = gl::GetUniformLocation(program, "texture".as_ptr() as *const i8);
        //let p_matrix_cstr = CString::new("p_matrix").unwrap();
        g_resources.flag_program.uniforms.p_matrix
            = gl::GetUniformLocation(program, "p_matrix".as_ptr() as *const i8);
        //let mv_matrix_cstr = CString::new("mv_matrix").unwrap();
        g_resources.flag_program.uniforms.mv_matrix
            = gl::GetUniformLocation(program, "mv_matrix".as_ptr() as *const i8);
        //let position_cstr = CString::new("position").unwrap();
        g_resources.flag_program.attributes.position
            = gl::GetAttribLocation(program, "position".as_ptr() as *const i8);
        //let normal_cstr = CString::new("normal").unwrap();
        g_resources.flag_program.attributes.normal
            = gl::GetAttribLocation(program, "normal".as_ptr() as *const i8);
        //let texcoord_cstr = CString::new("texcoord").unwrap();
        g_resources.flag_program.attributes.texcoord
            = gl::GetAttribLocation(program, "texcoord".as_ptr() as *const i8);
        //let shininess_cstr = CString::new("shininess").unwrap();
        g_resources.flag_program.attributes.shininess
            = gl::GetAttribLocation(program, "shininess".as_ptr() as *const i8);
        //let specular_cstr = CString::new("specular").unwrap();
        g_resources.flag_program.attributes.specular
            = gl::GetAttribLocation(program, "specular".as_ptr() as *const i8);
        }
}

fn main() {
    println!("Hello, world!");
}
