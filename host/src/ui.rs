use std::{cell::RefCell, collections::HashMap, path::Path, sync::Arc};

use fltk::{app::*, draw::*, window::*};

use crate::eval::State;
use crate::interact::run_interaction;
use crate::syntax::parse_line;
use crate::types::*;

#[derive(Default)]
struct Data {
    vec: Vec<Picture>,
}

struct Blender {
    alpha: f32,
    colors: HashMap<Point, (u8, u8, u8)>,
}

impl Blender {
    pub fn new(alpha: f32) -> Blender {
        Blender {
            alpha,
            colors: HashMap::new(),
        }
    }

    pub fn blend(&mut self, p: Point, r: u8, g: u8, b: u8) {
        if let Some(&(br, bg, bb)) = self.colors.get(&p) {
            self.colors.insert(
                p,
                (
                    self.blend_component(r, br),
                    self.blend_component(g, bg),
                    self.blend_component(b, bb),
                ),
            );
        } else {
            self.colors.insert(p, (r, g, b));
        }
    }

    fn blend_component(&self, c1: u8, c2: u8) -> u8 {
        ((c1 as f32 * self.alpha) + (c2 as f32 * (1.0 - self.alpha))) as u8
    }

    pub fn get(&self, p: &Point) -> (u8, u8, u8) {
        *self.colors.get(p).unwrap()
    }
}

pub fn ui_main(file: String, data_folder: &Path) -> std::io::Result<()> {
    let mut state = State::new();
    let mut protocol = None;
    // Skip the "INTERACTIVE" line
    for line in file.lines().skip(1) {
        if line.is_empty() {
        } else if let Some(l) = line.strip_prefix("PROTOCOL ") {
            println!("Protocol: {}", l);
            protocol = Some(l.to_string());
        } else if let Some(l) = line.strip_prefix("INCLUDE ") {
            let f = std::fs::read_to_string(data_folder.join(l))?;
            for l in f.lines() {
                state.interpret(parse_line(l));
            }
        } else {
            state.interpret(parse_line(line));
        }
    }

    let protocol = protocol.expect("Protocol was not defined in the instruction file");

    const VIEWPORT_WIDTH: u32 = 800;
    const VIEWPORT_HEIGHT: u32 = 600;
    const VIEWPORT_CENTER_X: i32 = (VIEWPORT_WIDTH / 2) as i32;
    const VIEWPORT_CENTER_Y: i32 = (VIEWPORT_HEIGHT / 2) as i32;
    const MAX_SCALE: i32 = 10;
    let COLORS: Vec<(u8, u8, u8)> = vec![
        (39, 39, 39),
        (116, 116, 116),
        (255, 101, 47),
        (255, 228, 0),
        (20, 167, 108),
    ];

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
    let scale = Arc::new(RefCell::new(1 as i32));

    let pics_capture = pics_data.clone();
    let scale_capture = scale.clone();
    window.draw(Box::new(move || {
        let pics = &pics_capture.borrow().vec;
        let (r, g, b) = COLORS[0];
        set_color_rgb(r, g, b);
        draw_rectf(0, 0, VIEWPORT_WIDTH as i32, VIEWPORT_HEIGHT as i32);
        let mut scale = 1;
        if !pics.is_empty() {
            let Picture {
                mut width,
                mut height,
                ..
            } = pics[0];
            for p in pics.iter() {
                if width < p.width {
                    width = p.width;
                }
                if height < p.height {
                    height = p.height;
                }
            }
            let scale_x = (VIEWPORT_WIDTH / width) as i32;
            let scale_y = (VIEWPORT_HEIGHT / height) as i32;
            scale = if scale_x < scale_y { scale_x } else { scale_y };
            if scale > MAX_SCALE {
                scale = MAX_SCALE;
            }
            let mut blender = Blender::new(0.7);
            for (i, p) in pics.iter().enumerate() {
                let (r, g, b) = if i >= COLORS.len() {
                    println!("WARNING: not enough colors");
                    (255, 0, 0)
                } else {
                    COLORS[i + 1]
                };
                for point in p.points.iter() {
                    blender.blend(*point, r, g, b);
                }
            }
            for p in pics {
                if scale == 1 {
                    for point in p.points.iter() {
                        let (r, g, b) = blender.get(point);
                        set_color_rgb(r, g, b);
                        draw_point(VIEWPORT_CENTER_X + point.x, VIEWPORT_CENTER_Y + point.y);
                    }
                } else {
                    for point in p.points.iter() {
                        let (r, g, b) = blender.get(point);
                        set_color_rgb(r, g, b);
                        draw_rectf(
                            VIEWPORT_CENTER_X + point.x * scale,
                            VIEWPORT_CENTER_Y + point.y * scale,
                            scale,
                            scale,
                        );
                    }
                }
            }
        }
        *scale_capture.borrow_mut() = scale;
    }));

    window.end();
    window.show();

    let (mut last_x, mut last_y) = (-1, -1);
    let mut first_coords = if &protocol == "galaxy" {
        Some((0, 0))
    } else {
        None
    };
    let mut interaction_state = NestedList::Nil;
    while app.wait().unwrap() {
        // println!("{:?}", event());
        match event() {
            fltk::enums::Event::Released => {
                let (mut x, mut y) = get_mouse();
                println!("{}, {}", x, y);
                x = x - window.x() - VIEWPORT_CENTER_X;
                y = y - window.y() - VIEWPORT_CENTER_Y;
                println!("{}, {}", x, y);
                if let Some((first_x, first_y)) = first_coords {
                    x = first_x;
                    y = first_y;
                    first_coords = None;
                }
                println!("{}, {}", x, y);
                let scale = *scale.borrow();
                if scale > 1 {
                    x /= scale as i32;
                    y /= scale as i32;
                }
                println!("{}, {}", x, y);
                if last_x != x || last_y != y {
                    println!("Clicked on ({}, {})", x, y);
                    let (new_state, pics) = run_interaction(
                        &mut state,
                        &protocol,
                        interaction_state,
                        x as i64,
                        y as i64,
                    );
                    interaction_state = new_state;

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
                            img.set_pixel(
                                point.x as u32,
                                point.y as u32,
                                bmp::Pixel::new(0, 0, 255),
                            );
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
