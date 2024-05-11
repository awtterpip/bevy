#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

#ifdef INPUT_ARRAY
@group(0) @binding(0) var in_texture: texture_2d_array<f32>;
#else
@group(0) @binding(0) var in_texture: texture_2d<f32>;
#endif
@group(0) @binding(1) var in_sampler: sampler;

@fragment
fn fs_main(
    in: FullscreenVertexOutput,
#ifdef MULTIVIEW
    @builtin(view_index) view_index: i32,
#endif
) -> @location(0) vec4<f32> {
#ifdef VIEW_OVERRIDE
    let index: i32 = #{VIEW_OVERRIDE}i;
#else ifdef MULTIVIEW
    let index: i32 = view_index;
#else
    let index: i32 = 0i;
#endif

    return textureSample(
        in_texture, 
        in_sampler, 
        in.uv,
#ifdef INPUT_ARRAY
        index,
#endif
    );
}
