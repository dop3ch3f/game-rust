use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

use crate::catvolleyball::Ball;

#[derive(SystemDesc)]
pub struct MoveBallsSystem;

pub const GRAVITY_ACCELERATION: f32 = -5.0;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut balls, mut locals, time): Self::SystemData) {
        for (ball, local) in (&mut balls, &mut locals).join() {
            local.prepend_translation_x(ball.velocity[0] * time.delta_seconds());

            local.prepend_translation_y(
                (ball.velocity[1] + time.delta_seconds() * GRAVITY_ACCELERATION / 2.0)
                    * time.delta_seconds(),
            );

            ball.velocity[1] = ball.velocity[1] + time.delta_seconds() * GRAVITY_ACCELERATION;
        }
    }
}
