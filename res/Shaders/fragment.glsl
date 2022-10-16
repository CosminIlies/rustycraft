#version 140

in vec2 v_tex_coords;

out vec4 color;

uniform sampler2D difuse_tex;

void main() {
    color = texture(difuse_tex, v_tex_coords);
}