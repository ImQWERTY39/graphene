use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::time::Duration;

mod coords;
use coords::*;

mod function;

fn main() {
    let sdl_context = sdl2::init().expect("Shouldn't have failed");
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Graphene", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("Shouldn't have failed");

    let mut canvas = window.into_canvas().build().expect("Shouldn't have failed");
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut time: f64 = 0.0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }

                _ => (),
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        draw_coordinate_axes(&mut canvas);

        let points = function::get_points(|x, y| y - 6.0 * (x - time.sin() * 2.0).sin());
        draw_graph(&mut canvas, points, Color::RGB(255, 255, 255));

        canvas.present();

        std::thread::sleep(Duration::from_nanos(1_000_000_000 / 120));
        time += 1.0 / 15.0;
    }
}

fn draw_graph(canvas: &mut Canvas<Window>, points: Vec<(i32, i32)>, colour: Color) {
    canvas.set_draw_color(colour);

    points
        .windows(2)
        .into_iter()
        .for_each(|i| canvas.draw_line(i[0], i[1]).unwrap());

    // points
    // .into_iter()
    // .for_each(|i| canvas.draw_point(i).unwrap());
}

fn draw_coordinate_axes(canvas: &mut Canvas<Window>) {
    let window_height = WINDOW_HEIGHT as i32;
    let window_width = WINDOW_WIDTH as i32;

    canvas.set_draw_color(Color::RGBA(64, 64, 64, 0));
    let mut i = -X_MAX;
    while i <= X_MAX {
        let x_screen = screenx_from_posx(i);

        canvas
            .draw_line((x_screen, 0), (x_screen, window_height))
            .expect("HUH");

        i += 1.0;
    }

    let mut i = -Y_MAX;
    while i <= Y_MAX {
        let y_screen = screeny_from_posy(i);

        canvas
            .draw_line((0, y_screen), (window_width, y_screen))
            .expect("HUH");

        i += 1.0;
    }

    canvas.set_draw_color(Color::RGB(128, 128, 128));

    canvas
        .draw_line((CENTER_WIDTH, 0), (CENTER_WIDTH, window_height))
        .expect("HUH");

    canvas
        .draw_line((0, CENTER_HEIGHT), (window_width, CENTER_HEIGHT))
        .expect("HUH");
}
