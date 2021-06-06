#version 460

layout (location = 0) out vec4 frag_color;
layout (set = 0, binding = 0) uniform utexture3D t_world_3d;
layout (set = 0, binding = 1) uniform sampler s_world_3d;

void main() {
    if (texelFetch(usampler3D(t_world_3d, s_world_3d), ivec3(1, 1, 1), 0).x != 0) {
        frag_color = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        frag_color = vec4(1.0, 1.0, 1.0, 1.0);
    }
}
