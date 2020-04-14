use gl;
use std::ffi::CString;

pub struct Program {
    id: u32,
}

impl Program {
    /// Creates a program and links shaders to it.
    ///
    /// # Panics
    /// If program linking fails, this function will `panic!`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use glamour::{Shader, ShaderType, Program};
    /// # let vert_shader_source = "";
    /// # let frag_shader_source = "";
    ///
    /// let vert_shader = Shader::new(ShaderType::Vertex, vert_shader_source);
    /// let frag_shader = Shader::new(ShaderType::Fragment, frag_shader_source);
    /// let shader_program = Program::new(&[vert_shader, frag_shader]);
    /// ```
    pub fn new(shaders: &[Shader]) -> Program {
        let id = gl_call!(gl::CreateProgram());
        for shader in shaders {
            gl_call!(gl::AttachShader(id, shader.id()));
        }
        gl_call!(gl::LinkProgram(id));
        let mut success: gl::types::GLint = 1;
        gl_call!(gl::GetProgramiv(id, gl::LINK_STATUS, &mut success));
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            gl_call!(gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len));
            let error = CString::new(" ".repeat(len as usize)).unwrap();
            gl_call!(gl::GetProgramInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            ));
            panic!(
                "program linking failed:\n{}",
                error.to_string_lossy().into_owned()
            );
        }
        Program { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn set_use(&self) {
        gl_call!(gl::UseProgram(self.id));
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        gl_call!(gl::DeleteProgram(self.id));
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
}

pub struct Shader {
    id: u32,
}

impl Shader {
    /// Creates a shader from source.
    ///
    /// # Panics
    /// If shader compilation fails, this function will `panic!`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use glamour::{Shader, ShaderType};
    ///
    /// let vert_shader_source = r#"
    /// #version 410 core
    /// layout (location = 0) in vec3 Position;
    /// void main()
    /// {
    ///     gl_Position = vec4(Position, 1.0);
    /// }
    /// "#;
    /// let vert_shader = Shader::new(ShaderType::Vertex, vert_shader_source);
    /// ```
    pub fn new(shader_type: ShaderType, source: &str) -> Shader {
        let id = gl_call!(gl::CreateShader(shader_type as gl::types::GLenum));
        let source = CString::new(source).unwrap();
        gl_call!(gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null()));
        gl_call!(gl::CompileShader(id));
        let mut success: gl::types::GLint = 1;
        gl_call!(gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success));
        if success == 0 {
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
        Shader { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl_call!(gl::DeleteShader(self.id));
    }
}
