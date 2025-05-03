use leptos::prelude::*;
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use thaw::Color;
use palette::Srgb;

/// A serializable struct to represent a thaw Color
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RGB {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl RGB {
    pub fn srgb(self) -> Srgb {
        Srgb::new(self.red, self.green, self.blue)
    }

    pub fn from_color(v: Color) -> Self {
        match v {
            Color::RGB(rgb) => {
                RGB {
                    red: rgb.red,
                    green: rgb.green,
                    blue: rgb.blue,
                }
            },
            Color::HSV(_) => todo!(),
            Color::HSL(_) => todo!(),
        }
    }

    pub fn get_text_color(&self) -> &'static str {
        // I'm storing rgb between 0 and 1 b/c thaw, so multiply by 255 at the end.
        let brightness = (self.red * 299. + self.green * 587. + self.blue * 114.) * 255.0 / 1000.;
        if brightness > 128. {
            "black"
        } else {
            "white"
        }
    }
}


/// Restores a json value from local storage
pub fn from_local_storage<T>(key: &str, default: T) -> T where T: DeserializeOwned {
    window()
        .local_storage() 
        .ok() 
        .flatten() 
        .and_then(|storage| { 
            storage.get_item(key).ok().flatten().and_then( 
                |value| serde_json::from_str::<T>(&value).ok(), 
            ) 
        }) 
        .unwrap_or(default)
}

/// Persists a json value from local storage
pub fn set_local_storage<T>(key: &str, value: T) where T: Serialize {
    window()
        .local_storage()
        .ok()
        .flatten()
        .unwrap()
        .set_item(key, &serde_json::to_string(&value).unwrap())
        .unwrap();
}

/// Persists a Color object to local storage
pub fn set_local_storage_color(key: &str, value: Color) {
    let value = RGB::from_color(value);
    set_local_storage(key, value);
}

/// Gets the preferred theme of the platform
pub fn get_system_theme_preference() -> String {
    let media_query = window()
        .match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
        .unwrap();

    if media_query.matches() {
        web_sys::console::log_1(&"Dark mode preferred".into());
    } else {
        web_sys::console::log_1(&"Light mode preferred".into());
    }

    if media_query.matches() {
        "dark"
    } else {
        "light"
    }.to_string()
}