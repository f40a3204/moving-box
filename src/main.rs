use bevy::prelude::*;

const PLAYER_SPRITE: &str = "squore.png";
const PLAYER_SIZE: (f32, f32) = (25., 25.);
const SPRITE_SCALE: f32 = 1.;
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

#[derive(Resource, Clone)]
struct GameTextures {
    player: Handle<Image>,
}

#[derive(Resource)]
struct EnemyCount(u32);

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Movable;

#[derive(Component)]
struct SpriteSize(Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

#[derive(Component)]
pub struct Player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "your mother".into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_startup_system(setup_system)
        .add_system(player_keyboard_event_system)
        .add_system(movable_system)
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
    };
    commands.insert_resource(game_textures.clone());
    commands.insert_resource(EnemyCount(0));
    commands
        .spawn(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(SpriteSize::from(PLAYER_SIZE))
        .insert(Movable)
        .insert(Velocity { x: 0., y: 0. });
}

fn movable_system(mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>) {
    for (_entity, velocity, mut transform, _movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };
        velocity.y = if kb.pressed(KeyCode::Down) {
            -1.
        } else if kb.pressed(KeyCode::Up) {
            1.
        } else {
            0.
        }
    }
}
