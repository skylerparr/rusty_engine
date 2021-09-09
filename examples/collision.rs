use rusty_engine::prelude::*;

const ROTATION_SPEED: f32 = 3.0;

fn main() {
    let mut game = Game::new();

    let mut race_car = game.add_actor("Player", ActorPreset::RacingCarGreen);
    race_car.translation = Vec2::new(0.0, 0.0);
    race_car.rotation = UP;
    race_car.layer = 100.0;
    race_car.collision = true;

    let mut actor_presets_iter = ActorPreset::variant_iter().peekable();
    'outer: for y in (-265..=400).step_by(175) {
        for x in (-550..=550).step_by(275) {
            if actor_presets_iter.peek().is_none() {
                break 'outer;
            }
            let actor_preset = actor_presets_iter.next().unwrap();
            let mut actor = game.add_actor(format!("{:?}", actor_preset), actor_preset);
            actor.translation = Vec2::new(x as f32, (-y) as f32);
            actor.collision = true;
        }
    }

    let mut text_actor = game.add_text_actor("collision text", "");
    text_actor.translation = Vec2::new(0.0, -200.0);

    game.run(logic);
}

fn logic(game_state: &mut GameState) {
    // If a collision event happened last frame, print it out and play a sound
    for event in game_state.collision_events.drain(..) {
        let text_actor = game_state.text_actors.get_mut("collision text").unwrap();
        match event.state {
            CollisionState::Begin => {
                text_actor.text = format!("{:?}", event.pair);
                game_state.audio_manager.play_sfx(SfxPreset::Switch1)
            }
            CollisionState::End => {
                text_actor.text = "".into();
                game_state.audio_manager.play_sfx(SfxPreset::Switch2)
            }
        }
    }

    if let Some(actor) = game_state.actors.get_mut("Player") {
        // Use the latest state of the mouse buttons to rotate the actor
        let mut rotation_amount = 0.0;
        if game_state.mouse_state.pressed(MouseButton::Left) {
            rotation_amount += ROTATION_SPEED * game_state.delta_f32;
        }
        if game_state.mouse_state.pressed(MouseButton::Right) {
            rotation_amount -= ROTATION_SPEED * game_state.delta_f32;
        }
        actor.rotation += rotation_amount;

        // Use the latest state of the mouse wheel to scale the actor
        if let Some(location) = game_state.mouse_state.location() {
            actor.translation = location
        }

        // Mousewheel scales the car
        for mouse_wheel in &game_state.mouse_wheel_events {
            actor.scale *= 1.0 + (0.05 * mouse_wheel.y);
            actor.scale = actor.scale.clamp(0.1, 4.0);
        }
    }
}
