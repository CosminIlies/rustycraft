#version 140

in vec3 position;
in vec2 tex_coords;

out vec2 v_tex_coords;

uniform mat4 view_mat;
uniform mat4 trans_mat;
uniform mat4 proj_mat;

void main() {
    mat4 modelview = view_mat * trans_mat;

    v_tex_coords = tex_coords;
    gl_Position = proj_mat * modelview * vec4(position, 1.0);
}