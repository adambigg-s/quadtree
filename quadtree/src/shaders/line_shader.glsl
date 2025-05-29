@vs line_vs
in vec2 i_pos;
in vec3 i_color;

out vec3 f_color;

layout (binding = 0) uniform v_params_world {
    vec2 world_dims;
};

void main() {
    f_color = i_color;

    vec2 ndc = (i_pos / world_dims) * 2. - 1.;
    gl_Position = vec4(ndc, 0., 1.);
}
@end

@fs line_fs
in vec3 f_color;

out vec3 color;

void main() {
    color = f_color;
}
@end

@program line line_vs line_fs
