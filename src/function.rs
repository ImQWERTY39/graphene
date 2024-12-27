use crate::{posx_from_screenx, posy_from_screeny, WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn get_points(f: impl Fn(f64, f64) -> f64, zoom: f64) -> Vec<(i32, i32)> {
    let mut points = Vec::new();

    for x_screen in 0..WINDOW_WIDTH as i32 {
        for y_screen in 0..WINDOW_HEIGHT as i32 {
            let x = posx_from_screenx(x_screen, zoom);
            let y = posy_from_screeny(y_screen, zoom);

            let res = f(x, y).abs();

            if res.is_normal() && res < 0.02 {
                points.push((x_screen, y_screen));
            }
        }
    }

    points
}
