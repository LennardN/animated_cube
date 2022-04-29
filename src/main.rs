use bevy::prelude::*;

const SPEED: f32 = 0.5;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sin_move)
        .run();
}

#[derive(Component)]
struct Column{
    delay: f32
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // set up the camera
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 5.0;
    camera.transform = Transform::from_xyz(-10.0, 8., -10.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y);

    // camera
    commands.spawn_bundle(camera);
    

    // cubes
    for i in 0..5{
        for j in 0..5{
            commands.spawn()
                .insert(Column{
                    delay: i as f32
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
                        translation: (Vec3::new(-2.5 + i as f32, 2.0 + (j as f32 / 5.), -2.5 + j as f32)), 
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
        transform: Transform::from_xyz(-3.0, 8.0, -5.0),
        ..default()
    });
}
fn sin_move(time: Res<Time>, mut cubes: Query<(&Column, &mut Transform)>){
    for  (nr, mut cube) in cubes.iter_mut(){
        cube.scale.y = f32::sin((time.seconds_since_startup() as f32 * SPEED) + nr.delay/2.) + 1.5;
    }
     //multiply by iteration; iteration in 
    //println!("{}", column_transform.scale.y);
}
