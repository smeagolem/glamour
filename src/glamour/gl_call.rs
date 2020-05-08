use gl;

/// Pop an error from the OpenGL error state.
///
/// # Examples
///
/// ```no_run
/// # use gl;
/// # use glamour::gl_call::gl_get_error;
/// #
/// let id = unsafe { gl::CreateShader(gl::LINEAR_MIPMAP_NEAREST) };
/// let error = gl_get_error();
/// assert_eq!(error.unwrap(), gl::INVALID_ENUM);
/// ```
pub fn gl_get_error() -> Option<gl::types::GLenum> {
    let error_code = unsafe { gl::GetError() };
    match error_code {
        gl::NO_ERROR => None,
        _ => Some(error_code),
    }
}

/// Get the enum name of an OpenGL error.
///
/// # Examples
///
/// ```
/// # use gl;
/// # use glamour::gl_call::gl_get_error_name;
/// #
/// assert_eq!("INVALID_ENUM", gl_get_error_name(&gl::INVALID_ENUM));
/// ```
pub fn gl_get_error_name(error_code: &gl::types::GLenum) -> String {
    match *error_code {
        gl::INVALID_ENUM => "INVALID_ENUM",
        gl::INVALID_VALUE => "INVALID_VALUE",
        gl::INVALID_OPERATION => "INVALID_OPERATION",
        gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
        gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
        gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
        gl::STACK_OVERFLOW => "STACK_OVERFLOW",
        _ => "Unknown Error",
    }
    .to_string()
}

pub fn gl_get_errors() -> Vec<u32> {
    let mut errors = Vec::<u32>::new();
    while let Some(error) = gl_get_error() {
        errors.push(error);
    }
    errors
}

pub fn gl_clear_errors() {
    while let Some(_error) = gl_get_error() {}
}

/// A wrapper for an unsafe OpenGL call that will `panic!` if any OpenGL errors exist.
///
/// # Panics
///
/// Panics if any OpenGL errors exist.
/// Will only `panic!` when `debug_assertions` is `true`.
///
/// # Examples
///
/// ```no_run
/// # use gl;
/// # use glamour::gl_call;
/// #
/// let mut id = 0;
/// gl_call!(gl::GenBuffers(1, &mut id));
/// assert_ne!(id, 0);
/// ```
#[macro_export]
macro_rules! gl_call {
    ($gl_fn:expr) => {{
        let result = unsafe { $gl_fn };
        if cfg!(debug_assertions) {
            let errors = $crate::gl_call::gl_get_errors();
            if !errors.is_empty() {
                panic!(
                    "OpenGL Errors: {}",
                    errors
                        .iter()
                        .map($crate::gl_call::gl_get_error_name)
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }
        result
    }};
}
