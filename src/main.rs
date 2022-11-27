use bevy::{prelude::*,diagnostic::{FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin,}, core::Zeroable};
use bevy_editor_pls::*;
use rand::Rng;

pub const WIDTH:f32 = 720.0;
pub const HEIGHT:f32 = 1280.0;

// амплитуда
pub const AMPLITUDE:f32 = 8.0;
// частота
pub const FREQUENCE: f32 = 0.2;

fn main() {
    App::new().add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(Color::rgb(0.2,0.2,0.2)))
    .insert_resource(WindowDescriptor{
        width: WIDTH,
        height: HEIGHT,
        title: "MyGame".to_string(),
        resizable: false,
        ..default()
    })
    .add_plugin(EditorPlugin)
    .add_plugin(FrameTimeDiagnosticsPlugin)
    .add_plugin(EntityCountDiagnosticsPlugin)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_basic_scene)
    .run();
}

fn spawn_camera(mut commands: Commands){
    commands.spawn_bundle(Camera3dBundle{
        transform: Transform::from_xyz(-2.0,2.5,5.0).looking_at(Vec3::ZERO,Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){

    for x in 0..70{
        for z in 0..70{
            let x: f32 = x as f32;
            let z: f32 = z as f32;
            let FREQUENC = (rand::thread_rng().gen_range(16, 20) as f32)/100.0;
            let AMP = (rand::thread_rng().gen_range(8, 10) as f32)/10.0;
            
            // good terrain
            let y: f32 = ((x*FREQUENC).sin() * AMP).round()+ 
                ((z*FREQUENC).sin()*AMP).round();


            commands.spawn_bundle(PbrBundle{
                mesh: meshes.add(Mesh::from(shape::Cube{size:1.0})),
                material: materials.add(Color::rgb(0.67,0.84,0.92).into()),
                transform: Transform::from_xyz(x, y, z).with_scale(Vec3::new(1.0,1.0,1.0)),
                ..default()
            });
    }}
}