
pub mod renderer;

use renderer::Renderer;
use renderer::{Shader, ShaderType};

const FRAG_SHADER_SRC : &str = r#"
#version 330 core
out vec4 FragColor;
in vec3 something;
uniform sampler2D tex;

void main()
{
    vec2 windowPos = (gl_FragCoord.xy / 1024);
	vec3 rgb = texture(tex, windowPos).rgb;
    FragColor = vec4(rgb, 1.0f);
} 

"#;

const VERT_SHADER_SRC : &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
"#;


fn main() {
    let mut r = Renderer::initialize(1024, 768, "Hello");

    let mut frag_shader = Shader::create(ShaderType::Fragment).unwrap();
    frag_shader.compile(FRAG_SHADER_SRC).expect("Failed to compile frag shader");


    let mut vert_shader = Shader::create(ShaderType::Vertex).unwrap();
    vert_shader.compile(VERT_SHADER_SRC).expect("Failed to compile vert shader");


    let mut p = renderer::Program::create().unwrap();
    p.attach(&frag_shader).expect("Failed to attach shader to program");
    p.attach(&vert_shader).expect("Failed to attach shader to program");

    let res = p.link();

    if let Err(error) = res {
        println!("{}", error);
    }



    while r.update() {
        // do something
    }

    println!("Shutting down engine!");

    r.shutdown();

}
