@vs simple_vs
in vec2 v_pos;
in vec3 v_color;

out vec3 f_color;

layout (binding = 0) uniform v_params_world {
    vec2 world_dims;
};

void main() {
    f_color = v_color;

    float ndcx = (v_pos.x / world_dims.x) * 2. - 1.;
    float ndcy = (v_pos.y / world_dims.y) * 2. - 1.;
    vec2 ndc = vec2(ndcx, ndcy);
    
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
