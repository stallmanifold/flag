use gl::types::GLfloat;
use std::f32;


const M_PI: GLfloat = 3.141592653589793;

fn vec_cross(out_result: &mut [GLfloat], u: &[GLfloat], v: &[GLfloat]) {
    out_result[0] = u[1]*v[2] - u[2]*v[1];
    out_result[1] = u[2]*v[0] - u[0]*v[2];
    out_result[2] = u[0]*v[1] - u[1]*v[0];
}

fn vec_length(v: &[GLfloat]) -> GLfloat {
    f32::sqrt(v[0]*v[0] + v[1]*v[1] + v[2]*v[2])
}

fn vec_normalize(inout_v: &mut [GLfloat]) {
    let rlen: GLfloat = 1.0 / vec_length(inout_v);
    inout_v[0] *= rlen;
    inout_v[1] *= rlen;
    inout_v[2] *= rlen;
}
