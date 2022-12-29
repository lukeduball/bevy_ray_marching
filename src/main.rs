use bevy::{prelude::*, sprite::{MaterialMesh2dBundle}, window::WindowResized, input::mouse::MouseMotion};
use bevy_inspector_egui::WorldInspectorPlugin;

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
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(RayMarchingMaterialPlugin)
    //Create the aspect ratio as a resource. Only one instance of this data is needed so a global resource was chosen
    .init_resource::<AspectRatio>()

    .add_startup_system(setup)
    .add_system(resize_event)
    .add_system(process_camera_translation)
    .add_system(process_camera_rotation);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RayMarchingMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(ScreenSpaceQuad::default())).into(),
        material: materials.add(RayMarchingMaterial::new()),
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

fn process_camera_translation(
    keys: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>, 
) {
    const SPEED: f32 = 1.0;
    for mut transform in camera_query.iter_mut() {
        let forward_vector = transform.forward();
        let horizontal_vector = transform.right();
        let vertical_vector = transform.up();
        if keys.pressed(KeyCode::W) {
            transform.translation += forward_vector * SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::S) {
            transform.translation -= forward_vector * SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::A) {
            transform.translation -= horizontal_vector * SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::D) {
            transform.translation += horizontal_vector * SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::R) {
            transform.translation += vertical_vector * SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::F) {
            transform.translation -= vertical_vector * SPEED * time.delta_seconds();
        }
    }
}

fn process_camera_rotation(
    mut motion_event: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>
) {
    for event in motion_event.iter() {
        const ROTATION_SPEED: f32 = 0.1;
        if mouse_buttons.pressed(MouseButton::Right) {
            for mut transform in camera_query.iter_mut() {
                transform.rotate_local_x(-event.delta.y * ROTATION_SPEED * time.delta_seconds());
                transform.rotate_local_y(-event.delta.x * ROTATION_SPEED * time.delta_seconds());
            }
        }
    }
}

