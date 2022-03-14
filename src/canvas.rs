// std imports
use std::collections::HashMap;
// wasm-bindgen imports
use gloo::events::EventListener;
use gloo::render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::JsCast;
use web_sys::*;
// outer crate imports
use crate::render::anim::{on_animation_frame, AnimItem};
use crate::render::pos::*;
use crate::render::util::*;
// util imports
use crate::util::Vector2;

/// Controller for canvas elements, related contexts, and event listeners
pub struct Canvas {
    pub canvas_element: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    pub canvas_bounds: Vector2,
    pub canvas_center: Vector2,

    pub hit_canvas_element: HtmlCanvasElement,
    pub hit_context: CanvasRenderingContext2d,
    pub hit_region_map: HashMap<String, String>,

    pub mousedown_listener: Option<EventListener>,
    pub mousemove_listener: Option<EventListener>,

    pub render_constants: RenderConstants,
    pub render_items: RenderHash,

    pub render_animation_frame_handle: AnimationFrame,
    pub anim_items: HashMap<RenderId, AnimItem>,
}

impl Canvas {
    /// get canvases from DOM and extract client bounds and center
    pub fn new(document: &Document) -> Canvas {
        let canvas_element: HtmlCanvasElement = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into()
            .unwrap();
        let hit_canvas_element: HtmlCanvasElement = document
            .get_element_by_id("hitCanvas")
            .unwrap()
            .dyn_into()
            .unwrap();

        let context = canvas_element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let hit_context = hit_canvas_element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let hit_region_map = HashMap::new();

        let canvas_bounds = Vector2 {
            x: f64::from(canvas_element.width()),
            y: f64::from(canvas_element.height()),
        };

        let canvas_center = Vector2 {
            x: canvas_bounds.x / 2.0,
            y: canvas_bounds.y / 2.0,
        };

        Canvas {
            canvas_element,
            context,
            canvas_bounds,
            canvas_center,
            hit_canvas_element,
            hit_context,
            hit_region_map,
            mousedown_listener: None,
            mousemove_listener: None,
            render_constants: RenderConstants {
                field_sizes: Sizes::default(),
                player_sizes: Sizes::default(),
                button_sizes: Sizes::default(),
            },
            render_items: HashMap::default(),
            render_animation_frame_handle: request_animation_frame(on_animation_frame),
            anim_items: HashMap::default(),
        }
    }

    /// recalculate canvas element sizes on resize
    pub fn resize(&mut self) {
        let window = web_sys::window().unwrap();
        let inner_width = window.inner_width().unwrap().as_f64().unwrap() as u32;
        let inner_height = window.inner_height().unwrap().as_f64().unwrap() as u32;

        self.canvas_element.set_width(inner_width);
        self.canvas_element.set_height(inner_height);
        self.hit_canvas_element.set_width(inner_width);
        self.hit_canvas_element.set_height(inner_height);

        self.rebounds();
        self.update_render_constants();
        self.calculate_render_positions();
    }

    /// recalculate canvas bounds and center on resize
    fn rebounds(&mut self) {
        let canvas_bounds = Vector2 {
            x: f64::from(self.canvas_element.width()),
            y: f64::from(self.canvas_element.height()),
        };

        let canvas_center = Vector2 {
            x: canvas_bounds.x / 2.0,
            y: canvas_bounds.y / 2.0,
        };

        self.canvas_bounds = canvas_bounds;
        self.canvas_center = canvas_center;
    }

    /// update sizes for player cards and field bases
    fn update_render_constants(&mut self) {
        let player_card_height = rem_to_px(String::from("9rem"));
        let player_card_width = player_card_height * 0.75;
        let gutter = player_card_width / 4.0;
        let radius = gutter / 4.0;

        // TODO: add balancing and min sizes for smaller screens
        let field_gutter = gutter * 2.0;
        let field_basis_height =
            (self.canvas_bounds.y - player_card_height * 2.0 - gutter * 2.0 - field_gutter * 3.0)
                / 2.0;
        let field_basis_width = field_basis_height * 0.75;
        let field_radius = field_gutter / 4.0;

        self.render_constants = RenderConstants {
            field_sizes: Sizes {
                width: field_basis_width,
                height: field_basis_height,
                gutter: field_gutter,
                radius: field_radius,
            },
            player_sizes: Sizes {
                width: player_card_width,
                height: player_card_height,
                gutter,
                radius,
            },
            button_sizes: Sizes {
                width: player_card_width,
                height: (player_card_height - gutter) / 2.0,
                gutter,
                radius: radius / 2.0,
            },
        };
    }

    /// calculate default render positions for all render items
    fn calculate_render_positions(&mut self) {
        self.render_items.clear();
        let field_pos = get_base_field_pos();
        let player_pos = get_base_player_pos();
        let button_pos = get_base_button_pos(&field_pos, &player_pos);
        self.render_items.extend(field_pos);
        self.render_items.extend(player_pos);
        self.render_items.extend(button_pos);
    }

    /// starts requestAnimationFrame callback
    pub fn start_anim(&mut self) {
        self.render_animation_frame_handle = request_animation_frame(on_animation_frame);
    }
}
