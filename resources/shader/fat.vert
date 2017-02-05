#version 330

uniform mat4 u_matrix;
uniform vec4 u_color;

in vec3 position;
in vec3 tex_coord;
in vec4 color;
in vec3 normal;

out vec4 v_color;
out vec3 v_tex_coord;
out vec3 v_normal;

void main() {
    gl_Position = u_matrix * vec4(position, 1.0);
    v_color = color * u_color;
    v_tex_coord = tex_coord;
    v_normal = normal; 
}
