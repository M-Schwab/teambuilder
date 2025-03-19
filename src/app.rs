use leptos::prelude::*;
use crate::components::players::Players;
use crate::components::teamgen::TeamGenerator;

#[component]
pub fn App() -> impl IntoView {

    let players = RwSignal::new(vec![]);

    view! {
        <main class="container">
            <div class="row">
                <Players players/>
                <TeamGenerator players/>
            </div>
        </main>
    }
}
