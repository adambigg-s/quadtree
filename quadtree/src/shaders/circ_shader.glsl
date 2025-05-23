@vs circle_vs
in vec2 v_pos;

out vec3 f_color;
out vec2 f_pos;
out vec2 f_center;
out float f_radius;

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
    f_radius = radius;

    float ndcx = (center.x / world_dims.x) * 2. - 1.;
    float ndcy = (center.y / world_dims.y) * 2. - 1.;
    
    f_center = vec2(ndcx, ndcy);
    f_pos = f_center + v_pos * radius;
    gl_Position = vec4(f_pos, 1., 1.);
}
@end

@fs circle_fs
in vec3 f_color;
in vec2 f_pos;
in vec2 f_center;
in float f_radius;

out vec3 color;

void main() {
    if (distance(f_pos, f_center) > f_radius) {
        discard;
    }

    color = f_color;
}
@end

@program circle circle_vs circle_fs

