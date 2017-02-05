#version 330

uniform sampler2DArray u_texture_array;

uniform float u_alpha_minimum;

in vec4 v_color;
in vec3 v_tex_coord;
in vec3 v_normal;

out vec4 f_color;

void main() {
    vec4 albedo_colour = texture(u_texture_array, v_tex_coord) * v_color;
    
    vec4 final_colour = albedo_colour; // * light;
    final_colour.a = albedo_colour.a; // ignore light's alpha

    if(final_colour.a < u_alpha_minimum) {
        discard;
    }
    f_color = final_colour;
}


