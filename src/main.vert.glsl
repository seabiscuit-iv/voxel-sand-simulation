#version 330

layout(location = 0) in vec4 vs_pos;
layout(location = 1) in vec4 vs_col;
layout(location = 2) in vec2 vs_uv;

out vec4 fs_col;
out vec2 fs_uv; 
out vec3 fs_pos;

uniform mat4 u_ViewProj;

void main() {
    // fs_col = vs_col;
    fs_col = vs_col;
    fs_uv = vs_uv;

    vec4 pos = vs_pos;
    pos.y *= -1.0;

    pos =  u_ViewProj * pos;
    // pos.z = 0;
    // pos /= pos.w;

    gl_Position = pos;
    fs_pos = vec3(pos);
    // gl_Position = vs_pos;
}