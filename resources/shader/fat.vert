#version 330

uniform mat4 u_matrix;
uniform vec4 u_color;

in vec3 a_position;
in vec3 a_tex_coord;
in vec4 a_color;
in vec3 a_normal;

out vec4 v_color;
out vec3 v_tex_coord;
out vec3 v_normal;

void main() {
    gl_Position = u_matrix * vec4(a_position, 1.0);
    v_color = a_color * u_color;
    v_tex_coord = a_tex_coord;
    v_normal = a_normal; 
}
