use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::ecs::prelude::Entity;
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::*,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use amethyst::ecs::prelude::{World, WorldExt};

use crate::audio::initialise_audio;

// CONSTANTS
// player config constants
pub const PLAYER_HEIGHT: f32 = 32.0;
pub const PLAYER_WIDTH: f32 = 32.0;

// game config constants
pub const ARENA_HEIGHT: f32 = 500.0;
pub const ARENA_WIDTH: f32 = 500.0;

// ball config constants
pub const BALL_VELOCITY_X: f32 = 60.0;
pub const BALL_VELOCITY_Y: f32 = -50.0;
pub const BALL_RADIUS: f32 = 16.0;

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

// initialize one ball in the middleish of the arena
fn initialize_ball(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    // create translation
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    // assign the sprite for the ball
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}

// ** PLAYER COMPONENT **

// enum to hold the side of each player as its a 2d game
#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

// initialize a player struct with config and information about the player
pub struct Player {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

// implement methods especially new that will be available for each player instance
impl Player {
    fn new(side: Side) -> Player {
        Player {
            side,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
        }
    }
}

// implement Component to store it in a default memory type that is fast (contiguous vector)
impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_players(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    // setup player sprite render from registered sprites
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // cat is the first sprite in the sprites list
    };

    // setup transform vairables for each player to configure location
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = PLAYER_HEIGHT / 2.0;

    // configure each player location
    left_transform.set_translation_xyz(PLAYER_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PLAYER_WIDTH * 0.5, y, 0.0);

    // attach first player to world
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Player::new(Side::Left))
        .with(left_transform)
        .build();
    // attach second player to world
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Player::new(Side::Right))
        .with(right_transform)
        .build();
}

// ** SPRITE CONFIG **

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

// ** ARENA CONFIG **

// setup the camera for the 2d game
fn initialize_camera(world: &mut World) {
    // initialize a transform to use to position the camera
    let mut transform = Transform::default();

    // configure the transfrom
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    // attach the camera to the 2d world with the transform config
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

// ** SCOREBOARD CONFIG **

// struct to hold scores
#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

fn initialize_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "P1".to_string(),  // ID
        Anchor::TopMiddle, // anchor
        Anchor::Middle,    // pivot
        -50.,              // x
        -50.,              // y
        1.,                // z
        200.,              // width
        50.,               // height
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        Anchor::Middle,
        50.,
        -50.,
        1.,
        200.,
        50.,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),  // initial text: 0 points
            [1., 1., 1., 1.], // color (RGBA): white
            50.,              // font size
        ))
        .build();
    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();
    world.insert(ScoreText { p1_score, p2_score });
}

// struct to hold the global game state
#[derive(Default)]
pub struct CatVolleyBall;

// implement simple state for game state enum to intialize the game
impl SimpleState for CatVolleyBall {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        initialize_ball(world, sprite_sheet_handle.clone());
        initialize_players(world, sprite_sheet_handle);
        initialize_scoreboard(world);
        initialise_audio(world);
        initialize_camera(world);
    }
}
