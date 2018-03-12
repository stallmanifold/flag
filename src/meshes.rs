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

const FLAGPOLE_TRUCK_TOP: GLfloat           = 0.5;
const FLAGPOLE_TRUCK_CROWN: GLfloat         = 0.41;
const FLAGPOLE_TRUCK_BOTTOM: GLfloat        = 0.38;
const FLAGPOLE_SHAFT_TOP: GLfloat           = 0.3775;
const FLAGPOLE_SHAFT_BOTTOM: GLfloat        = -1.0;
const FLAGPOLE_TRUCK_TOP_RADIUS: GLfloat    = 0.005;
const FLAGPOLE_TRUCK_CROWN_RADIUS: GLfloat  = 0.020;
const FLAGPOLE_TRUCK_BOTTOM_RADIUS: GLfloat = 0.015;
const FLAGPOLE_SHAFT_RADIUS: GLfloat        = 0.010;
const FLAGPOLE_SHININESS: GLfloat           = 4.0;

fn init_background_mesh(out_mesh: &mut FlagMesh) {
    const FLAGPOLE_RES: GLsizei = 16;
    const FLAGPOLE_SLICE: GLsizei = 6;

    let FLAGPOLE_AXIS_XZ: [GLfloat; 2] = [-FLAGPOLE_SHAFT_RADIUS, 0.0];
    
    const FLAGPOLE_SPECULAR: [GLubyte; 4] = [255, 255, 192, 0];

    let GROUND_LO: [GLfloat; 3] = [-0.875, FLAGPOLE_SHAFT_BOTTOM, -2.45];
    let GROUND_HI: [GLfloat; 3] = [1.875, FLAGPOLE_SHAFT_BOTTOM,  0.20];
    let WALL_LO: [GLfloat; 3] = [GROUND_LO[0], FLAGPOLE_SHAFT_BOTTOM, GROUND_HI[2]];
    let WALL_HI: [GLfloat; 3] = [GROUND_HI[0], FLAGPOLE_SHAFT_BOTTOM + 3.0, GROUND_HI[2]];

    static TEX_FLAGPOLE_LO: [GLfloat; 2] = [ 0.0,    0.0 ];
    static TEX_FLAGPOLE_HI: [GLfloat; 2] = [ 0.03125,  1.0 ];
    static TEX_GROUND_LO: [GLfloat; 2]   = [ 0.03125,  0.0078125 ];
    static TEX_GROUND_HI: [GLfloat; 2]   = [ 0.515625, 0.9921875 ];
    static TEX_WALL_LO: [GLfloat; 2]     = [ 0.515625, 0.0078125 ];
    static TEX_WALL_HI: [GLfloat; 2]     = [ 1.0,      0.9921875 ];

    macro_rules! __flagpole_t {
        ($x:ident) => {
            TEX_FLAGPOLE_LO[1] 
                + (TEX_FLAGPOLE_HI[1] - TEX_FLAGPOLE_LO[1])
                * ($x - FLAGPOLE_TRUCK_TOP) / (FLAGPOLE_SHAFT_BOTTOM - FLAGPOLE_TRUCK_TOP)
        };
    }

    
    let theta_step: GLfloat = 2.0 * vec_util::M_PI / (FLAGPOLE_RES as GLfloat);
    let s_step: GLfloat = (TEX_FLAGPOLE_HI[0] - TEX_FLAGPOLE_LO[0]) / (FLAGPOLE_RES as GLfloat);
    let t_truck_top: GLfloat    = TEX_FLAGPOLE_LO[1];
    let t_truck_crown: GLfloat  = __flagpole_t!(FLAGPOLE_TRUCK_CROWN);
    let t_truck_bottom: GLfloat = __flagpole_t!(FLAGPOLE_TRUCK_BOTTOM);
    let t_shaft_top: GLfloat    = __flagpole_t!(FLAGPOLE_SHAFT_TOP);
    let t_shaft_bottom: GLfloat = __flagpole_t!(FLAGPOLE_SHAFT_BOTTOM);

    let flagpole_vertex_count: GLsizei = 2 + FLAGPOLE_RES * FLAGPOLE_SLICE;
    let wall_vertex_count: GLsizei = 4;
    let ground_vertex_count: GLsizei = 4;
    let vertex_count: GLsizei = flagpole_vertex_count + wall_vertex_count + ground_vertex_count;

    let mut element_i = 0;

    let flagpole_element_count: GLsizei = 3 * ((FLAGPOLE_SLICE - 1) * 2 * FLAGPOLE_RES);
    let wall_element_count: GLsizei = 6;
    let ground_element_count: GLsizei = 6;
    let element_count: GLsizei = flagpole_element_count + wall_element_count + ground_element_count;

    let mut vertex_data = vec![FlagVertex::zero(); vertex_count as usize]; 

    let mut element_data = vec![0 as GLushort; element_count as usize];

    vertex_data[0].position[0] = GROUND_LO[0];
    vertex_data[0].position[1] = GROUND_LO[1];
    vertex_data[0].position[2] = GROUND_LO[2];
    vertex_data[0].position[3] = 1.0;
    vertex_data[0].normal[0]   = 0.0;
    vertex_data[0].normal[1]   = 1.0;
    vertex_data[0].normal[2]   = 0.0;
    vertex_data[0].normal[3]   = 0.0;
    vertex_data[0].texcoord[0] = TEX_GROUND_LO[0];
    vertex_data[0].texcoord[1] = TEX_GROUND_LO[1];
    vertex_data[0].shininess   = 0.0;
    vertex_data[0].specular[0] = 0;
    vertex_data[0].specular[1] = 0;
    vertex_data[0].specular[2] = 0;
    vertex_data[0].specular[3] = 0;

    vertex_data[1].position[0] = GROUND_HI[0];
    vertex_data[1].position[1] = GROUND_LO[1];
    vertex_data[1].position[2] = GROUND_LO[2];
    vertex_data[1].position[3] = 1.0;
    vertex_data[1].normal[0]   = 0.0;
    vertex_data[1].normal[1]   = 1.0;
    vertex_data[1].normal[2]   = 0.0;
    vertex_data[1].normal[3]   = 0.0;
    vertex_data[1].texcoord[0] = TEX_GROUND_HI[0];
    vertex_data[1].texcoord[1] = TEX_GROUND_LO[1];
    vertex_data[1].shininess   = 0.0;
    vertex_data[1].specular[0] = 0;
    vertex_data[1].specular[1] = 0;
    vertex_data[1].specular[2] = 0;
    vertex_data[1].specular[3] = 0;

    vertex_data[2].position[0] = GROUND_HI[0];
    vertex_data[2].position[1] = GROUND_LO[1];
    vertex_data[2].position[2] = GROUND_HI[2];
    vertex_data[2].position[3] = 1.0;
    vertex_data[2].normal[0]   = 0.0;
    vertex_data[2].normal[1]   = 1.0;
    vertex_data[2].normal[2]   = 0.0;
    vertex_data[2].normal[3]   = 0.0;
    vertex_data[2].texcoord[0] = TEX_GROUND_HI[0];
    vertex_data[2].texcoord[1] = TEX_GROUND_HI[1];
    vertex_data[2].shininess   = 0.0;
    vertex_data[2].specular[0] = 0;
    vertex_data[2].specular[1] = 0;
    vertex_data[2].specular[2] = 0;
    vertex_data[2].specular[3] = 0;

    vertex_data[3].position[0] = GROUND_LO[0];
    vertex_data[3].position[1] = GROUND_LO[1];
    vertex_data[3].position[2] = GROUND_HI[2];
    vertex_data[3].position[3] = 1.0;
    vertex_data[3].normal[0]   = 0.0;
    vertex_data[3].normal[1]   = 1.0;
    vertex_data[3].normal[2]   = 0.0;
    vertex_data[3].normal[3]   = 0.0;
    vertex_data[3].texcoord[0] = TEX_GROUND_LO[0];
    vertex_data[3].texcoord[1] = TEX_GROUND_HI[1];
    vertex_data[3].shininess   = 0.0;
    vertex_data[3].specular[0] = 0;
    vertex_data[3].specular[1] = 0;
    vertex_data[3].specular[2] = 0;
    vertex_data[3].specular[3] = 0;

    vertex_data[4].position[0] = WALL_LO[0];
    vertex_data[4].position[1] = WALL_LO[1];
    vertex_data[4].position[2] = WALL_LO[2];
    vertex_data[4].position[3] = 1.0;
    vertex_data[4].normal[0]   = 0.0;
    vertex_data[4].normal[1]   = 0.0;
    vertex_data[4].normal[2]   = -1.0;
    vertex_data[4].normal[3]   = 0.0;
    vertex_data[4].texcoord[0] = TEX_WALL_LO[0];
    vertex_data[4].texcoord[1] = TEX_WALL_LO[1];
    vertex_data[4].shininess   = 0.0;
    vertex_data[4].specular[0] = 0;
    vertex_data[4].specular[1] = 0;
    vertex_data[4].specular[2] = 0;
    vertex_data[4].specular[3] = 0;

    vertex_data[5].position[0] = WALL_HI[0];
    vertex_data[5].position[1] = WALL_LO[1];
    vertex_data[5].position[2] = WALL_LO[2];
    vertex_data[5].position[3] = 1.0;
    vertex_data[5].normal[0]   = 0.0;
    vertex_data[5].normal[1]   = 0.0;
    vertex_data[5].normal[2]   = -1.0;
    vertex_data[5].normal[3]   = 0.0;
    vertex_data[5].texcoord[0] = TEX_WALL_HI[0];
    vertex_data[5].texcoord[1] = TEX_WALL_LO[1];
    vertex_data[5].shininess   = 0.0;
    vertex_data[5].specular[0] = 0;
    vertex_data[5].specular[1] = 0;
    vertex_data[5].specular[2] = 0;
    vertex_data[5].specular[3] = 0;

    vertex_data[6].position[0] = WALL_HI[0];
    vertex_data[6].position[1] = WALL_HI[1];
    vertex_data[6].position[2] = WALL_LO[2];
    vertex_data[6].position[3] = 1.0;
    vertex_data[6].normal[0]   = 0.0;
    vertex_data[6].normal[1]   = 0.0;
    vertex_data[6].normal[2]   = -1.0;
    vertex_data[6].normal[3]   = 0.0;
    vertex_data[6].texcoord[0] = TEX_WALL_HI[0];
    vertex_data[6].texcoord[1] = TEX_WALL_HI[1];
    vertex_data[6].shininess   = 0.0;
    vertex_data[6].specular[0] = 0;
    vertex_data[6].specular[1] = 0;
    vertex_data[6].specular[2] = 0;
    vertex_data[6].specular[3] = 0;

    vertex_data[7].position[0] = WALL_LO[0];
    vertex_data[7].position[1] = WALL_HI[1];
    vertex_data[7].position[2] = WALL_LO[2];
    vertex_data[7].position[3] = 1.0;
    vertex_data[7].normal[0]   = 0.0;
    vertex_data[7].normal[1]   = 0.0;
    vertex_data[7].normal[2]   = -1.0;
    vertex_data[7].normal[3]   = 0.0;
    vertex_data[7].texcoord[0] = TEX_WALL_LO[0];
    vertex_data[7].texcoord[1] = TEX_WALL_HI[1];
    vertex_data[7].shininess   = 0.0;
    vertex_data[7].specular[0] = 0;
    vertex_data[7].specular[1] = 0;
    vertex_data[7].specular[2] = 0;
    vertex_data[7].specular[3] = 0;

    vertex_data[8].position[0] = FLAGPOLE_AXIS_XZ[0];
    vertex_data[8].position[1] = FLAGPOLE_TRUCK_TOP;
    vertex_data[8].position[2] = FLAGPOLE_AXIS_XZ[1];
    vertex_data[8].position[3] = 1.0;
    vertex_data[8].normal[0]   = 0.0;
    vertex_data[8].normal[1]   = 1.0;
    vertex_data[8].normal[2]   = 0.0;
    vertex_data[8].normal[3]   = 0.0;
    vertex_data[8].texcoord[0] = TEX_FLAGPOLE_LO[0];
    vertex_data[8].texcoord[1] = t_truck_top;
    vertex_data[8].shininess   = FLAGPOLE_SHININESS;
    vertex_data[8].specular[0] = 0;
    vertex_data[8].specular[1] = 0;
    vertex_data[8].specular[2] = 0;
    vertex_data[8].specular[3] = 0;

    let mut vertex_i = 9;
    for i in 0..FLAGPOLE_RES {
        let sn: f32 = f32::sin(theta_step * (i as f32));
        let cs: f32 = f32::cos(theta_step * (i as f32));
        let s: f32 = TEX_FLAGPOLE_LO[0] + s_step * (i as f32);

        vertex_data[vertex_i].position[0]
            = FLAGPOLE_AXIS_XZ[0] + FLAGPOLE_TRUCK_TOP_RADIUS * cs;
        vertex_data[vertex_i].position[1] = FLAGPOLE_TRUCK_TOP;
        vertex_data[vertex_i].position[2]
            = FLAGPOLE_AXIS_XZ[1] + FLAGPOLE_TRUCK_TOP_RADIUS * sn;
        vertex_data[vertex_i].position[3] = 1.0;
        vertex_data[vertex_i].normal[0]   = cs * 0.5;
        vertex_data[vertex_i].normal[1]   = f32::sqrt(3.0/4.0);
        vertex_data[vertex_i].normal[2]   = sn * 0.5;
        vertex_data[vertex_i].normal[3]   = 0.0;
        vertex_data[vertex_i].texcoord[0] = s;
        vertex_data[vertex_i].texcoord[1] = t_truck_top;
        vertex_data[vertex_i].shininess   = FLAGPOLE_SHININESS;
        vertex_data[vertex_i].specular[0] = FLAGPOLE_SPECULAR[0];
        vertex_data[vertex_i].specular[1] = FLAGPOLE_SPECULAR[1];
        vertex_data[vertex_i].specular[2] = FLAGPOLE_SPECULAR[2];
        vertex_data[vertex_i].specular[3] = FLAGPOLE_SPECULAR[3];
        vertex_i += 1;

        vertex_data[vertex_i].position[0]
            = FLAGPOLE_AXIS_XZ[0] + FLAGPOLE_TRUCK_CROWN_RADIUS * cs;
        vertex_data[vertex_i].position[1] = FLAGPOLE_TRUCK_CROWN;
        vertex_data[vertex_i].position[2]
            = FLAGPOLE_AXIS_XZ[1] + FLAGPOLE_TRUCK_CROWN_RADIUS * sn;
        vertex_data[vertex_i].position[3] = 1.0;
        vertex_data[vertex_i].normal[0]   = cs;
        vertex_data[vertex_i].normal[1]   = 0.0;
        vertex_data[vertex_i].normal[2]   = sn;
        vertex_data[vertex_i].normal[3]   = 0.0;
        vertex_data[vertex_i].texcoord[0] = s;
        vertex_data[vertex_i].texcoord[1] = t_truck_crown;
        vertex_data[vertex_i].shininess   = FLAGPOLE_SHININESS;
        vertex_data[vertex_i].specular[0] = FLAGPOLE_SPECULAR[0];
        vertex_data[vertex_i].specular[1] = FLAGPOLE_SPECULAR[1];
        vertex_data[vertex_i].specular[2] = FLAGPOLE_SPECULAR[2];
        vertex_data[vertex_i].specular[3] = FLAGPOLE_SPECULAR[3];
        vertex_i += 1;

        vertex_data[vertex_i].position[0]
            = FLAGPOLE_AXIS_XZ[0] + FLAGPOLE_TRUCK_BOTTOM_RADIUS * cs;
        vertex_data[vertex_i].position[1] = FLAGPOLE_TRUCK_BOTTOM;
        vertex_data[vertex_i].position[2]
            = FLAGPOLE_AXIS_XZ[1] + FLAGPOLE_TRUCK_BOTTOM_RADIUS * sn;
        vertex_data[vertex_i].position[3] = 1.0;
        vertex_data[vertex_i].normal[0]   = cs * f32::sqrt(15.0/16.0);
        vertex_data[vertex_i].normal[1]   = -0.25;
        vertex_data[vertex_i].normal[2]   = sn * f32::sqrt(15.0/16.0);
        vertex_data[vertex_i].normal[3]   = 0.0;
        vertex_data[vertex_i].texcoord[0] = s;
        vertex_data[vertex_i].texcoord[1] = t_truck_bottom;
        vertex_data[vertex_i].shininess   = FLAGPOLE_SHININESS;
        vertex_data[vertex_i].specular[0] = FLAGPOLE_SPECULAR[0];
        vertex_data[vertex_i].specular[1] = FLAGPOLE_SPECULAR[1];
        vertex_data[vertex_i].specular[2] = FLAGPOLE_SPECULAR[2];
        vertex_data[vertex_i].specular[3] = FLAGPOLE_SPECULAR[3];
        vertex_i += 1;

        vertex_data[vertex_i].position[0]
            = FLAGPOLE_AXIS_XZ[0] + FLAGPOLE_SHAFT_RADIUS * cs;
        vertex_data[vertex_i].position[1] = FLAGPOLE_SHAFT_TOP;
        vertex_data[vertex_i].position[2]
            = FLAGPOLE_AXIS_XZ[1] + FLAGPOLE_SHAFT_RADIUS * sn;
        vertex_data[vertex_i].position[3] = 1.0;
        vertex_data[vertex_i].normal[0]   = cs;
        vertex_data[vertex_i].normal[1]   = 0.0;
        vertex_data[vertex_i].normal[2]   = sn;
        vertex_data[vertex_i].normal[3]   = 0.0;
        vertex_data[vertex_i].texcoord[0] = s;
        vertex_data[vertex_i].texcoord[1] = t_shaft_top;
        vertex_data[vertex_i].shininess   = FLAGPOLE_SHININESS;
        vertex_data[vertex_i].specular[0] = FLAGPOLE_SPECULAR[0];
        vertex_data[vertex_i].specular[1] = FLAGPOLE_SPECULAR[1];
        vertex_data[vertex_i].specular[2] = FLAGPOLE_SPECULAR[2];
        vertex_data[vertex_i].specular[3] = FLAGPOLE_SPECULAR[3];
        vertex_i += 1;

        vertex_data[vertex_i].position[0]
            = FLAGPOLE_AXIS_XZ[0] + FLAGPOLE_SHAFT_RADIUS * cs;
        vertex_data[vertex_i].position[1] = FLAGPOLE_SHAFT_BOTTOM;
        vertex_data[vertex_i].position[2]
            = FLAGPOLE_AXIS_XZ[1] + FLAGPOLE_TRUCK_BOTTOM_RADIUS * sn;
        vertex_data[vertex_i].position[3] = 1.0;
        vertex_data[vertex_i].normal[0]   = cs;
        vertex_data[vertex_i].normal[1]   = 0.0;
        vertex_data[vertex_i].normal[2]   = sn;
        vertex_data[vertex_i].normal[3]   = 0.0;
        vertex_data[vertex_i].texcoord[0] = s;
        vertex_data[vertex_i].texcoord[1] = t_shaft_bottom;
        vertex_data[vertex_i].shininess   = FLAGPOLE_SHININESS;
        vertex_data[vertex_i].specular[0] = FLAGPOLE_SPECULAR[0];
        vertex_data[vertex_i].specular[1] = FLAGPOLE_SPECULAR[1];
        vertex_data[vertex_i].specular[2] = FLAGPOLE_SPECULAR[2];
        vertex_data[vertex_i].specular[3] = FLAGPOLE_SPECULAR[3];
        vertex_i += 1;

        vertex_data[vertex_i].position[0]
            = FLAGPOLE_AXIS_XZ[0] + FLAGPOLE_SHAFT_RADIUS * cs;
        vertex_data[vertex_i].position[1] = FLAGPOLE_SHAFT_BOTTOM;
        vertex_data[vertex_i].position[2]
            = FLAGPOLE_AXIS_XZ[1] + FLAGPOLE_TRUCK_BOTTOM_RADIUS * sn;
        vertex_data[vertex_i].position[3] =  1.0;
        vertex_data[vertex_i].normal[0]   =  0.0;
        vertex_data[vertex_i].normal[1]   = -1.0;
        vertex_data[vertex_i].normal[2]   =  0.0;
        vertex_data[vertex_i].normal[3]   =  0.0;
        vertex_data[vertex_i].texcoord[0] =  s;
        vertex_data[vertex_i].texcoord[1] =  t_shaft_bottom;
        vertex_data[vertex_i].shininess   =  FLAGPOLE_SHININESS;
        vertex_data[vertex_i].specular[0] = FLAGPOLE_SPECULAR[0];
        vertex_data[vertex_i].specular[1] = FLAGPOLE_SPECULAR[1];
        vertex_data[vertex_i].specular[2] = FLAGPOLE_SPECULAR[2];
        vertex_data[vertex_i].specular[3] = FLAGPOLE_SPECULAR[3];
        vertex_i += 1;
    }

    vertex_data[vertex_i].position[0] =  0.0;
    vertex_data[vertex_i].position[1] =  FLAGPOLE_SHAFT_BOTTOM;
    vertex_data[vertex_i].position[2] =  0.0;
    vertex_data[vertex_i].position[3] =  1.0;
    vertex_data[vertex_i].normal[0]   =  0.0;
    vertex_data[vertex_i].normal[1]   = -1.0;
    vertex_data[vertex_i].normal[2]   =  0.0;
    vertex_data[vertex_i].normal[3]   =  0.0;
    vertex_data[vertex_i].texcoord[0] =  0.5;
    vertex_data[vertex_i].texcoord[1] =  t_shaft_bottom;
    vertex_data[vertex_i].shininess   =  FLAGPOLE_SHININESS;
    vertex_data[vertex_i].specular[0] = FLAGPOLE_SPECULAR[0];
    vertex_data[vertex_i].specular[1] = FLAGPOLE_SPECULAR[1];
    vertex_data[vertex_i].specular[2] = FLAGPOLE_SPECULAR[2];
    vertex_data[vertex_i].specular[3] = FLAGPOLE_SPECULAR[3];

    element_i = 0;

    element_data[element_i] = 0;
    element_i += 1;
    element_data[element_i] = 1;
    element_i += 1;
    element_data[element_i] = 2;
    element_i += 1;

    element_data[element_i] = 0;
    element_i += 1;
    element_data[element_i] = 2;
    element_i += 1;
    element_data[element_i] = 3;
    element_i += 1;

    element_data[element_i] = 4;
    element_i += 1;
    element_data[element_i] = 5;
    element_i += 1;
    element_data[element_i] = 6;
    element_i += 1;

    element_data[element_i] = 4;
    element_i += 1;
    element_data[element_i] = 6;
    element_i += 1;
    element_data[element_i] = 7;
    element_i += 1;

    for i in 0..(FLAGPOLE_RES - 1) {
        element_data[element_i] = 8;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*i        ) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*(i+1)    ) as GLushort;
        element_i += 1;

        element_data[element_i] = 9 + (FLAGPOLE_SLICE*i        ) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*i     + 1) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*(i+1)    ) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*i     + 1) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*(i+1) + 1) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*(i+1)    ) as GLushort;
        element_i += 1;

        element_data[element_i] = 9 + (FLAGPOLE_SLICE*i     + 1) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*i     + 2) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*(i+1) + 1) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*i     + 2) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*(i+1) + 2) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*(i+1) + 1) as GLushort;
        element_i += 1;

        element_data[element_i] = 9 + (FLAGPOLE_SLICE*i     + 2) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*i     + 3) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*(i+1) + 2) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*i     + 3) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*(i+1) + 3) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*(i+1) + 2) as GLushort;
        element_i += 1;

        element_data[element_i] = 9 + (FLAGPOLE_SLICE*i     + 3) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*i     + 4) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*(i+1) + 3) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*i     + 4) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*(i+1) + 4) as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*(i+1) + 3) as GLushort;
        element_i += 1;

        element_data[element_i] = 9 + (FLAGPOLE_SLICE*i     + 5) as GLushort;
        element_i += 1;
        element_data[element_i] = vertex_i as GLushort;
        element_i += 1;
        element_data[element_i] = 9 + (FLAGPOLE_SLICE*(i+1) + 5) as GLushort;
        element_i += 1;
    }

    element_data[element_i] = 8;
    element_i += 1;
    element_data[element_i] = 9 + (FLAGPOLE_SLICE*(FLAGPOLE_RES-1)    ) as GLushort;
    element_i += 1;
    element_data[element_i] = 9;
    element_i += 1;

    element_data[element_i] = 9 + (FLAGPOLE_SLICE*(FLAGPOLE_RES-1)    ) as GLushort;
    element_i += 1;
    element_data[element_i] = 9 + (FLAGPOLE_SLICE*(FLAGPOLE_RES-1) + 1) as GLushort;
    element_i += 1;
    element_data[element_i] = 9;
    element_i += 1;
    element_data[element_i] = 9 + (FLAGPOLE_SLICE*(FLAGPOLE_RES-1) + 1) as GLushort;
    element_i += 1;
    element_data[element_i] = 9 + 1;
    element_i += 1;
    element_data[element_i] = 9;
    element_i += 1;

    element_data[element_i] = 9 + (FLAGPOLE_SLICE*(FLAGPOLE_RES-1) + 1) as GLushort;
    element_i += 1;
    element_data[element_i] = 9 + (FLAGPOLE_SLICE*(FLAGPOLE_RES-1) + 2) as GLushort;
    element_i += 1;
    element_data[element_i] = 9 + 1;
    element_i += 1;
    element_data[element_i] = 9 + (FLAGPOLE_SLICE*(FLAGPOLE_RES-1) + 2) as GLushort;
    element_i += 1;
    element_data[element_i] = 9 + 2;
    element_i += 1;
    element_data[element_i] = 9 + 1;
    element_i += 1;

    element_data[element_i] = 9 + (FLAGPOLE_SLICE*(FLAGPOLE_RES-1) + 2) as GLushort;
    element_i += 1;
    element_data[element_i] = 9 + (FLAGPOLE_SLICE*(FLAGPOLE_RES-1) + 3) as GLushort;
    element_i += 1;
    element_data[element_i] = 9 + 2;
    element_i += 1;
    element_data[element_i] = 9 + (FLAGPOLE_SLICE*(FLAGPOLE_RES-1) + 3) as GLushort;
    element_i += 1;
    element_data[element_i] = 9 + 3;
    element_i += 1;
    element_data[element_i] = 9 + 2;
    element_i += 1;

    element_data[element_i] = 9 + (FLAGPOLE_SLICE*(FLAGPOLE_RES-1) + 3) as GLushort;
    element_i += 1;
    element_data[element_i] = 9 + (FLAGPOLE_SLICE*(FLAGPOLE_RES-1) + 4) as GLushort;
    element_i += 1;
    element_data[element_i] = 9 + 3;
    element_i += 1;
    element_data[element_i] = 9 + (FLAGPOLE_SLICE*(FLAGPOLE_RES-1) + 4) as GLushort;
    element_i += 1;
    element_data[element_i] = 9 + 4;
    element_i += 1;
    element_data[element_i] = 9 + 3;
    element_i += 1;

    element_data[element_i] = 9 + (FLAGPOLE_SLICE*(FLAGPOLE_RES-1) + 5) as GLushort;
    element_i += 1;
    element_data[element_i] = vertex_i as GLushort;
    element_i += 1;
    element_data[element_i] = 9 + 5;
    element_i += 1;

    init_mesh(
        out_mesh,
        &vertex_data, vertex_count,
        &element_data, element_count,
        gl::STATIC_DRAW
    );
}

fn update_flag_mesh(
    mesh: &FlagMesh, vertex_data: &mut [FlagVertex], time: GLfloat
) {
    let mut i = 0;
    for t in 0..FLAG_Y_RES {
        for s in 0..FLAG_X_RES {
            let ss: GLfloat = FLAG_S_STEP * (s as GLfloat);
            let tt: GLfloat = FLAG_T_STEP * (t as GLfloat);

            calculate_flag_vertex(&mut vertex_data[i], ss, tt, time);

            i += 1;
        }
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vertex_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (FLAG_VERTEX_COUNT as usize * mem::size_of::<FlagVertex>()) as GLsizeiptr,
            mem::transmute(&vertex_data[0]),
            gl::STREAM_DRAW
        );
    }
}
