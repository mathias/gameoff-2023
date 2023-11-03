use comfy::*;

simple_game!("Sprite Example", setup, update);

fn setup(c: &mut EngineContext) {
    let mut camera = main_camera_mut();
    camera.zoom = 40.0;
}

fn update(c: &mut EngineContext) {
    if is_key_pressed(KeyCode::Escape) {
        *c.quit_flag = true;
    }

    let our_mouse_world = mouse_world(); //screen_to_world(mouse_position());
    draw_text(format!("X:{} Y: {}", our_mouse_world.x, our_mouse_world.y).as_str(), Vec2::ZERO, RED, TextAlign::Center);
}
