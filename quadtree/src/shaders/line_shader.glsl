@vs line_vs
in vec2 v_pos;
in vec3 v_color;

out vec3 f_color;

void main() {
    f_color = v_color;

    gl_Position = vec4(v_pos, 1., 1.);
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
