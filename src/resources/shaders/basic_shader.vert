#version 450

layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
layout (location = 0) out gl_PerVertex {
    vec4 gl_Position;
};
layout (location = 1) out vec3 frag_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    frag_color = color;
}
