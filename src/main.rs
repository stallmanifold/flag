extern crate glfw;
extern crate gl;
extern crate tga;

mod file_util;
mod gl_util;
mod vec_util;
mod meshes;

use gl::types::*;


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
    matrix[ 4] = 0.0; matrix[ 5] = r_y;  matrix[ 6] = 0.0; matrix[ 7] = 0.0;
    matrix[ 8] = 0.0; matrix[ 9] = 0.0; matrix[10] = r_z;  matrix[11] = 1.0;
    matrix[12] = 0.0; matrix[13] = 0.0; matrix[14] = r_w;  matrix[15] = 0.0;
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

fn main() {
    println!("Hello, world!");
}
