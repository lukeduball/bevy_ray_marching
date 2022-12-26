use bevy::{prelude::*, render::{render_resource::{AsBindGroup, ShaderRef, ShaderType, OwnedBindingResource, encase}, Extract, renderer::RenderQueue, RenderApp, RenderStage}, reflect::TypeUuid, sprite::{Material2d, RenderMaterials2d, Material2dPlugin}};

use crate::AspectRatio;

pub struct RayMarchingMaterialPlugin;

impl Plugin for RayMarchingMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<RayMarchingMaterial>::default());

        //Add our custom extract and prepare systems to the app
        app.sub_app_mut(RenderApp)
        .add_system_to_stage(RenderStage::Extract, extract_raymarching_material)
        .add_system_to_stage(RenderStage::Prepare, prepare_raymarching_material);
    }
}

//New material created to setup custom shader
#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "084f230a-b958-4fc4-8aaf-ca4d4eb16412"]
pub struct RayMarchingMaterial {
    //Set the uniform at binding 0 to have the following information - connects to Camera struct in ray_marching_material.wgsl
    #[uniform(0)]
    pub position: Vec4,
    #[uniform(0)]
    pub aspect_ratio: f32,
}

//Setup the RayMarchingMaterial to use the custom shader file for the vertex and fragment shader
//Note: one of these can be removed to use the default material 2D bevy shaders for the vertex/fragment shader
impl Material2d for RayMarchingMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/ray_marching_material.wgsl".into()    
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/ray_marching_material.wgsl".into()
    }
}

//Uniform data struct to move data from the "Game World" to the "Render World" with the ShaderType derived
#[derive(ShaderType, Clone)]
struct RayMarchingMaterialUniformData {
    apsect_ratio: f32,
}

//Move information from the "Game World" to the "Render World"
fn extract_raymarching_material(
    mut commands: Commands,
    ray_marching_query: Extract<Query<(Entity, &Handle<RayMarchingMaterial>)>>,
    aspect_ratio_resource: Extract<Res<AspectRatio>>
) {
    for (entity, material_handle) in ray_marching_query.iter() {
        commands.get_or_spawn(entity)
            .insert(material_handle.clone());
    }

    commands.insert_resource(AspectRatio {
        aspect_ratio: aspect_ratio_resource.aspect_ratio,
    });
}

//Update the buffers with the data taken from the "Game World" and sent to the "Render World" so they can be used by the GPU
fn prepare_raymarching_material(
    materials: Res<RenderMaterials2d<RayMarchingMaterial>>,
    material_query: Query<&Handle<RayMarchingMaterial>>,
    render_queue: Res<RenderQueue>,
    aspect_ratio_resource: Res<AspectRatio>
) {
    for material_handle in &material_query {
        if let Some(material) = materials.get(material_handle) {
            for binding in material.bindings.iter() {
                if let OwnedBindingResource::Buffer(current_buffer) = binding {
                    let mut buffer = encase::UniformBuffer::new(Vec::new());
                    buffer.write(&RayMarchingMaterialUniformData {
                        apsect_ratio: aspect_ratio_resource.aspect_ratio,
                    }).unwrap();
                    //Write to an offset in the buffer so the position data is not over-written
                    render_queue.write_buffer(current_buffer, 16, buffer.as_ref());
                }
            }
        }
    }
}