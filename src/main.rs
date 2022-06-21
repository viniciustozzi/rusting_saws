use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::thread_rng;
use rand::Rng;
use std::time::Duration;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_startup_system(setup_saw_spawning)
        .add_system(spawn_saw_system)
        .add_system(rotate_system)
        .add_system(move_system)
        .run();
}

#[derive(Component)]
struct Geometry {
    direction: Vec2,
}

impl Geometry {
    fn new() -> Geometry {
        if rand::random() {
            Geometry {
                direction: Vec2::new(random_dir(), random_dir()),
            }
        } else {
            Geometry {
                direction: Vec2::new(random_dir(), random_dir()),
            }
        }
    }
}
fn random_dir() -> f32 {
    return thread_rng().gen_range(-1.0..1.0);
}

struct SawSpawnConfig {
    timer: Timer,
}

fn setup_system(mut commands: Commands, windows: ResMut<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    spawn_saw(commands, windows);
}

fn setup_saw_spawning(mut commands: Commands) {
    commands.insert_resource(SawSpawnConfig {
        timer: Timer::new(Duration::from_secs_f32(0.5), true),
    })
}

fn spawn_saw_system(
    commands: Commands,
    time: Res<Time>,
    mut config: ResMut<SawSpawnConfig>,
    windows: ResMut<Windows>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        spawn_saw(commands, windows);
    }
}

fn rotate_system(mut geometry: Query<(&mut Transform, &Geometry)>) {
    for (mut t, _g) in geometry.iter_mut() {
        t.rotation *= Quat::from_rotation_z(3.0);
    }
}

fn move_system(mut geometry: Query<(&mut Transform, &Geometry)>) {
    for (mut t, g) in geometry.iter_mut() {
        t.translation.x += 2.0 * g.direction.x;
        t.translation.y += 2.0 * g.direction.y;
    }
}

fn spawn_saw(mut commands: Commands, mut windows: ResMut<Windows>) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(60.0),
        ..shapes::RegularPolygon::default()
    };

    let window = windows.get_primary_mut().unwrap();
    let x = thread_rng().gen_range(-window.width()..window.width());
    let y = thread_rng().gen_range(-window.height()..window.height());

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::BLUE),
                outline_mode: StrokeMode::new(Color::BLACK, 6.0),
            },
            Transform::from_xyz(x, y, 0.0),
        ))
        .insert(Geometry::new());
}
