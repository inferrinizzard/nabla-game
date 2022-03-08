// std imports
use std::collections::HashMap;
// wasm-bindgen imports
use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlInputElement};
// outer crate imports
use crate::game::flags::*;
use crate::game::structs::{Game, GameState};
// root imports
use super::{GAME, MENU};

/// controller for the main menu and submenus
pub struct Menu {
    pub menu_children: HashMap<String, Element>,
    pub menu_element: Element,

    pub main_menu_button: Element,
    pub main_menu_listener: EventListener,
    pub game_over_menu: Element,
    pub game_over_listener: EventListener,

    pub main_menu: MainMenu,
    pub settings_menu: SettingsMenu,
}

impl Menu {
    /// extracts child elements from DOM and stores with id as key
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
            let child_id = child.id();
            let id_kvp = child_id.split("-").collect::<Vec<&str>>();
            if id_kvp[0] == "menu" {
                menu_children.insert(id_kvp[1].to_string(), child.dyn_into::<Element>().unwrap());
            }
        }

        let main_menu_button = document.get_element_by_id("button-MENU").unwrap();
        let main_menu_listener = EventListener::new(&main_menu_button, "click", |_e| {
            let (game, menu_ref) = unsafe { (GAME.as_mut().unwrap(), MENU.as_ref()) };
            game.set_state(GameState::from("MENU"));

            if menu_ref.is_some() {
                menu_ref.unwrap().activate("MENU".to_string());
            }
        });

        let game_over_menu = document.get_element_by_id("menu-GAMEOVER").unwrap();
        let game_over_listener = EventListener::new(
            &document.get_element_by_id("gameover-RESTART").unwrap(),
            "click",
            |_e| {
                let menu_ref = unsafe { MENU.as_ref() };
                if menu_ref.is_some() {
                    menu_ref.unwrap().activate("MENU".to_string());
                    unsafe {
                        GAME = Some(Game::new());
                    }
                }
            },
        );

        let main_menu = MainMenu::new(document);
        let settings_menu = SettingsMenu::new(document);

        Menu {
            menu_children,
            menu_element,
            main_menu_button,
            main_menu_listener,
            game_over_menu,
            game_over_listener,
            main_menu,
            settings_menu,
        }
    }

    /// activate specific submenu, deactivate all others
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

    /// hide main menu and show game
    pub fn close(&self) {
        self.menu_element
            .set_attribute("hidden", "true")
            .expect("Failed to hide main menu");
    }

    /// show main menu and hide game
    pub fn open(&self) {
        self.menu_element
            .remove_attribute("hidden")
            .expect("Failed to show main menu");
    }
}

/// controller for the main menu
pub struct MainMenu {
    pub button_elements: Vec<Element>,
    pub button_listeners: HashMap<String, EventListener>,
}

impl MainMenu {
    /// extracts child elements from DOM and adds event listeners for each button
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
            let element_target = element.dyn_ref::<Element>().unwrap();
            let target_id = element_target.id();
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
            button_listeners.insert(element_target.id(), listener);
        }

        Self {
            button_elements,
            button_listeners,
        }
    }
}

/// controller for the settings menu
#[allow(dead_code)]
pub struct SettingsMenu {
    checkboxes: Vec<Element>,
    checkbox_listeners: HashMap<String, EventListener>,
}

impl SettingsMenu {
    /// extracts child elements from DOM and adds event listeners for each checkbox
    pub fn new(document: &Document) -> Self {
        let checkboxes: Vec<Element> = vec![
            "DISPLAY_LN_FOR_LOG",
            "ALLOW_LINEAR_DEPENDENCE",
            "ALLOW_LIMITS_BEYOND_BOUNDS",
            "FULL_COMPUTE",
            "LIMIT_FIELD_BASIS",
        ]
        .iter()
        .map(|state| {
            document
                .get_element_by_id(&format!("checkbox-{}", state).to_owned()[..])
                .unwrap()
        })
        .collect();

        let mut checkbox_listeners: HashMap<String, EventListener> = HashMap::new();
        for element in checkboxes.iter() {
            let listener = EventListener::new(element, "change", move |e| {
                let event_target = e.target().unwrap();
                let event_target_element = event_target.dyn_ref::<HtmlInputElement>().unwrap();

                // split id from 'checkbox-FLAG'
                let target_id = event_target_element.id();
                let flag_name = target_id.split("-").nth(1).unwrap();
                let flag_value = event_target_element.checked();

                unsafe {
                    match flag_name {
                        "DISPLAY_LN_FOR_LOG" => DISPLAY_LN_FOR_LOG = flag_value,
                        "ALLOW_LINEAR_DEPENDENCE" => ALLOW_LINEAR_DEPENDENCE = flag_value,
                        "ALLOW_LIMITS_BEYOND_BOUNDS" => ALLOW_LIMITS_BEYOND_BOUNDS = flag_value,
                        "FULL_COMPUTE" => FULL_COMPUTE = flag_value,
                        "LIMIT_FIELD_BASIS" => LIMIT_FIELD_BASIS = flag_value,
                        _ => panic!("Unknown flag name: {}", flag_name),
                    }
                }
            });
            checkbox_listeners.insert(element.id(), listener);
        }

        Self {
            checkboxes,
            checkbox_listeners,
        }
    }
}
