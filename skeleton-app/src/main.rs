use nannou::prelude::*;
use nannou::noise::*;

struct Point {
    coordinates: Point2,
    color: Rgba
}

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    points: Vec<Point>
}

// Builds the model
fn model(app: &App) -> Model {
    let width = 720;
    let height = 720;

    // The window is created here; we could
    // also create it in the main function
    app.new_window()
        .size(width, height)
        .build()
        .unwrap();
    
    let mut points = Vec::new();

    let density = 20.0;
    let space = width as f32 / density;

    let half_w = width as f32 * 0.5;
    let half_h = height as f32 * 0.5;

    for x in (0..width).step_by(space as usize) {
        for y in (0..height).step_by(space as usize) {
            let coordinates = pt2(x as f32 - half_w + density, y as f32 - half_h + density);
            let color = rgba(random_f32(), random_f32(), random_f32(), random_f32());

            points.push(Point { coordinates, color });
        }
    }

    // We just return an empty struct here
    Model { points }
}

// Updates the model (note the mutable reference)
fn update(_app: &App, model: &mut Model, _update: Update) {
    let noise = Perlin::new();
    let m = 0.0005;

    model.points.iter_mut()
        .for_each(|point| {
            let noise_value = noise.get([m * point.coordinates.x as f64, m * point.coordinates.y as f64]) as f32;
            let noise_value_map = deg_to_rad(map_range(noise_value, -1.0, 1.0, -360.0, 360.0));
            point.coordinates += pt2(noise_value_map.cos(), noise_value_map.sin())
        });
}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();
    
    // Let's first color the background
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    
    for point in &model.points {
        draw.ellipse()
            .xy(point.coordinates)
            .w_h(3.0, 3.0)
            .color(point.color);
    }

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model)
        .view(view)
        .update(update)
        .run();
}