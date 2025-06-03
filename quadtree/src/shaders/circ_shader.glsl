@vs circle_vs
in vec2 v_pos;

in vec2 i_center;
in float i_radius;
in vec3 i_color;

out vec2 f_pos;
out vec3 f_color;

layout (binding = 1) uniform v_params_world {
    vec2 world_dims;
};

void main() {
    f_color = i_color;
    f_pos = v_pos;
    
    vec2 ndc_center = (i_center / world_dims) * 2. - 1.;
    vec2 ndc_radius = (i_radius / world_dims) * 2.;

    vec2 pos = ndc_center + v_pos * ndc_radius;
    gl_Position = vec4(pos, 0., 1.);
}
@end

@fs circle_fs
in vec2 f_pos;
in vec3 f_color;

out vec4 color;

void main() {
    if (length(f_pos) > 1.) {
        discard;
    }

    color = vec4(f_color, 0.9);
}
@end

@program circle circle_vs circle_fs

