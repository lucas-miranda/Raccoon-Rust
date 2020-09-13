#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec2 v_uv;
layout(location = 0) out vec4 target0;

layout(set = 0, binding = 0) uniform texture2D u_texture;
layout(set = 0, binding = 1) uniform sampler u_sampler;

void main() {
    target0 = texture(sampler2D(u_texture, u_sampler), v_uv);
}

/*
#version 450

layout (push_constant) uniform PushConsts {
    float time;
} push;
layout (location = 1) in vec3 frag_color;
layout (location = 0) out vec4 color;

void main() {
    float time01 = -0.9 * abs(sin(push.time * 0.9)) + 0.9;
    color = vec4(frag_color, 1.0) * vec4(time01, time01, time01, 1.0);
}
*/
