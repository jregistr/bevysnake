use bevy::{core::FixedTimestep, prelude::*};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup::setup.system());
        app.add_startup_stage("snake_setup", SystemStage::single(setup::spawn_snake.system()));
        // app.add_system(snake_movement.system());
        app.add_system_set_to_stage(
            CoreStage::PostUpdate, 
            SystemSet::new()
            .with_system(grid_size_to_sprite_size.system())
            .with_system(grid_position_to_translation.system())
        );

        app.add_system(
            snake_input.system()
            .label(SnakeMovement::Input)
            .before(SnakeMovement::Movement)
        );

        app.add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.25))
            .with_system(
                snake_fixed_movement.system()
                .label(SnakeMovement::Movement)
            )
        );
    }
}

const ARENA_GRID_SIZE: f32 = 10.;

// Component defs
struct SnakeHead {
    dir: SnakeDirection
}
struct SnakeMaterials {
    head_material: Handle<ColorMaterial>
}

struct SizeInGrid {
    size: f32
}

struct PositionInGrid {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum SnakeDirection {
    Left,
    Right,
    Up,
    Down,
}

impl SnakeDirection {
    fn flipped(self) -> Self {
        match self {
            SnakeDirection::Up => SnakeDirection::Down,
            SnakeDirection::Down => SnakeDirection::Up,
            SnakeDirection::Left => SnakeDirection::Right,
            SnakeDirection::Right => SnakeDirection::Left
        }
    }

    fn to_xy(&self) -> (i32, i32) {
        match &self {
            SnakeDirection::Up => (0, 1),
            SnakeDirection::Down => (0, -1),
            SnakeDirection::Left => (-1, 0),
            SnakeDirection::Right => (1, 0)
        }
    }
}

#[derive(PartialEq, Debug, Eq, Clone, Hash, SystemLabel)]
enum SnakeMovement {
    Input,
    Movement,
}

mod setup {
    use super::*;

    pub(super) fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
        let head_material = materials.add(Color::ALICE_BLUE.into());
        commands.insert_resource(SnakeMaterials { head_material });
    }

    pub(super) fn spawn_snake(mut commands: Commands, materials: Res<SnakeMaterials>) {
        let sprite = Sprite::new(Vec2::new(10.0, 10.0));
        let sprite_bundle = SpriteBundle {
            material : materials.head_material.clone(),
            sprite,
            ..Default::default()
        };
        commands.spawn_bundle(sprite_bundle)
        .insert(SnakeHead {dir: SnakeDirection::Up})
        .insert(SizeInGrid {size: 0.8})
        .insert(PositionInGrid {x: 3, y: 3});
    }
}

fn snake_input(
    key_input: Res<Input<KeyCode>>,
    mut heads: Query<&mut SnakeHead>
) {
    
    if let Some(mut head) = heads.iter_mut().next() {
        let dir = if key_input.pressed(KeyCode::A) {
            SnakeDirection::Left
        } else if key_input.pressed(KeyCode::D) {
            SnakeDirection::Right
        } else if key_input.pressed(KeyCode::W) {
            SnakeDirection::Up
        } else if key_input.pressed(KeyCode::S) {
            SnakeDirection::Down
        } else {
            head.dir
        };

        // prevent snake from immediately going opposite direction
        if dir != head.dir.flipped() {
            head.dir = dir;
        }
    }
}

fn snake_fixed_movement(
    mut query: Query<(&mut PositionInGrid, &SnakeHead)>
) {
    for (mut grid_pos, head) in query.iter_mut() {
        let (dx, dy) = head.dir.to_xy();
        grid_pos.x += dx;
        grid_pos.y += dy;
    }
}

fn grid_size_to_sprite_size(
    windows: Res<Windows>,
    mut query: Query<(&mut SizeInGrid, &mut Sprite)>
) {
    let primary_win = windows.get_primary().expect("Expected a primary window");
    let window_width = primary_win.width();
    // Width logic: If we have a 40x40 grid, a snakehead that is 1 grid unit in size
    // and a window that is 400 pixels across, then the sprite size should be 40
    for (size_in_grid, mut sprite) in query.iter_mut() {
        let size_in_grid = size_in_grid.size;
        let sprite_size = size_in_grid / ARENA_GRID_SIZE * window_width;
        let sprite_size = Vec2::new(sprite_size, sprite_size);
        sprite.size = sprite_size;
    }
}

fn grid_position_to_translation(
    windows: Res<Windows>,
    mut query: Query<(&PositionInGrid, &mut Transform)>
) {
    /*
    If the snake head is at position 5 in grid, 
    the width in our grid is 10, and the window width is 200, 
    then the coordinate should be 5 / 10 * 200 - 200 / 2.0
    */
    fn to_window_pos(grid_pos: f32, window_bound: f32, grid_bound: f32) -> f32 {
        let tile_size = window_bound / grid_bound;
        grid_pos / grid_bound * window_bound - (window_bound / 2.) + (tile_size / 2.)
    }

    let primary_win = windows.get_primary().expect("Expected a primary window");
    let window_width = primary_win.width();
    for (grid_pos, mut transform) in query.iter_mut() {
        let vec = Vec3::new(
            to_window_pos(grid_pos.x as f32, window_width, ARENA_GRID_SIZE),
            to_window_pos(grid_pos.y as f32, window_width, ARENA_GRID_SIZE),
            0.0
        );
        // println!("{:?}", vec);
        // transform.translation.x = to_window_pos(grid_pos.x as f32, window_width, ARENA_GRID_SIZE);
        transform.translation = vec;
    }
}