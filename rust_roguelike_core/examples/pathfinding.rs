extern crate rust_roguelike_rendering_glium;

use rust_roguelike_core::interface::input::{KeyCode, MouseButton};
use rust_roguelike_core::interface::rendering::{Renderer, TextureId, Window};
use rust_roguelike_core::interface::App;
use rust_roguelike_core::math::color::{BLACK, WHITE};
use rust_roguelike_core::math::graph::map2d::{Direction2d, Map2d};
use rust_roguelike_core::math::graph::{Graph, Neighbor};
use rust_roguelike_core::math::size2d::Size2d;
use rust_roguelike_core::rendering::tile::TileRenderer;
use rust_roguelike_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct OccupancyMap {
    pub cells: Vec<bool>,
    size: Size2d,
}

impl OccupancyMap {
    pub fn new(size: Size2d, default: bool) -> Self {
        let cells = vec![default; size.get_tiles()];
        OccupancyMap { cells, size }
    }

    pub fn add_border(&mut self) {
        self.add_rectangle(0, 0, self.size.x(), self.size.y(), true)
    }

    pub fn add_rectangle(
        &mut self,
        start_x: u32,
        start_y: u32,
        size_x: u32,
        size_y: u32,
        value: bool,
    ) {
        let end_x = start_x + size_x;
        let end_y = start_y + size_y;

        for x in start_x..end_x {
            self.cells[self.size.to_index(x, start_y)] = value;
            self.cells[self.size.to_index(x, end_y - 1)] = value;
        }

        for y in start_y..end_y {
            self.cells[self.size.to_index(start_x, y)] = value;
            self.cells[self.size.to_index(end_x - 1, y)] = value;
        }
    }
}

impl Graph<bool, Direction2d> for OccupancyMap {
    fn get_size(&self) -> usize {
        self.size.get_tiles()
    }

    fn get_node(&self, index: usize) -> Option<&bool> {
        self.cells.get(index)
    }

    fn get_neighbors(&self, _index: usize) -> &[Neighbor<Direction2d>] {
        unimplemented!()
    }
}

impl Map2d<bool, Direction2d> for OccupancyMap {
    fn get_size_2d(&self) -> Size2d {
        self.size
    }
}

#[derive(Default)]
pub struct PathfindingExample {
    texture_id: TextureId,
    occupancy_map: OccupancyMap,
}

impl App for PathfindingExample {
    fn init(&mut self, renderer: &mut dyn Renderer) {
        self.texture_id = renderer.load_texture("ascii.png");

        self.occupancy_map = OccupancyMap::new(renderer.get_size(), false);
        self.occupancy_map.add_border();
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        let mut tile_renderer = TileRenderer::new(self.texture_id);

        renderer.start(BLACK);

        for i in 0..self.occupancy_map.get_size() {
            let ascii = if *self.occupancy_map.get_node(i).unwrap_or(&false) {
                b'#'
            } else {
                b'.'
            };
            tile_renderer.render_ascii(renderer, i, ascii, WHITE);
        }

        renderer.finish();
    }

    fn on_key_released(&mut self, key: KeyCode) {
        println!("Released {:?}", key);
    }

    fn on_button_released(&mut self, button: MouseButton, index: usize) {
        println!("Released {:?} at {}", button, index);
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Pathfinding Example");
    let app = Rc::new(RefCell::new(PathfindingExample::default()));

    window.run(app.clone());
}
