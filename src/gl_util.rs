use gl;
use gl::types::{GLenum, GLuint, GLint, GLchar};
use std::ptr;
use file_util;


pub fn make_texture(filename: &str) -> GLuint {
    let (pixels, height, width) = match file_util::read_tga(filename) {
        Ok(tuple) => tuple,
        Err(_) => return 0,
    };
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        gl::TexImage2D(
            gl::TEXTURE_2D, 0,
            gl::RGB8 as GLint,
            width as GLint, height as GLint, 0,
            gl::BGR, gl::UNSIGNED_BYTE,
            pixels
        );
    }

    texture
}

pub fn make_shader(shader_type: GLenum, filename: &str) -> GLuint {
    let source = match file_util::file_contents(filename) {
        Ok(val) => val,
        Err(_) => return 0,
    };

    unsafe {
        let mut shader_ok = 0;
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &source.as_ptr(), ptr::null());
        gl::CompileShader(shader);
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut shader_ok);

        if shader_ok == 0 {
            eprintln!("Failed to compile {}", filename);
            // BEGIN show_info_log.
            let mut log_length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);
            let log: Vec<i8> = Vec::with_capacity(log_length as usize);
            gl::GetShaderInfoLog(shader, log_length, &mut 0, log.as_ptr() as *mut GLchar);
            eprintln!("{:?}", log);
            // END show_info_log.
            gl::DeleteShader(shader);
        
            return 0;
        }

        shader
    }
}

pub fn make_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    let mut program_ok: GLint = 0;
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut program_ok);

        if program_ok == 0 {
            eprintln!("Failed to link shader program:");
            // BEGIN show_info_log.
            let mut log_length = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);
            let log: Vec<i8> = Vec::with_capacity(log_length as usize);
            gl::GetShaderInfoLog(program, log_length, &mut 0, log.as_ptr() as *mut i8);
            eprintln!("{:?}", log);
            // END show_info_log.
            gl::DeleteProgram(program);

            return 0;
        }

        program
    }
}

