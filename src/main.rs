use bevy::{prelude::*, window::PresentMode};
use constants::{N_CELLS, N_TURNS};
use entities::entity::Entity as SimulationEntity;
use population::Population;

mod constants;
mod entities;
mod population;

#[derive(Resource)]
struct HistoryResource {
    history_points: Vec<Population>,
    current_point: usize,
}

fn main() {
    let mut history = vec![];
    (0..N_TURNS).fold(Population::new(), |pop, idx| {
        history.push(pop.clone());
        println!("{idx}");
        pop.advance()
    });

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(HistoryResource {
            history_points: history,
            current_point: 0,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Symulacja choroby".into(),
                resolution: (1300., 800.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(redraw_entities)
        .add_system(next_in_history_on_space)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn get_entity_color(entity: &SimulationEntity) -> Color {
    match entity.health {
        entities::health::HealthState::Infected { days_until_sick: _ } => Color::YELLOW,
        entities::health::HealthState::Sick {
            days_until_recovering: _,
        } => Color::RED,
        entities::health::HealthState::Recovering {
            days_until_healthy: _,
        } => Color::ORANGE,
        entities::health::HealthState::Healthy => Color::GREEN,
    }
}
fn redraw_entities(
    mut commands: Commands,
    history: Res<HistoryResource>,
    query: Query<Entity, With<Sprite>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    let current_pointer: usize = history.current_point;
    let rect_size = 7.;
    for entity in &history.history_points[current_pointer].entities {
        let pos = entity.position;
        draw_rect(
            &mut commands,
            pos.x as f32,
            pos.y as f32,
            get_entity_color(entity),
            rect_size,
        );
    }
}

fn draw_rect(commands: &mut Commands, x: f32, y: f32, color: Color, side_size: f32) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(side_size, side_size)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            (x - N_CELLS as f32 / 2.) * side_size,
            (y - N_CELLS as f32 / 2.) * side_size,
            1.,
        )),
        ..default()
    });
}

fn next_in_history_on_space(mut history_query: ResMut<HistoryResource>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Left) {
        if history_query.current_point == 0 {
            return;
        }
        history_query.current_point -= 1;
    }

    if keys.just_pressed(KeyCode::Right)
        && history_query.current_point < history_query.history_points.len() - 1
    {
        history_query.current_point += 1;
    }
}
