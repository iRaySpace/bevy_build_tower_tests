use bevy::{input::system::exit_on_esc_system, prelude::*};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component, Debug)]
struct GridMarker;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(15.0),
                    bottom: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Tower",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 36.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    TextAlignment {
                        ..Default::default()
                    },
                ),
                ..Default::default()
            });
        });
}

fn btn_system(
    mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (interaction, mut color, _children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                commands
                    .spawn_bundle(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, 0.0),
                            scale: Vec3::new(30.0, 30.0, 0.0),
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: Color::rgb(0.47, 0.87, 0.47),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(GridMarker);
            }
        }
    }
}

fn grid_marker_mouse_system(
    windows: Res<Windows>,
    mut query: Query<(&GridMarker, &mut Transform)>,
) {
    let win = windows.get_primary().expect("no primary window");
    let size = Vec2::new(win.width() as f32, win.height() as f32);
    let default_orthographic_pos = size / 2.0;
    let mouse_pos = win.cursor_position();
    match mouse_pos {
        None => {}
        Some(vec2) => {
            let result = query.get_single_mut();
            match result {
                Result::Err(..) => {}
                Result::Ok((_grid_marker, mut transform)) => {
                    let world_pos = vec2 - default_orthographic_pos;
                    let grid_x = (world_pos.x / 30.0).floor() * 30.0;
                    let grid_y = (world_pos.y / 30.0).floor() * 30.0;
                    transform.translation.x = grid_x;
                    transform.translation.y = grid_y;
                }
            }
        }
    }
}

fn build_mouse_system(
    mouse_system: Res<Input<MouseButton>>,
    query: Query<(Entity, &Transform, &GridMarker)>,
    mut commands: Commands,
) {
    if mouse_system.just_pressed(MouseButton::Right) {
        let result = query.get_single();
        match result {
            Result::Err(..) => {}
            Result::Ok((entity, transform, _grid_marker)) => {
                commands.entity(entity).despawn();
                commands
                    .spawn_bundle(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(transform.translation.x, transform.translation.y, 0.0),
                            scale: Vec3::new(30.0, 30.0, 0.0),
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: Color::rgb(0.95, 0.95, 0.95),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_system(btn_system)
        .add_system(grid_marker_mouse_system)
        .add_system(build_mouse_system)
        .add_system(exit_on_esc_system)
        .run();
}
