use bevy::app::AppExit;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bouncing Box".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        })) // disp window
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .add_systems(Update, exit_on_escape)
        .run();
}

#[derive(Component)]
struct Velocity(Vec2);

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.7, 0.9),
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Velocity(Vec2::new(150.0, 100.0)),
    ));
}

fn update(
    time: Res<Time>,
    windows: Query<&Window>,
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    let window = match windows.single() {
        Ok(w) => w,
        Err(_) => return,
    };

    let half_width = window.resolution.width() / 2.0;
    let half_height = window.resolution.height() / 2.0;

    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();

        let object_size = 32.0;
        if transform.translation.x + object_size > half_width
            || transform.translation.x - object_size < -half_width
        {
            velocity.0.x *= -1.0;
        }
        if transform.translation.y + object_size > half_height
            || transform.translation.y - object_size < -half_height
        {
            velocity.0.y *= -1.0;
        }
    }
}

fn exit_on_escape(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
