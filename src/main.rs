use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

fn update_camera (handle: &RaylibDrawHandle, camera: &mut Camera2D, previous_mouse_position: &mut Vector2) {
    let previous_zoom = camera.zoom;
    let screen_center = Vector2::new(
        SCREEN_WIDTH as f32 / 2.0,
        SCREEN_HEIGHT as f32 / 2.0
    );
    let current_mouse_position = handle.get_mouse_position();

    // TODO take into account mouse position when zooming
    // zoom controls
    camera.zoom += handle.get_mouse_wheel_move() as f32 * 0.1;
    camera.zoom = camera.zoom.max(0.1).min(5.0);

    // adjust camera offset to follow zoom point
    let camera_zoom_scale = (1.0 - camera.zoom / previous_zoom).max(-1.0).min(1.0);
    let mouse_offset = Vector2::new(
        current_mouse_position.x - screen_center.x,
        current_mouse_position.y - screen_center.y
    );

    camera.offset.x -= mouse_offset.x * camera_zoom_scale;
    camera.offset.y -= mouse_offset.y * camera_zoom_scale;

    // pan controls
    if handle.is_mouse_button_pressed(MouseButton::MOUSE_MIDDLE_BUTTON) {
        previous_mouse_position.x = current_mouse_position.x;
        previous_mouse_position.y = current_mouse_position.y;
    }
    if handle.is_mouse_button_down(MouseButton::MOUSE_MIDDLE_BUTTON) {
        camera.offset.x += current_mouse_position.x - previous_mouse_position.x;
        camera.offset.y += current_mouse_position.y - previous_mouse_position.y;

        previous_mouse_position.x = current_mouse_position.x;
        previous_mouse_position.y = current_mouse_position.y;
    }
}

fn main() {
    let (mut handle, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
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
        d.draw_text(&format!("Camera offset: x{}, y{}", camera.offset.x, camera.offset.y), 12, 25, 16, Color::BLACK);
        d.draw_text(&format!("Mouse scroll: {}", d.get_mouse_wheel_move()), 12, 40, 16, Color::BLACK);

        let mut drawer = d.begin_mode_2D(&camera);
        {
            drawer.draw_text("This is a test", 45, 45, 12, Color::PURPLE);
            update_camera(&drawer, &mut camera, &mut mouse_positon);
        }
    }
}
