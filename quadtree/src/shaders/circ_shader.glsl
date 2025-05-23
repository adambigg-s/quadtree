@vs circle_vs
in vec2 v_pos;

out vec2 f_pos;
out vec3 f_color;

layout (binding = 0) uniform v_params {
    vec3 color;
    vec2 center;
    float radius;
};

layout (binding = 1) uniform v_params_world {
    vec2 world_dims;
};

void main() {
    f_color = color;
    f_pos = v_pos;
    
    vec2 ndc_center = (center / world_dims) * 2. - 1.;
    vec2 ndc_radius = (radius / world_dims) * 2.;

    vec2 pos = ndc_center + v_pos * ndc_radius;
    gl_Position = vec4(pos, 0., 1.);
}
@end

@fs circle_fs
in vec2 f_pos;
in vec3 f_color;

out vec3 color;

void main() {
    if (length(f_pos) > 1.) {
        discard;
    }

    color = f_color;
}
@end

@program circle circle_vs circle_fs

