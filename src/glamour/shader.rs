use anyhow::{anyhow, Result};
use gl;
use std::ffi::{CStr, CString};

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn new(shaders: &[Shader]) -> Result<Program> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(id, shader.id()) };
        }

        unsafe { gl::LinkProgram(id) };

        let mut success: gl::types::GLint = 1;
        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success) };

        match success {
            0 => {
                let mut len: gl::types::GLint = 0;
                unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len) };

                let error = CString::new(" ".repeat(len as usize)).unwrap();
                unsafe {
                    gl::GetProgramInfoLog(
                        id,
                        len,
                        std::ptr::null_mut(),
                        error.as_ptr() as *mut gl::types::GLchar,
                    );
                }
                Err(anyhow!(
                    "program compilation failed\n{}",
                    error.to_string_lossy().into_owned()
                ))
            }
            _ => {
                for shader in shaders {
                    unsafe { gl::DetachShader(id, shader.id()) };
                }
                Ok(Program { id })
            }
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_use(&self) {
        unsafe { gl::UseProgram(self.id) };
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) };
    }
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn new(source: &CStr, shader_type: gl::types::GLenum) -> Shader {
        let id = gl_call!(gl::CreateShader(shader_type));
        gl_call!(gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null()));
        gl_call!(gl::CompileShader(id));
        let mut success: gl::types::GLint = 1;
        gl_call!(gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success));
        match success {
            0 => {
                let mut len: gl::types::GLint = 0;
                gl_call!(gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len));
                let error = CString::new(" ".repeat(len as usize)).unwrap();
                gl_call!(gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                ));
                panic!(
                    "shader compilation failed:\n{}",
                    error.to_string_lossy().into_owned()
                );
            }
            _ => Shader { id },
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl_call!(gl::DeleteShader(self.id));
    }
}
