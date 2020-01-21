use raylib::prelude::*;

fn update_camera (handle: &RaylibDrawHandle, camera: &mut Camera2D, previous_mouse_position: &mut Vector2) {
    // TODO take into account mouse position when zooming
    // zoom controls
    camera.zoom += handle.get_mouse_wheel_move() as f32 * 0.5;
    camera.zoom = camera.zoom.max(0.1).min(5.0);

    // pan controls
    if handle.is_mouse_button_pressed(MouseButton::MOUSE_MIDDLE_BUTTON) {
        let new_pos = handle.get_mouse_position();
        
        previous_mouse_position.x = new_pos.x;
        previous_mouse_position.y = new_pos.y;
    }
    if handle.is_mouse_button_down(MouseButton::MOUSE_MIDDLE_BUTTON) {
        let mouse_positon = handle.get_mouse_position();

        camera.offset.x += mouse_positon.x - previous_mouse_position.x;
        camera.offset.y += mouse_positon.y - previous_mouse_position.y;

        previous_mouse_position.x = mouse_positon.x;
        previous_mouse_position.y = mouse_positon.y;
    }
}

fn main() {
    let (mut handle, thread) = raylib::init()
        .size(640, 480)
        .build();
    let mut camera = Camera2D {
        target: Vector2::new(0.0,0.0),
        offset: Vector2::new(0.0, 0.0),
        rotation: 0.0,
        zoom: 1.0
    };
    
    let mut mouse_positon = Vector2::new(0.0, 0.0);

    while !handle.window_should_close() {
        let mut d = handle.begin_drawing(&thread);
        
        d.clear_background(Color::WHITE);
        d.draw_text(&format!("Camera offset: x{}, y{}", camera.offset.x, camera.offset.y), 12, 25, 20, Color::BLACK);

        let mut drawer = d.begin_mode_2D(&camera);
        {
            drawer.draw_text("This is a test", 45, 45, 12, Color::PURPLE);
            update_camera(&drawer, &mut camera, &mut mouse_positon);
        }
    }
}
