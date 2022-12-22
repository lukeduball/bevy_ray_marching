use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Material2d, Material2dPlugin, RenderMaterials2d}, render::{render_resource::{AsBindGroup, ShaderRef, PrimitiveTopology, OwnedBindingResource, ShaderType, encase}, mesh::Indices, renderer::RenderQueue, Extract, RenderStage, RenderApp}, reflect::TypeUuid, window::WindowResized};

pub const WIDTH: f32 = 720.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Ray Marching Scene".to_string(),
            resizable: true,
            ..default()
        },
        ..default()
    }))
    .add_plugin(Material2dPlugin::<RayMarchingMaterial>::default())
    //Create the aspect ratio as a resource. Only one instance of this data is needed so a global resource was chosen
    .init_resource::<AspectRatio>()

    .add_startup_system(setup)
    .add_system(resize_event);

    //Add our custom extract and prepare systems to the app
    app.sub_app_mut(RenderApp)
        .add_system_to_stage(RenderStage::Extract, extract_raymarching_material)
        .add_system_to_stage(RenderStage::Prepare, prepare_raymarching_material);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RayMarchingMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(ScreenSpaceQuad::default())).into(),
        material: materials.add(RayMarchingMaterial { position: Vec4::new(0.0, 0.0, -5.0, 1.0), aspect_ratio: WIDTH / HEIGHT}),
        ..default()
    });
}

//Struct which becomes the Global Resource for the aspect ratio
#[derive(Default, Resource)]
pub struct AspectRatio {
    aspect_ratio: f32,
}

//Handle a window resize event to set the AspectRatio so it can be updated in the uniform that is sent to our shader
fn resize_event( 
    mut resize_reader: EventReader<WindowResized>,
    mut aspect_ratio_resource: ResMut<AspectRatio>,
) {
    for event in resize_reader.iter() {
        aspect_ratio_resource.aspect_ratio = event.width / event.height;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ScreenSpaceQuad {
    //Scale of the quad in screen space with 1,1 taking the whole screen
    pub scale: Vec2,
}

impl Default for ScreenSpaceQuad {
    fn default() -> Self {
        Self::new(Vec2::ONE)
    }
}

impl ScreenSpaceQuad {
    fn new(scale: Vec2) -> Self {
        Self { scale }
    }
}

//Creating the custom vertex data for our ScreenSpaceQuad
//Note: the normal and uv attribute had to be included for this to work. Seems to be some Bevy limitation
impl From<ScreenSpaceQuad> for Mesh {
    fn from(screen_space_quad: ScreenSpaceQuad) -> Self {
        let vertices = vec![[-1.0*screen_space_quad.scale.x, -1.0*screen_space_quad.scale.y, 0.0],
                            [-1.0*screen_space_quad.scale.x,  1.0*screen_space_quad.scale.y, 0.0],
                            [ 1.0*screen_space_quad.scale.x, -1.0*screen_space_quad.scale.y, 0.0],
                            [ 1.0*screen_space_quad.scale.x,  1.0*screen_space_quad.scale.y, 0.0]];

        let indices = Indices::U32(vec![0, 2, 1, 2, 3, 1]);

        let normals = vec![[0.0, 0.0, 1.0],
                            [0.0, 0.0, 1.0],
                            [0.0, 0.0, 1.0],
                            [0.0, 0.0, 1.0]];

        let uvs = vec![[0.0, 0.0],
                        [0.0, 1.0],
                        [1.0, 0.0],
                        [1.0,1.0]];

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

//New material created to setup custom shader
#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "084f230a-b958-4fc4-8aaf-ca4d4eb16412"]
pub struct RayMarchingMaterial {
    //Set the uniform at binding 0 to have the following information - connects to Camera struct in ray_marching_material.wgsl
    #[uniform(0)]
    position: Vec4,
    #[uniform(0)]
    aspect_ratio: f32,
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

