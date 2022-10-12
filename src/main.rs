use std::f32::consts::PI;
use std::fs;
use bevy::prelude::*;
use bevy_editor_pls::*;
use std::io::Cursor;
use byteorder::*;

const SPEED: f32 = 0.5;
const AMP: f32 = 2.;
const MIN_AMP: f32 = 1.; //f32 to prevent to many annoying casts
const COLUMN: f32 = 25.;
const ROW: f32 = 25.;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup)
        .add_system(wave_animation)
        .run();
}

#[derive(Component)]
struct Column{
    delay_x: f32,
    delay_y: f32
}

struct RiffFileHandler{
    riff: Vec<u8>,
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = f32::sqrt(COLUMN as f32) * 2.2;
    camera.transform = Transform::from_xyz(-10.0, 12., -10.0).looking_at(Vec3::new(0.0, 3.0, 0.0), Vec3::Y);

    // camera
    commands.spawn_bundle(camera);



    let file = fs::read("./assets/habstraktVibeMono.wav").expect("no file found");  
    //let mut rdr = Cursor::new(file);
    //rdr.set_position(44);
    commands.insert_resource(RiffFileHandler{
        riff: file,
    });


    // cubes
    for i in 0..ROW as i32{
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
                        scale: (Vec3::new(1.0, 1.0, 1.0)),
                        ..Default::default()
                        },
                    ..Default::default()
                    });
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
        transform: Transform::from_xyz(COLUMN * -0.5, 12.0, ROW * -0.75),
        point_light: PointLight {
            intensity: 200.0 * ROW, 
            shadows_enabled: false,
            range: 100.,
            ..default()
        },
        ..default()
    });
}

fn wave_animation(time: Res<Time>, file: Res<RiffFileHandler>, mut cubes: Query<(&Column, &mut Transform)>){
    let x: f32 = time.seconds_since_startup() as f32;

    for  (id, mut cube) in cubes.iter_mut(){
            cube.scale.y = MIN_AMP + AMP + AMP * iterate_db_audio( //sine-func => iterate_Db_Audio()
                SPEED * PI * 2. * x + f32::sqrt(
                    f32::powi(id.delay_x - ((COLUMN-1.)/2.), 2) + f32::powi(id.delay_y - ((ROW-1.)/2.), 2)), &file.riff);
                    /*cube.scale.y = MIN_AMP + AMP + AMP * f32::sin(SPEED * PI * -2. * x + f32::sqrt(f32::powi(id.delay_x - ((COLUMN-1.)/2.), 2) + f32::powi(id.delay_y - ((ROW-1.)/2.), 2))); */
    }
}

fn iterate_db_audio(sec: f32, file: &Vec<u8>) -> f32{
    let mut rdr = Cursor::new(file);
    //println!("{sec}");
    rdr.set_position((sec) as u64 * 44100);
    println!("{sec}");
    
    return (rdr.read_i16::<LittleEndian>().unwrap() as f32) / i16::MAX as f32;
}
