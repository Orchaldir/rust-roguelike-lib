extern crate rust_roguelike_rendering_glium;

use rust_roguelike_core::algorithm::pathfinding::a_star::AStar;
use rust_roguelike_core::algorithm::pathfinding::{PathfindingAlgorithm, PathfindingResult};
use rust_roguelike_core::interface::input::{KeyCode, MouseButton};
use rust_roguelike_core::interface::rendering::{Renderer, TextureId, Window};
use rust_roguelike_core::interface::App;
use rust_roguelike_core::math::color::{BLACK, BLUE, GREEN, RED, WHITE};
use rust_roguelike_core::math::graph::occupancy::OccupancyMap;
use rust_roguelike_core::math::graph::Graph;
use rust_roguelike_core::rendering::tile::TileRenderer;
use rust_roguelike_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct PathfindingExample {
    texture_id: TextureId,
    occupancy_map: OccupancyMap,
    start: usize,
    goal: usize,
    result: PathfindingResult,
}

impl PathfindingExample {
    fn update(&mut self) {
        let algorithm = AStar::default();

        self.result = algorithm.find(&self.occupancy_map, self.start, self.goal);
        println!("Result={:?}", self.result)
    }
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

        tile_renderer.render_ascii(renderer, self.start, b'S', GREEN);

        if let PathfindingResult::Path { indices } = &self.result {
            for node in indices {
                tile_renderer.render_ascii(renderer, *node, b'+', BLUE);
            }
        }

        tile_renderer.render_ascii(renderer, self.goal, b'G', RED);

        renderer.finish();
    }

    fn on_key_released(&mut self, key: KeyCode) {
        println!("Released {:?}", key);
    }

    fn on_button_released(&mut self, button: MouseButton, index: usize) {
        println!("Released {:?} at {}", button, index);

        match button {
            MouseButton::Left => self.start = index,
            MouseButton::Right => self.goal = index,
            _ => {}
        }

        self.update();
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Pathfinding Example");
    let app = Rc::new(RefCell::new(PathfindingExample::default()));

    window.run(app.clone());
}
