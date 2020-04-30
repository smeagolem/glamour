use crate::glm;
use gl;
use std::ffi::CString;

pub struct ShaderBuilder {
    vert_src: String,
    frag_src: String,
    uniforms: Vec<Uniform>,
}

impl ShaderBuilder {
    pub fn new(vert_src: &str, frag_src: &str) -> ShaderBuilder {
        ShaderBuilder {
            vert_src: vert_src.to_string(),
            frag_src: frag_src.to_string(),
            uniforms: Vec::new(),
        }
    }
    pub fn with_float4(mut self, name: &str, value: glm::Vec4) -> Self {
        let uniform = Uniform {
            name: name.to_string(),
            value: UniformValue::Float4(value),
        };
        self.uniforms.push(uniform);
        self
    }
    pub fn with_mat4(mut self, name: &str, value: glm::Mat4) -> Self {
        let uniform = Uniform {
            name: name.to_string(),
            value: UniformValue::Mat4(value),
        };
        self.uniforms.push(uniform);
        self
    }
    pub fn build(&self) -> ShaderProgram {
        let vert = Shader::new(ShaderType::Vertex, &self.vert_src);
        let frag = Shader::new(ShaderType::Fragment, &self.frag_src);
        let prog = ShaderProgram::new(&[vert, frag]);
        prog.bind();
        for uniform in &self.uniforms {
            let name = CString::new(uniform.name.as_str()).unwrap();
            let location = gl_call!(gl::GetUniformLocation(prog.id(), name.as_ptr()));
            // if -1, could not find uniform, might be fine if unused and stripped by shader compilation.
            if location == -1 {
                continue;
            }
            match &uniform.value {
                UniformValue::Float4(v) => gl_call!(gl::Uniform4f(location, v.x, v.y, v.z, v.w)),
                UniformValue::Mat4(v) => {
                    gl_call!(gl::UniformMatrix4fv(location, 1, gl::FALSE, v.as_ptr()))
                }
                _ => unimplemented!("not currently supported."),
            }
        }
        prog
    }
}

#[derive(Debug)]
struct Uniform {
    name: String,
    value: UniformValue,
}

#[allow(dead_code)]
#[derive(Debug)]
enum UniformValue {
    Int(u32),
    IntArray(Vec<u32>),
    Float(f32),
    Float3(glm::Vec3),
    Float4(glm::Vec4),
    Mat4(glm::Mat4),
}

pub struct ShaderProgram {
    id: u32,
}

impl ShaderProgram {
    /// Creates a program and links shaders to it.
    ///
    /// # Panics
    /// If program linking fails, this function will `panic!`.
    fn new(shaders: &[Shader]) -> ShaderProgram {
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
        ShaderProgram { id }
    }

    fn id(&self) -> u32 {
        self.id
    }

    pub fn bind(&self) {
        gl_call!(gl::UseProgram(self.id));
    }

    pub fn unbind(&self) {
        gl_call!(gl::UseProgram(0));
    }

    pub fn set_int(&self, name: &str, value: i32) {
        self.bind();
        let name = CString::new(name).unwrap();
        let location = gl_call!(gl::GetUniformLocation(self.id(), name.as_ptr()));
        gl_call!(gl::Uniform1i(location, value));
        self.unbind();
    }

    pub fn set_float3(&self, name: &str, value: &glm::Vec3) {
        self.bind();
        let name = CString::new(name).unwrap();
        let location = gl_call!(gl::GetUniformLocation(self.id(), name.as_ptr()));
        gl_call!(gl::Uniform3f(location, value.x, value.y, value.z));
        self.unbind();
    }

    pub fn set_float4(&self, name: &str, value: &glm::Vec4) {
        self.bind();
        let name = CString::new(name).unwrap();
        let location = gl_call!(gl::GetUniformLocation(self.id(), name.as_ptr()));
        gl_call!(gl::Uniform4f(location, value.x, value.y, value.z, value.w));
        self.unbind();
    }

    pub fn set_mat4(&self, name: &str, value: &glm::Mat4) {
        self.bind();
        let name = CString::new(name).unwrap();
        let location = gl_call!(gl::GetUniformLocation(self.id(), name.as_ptr()));
        gl_call!(gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr()));
        self.unbind();
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        gl_call!(gl::DeleteProgram(self.id));
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
enum ShaderType {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
}

struct Shader {
    id: u32,
}

impl Shader {
    /// Creates a shader from source.
    ///
    /// # Panics
    /// If shader compilation fails, this function will `panic!`.
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
