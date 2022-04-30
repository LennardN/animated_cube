use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_editor_pls::*;

const SPEED: f32 = 0.5;
const AMP: f32 = 2.;
const MIN_AMP: f32 = 1.;
const COLUMN: f32 = 15.;
const ROW: f32 = 15.;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup)
        .add_system(sin_move)
        .run();
}

#[derive(Component)]
struct Column{
    delay_x: f32,
    delay_y: f32
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // set up the camera
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = f32::sqrt(COLUMN as f32) * 2.2;
    camera.transform = Transform::from_xyz(-10.0, 12., -10.0).looking_at(Vec3::new(0.0, 3.0, 0.0), Vec3::Y);

    // camera
    commands.spawn_bundle(camera);
    

    // cubes
    
    for i in 0..ROW as i32{
        println!("{}", i);
        for j in 0..COLUMN as i32{
            commands.spawn()
                .insert(Column{
                    delay_x: j as f32,
                    delay_y: i as f32
                })
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box {
                        min_x: -0.5,
                        min_y: -0.5,
                        min_z: -0.5,
                        max_x: 0.5,
                        max_y: 0.5,
                        max_z: 0.5
                    })),
                    material: materials.add(Color::rgb(1.,0.3, 0.).into()),
                    transform: Transform { 
                        translation: (Vec3::new((ROW/-2.) + i as f32, 3.0, (COLUMN/-2.) + j as f32)), 
                        scale: (Vec3::new(0.8, 1.0, 0.8)),
                        ..Default::default()
                        },
                    ..Default::default()
                    });
                    //Transform::from_xyz(-2.5 + (j as f32), 1.5, -2.5 + (i as f32))
        }
    }

    //plane
    commands.spawn_bundle(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 100.
        })),
        material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
        transform: Transform::from_xyz(5., 0., 5.),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        // transform: Transform::from_xyz(5.0, 8.0, 2.0),
        transform: Transform::from_xyz(COLUMN as f32 * -0.5, 12.0, ROW as f32 * -0.75),
        point_light: PointLight {
            intensity: 200.0 * ROW, 
            shadows_enabled: false,
            range: 100.,
            ..default()
        },
        ..default()
    });
}

fn sin_move(time: Res<Time>, _input: Res<Input<KeyCode>>, mut cubes: Query<(&Column, &mut Transform)>){
    let x: f32 = time.seconds_since_startup() as f32;
    for  (id, mut cube) in cubes.iter_mut(){
            cube.scale.y = MIN_AMP + AMP + AMP * f32::sin(
                SPEED * PI * 2. * x + f32::sqrt(
                    f32::powi(id.delay_x - ((COLUMN-1.)/2.), 2) + f32::powi(id.delay_y - ((ROW-1.)/2.), 2)));
                    //f32::powi(id.delay_x - (COLUMN/2.), 2) + f32::powi(id.delay_y - (ROW/2.), 2)));
    }
}