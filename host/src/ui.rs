use std::{cell::RefCell, sync::Arc};

use fltk::{app::*, draw::*, window::*};

use crate::eval::{Picture, State};
use crate::interact::run_interaction;
use crate::syntax::parse_line;

#[derive(Default)]
struct Data {
    vec: Vec<Picture>,
}

pub fn ui_main() -> std::io::Result<()> {
    const VIEWPORT_WIDTH: u32 = 800;
    const VIEWPORT_HEIGHT: u32 = 600;

    let app = App::default();
    let mut window = Window::new(
        0,
        0,
        VIEWPORT_WIDTH as i32,
        VIEWPORT_HEIGHT as i32,
        "Galaxy Explorer",
    );
    window.set_type(WindowType::Double);
    window = window.center_screen();
    let pics_data = Arc::new(RefCell::new(Data::default()));

    let pics_capture = pics_data.clone();
    window.draw(Box::new(move || {
        let pics = &pics_capture.borrow().vec;
        set_color_rgb(0, 0, 0);
        draw_rectf(0, 0, VIEWPORT_WIDTH as i32, VIEWPORT_HEIGHT as i32);
        set_color_rgb(0, 0, 255);
        for p in pics {
            for point in p.points.iter() {
                // img.set_pixel(point.x, point.y, bmp::Pixel::new(0, 0, 255));
                draw_point(point.x as i32, point.y as i32);
            }
        }
    }));

    window.end();
    window.show();

    let mut state = State::new();
    state.interpret(parse_line("statelessdraw = ap ap c ap ap b b ap ap b ap b ap cons 0 ap ap c ap ap b b cons ap ap c cons nil ap ap c ap ap b cons ap ap c cons nil nil"));

    let (mut last_x, mut last_y) = (-1, -1);
    while app.wait().unwrap() {
        // println!("{:?}", event());
        match event() {
            fltk::enums::Event::Released => {
                let (mut x, mut y) = get_mouse();
                x -= window.x();
                y -= window.y();
                if last_x != x || last_y != y {
                    println!("Clicked on ({}, {})", x, y);
                    let pics = run_interaction(&mut state, "statelessdraw", x as i64, y as i64);
                    println!("{:?}", pics);

                    pics_data.borrow_mut().vec = pics;

                    last_x = x;
                    last_y = y;

                    window.redraw();
                }
            }
            fltk::enums::Event::KeyUp => match event_key() {
                fltk::enums::Key::Enter => {
                    println!("Saving the current picture...");
                    let mut img = bmp::Image::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT);
                    let mut img_data = vec![];

                    let pics = &pics_data.borrow().vec;
                    for p in pics.iter() {
                        for point in p.points.iter() {
                            img.set_pixel(point.x, point.y, bmp::Pixel::new(0, 0, 255));
                        }
                    }

                    img.to_writer(&mut img_data).unwrap();
                    img.save("./current.bmp").unwrap();
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
