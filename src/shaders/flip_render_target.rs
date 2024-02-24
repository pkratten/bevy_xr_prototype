use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_graph::{self, RenderGraph, RenderLabel},
        render_resource::{
            AsBindGroup, BindGroupLayout, BindGroupLayoutEntries, CachedComputePipelineId,
            PipelineCache, ShaderStages,
        },
        renderer::{RenderContext, RenderDevice},
        Render, RenderApp, RenderSet,
    },
    utils::{HashMap, HashSet},
};

pub struct FlipRenderTargetPlugin;

#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct FlipRenderTargets {
    render_targets: HashMap<RenderTarget, FlipDirection>,
}

pub enum FlipDirection {
    X,
    Y,
    XY,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
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
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            prepare_bind_group.in_set(RenderSet::PrepareBindGroups),
        );

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node(FlipRenderTargetLabel, FlipRenderTargetNode::default());
        render_graph.add_node_edge(
            bevy::render::graph::CameraDriverLabel,
            FlipRenderTargetLabel,
        );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<FlipRenderTargetPipeline>();
    }
}

#[derive(Resource)]
struct FlipRenderTargetPipeline {
    texture_bind_group_layout: BindGroupLayout,
    pipeline: CachedComputePipelineId,
}

#[derive(Resource, Clone, Deref, ExtractResource, AsBindGroup)]
struct FlipRenderTargetBindGroup {
    #[storage_texture(0, image_format = Rgba8Unorm, access = ReadWrite)]
    texture: Handle<Image>,
}

impl FromWorld for FlipRenderTargetPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let texture_bind_group_layout = render_device.create_bind_group_layout(
            "post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::COMPUTE,
                (
                    // The screen texture
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    // The sampler that will be used to sample the screen texture
                    sampler(SamplerBindingType::Filtering),
                    // The settings uniform that will control the effect
                    uniform_buffer::<PostProcessSettings>(false),
                ),
            ),
        );
        let shader = FLIP_RENDERTARGET_SHADER_HANDLE;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("compute"),
        });

        GameOfLifePipeline {
            texture_bind_group_layout,
            pipeline,
        }
    }
}

struct FlipRenderTargetNode;

impl render_graph::Node for FlipRenderTargetNode {
    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<FlipRenderTargetPipeline>();

        Ok(())
    }
}
