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

    let update_max_team_delta = move |ev| {
        let v = event_target_value(&ev);
        let number = v.parse().unwrap();
        team_delta.set(number);
    };

    let team_gen_event = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local( async move {
            let players = players.get();
            let (a, b) = get_even_teams(&players, team_delta.get());

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
            <form class="row" on:submit=team_gen_event>
                <label for="team-delta-input" class="team-delta-label">Max Team Strength Delta:</label>
                <input
                    id="team-delta-input"
                    type="number"
                    value="1.0"
                    on:input=update_max_team_delta
                    class="team-delta-input"
                />
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
                        <td style:background-color=move || get_color_code(team_a_color.get())>{i+1}". "{a.name}</td>
                        <td style:background-color=move || get_color_code(team_b_color.get())>{i+1}". "{b.name}</td>
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
