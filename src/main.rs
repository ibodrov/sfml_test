extern crate sfml;
extern crate time;

use std::vec::Vec;
use sfml::graphics::*;
use sfml::window::{ContextSettings, VideoMode, window_style, Key, MouseButton};
use sfml::window::event::Event;
use sfml::system::Vector2f;

const MAP_WIDTH: u32 = 32;
const MAP_HEIGHT: u32 = 32;
const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;
const TILES_PER_ROW: u32 = 8;

struct Map {
    width: u32,
    height: u32,
    tiles: Vec<u32>,
}

impl Map {
    fn idx(&self, i: u32, j: u32) -> usize {
        (i + j * self.width) as usize
    }

    fn set_tile(&mut self, i: u32, j: u32, tile: u32) {
        let n = self.idx(i, j);
        self.tiles[n] = tile;
    }
}

fn create_mv(m: &Map) -> VertexArray {
    let red = Color::new_rgb(255, 0, 0);
    let green = Color::new_rgb(0, 255, 0);

    let mut mv = VertexArray::new().unwrap();
    mv.set_primitive_type(PrimitiveType::sfQuads);

    for i in 0..m.width {
        for j in 0..m.height {
            let n = m.idx(i, j);
            let tile = m.tiles[n];
            let tu = tile % TILES_PER_ROW;
            let tv = tile / TILES_PER_ROW;

            let (x1, y1) = ((i * TILE_WIDTH) as f32, (j * TILE_HEIGHT) as f32);
            let (x2, y2) = (((i + 1) * TILE_WIDTH) as f32, (j * TILE_HEIGHT) as f32);
            let (x3, y3) = (((i + 1) * TILE_WIDTH) as f32, ((j + 1) * TILE_HEIGHT) as f32);
            let (x4, y4) = ((i * TILE_WIDTH) as f32, ((j + 1) * TILE_HEIGHT) as f32);

            let (u1, v1) = ((tu * TILE_WIDTH) as f32, (tv * TILE_HEIGHT) as f32);
            let (u2, v2) = (((tu + 1) * TILE_WIDTH) as f32, (tv * TILE_HEIGHT) as f32);
            let (u3, v3) = (((tu + 1) * TILE_WIDTH) as f32, ((tv + 1) * TILE_HEIGHT) as f32);
            let (u4, v4) = ((tu * TILE_WIDTH) as f32, ((tv + 1) * TILE_HEIGHT) as f32);

            let z1 = Vertex::new(&Vector2f::new(x1, y1), &red, &Vector2f::new(u1, v1));
            let z2 = Vertex::new(&Vector2f::new(x2, y2), &green, &Vector2f::new(u2, v2));
            let z3 = Vertex::new(&Vector2f::new(x3, y3), &red, &Vector2f::new(u3, v3));
            let z4 = Vertex::new(&Vector2f::new(x4, y4), &red, &Vector2f::new(u4, v4));

            mv.append(&z1);
            mv.append(&z2);
            mv.append(&z3);
            mv.append(&z4);
        }
    }

    mv
}

fn load_tiles(path: &str, mask: &Color) -> Texture {
    let img = Image::new_from_file(path).unwrap();
    img.create_mask_from_color(mask, 0);

    Texture::new_from_image(&img).unwrap()
}

fn main() {
    let settings = ContextSettings::default();
    let mut window = RenderWindow::new(VideoMode::new_init(1024, 768, 32),
                                       "SFML Test",
                                       window_style::TITLEBAR | window_style::CLOSE,
                                       &settings).unwrap();

    window.set_vertical_sync_enabled(false);

    let black = Color::new_rgb(0, 0, 0);
    let magenta = Color::new_rgb(255, 0, 255);
    let shader = Shader::new_from_file(None, Some("shader.frag")).unwrap();
    shader.set_current_texture_parameter("texture");

    let tiles = load_tiles("tiles.png", &magenta);
    let mut rs = RenderStates::default();
    rs.texture = Some(&tiles);
    rs.shader = Some(&shader);

    let s = (MAP_WIDTH * MAP_HEIGHT) as usize;
    let mut v = Vec::with_capacity(s);
    v.resize(s, 1);

    let mut map = Map { width: MAP_WIDTH, height: MAP_HEIGHT, tiles: v };

    let mut mv = create_mv(&map);

    let mut frames = 0;
    let mut t1 = time::precise_time_s();

    while window.is_open() {
        let t0 = time::precise_time_s();

        for e in window.events() {
            match e {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => {
                    return;
                },

                Event::MouseButtonReleased { button: MouseButton::Left, x: mx, y: my } => {
                    println!("click! {}/{}", mx, my);
                    let i = (mx / TILE_WIDTH as i32) as u32;
                    let j = (my / TILE_HEIGHT as i32) as u32;
                    if i < map.width && j < map.height {
                        map.set_tile(i, j, 3);
                    }

                    mv = create_mv(&map);
                },

                _ => { },
            }
        }

        window.clear(&black);
        window.draw_with_renderstates(&mv, &mut rs);
        window.display();

        frames += 1;

        let t2 = time::precise_time_s();
        let dt = t2 - t1;
        if dt >= 1.0 {
            let fdt = t2 - t0;
            let s = format!("~{} fps, {}s", frames, fdt);
            window.set_title(&s);
            t1 = t2;
            frames = 0;
        }
    }
}
