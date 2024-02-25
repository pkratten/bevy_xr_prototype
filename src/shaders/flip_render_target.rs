use std::borrow::Cow;
use std::default;

use bevy::core_pipeline::core_3d;
use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::ecs::query::QueryItem;
use bevy::render::camera::ExtractedCamera;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_graph::{RenderGraphApp, ViewNode, ViewNodeRunner};
use bevy::render::render_resource::{
    BindGroupEntries, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState,
    MultisampleState, Operations, PipelineCache, PrimitiveState, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor,
    ShaderStages, ShaderType, TextureFormat, TextureSampleType, TextureViewDimension,
    UniformBuffer,
};
use bevy::render::texture::BevyDefault;
use bevy::render::view::ViewTarget;
use bevy::ui::draw_ui_graph;
use bevy::{
    asset::load_internal_asset,
    prelude::*,
    render::{
        camera::{ManualTextureViews, NormalizedRenderTarget},
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph},
        renderer::{RenderContext, RenderDevice},
        view::ExtractedWindows,
        RenderApp,
    },
    utils::HashMap,
};

pub struct FlipRenderTargetPlugin;

#[derive(Default, Resource, Clone, Deref, DerefMut, ExtractResource)]
pub struct FlipRenderTargets(HashMap<NormalizedRenderTarget, FlipDirection>);

#[derive(Clone)]
pub enum FlipDirection {
    X,
    Y,
    XY,
}

impl ExtractComponent for FlipDirection{
    
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)] //RenderLabel)]
struct FlipRenderTargetLabel;

const FLIP_RENDERTARGET_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(9837534426033940724);

impl Plugin for FlipRenderTargetPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            FLIP_RENDERTARGET_SHADER_HANDLE,
            "flip_render_target.wgsl",
            Shader::from_wgsl
        );

        // Extract the game of life image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.
        app.add_plugins(ExtractResourcePlugin::<FlipRenderTargets>::default());
        app.init_resource::<FlipRenderTargets>();

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<FlipRenderTargetNode>>(
                // Specify the name of the graph, in this case we want the graph for 3d
                core_3d::graph::NAME,
                // It also needs the name of the node
                "FlipRenderTargets",
            )
            .add_render_graph_edges(
                core_3d::graph::NAME,
                // Specify the node ordering.
                // This will automatically create all required node edges to enforce the given ordering.
                &[
                    draw_ui_graph::node::UI_PASS,
                    "FlipRenderTargets",
                    core_3d::graph::node::UPSCALING,
                ],
            );

        // render_app
        //     .add_render_graph_node::<ViewNodeRunner<FlipRenderTargetNode>>(
        //         // Specify the label of the graph, in this case we want the graph for 3d
        //         Core3d,
        //         // It also needs the label of the node
        //         FlipRenderTargetLabel,
        //     )
        //     .add_render_graph_edges(
        //         Core3d,
        //         // Specify the node ordering.
        //         // This will automatically create all required node edges to enforce the given ordering.
        //         (Node3d::Upscaling, FlipRenderTargetLabel),
        //     );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<FlipRenderTargetPipeline>();
    }
}

#[derive(Resource)]
struct FlipRenderTargetPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for FlipRenderTargetPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        // let layout = render_device.create_bind_group_layout(
        //     "flip_render_target_bind_group_layout",
        //     &BindGroupLayoutEntries::sequential(
        //         // The layout entries will only be visible in the fragment stage
        //         ShaderStages::FRAGMENT,
        //         (
        //             // The render target texture
        //             texture_2d(TextureSampleType::Float { filterable: true }),
        //             // The sampler that will be used to sample the screen texture
        //             sampler(SamplerBindingType::Filtering),
        //             // The settings uniform that will control the effect
        //             uniform_buffer::<Vec4>(false),
        //         ),
        //     ),
        // );

        let layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("post_process_bind_group_layout"),
            entries: &[
                // The screen texture
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // The sampler that will be used to sample the screen texture
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                // The settings uniform that will control the effect
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: bevy::render::render_resource::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(Vec4::min_size()),
                    },
                    count: None,
                },
            ],
        });

        // We can create the sampler here since it won't change at runtime and doesn't depend on the view
        let sampler = render_device.create_sampler(&SamplerDescriptor {
            address_mode_u: bevy::render::render_resource::AddressMode::Repeat,
            address_mode_v: bevy::render::render_resource::AddressMode::Repeat,
            ..default()
        });

        let shader = FLIP_RENDERTARGET_SHADER_HANDLE;
        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("post_process_pipeline".into()),
                layout: vec![layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        FlipRenderTargetPipeline {
            layout,
            sampler,
            pipeline_id,
        }
    }
}

// This is the component that will get passed to the shader
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
struct FlipDirectionBuffer {
    intensity: Vec2,
    // WebGL2 structs must be 16 byte aligned.
    #[cfg(feature = "webgl2")]
    _webgl2_padding: Vec2,
}

#[derive(Default)]
struct FlipRenderTargetNode {}

impl ViewNode for FlipRenderTargetNode {
    type ViewQuery = (&'static ViewTarget, &'static ExtractedCamera);

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, camera): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let render_targets = world.resource::<FlipRenderTargets>();
        if render_targets.is_empty() {
            return Ok(());
        }

        // The pipeline cache is a cache of all previously created pipelines.
        // It is required to avoid creating a new pipeline each frame,
        // which is expensive due to shader compilation.
        let pipeline_cache = world.resource::<PipelineCache>();

        // Get the pipeline resource that contains the global data we need
        // to create the render pipeline
        let flip_render_target_pipeline = world.resource::<FlipRenderTargetPipeline>();

        // Get the pipeline from the cache
        let Some(pipeline) =
            pipeline_cache.get_render_pipeline(flip_render_target_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let Some(flip) = world
            .resource::<FlipRenderTargets>()
            .iter()
            .find(|(render_target, _)| Some(*render_target) == camera.target.as_ref())
            .map(|(_, flip)| flip.to_owned())
        else {
            return Ok(());
        };

        info!("flipping?");

        let flip = match flip {
            FlipDirection::X => Vec4::new(-1.0, 1.0, 1.0, 1.0),
            FlipDirection::Y => Vec4::new(1.0, -1.0, 1.0, 1.0),
            FlipDirection::XY => Vec4::new(-1.0, -1.0, 1.0, 1.0),
        };
        let flip = UniformBuffer::from(flip);
        flip.write_buffer(device, queue)

        let postprocess = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            "flip_render_target_bind_group",
            &flip_render_target_pipeline.layout,
            // It's important for this to match the BindGroupLayout defined in the PostProcessPipeline
            &BindGroupEntries::sequential((
                postprocess.source,
                &flip_render_target_pipeline.sampler,
                flip,
            )),
        );

        // Begin the render pass
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                // We need to specify the post process destination view here
                // to make sure we write to the appropriate texture.
                view: &postprocess.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            //timestamp_writes: None,
            //occlusion_query_set: None,
        });

        // This is mostly just wgpu boilerplate for drawing a fullscreen triangle,
        // using the pipeline/bind_group created above
        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        info!("flipped?");

        Ok(())
    }
}
