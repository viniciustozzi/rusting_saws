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
        .add_startup_system(setup_camera)
        .add_startup_system(setup_saw_spawning)
        .add_startup_system(setup_player)
        .add_system(spawn_saw_system)
        .add_system(saw_rotate_system)
        .add_system(saw_move_system)
        .run();
}

#[derive(Component)]
struct Saw {
    dir: Vec2,
}

impl Saw {
    fn new() -> Saw {
        if rand::random() {
            Saw {
                dir: Vec2::new(random_dir(), random_dir()),
            }
        } else {
            Saw {
                dir: Vec2::new(random_dir(), random_dir()),
            }
        }
    }
}

#[derive(Component)]
struct Player {
    health: i32,
    dir: Vec2,
}

fn random_dir() -> f32 {
    return thread_rng().gen_range(-1.0..1.0);
}

struct SawSpawnConfig {
    timer: Timer,
}

fn setup_camera(mut commands: Commands, windows: ResMut<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_player(mut commands: Commands) {
    let circle_shape = shapes::Circle {
        radius: 5.0,
        center: Vec2::ZERO,
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &circle_shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::GREEN),
                outline_mode: StrokeMode::new(Color::BLUE, 0.5),
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .insert(Player {
            health: 3,
            dir: Vec2::new(0.0, 0.0),
        });
}

fn player_movment(mut player: Query<(&mut Transform, &Player)>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::A) {
        for (mut t, p) in player.iter_mut() {
        }
    }
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

fn saw_rotate_system(mut geometry: Query<(&mut Transform, &Saw)>) {
    for (mut t, _s) in geometry.iter_mut() {
        t.rotation *= Quat::from_rotation_z(3.0);
    }
}

fn saw_move_system(mut geometry: Query<(&mut Transform, &Saw)>) {
    for (mut t, s) in geometry.iter_mut() {
        t.translation.x += 2.0 * s.dir.x;
        t.translation.y += 2.0 * s.dir.y;
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
                outline_mode: StrokeMode::new(Color::RED, 6.0),
            },
            Transform::from_xyz(x, y, 0.0),
        ))
        .insert(Saw::new());
}
