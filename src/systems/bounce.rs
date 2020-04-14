use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
    ecs::{Read, ReadExpect},
};

use std::ops::Deref;

extern crate rand;
use crate::audio::{play_bounce_sound, Sounds};
use rand::Rng;

use crate::catvolleyball::{Ball, Player, Side, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (mut balls, players, transforms, storage, sounds, audio_output): Self::SystemData,
    ) {
        /* Check whether a ball collided, and bounce off
        accordingly.
        We also check for the velocity of the ball every time, to prevent multiple collisions
        from occurring. */

        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;
            // Bounce at the four sides of the arena.
            if ball_y <= ball.radius && ball.velocity[1] < 0.0 {
                ball.velocity[1] = -ball.velocity[1];
                play_bounce_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            } else if ball_y >= (ARENA_HEIGHT - ball.radius) && ball.velocity[1] > 0.0 {
                ball.velocity[1] = -ball.velocity[1];
                play_bounce_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            } else if ball_x <= (ball.radius) && ball.velocity[0] < 0.0 {
                ball.velocity[0] = -ball.velocity[0];
                play_bounce_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            } else if ball_x >= (ARENA_WIDTH - ball.radius) && ball.velocity[0] > 0.0 {
                ball.velocity[0] = -ball.velocity[0];
                play_bounce_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            }

            // Bounce at the players.
            for (player, player_transform) in (&players, &transforms).join() {
                let player_x = player_transform.translation().x - (player.width * 0.5);
                let player_y = player_transform.translation().y + player.height;

                // To determine whether the ball has collided with a paddle, we create a larger
                // rectangle around the current one, by subtracting the ball radius from the
                // lowest coordinates, and adding the ball radius to the highest ones. The ball
                // is then within the paddle if its center is within the larger wrapper
                // rectangle.

                if point_in_rect(
                    ball_x,                                // x
                    ball_y,                                // y
                    player_x - ball.radius,                // left
                    player_x + ball.radius,                // right
                    player_x + player.width + ball.radius, // top
                    player_y,                              // bottom
                ) {
                    if ball.velocity[1] < 0.0 {
                        // Only bounce when ball is falling
                        ball.velocity[1] = -ball.velocity[1];
                        let mut rng = rand::thread_rng();
                        match player.side {
                            Side::Left => {
                                ball.velocity[0] =
                                    -ball.velocity[0].abs() * rng.gen_range(0.6, 3.0);
                                play_bounce_sound(
                                    &*sounds,
                                    &storage,
                                    audio_output.as_ref().map(|o| o.deref()),
                                );
                            }
                            Side::Right => {
                                ball.velocity[0] =
                                    -ball.velocity[0].abs() * rng.gen_range(0.6, 3.0);
                                play_bounce_sound(
                                    &*sounds,
                                    &storage,
                                    audio_output.as_ref().map(|o| o.deref()),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

// A point is in a box when its coordinates are smaller or // equal than the top right and larger or equal than
// the bottom left.
fn point_in_rect(
    x: f32, // ball's x and y location
    y: f32,
    left: f32, // the player box's boundary bottom: f32,
    right: f32,
    top: f32,
    bottom: f32,
) -> bool {
    x >= left && x <= right && y <= bottom && y <= top
}
