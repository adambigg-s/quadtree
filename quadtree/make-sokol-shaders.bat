@echo off

set arg=%1
if "%arg%"=="" set arg=build

echo. & echo + building shaders +
sokol-shdc.exe -i ./src/shaders/tri_shader.glsl  -o ./src/compiled_shaders/tri_shader.rs  --slang hlsl5:wgsl:glsl430 -f sokol_rust
sokol-shdc.exe -i ./src/shaders/circ_shader.glsl -o ./src/compiled_shaders/circ_shader.rs --slang hlsl5:wgsl:glsl430 -f sokol_rust
sokol-shdc.exe -i ./src/shaders/line_shader.glsl -o ./src/compiled_shaders/line_shader.rs --slang hlsl5:wgsl:glsl430 -f sokol_rust

echo. & echo + building project +
cargo %arg%
