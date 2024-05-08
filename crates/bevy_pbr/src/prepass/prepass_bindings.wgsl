#define_import_path bevy_pbr::prepass_bindings

#ifdef MOTION_VECTOR_PREPASS
@group(0) @binding(2) var<storage> previous_view_uniforms: array<PreviousViewUniforms>;
@group(0) @binding(28) var<uniform> previous_view_index: u32;
#endif // MOTION_VECTOR_PREPASS

// Material bindings will be in @group(2)
