use bevy::{prelude::*, sprite::{MaterialMesh2dBundle}, window::WindowResized};

mod screen_space_quad;
use crate::screen_space_quad::ScreenSpaceQuad;

mod ray_marching_material;
use crate::ray_marching_material::{RayMarchingMaterial, RayMarchingMaterialPlugin};

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
    .add_plugin(RayMarchingMaterialPlugin)
    //Create the aspect ratio as a resource. Only one instance of this data is needed so a global resource was chosen
    .init_resource::<AspectRatio>()

    .add_startup_system(setup)
    .add_system(resize_event);

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

