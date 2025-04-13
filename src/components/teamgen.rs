use std::collections::BTreeMap;

use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use thaw::{ColorPicker, ConfigProvider};
use thaw::Color;
use palette::Srgb;

use crate::teamgen::{Player, get_even_teams};

fn get_color_code(c: Color) -> String {
    let c: Srgb<u8> = match c {
        Color::RGB(c) => c,
        _ => panic!("AHHH"),
    }.into_format();
    format!("rgb({}, {}, {})", 
        c.red, 
        c.green, 
        c.blue
    )
}

#[component]
pub fn TeamGenerator(players: RwSignal<Vec<Player>>) -> impl IntoView {

    // Team signals
    let team_a = RwSignal::new(vec![]);
    let team_b = RwSignal::new(vec![]);

    let team_a_color = RwSignal::new(Color::from(Srgb::new(255.0, 123.0, 0.0)));
    let team_b_color = RwSignal::new(Color::from(Srgb::new(0.0, 123.0, 255.0)));
    let team_delta = RwSignal::new(1.0);

    let min_defense: RwSignal<usize> = RwSignal::new(1);
    let min_forward = RwSignal::new(1);
    let min_gk = RwSignal::new(1);
    let min_midfield = RwSignal::new(1);

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
        spawn_local( async move {
            let players = players.get();
            let mut pos = BTreeMap::new();

            pos.insert("gk".to_string(), min_gk.get());
            pos.insert("df".to_string(), min_defense.get());
            pos.insert("mid".to_string(), min_midfield.get());
            pos.insert("fw".to_string(), min_forward.get());
            
            let (a, b) = get_even_teams(&players, team_delta.get(), &pos);
            team_a.set(a);
            team_b.set(b);
        });
    };

    view! {
        <div id="teams">
            <ConfigProvider>
                <ColorPicker value=team_a_color/>
                <ColorPicker value=team_b_color/>
            </ConfigProvider>
            <form class="col" on:submit=team_gen_event>
                <div class="row">
                <label for="team-delta-input" class="team-delta-label">Max Team Strength Delta:</label>
                <input
                    id="team-delta-input"
                    type="number"
                    value="1.0"
                    on:input=update_max_team_delta
                    class="team-delta-input"
                />
                </div>
                <div class="row">
                <label for="min-fwd-input" class="team-delta-label">Min Forwards:</label>
                <input
                    id="min-fwd-input"
                    type="number"
                    value="1"
                    on:input=update_min_forward
                    class="team-delta-input"
                />
                </div>
                <div class="row">
                <label for="min-mid-input" class="team-delta-label">Min Midfield:</label>
                <input
                    id="min-mid-input"
                    type="number"
                    value="0"
                    on:input=update_min_midfield
                    class="team-delta-input"
                />
                </div>
                <div class="row">
                <label for="min-d-input" class="team-delta-label">Min Defense:</label>
                <input
                    id="min-d-input"
                    type="number"
                    value="2"
                    on:input=update_min_defense
                    class="team-delta-input"
                />
                </div>
                <div class="row">
                <label for="min-gk-input" class="team-delta-label">Min GK:</label>
                <input
                    id="min-gk-input"
                    type="number"
                    value="1"
                    on:input=update_min_gk
                    class="team-delta-input"
                />
                </div>
                
                <button type="submit">Generate Teams</button>
            </form>
            <div class="row">
            <table id="generated-teams">
                <tr>
                    <th class="teamgen-h"> Team A </th><th class="teamgen-h"> Team B </th>
                </tr>
                { move || team_a.get().into_iter()
                    .zip(team_b.get())
                    .enumerate()
                    .map(|(i, (a, b))| view!{ <tr>
                        <td contenteditable="true" style:background-color=move || get_color_code(team_a_color.get())>{i+1}". "{a.name}</td>
                        <td contenteditable="true" style:background-color=move || get_color_code(team_b_color.get())>{i+1}". "{b.name}</td>
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
