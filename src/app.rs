use leptos::prelude::*;
use thaw::{ConfigProvider, Theme, ToasterProvider};

use crate::components::players::Players;
use crate::components::teamgen::TeamGenerator;
use crate::utils::get_system_theme_preference;

#[component]
pub fn App() -> impl IntoView {

    let players = RwSignal::new(vec![]);
    let theme = get_system_theme_preference();
    let theme = RwSignal::new(Theme::from(theme));

    view! {
        <ConfigProvider theme>
            <ToasterProvider>
            <main class="container">
                <div class="row">
                    <Players players/>
                    <TeamGenerator players/>
                </div>
            </main>
            </ToasterProvider>
        </ConfigProvider>
    }
}
