@vs simple_vs
in vec2 v_pos;
in vec3 v_color;

out vec3 f_color;

layout (binding = 0) uniform v_params_world {
    vec2 world_dims;
};

void main() {
    f_color = v_color;

    vec2 ndc = (v_pos / world_dims) * 2. - 1;
    gl_Position = vec4(ndc, 0., 1.);
}
@end

@fs simple_fs
in vec3 f_color;

out vec3 color;

void main() {
    color = f_color;
}
@end

@program simple simple_vs simple_fs
