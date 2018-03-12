use gl;
use gl::types::*;
use std::mem;
use vec_util;
use std::f32;

const FLAG_X_RES: GLushort = 100;
const FLAG_Y_RES: GLushort = 75;
const FLAG_VERTEX_COUNT: GLushort = FLAG_X_RES * FLAG_Y_RES;
const FLAG_S_STEP: GLfloat = 1.0 / ((FLAG_X_RES - 1) as GLfloat);
const FLAG_T_STEP: GLfloat = 1.0 / ((FLAG_Y_RES - 1) as GLfloat);


struct FlagMesh {
    vertex_buffer: GLuint, 
    element_buffer: GLuint,
    element_count: GLsizei,
    texture: GLuint,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct FlagVertex {
    position: [GLfloat; 4],
    normal: [GLfloat; 4],
    texcoord: [GLfloat; 2],
    shininess: GLfloat,
    specular: [GLubyte; 4],
}

impl FlagVertex {
    fn zero() -> FlagVertex {
        FlagVertex {
            position: [0.0; 4],
            normal: [0.0; 4],
            texcoord: [0.0; 2],
            shininess: 0.0,
            specular: [0; 4],
        }
    }
}

fn init_mesh(
    out_mesh: &mut FlagMesh, 
    vertex_data: &[FlagVertex], vertex_count: GLsizei,
    element_data: &[GLushort], element_count: GLsizei, 
    hint: GLenum
) {
    unsafe {
        gl::GenBuffers(1, &mut out_mesh.vertex_buffer);
        gl::GenBuffers(1, &mut out_mesh.element_buffer);
        out_mesh.element_count = element_count;

        gl::BindBuffer(gl::ARRAY_BUFFER, out_mesh.vertex_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertex_count * (mem::size_of::<FlagVertex>() as GLsizei)) as  GLsizeiptr,
            mem::transmute(&vertex_data[0]),
            hint
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, out_mesh.element_buffer);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (element_count * mem::size_of::<GLushort>() as GLsizei) as  GLsizeiptr,
            mem::transmute(&element_data[0]),
            gl::STATIC_DRAW
        );
    }
}

fn init_flag_mesh(out_mesh: &mut FlagMesh) -> Vec<FlagVertex> {
    let mut vertex_data = vec![FlagVertex::zero(); FLAG_VERTEX_COUNT as usize];
    let element_count = 6 * (FLAG_X_RES - 1) * (FLAG_Y_RES - 1);
    let mut element_data = vec![0 as GLushort; element_count as usize];

    let mut i = 0;
    for t in 0..FLAG_Y_RES {
        for s in 0..FLAG_X_RES {
            let ss: GLfloat = FLAG_S_STEP * (s as GLfloat);
            let tt: GLfloat = FLAG_T_STEP * (t as GLfloat);

            calculate_flag_vertex(&mut vertex_data[i], ss, tt, 0.0);

            vertex_data[i].texcoord[0] = ss;
            vertex_data[i].texcoord[1] = tt;
            vertex_data[i].shininess   = 0.0;
            vertex_data[i].specular[0] = 0;
            vertex_data[i].specular[1] = 0;
            vertex_data[i].specular[2] = 0;
            vertex_data[i].specular[3] = 0;

            i += 1;
        }
    }

    i = 0;
    let mut index = 0;
    for t in 0..FLAG_Y_RES {
        for s in 0..(FLAG_X_RES - 1) {
            i += 1;
            element_data[i] = index;
            i += 1;
            element_data[i] = index + 1;
            i += 1;
            element_data[i] = index + FLAG_X_RES;
            i += 1;
            element_data[i] = index + 1;
            i += 1;
            element_data[i] = index + FLAG_X_RES + 1;
            i += 1;
            element_data[i] = index + FLAG_X_RES;

            index += 1;
        }

        index += 1;
    }

    init_mesh(
        out_mesh,
        &vertex_data, FLAG_VERTEX_COUNT as GLsizei,
        &element_data, element_count as GLsizei,
        gl::STREAM_DRAW
    );

    vertex_data
}

fn calculate_flag_vertex(
    v: &mut FlagVertex,
    s: GLfloat, t: GLfloat, time: GLfloat
) {
    let sgrad: [GLfloat; 3] = [
        1.0 + 0.5*(0.0625 + 0.03125 * f32::sin(vec_util::M_PI * time)) * t * (t - 1.0),
        0.0,
        0.125*(
            f32::sin(1.5*vec_util::M_PI * (time + s)) 
            + s * f32::cos(1.5 * vec_util::M_PI * (time + s)) * (1.5 * vec_util::M_PI)
        )
    ];
    let tgrad: [GLfloat; 3] = [
        -(0.0625 + 0.03125 * f32::sin(vec_util::M_PI * time)) * (1.0 - s) * (2.0 * t - 1.0),
        0.75,
        0.0
    ];

    v.position[0] = s - (0.0625 + 0.03125 * f32::sin(vec_util::M_PI * time)) * (1.0 - 0.5 * s) * t * (t - 1.0);
    v.position[1] = 0.75 * t - 0.375;
    v.position[2] = 0.125 * (s * f32::sin(1.5* vec_util::M_PI*(time + s)));
    v.position[3] = 0.0;

    vec_util::vec_cross(&mut v.normal, &tgrad, &sgrad);
    vec_util::vec_normalize(&mut v.normal);
    v.normal[3] = 0.0;
}
/*
void init_background_mesh(struct flag_mesh *out_mesh);
void update_flag_mesh(
    struct flag_mesh const *mesh,
    struct flag_vertex *vertex_data,
    GLfloat time
);
*/