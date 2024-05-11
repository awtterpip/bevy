use std::num::NonZeroU32;

use bevy_app::{App, Plugin};
use bevy_asset::{load_internal_asset, Handle};
use bevy_ecs::prelude::*;
use bevy_render::{
    render_resource::{
        binding_types::{sampler, texture_2d, texture_2d_array},
        *,
    },
    renderer::RenderDevice,
    RenderApp,
};

use crate::fullscreen_vertex_shader::fullscreen_shader_vertex_state;

pub const BLIT_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(2312396983770133547);

/// Adds support for specialized "blit pipelines", which can be used to write one texture to another.
pub struct BlitPlugin;

impl Plugin for BlitPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, BLIT_SHADER_HANDLE, "blit.wgsl", Shader::from_wgsl);

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.allow_ambiguous_resource::<SpecializedRenderPipelines<BlitPipeline>>();
        }
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<BlitPipeline>()
            .init_resource::<SpecializedRenderPipelines<BlitPipeline>>();
    }
}

#[derive(Resource)]
pub struct BlitPipeline {
    pub texture_bind_group: BindGroupLayout,
    pub array_texture_bind_group: BindGroupLayout,
    pub sampler: Sampler,
}

impl FromWorld for BlitPipeline {
    fn from_world(render_world: &mut World) -> Self {
        let render_device = render_world.resource::<RenderDevice>();

        let texture_bind_group = render_device.create_bind_group_layout(
            "blit_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: false }),
                    sampler(SamplerBindingType::NonFiltering),
                ),
            ),
        );

        let array_texture_bind_group = render_device.create_bind_group_layout(
            "array_blit_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d_array(TextureSampleType::Float { filterable: false }),
                    sampler(SamplerBindingType::NonFiltering),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        BlitPipeline {
            texture_bind_group,
            array_texture_bind_group,
            sampler,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BlitPipelineKey {
    pub texture_format: TextureFormat,
    pub blend_state: Option<BlendState>,
    pub samples: u32,
    pub input_array: bool,
    pub multiview: Option<NonZeroU32>,
    pub view_override: Option<u32>,
}

impl SpecializedRenderPipeline for BlitPipeline {
    type Key = BlitPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut shader_defs = vec![];
        let texture_bind_group = if key.input_array {
            shader_defs.push("INPUT_ARRAY".into());
            self.array_texture_bind_group.clone()
        } else {
            self.texture_bind_group.clone()
        };

        if key.multiview.is_some() {
            shader_defs.push("MULTIVIEW".into());
        }

        if let Some(view_override) = key.view_override {
            shader_defs.push(ShaderDefVal::UInt("VIEW_OVERRIDE".into(), view_override))
        }

        RenderPipelineDescriptor {
            label: Some("blit pipeline".into()),
            layout: vec![texture_bind_group],
            vertex: fullscreen_shader_vertex_state(),
            fragment: Some(FragmentState {
                shader: BLIT_SHADER_HANDLE,
                shader_defs,
                entry_point: "fs_main".into(),
                targets: vec![Some(ColorTargetState {
                    format: key.texture_format,
                    blend: key.blend_state,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState {
                count: key.samples,
                ..Default::default()
            },
            push_constant_ranges: Vec::new(),
            multiview: key.multiview,
        }
    }
}
