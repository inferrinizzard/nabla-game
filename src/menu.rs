use std::collections::HashMap;

use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element};

use crate::game::structs::GameState;
use crate::{GAME, MENU};

pub struct Menu {
    pub main_menu: MainMenu,
    pub menu_children: HashMap<String, Element>,
    pub menu_element: Element,
}

impl Menu {
    pub fn new(document: &Document) -> Self {
        let menu_element = document.get_element_by_id("menu").unwrap();
        let mut menu_children = HashMap::new();
        let menu_html_children = menu_element
            .get_elements_by_class_name("button-wrapper")
            .item(0)
            .unwrap()
            .children();
        for i in 0..menu_html_children.length() {
            let child = menu_html_children.item(i).unwrap();
            // split id from 'menu-ID'
            let child_id = child.id().split("-").collect::<Vec<&str>>()[1].to_string();
            menu_children.insert(child_id, child.dyn_into::<Element>().unwrap());
        }

        let main_menu = MainMenu::new(document);

        Menu {
            main_menu,
            menu_children,
            menu_element,
        }
    }

    pub fn activate(&self, id: String) {
        for (element_id, element) in self.menu_children.iter() {
            if element_id == &id {
                element.remove_attribute("hidden").expect(
                    format!("Failed to hide {:?} with id {}", element, element_id).as_str(),
                );
            } else {
                element.set_attribute("hidden", "true").expect(
                    format!("Failed to show {:?} with id {}", element, element_id).as_str(),
                );
            }
        }
    }

    pub fn close(&self) {
        self.menu_element
            .set_attribute("hidden", "true")
            .expect("Failed to hide main menu");
    }

    pub fn open(&self) {
        self.menu_element
            .remove_attribute("hidden")
            .expect("Failed to show main menu");
    }
}

pub struct MainMenu {
    pub button_elements: Vec<Element>,
    pub button_listeners: HashMap<String, EventListener>,
}

impl MainMenu {
    pub fn new(document: &Document) -> Self {
        let button_elements: Vec<Element> =
            vec!["PLAYVS", "PLAYAI", "TUTORIAL", "SETTINGS", "CREDITS"]
                .iter()
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
                let (game, menu_ref) = unsafe { (GAME.as_mut().unwrap(), MENU.as_ref()) };
                // split id from 'button-ID'
                let target_state = target_id.split("-").nth(1).unwrap();
                game.set_state(GameState::from(target_state));

                if menu_ref.is_some() {
                    let menu = menu_ref.unwrap();
                    if menu.menu_children.contains_key(target_state) {
                        menu.activate(target_state.to_string());
                    }
                }
            });
            button_listeners.insert(id_clone, listener);
        }

        Self {
            button_elements,
            button_listeners,
        }
    }
}
