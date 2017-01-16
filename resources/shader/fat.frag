#version 330

uniform sampler2DArray u_texture_array;

uniform float u_alpha_minimum;
uniform vec3 u_sun_direction;

in vec4 v_color;
in vec3 v_tex_coord;
in vec3 v_normal;

out vec4 f_color;

void main() {
    vec3 to_light = normalize(vec3(0.0, 1.0, 0.0));
    float light = clamp(dot(v_normal, u_sun_direction), 0.2, 1.0);

    vec4 albedo_colour = texture(u_texture_array, v_tex_coord) * v_color;
    
    vec4 final_colour = albedo_colour; // * light;
    final_colour.a = albedo_colour.a; // ignore light's alpha

    if(final_colour.a < u_alpha_minimum) {
        discard;
    }
    f_color = final_colour;
    f_color *= 1.0;
}


