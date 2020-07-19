use std::{cell::RefCell, collections::HashMap, path::Path, sync::Arc};

use fltk::{app::*, draw::*, window::*};

use crate::eval::State;
use crate::interact::run_interaction;
use crate::modem::*;
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

fn draw_pixel(point: &Point, blender: &Blender, scale: i32, center_x: i32, center_y: i32) {
    let (r, g, b) = blender.get(point);
    set_color_rgb(r, g, b);
    if scale == 1 {
        draw_point(center_x + point.x, center_y + point.y);
    } else {
        let (r, g, b) = blender.get(point);
        set_color_rgb(r, g, b);
        draw_rectf(
            center_x + point.x * scale,
            center_y + point.y * scale,
            scale,
            scale,
        );
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

    const VIEWPORT_WIDTH: u32 = 1024;
    const VIEWPORT_HEIGHT: u32 = 768;
    const VIEWPORT_CENTER_X: i32 = (VIEWPORT_WIDTH / 2) as i32;
    const VIEWPORT_CENTER_Y: i32 = (VIEWPORT_HEIGHT / 2) as i32;
    const MAX_SCALE: i32 = 10;
    let COLORS: Vec<(u8, u8, u8)> = vec![
        (39, 39, 39),
        (17, 100, 102),
        (255, 101, 47),
        (255, 228, 0),
        (20, 167, 108),
        (209, 232, 226),
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
            for (i, p) in pics.iter().rev().enumerate() {
                let (r, g, b) = if i + 1 >= COLORS.len() {
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
                for point in p.points.iter() {
                    draw_pixel(point, &blender, scale, VIEWPORT_CENTER_X, VIEWPORT_CENTER_Y);
                }
            }
        }
        *scale_capture.borrow_mut() = scale;
    }));

    window.end();
    window.show();

    let mut first_coords = if &protocol == "galaxy" {
        Some((0, 0))
    } else {
        None
    };
    let mut interaction_state = NestedList::Nil;
    let mut prev_state = NestedList::Nil;
    let mut prev_coords = (0, 0);
    let mut just_loaded = false;
    while app.wait().unwrap() {
        // println!("{:?}", event());
        match event() {
            fltk::enums::Event::Released => {
                just_loaded = false;
                let (mut x, mut y) = get_mouse();
                // Coords processing
                x = x - window.x() - VIEWPORT_CENTER_X;
                y = y - window.y() - VIEWPORT_CENTER_Y;
                if let Some((first_x, first_y)) = first_coords {
                    x = first_x;
                    y = first_y;
                    first_coords = None;
                }
                let scale = *scale.borrow();
                if scale > 1 {
                    x = if x < 0 {
                        (x - scale) / scale
                    } else {
                        x / scale
                    };
                    y = if y < 0 {
                        (y - scale) / scale
                    } else {
                        y / scale
                    };
                }
                // Click
                println!("Clicked on ({}, {})", x, y);
                let (new_state, pics) = run_interaction(
                    &mut state,
                    &protocol,
                    interaction_state.clone(),
                    x as i64,
                    y as i64,
                );
                prev_state = interaction_state;
                prev_coords = (x, y);
                interaction_state = new_state;

                pics_data.borrow_mut().vec = pics;

                window.redraw();
            }
            fltk::enums::Event::KeyUp => match event_key() {
                fltk::enums::Key::Enter => {
                    just_loaded = false;
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
                k => {
                    if k == fltk::enums::Key::from_i32(0xffc2) {
                        // F5 - save
                        just_loaded = false;
                        println!("Saving state...");
                        let data = NestedList::Cons(
                            Box::new(prev_state.clone()),
                            Box::new(NestedList::Cons(
                                Box::new(NestedList::Number(prev_coords.0 as i64)),
                                Box::new(NestedList::Number(prev_coords.1 as i64)),
                            )),
                        );
                        let serialized = mod_list(&data);
                        let str: Vec<u8> = serialized
                            .into_iter()
                            .map(|b| if b { b'1' } else { b'0' })
                            .collect();
                        std::fs::write("./save.dat", str)?;
                    } else if k == fltk::enums::Key::from_i32(0xffc5) {
                        // F8 - load
                        if !just_loaded {
                            println!("Loading state...");
                            let file = std::fs::read("./save.dat")?;
                            let serialized: Vec<_> = file
                                .into_iter()
                                .map(|c| if c == b'1' { true } else { false })
                                .collect();
                            let list = dem_list(&serialized);
                            let (st, coords) = list.unwrap_cons();
                            let (x, y) = coords.unwrap_cons();
                            interaction_state = st;
                            let x = x.unwrap_number() as i32;
                            let y = y.unwrap_number() as i32;

                            first_coords = None;

                            let (new_state, pics) = run_interaction(
                                &mut state,
                                &protocol,
                                interaction_state.clone(),
                                x as i64,
                                y as i64,
                            );
                            prev_state = interaction_state;
                            prev_coords = (x, y);
                            interaction_state = new_state;

                            pics_data.borrow_mut().vec = pics;

                            window.redraw();

                            just_loaded = true;
                        }
                    } else {
                        println!("Unhandled key: {:?}", k);
                    }
                }
            },
            _ => {}
        }
    }
    Ok(())
}
