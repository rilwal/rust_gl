

/*
    The purpose of this crate is to wrap OpenGL so we don't need to directly use GL calls,
    and we can avoid a lot of unsafe tags all around our code by putting them all in here

    Ideally we can interact with this wrapper using normal Rust types, for example passing
    slices to functions that take arrays, rather than raw pointers with sizes.

    Functionality: */
    /*TODO
        * Create a window and context using GLFW
        * Load OpenGL using the gl crate
        * Allow management of resources:
            * Shaders / Programs
            * VBOs
            * Buffers
        * Finish this todo list
*/

extern crate gl;
extern crate glm;
extern crate glfw;

use glfw::{Glfw, Window, WindowEvent, Key, Action, Context};
use std::{sync::mpsc::Receiver, ffi::{c_void, CStr, CString}, str::from_utf8_unchecked, fmt};

pub struct Renderer {
    glfw : Glfw,
    window : Window,
    events : Receiver<(f64, WindowEvent)>,
}

#[derive(Debug, Clone)]
pub enum GLError {
    ResourceAllocationFailed,
    ResourceNotAllocated,
    ShaderCompileError(String),
    ProgramLinkError(String)
}


impl fmt::Display for GLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", match self {
            Self::ResourceAllocationFailed  =>   "GlError: Resource allocation failed",
            Self::ResourceNotAllocated      =>   "GlError: Resource not allocated",
            Self::ShaderCompileError(_)     =>   "GlError: Shader compilation failed: ",
            Self::ProgramLinkError(_)       =>   "GlError: Program linking failed: ",

            _ => "Other GL Error"
        },
        match self {
            Self::ShaderCompileError(s) => s,
            Self::ProgramLinkError(s)   => s,
            _ => ""
        })
    }
}


pub enum ShaderType {
    Fragment,
    Vertex,
    Geometry, 
}


pub enum GLType {
    Vector3,
    Vector4,
    Float
}

pub struct Shader {
    id: u32,
    kind: ShaderType
}


pub struct Program {
    id: u32,
}


impl Renderer {
    pub fn initialize(width: u32, height: u32, title: &str) -> Self {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW");
        let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed).expect("Failed to create window");

        window.set_key_polling(true);
        window.make_current();
    
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        Self {glfw: glfw, window: window, events : events}
    }

    // This function should deallocate all OpenGL related resources,
    // which can't clean themselves up because they come from FFI
    pub fn shutdown(&mut self) {
    }

    pub fn update(&mut self) -> bool {
        self.window.swap_buffers();

        // TODO: Don't handle input in the renderer update function, it's stupid af
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                }
                _ => {}
            }
        }

        !self.window.should_close()
    }
}


impl Shader {
    pub fn create(kind : ShaderType) -> Result<Self, GLError> {
        let mut shader = 0;
        unsafe {
            shader = gl::CreateShader(match kind {
                ShaderType::Fragment => gl::FRAGMENT_SHADER,
                ShaderType::Vertex => gl::VERTEX_SHADER,
                ShaderType::Geometry => gl::GEOMETRY_SHADER
            });
        }

        match shader {
            0 => Err(GLError::ResourceAllocationFailed),
            _ => Ok(Self {id: shader, kind: kind})
        }
    }

    pub fn compile(&mut self, source : &str) -> Result<(), GLError> {
        if self.id == 0 {
            return Err(GLError::ResourceNotAllocated)
        }

        unsafe {
            let src_ptr = source.as_ptr() as *const i8;
            let src_len = source.len() as i32;

            gl::ShaderSource(self.id, 1, &src_ptr, &src_len);
            gl::CompileShader(self.id);

            let mut compile_status = 0i32;
            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut compile_status as *mut i32);

            if compile_status != gl::TRUE as i32 {
                // If we fail to compile, return a GLError::ShaderCompileError with the erorr log
                let mut info_log_len = 0i32;
                gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut info_log_len as *mut i32);

                let mut info_log_bytes = vec![0u8; info_log_len as usize];
                gl::GetShaderInfoLog(self.id, info_log_len, std::ptr::null_mut(), info_log_bytes.as_mut_ptr() as *mut i8);
                
                let info_log_str = String::from_utf8_unchecked(info_log_bytes);

                return Err(GLError::ShaderCompileError(info_log_str))
            }
        }

        Ok(())
    }
}


impl Program {
    pub fn create() -> Result<Self, GLError> {
        let mut program = 0;

        unsafe {
            program = gl::CreateProgram();
        }

        match program {
            0 => Err(GLError::ResourceAllocationFailed),
            _ => Ok(Self{id: program})
        }
    }

    pub fn attach(&mut self, shader : &Shader) -> Result<(), GLError>{
        unsafe {
            gl::AttachShader(self.id, shader.id);

            if gl::GetError() != gl::NO_ERROR {
                return Err(GLError::ResourceAllocationFailed); // TODO: Better error here!!!
            }
        }

        Ok(())
    }

    pub fn link(&mut self) -> Result<(), GLError> {
        if self.id == 0 {
            return Err(GLError::ResourceNotAllocated)
        }

        unsafe {
            
            gl::LinkProgram(self.id);

            let mut link_status = 0i32;
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut link_status as *mut i32);

            if link_status != gl::TRUE as i32 {
                // If we fail to compile, return a GLError::ShaderCompileError with the erorr log
                let mut info_log_len = 0i32;
                gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut info_log_len as *mut i32);

                let mut info_log_bytes = vec![0u8; info_log_len as usize];
                gl::GetProgramInfoLog(self.id, info_log_len, std::ptr::null_mut(), info_log_bytes.as_mut_ptr() as *mut i8);
                
                let info_log_str = String::from_utf8_unchecked(info_log_bytes);

                return Err(GLError::ProgramLinkError(info_log_str))
            }
        }

        Ok(())
    }
}

