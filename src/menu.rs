use std::collections::HashMap;

use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element};

use crate::game::structs::GameState;
use crate::GAME;

pub struct Menu {
    pub menu_element: Element,
    pub button_elements: Vec<Element>,
    pub button_listeners: HashMap<String, EventListener>,
}

impl Menu {
    pub fn new(document: &Document) -> Menu {
        let menu_element = document.get_element_by_id("menu").unwrap();

        let button_elements: Vec<Element> = GameState::vec()
            .iter()
            .map(|state| state.to_string())
            .map(|state| {
                document
                    .get_element_by_id(&format!("button-{}", state).to_owned()[..])
                    .unwrap()
            })
            .collect();

        let mut button_listeners: HashMap<String, EventListener> = HashMap::new();
        for element in button_elements.iter() {
            let target_id = element.dyn_ref::<Element>().unwrap().id().clone();
            let id_clone = target_id.clone();
            let listener = EventListener::new(element, "click", move |_e| {
                let game = unsafe { GAME.as_mut().unwrap() };
                let target_state = target_id.split("-").nth(1).unwrap();
                game.set_state(GameState::from(target_state));
            });
            button_listeners.insert(id_clone, listener);
        }

        Menu {
            menu_element,
            button_elements,
            button_listeners,
        }
    }

    pub fn disable(&self) {
        self.menu_element
            .set_attribute("class", "disable")
            .expect("Failed to set attribute");
    }

    pub fn enable(&self) {
        self.menu_element
            .remove_attribute("class")
            // .set_attribute("class", "initial")
            .expect("Failed to set attribute");
    }
}
