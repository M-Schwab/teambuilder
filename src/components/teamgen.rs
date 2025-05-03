use std::collections::BTreeMap;

use leptos::{ev::SubmitEvent, prelude::*};
use thaw::ColorPicker;
use thaw::{Toast, ToastBody, ToastTitle, ToasterInjection};
use thaw::Color;
use palette::Srgb;

use crate::teamgen::{Player, get_even_teams};
use crate::utils::*;

/// Generates a rw signal that persists changes to local storage, and will load that as the default on page refresh.
macro_rules! local_storage_signal {
    ($sig:ident, $default_ident:ident, $default:expr) => {
        let $default_ident = from_local_storage(stringify!($sig), $default);
        let $sig = RwSignal::new($default_ident);
        Effect::new(move || {set_local_storage(stringify!($sig), $sig.get());})
    };
}

fn get_color_code(c: Color) -> String {
    let c: Srgb<u8> = match c {
        Color::RGB(c) => c,
        _ => panic!("AH"),
    }.into_format();
    format!("rgb({}, {}, {})", 
        c.red, 
        c.green, 
        c.blue
    )
}

#[component]
pub fn TeamGenerator(players: RwSignal<Vec<Player>>) -> impl IntoView {

    // Thaw requires this to generate toasts
    let toaster = ToasterInjection::expect_context();

    // Team signals
    let team_a = RwSignal::new(vec![]);
    let team_b = RwSignal::new(vec![]);

    let team_a_color_default = from_local_storage("team_a_color", RGB{red: 255.0, green: 123.0, blue: 0.0});
    let team_a_color = RwSignal::new(Color::from(team_a_color_default.clone().srgb()));
    Effect::new(move || {set_local_storage_color("team_a_color", team_a_color.get());});

    let team_b_color_default = from_local_storage("team_b_color", RGB{red: 0.0, green: 123.0, blue: 255.0});
    let team_b_color = RwSignal::new(Color::from(team_b_color_default.clone().srgb()));
    Effect::new(move || {set_local_storage_color("team_b_color", team_b_color.get());});
    
    local_storage_signal!(team_delta, team_delta_default, 1.0);
    local_storage_signal!(min_defense, min_defense_default, 1);
    local_storage_signal!(min_forward, min_forward_default, 1);
    local_storage_signal!(min_gk, min_gk_default, 1);
    local_storage_signal!(min_midfield, min_midfield_default, 1);

    let update_max_team_delta = move |ev| {
        let v = event_target_value(&ev);
        let number = v.parse().unwrap();
        team_delta.set(number); 
    };
    

    let update_min_defense = move |ev| {
        let v = event_target_value(&ev);
        let number = v.parse().unwrap();
        min_defense.set(number);
    };

    let update_min_forward = move |ev| {
        let v = event_target_value(&ev);
        let number = v.parse().unwrap();
        min_forward.set(number);
    };

    let update_min_gk = move |ev| {
        let v = event_target_value(&ev);
        let number = v.parse().unwrap();
        min_gk.set(number);
    };

    let update_min_midfield = move |ev| {
        let v = event_target_value(&ev);
        let number = v.parse().unwrap();
        min_midfield.set(number);
    };

    let team_gen_event = move |ev: SubmitEvent| {
        ev.prevent_default();
        let players = players.get();
        let mut pos = BTreeMap::new();

        pos.insert("gk".to_string(), min_gk.get());
        pos.insert("df".to_string(), min_defense.get());
        pos.insert("mid".to_string(), min_midfield.get());
        pos.insert("fw".to_string(), min_forward.get());
        
        match get_even_teams(&players, team_delta.get(), &pos) {
            Ok((mut a, mut b)) => {
                a.sort_by(|x, y| x.name.cmp(&y.name));
                b.sort_by(|x, y| x.name.cmp(&y.name));
                team_a.set(a);
                team_b.set(b);
            },
            Err(e) => {
                toaster.dispatch_toast(move || view! {
                    <Toast>
                        <ToastTitle>"Failed To Generate Equal Teams"</ToastTitle>
                        <ToastBody>
                            {e}
                        </ToastBody>
                    </Toast>
                }, Default::default());
            }
        };
    };

    view! {
        <div id="teams">
            // <ConfigProvider>
                <ColorPicker value=team_a_color/>
                <ColorPicker value=team_b_color/>
            // </ConfigProvider>
            <form class="col" on:submit=team_gen_event>
                <div class="row">
                <label for="team-delta-input" class="team-delta-label">Max Team Strength Delta:</label>
                <input
                    id="team-delta-input"
                    type="number"
                    value=team_delta_default
                    on:input=update_max_team_delta
                    class="team-delta-input"
                />
                </div>
                <div class="row">
                <label for="min-fwd-input" class="team-delta-label">Min Forwards:</label>
                <input
                    id="min-fwd-input"
                    type="number"
                    value=min_forward_default
                    on:input=update_min_forward
                    class="team-delta-input"
                />
                </div>
                <div class="row">
                <label for="min-mid-input" class="team-delta-label">Min Midfield:</label>
                <input
                    id="min-mid-input"
                    type="number"
                    value=min_midfield_default
                    on:input=update_min_midfield
                    class="team-delta-input"
                />
                </div>
                <div class="row">
                <label for="min-d-input" class="team-delta-label">Min Defense:</label>
                <input
                    id="min-d-input"
                    type="number"
                    value=min_defense_default
                    on:input=update_min_defense
                    class="team-delta-input"
                />
                </div>
                <div class="row">
                <label for="min-gk-input" class="team-delta-label">Min GK:</label>
                <input
                    id="min-gk-input"
                    type="number"
                    value=min_gk_default
                    on:input=update_min_gk
                    class="team-delta-input"
                />
                </div>
                
                <button type="submit">Generate Teams</button>
            </form>
            <div class="row">
            <table id="generated-teams">
                <tr>
                    <th class="teamgen-h" contenteditable="true"> Team A </th>
                    <th class="teamgen-h" contenteditable="true"> Team B </th>
                </tr>
                { move || team_a.get().into_iter()
                    .zip(team_b.get())
                    .enumerate()
                    .map(|(i, (a, b))| view!{ <tr>
                        <td contenteditable="true" 
                            style:background-color=move || get_color_code(team_a_color.get())
                            style:color=move || RGB::from_color(team_a_color.get()).get_text_color()
                        >{i+1}". "{a.name}</td>
                        <td contenteditable="true" 
                            style:background-color=move || get_color_code(team_b_color.get())
                            style:color=move || RGB::from_color(team_b_color.get()).get_text_color()
                        >{i+1}". "{b.name}</td>
                    </tr> })
                    .collect_view() }
                <tr>
                    <td>"Score: "{ move || team_a.get().iter().map(|p| p.rating - 5.0).sum::<f32>()}</td>
                    <td>"Score: "{ move || team_b.get().iter().map(|p| p.rating - 5.0).sum::<f32>()}</td>
                </tr>
            </table>
            </div>
        </div>
    }
}
