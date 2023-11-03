use comfy::*;

simple_game!("Sprite Example", setup, update);

fn setup(c: &mut EngineContext) {
    c.load_texture_from_bytes(
        // Name of our sprite
        "comfy",
        // &[u8] with the image data.
        include_bytes!("assets/comfy.png")
    );
}

fn update(_c: &mut EngineContext) {
    draw_sprite(
        // Sprites are referenced with their string name.
        // Comfy doesn't use asset handles.
        texture_id("comfy"),
        // position
        Vec2::ZERO,
        // color tint
        WHITE,
        // z_index
        0,
        // size
        splat(5.0),
    );
}
