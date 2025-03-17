use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use gloo_net::http::Request;
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
pub fn App() -> impl IntoView {

    let (player_sheet_url, set_player_sheet_url) = signal(String::new());
    let (players_sig, set_players) = signal(vec![]);

    // Team signals
    let (get_team_a, set_team_a) = signal(vec![]);
    let (get_team_b, set_team_b) = signal(vec![]);

    let team_a_color = RwSignal::new(Color::from(Srgb::new(255.0, 123.0, 0.0)));
    let team_b_color = RwSignal::new(Color::from(Srgb::new(0.0, 123.0, 255.0)));
    let team_delta = RwSignal::new(1.0);

    let update_player_sheet_url = move |ev| {
        let v = event_target_value(&ev);
        set_player_sheet_url.set(v);
    };

    let update_max_team_delta = move |ev| {
        let v = event_target_value(&ev);
        let number = v.parse().unwrap();
        team_delta.set(number);
    };

    let players_event = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let mut player_sheet_url = player_sheet_url.get_untracked();
            if player_sheet_url.is_empty() {
                player_sheet_url.push_str("https://docs.google.com/spreadsheets/d/14dwWyphNZViiwSOMsvWCGZYuPfuRSqy1FbaVhwlZpj0/edit?gid=0#gid=0");
            }

            let csv_export_url = "https://docs.google.com/spreadsheets/d/14dwWyphNZViiwSOMsvWCGZYuPfuRSqy1FbaVhwlZpj0/export?format=csv&gid=0";

            let csv_resp = Request::get(csv_export_url)
                .send()
                .await.unwrap();

            let players_csv = csv_resp.text().await.unwrap();

            let mut lines = players_csv.split("\n");

            while let Some(l) = lines.next() {
                if l.starts_with("fName") {
                    break;
                }
            }
            set_players.set(lines.filter_map(|l| Player::from_row(l)).collect());
        });
    };

    let team_gen_event = move |ev: SubmitEvent| {
        //TODO: implement me!
        ev.prevent_default();
        spawn_local( async move {
            let players = players_sig.get();
            let (a, b) = get_even_teams(&players, team_delta.get());

            set_team_a.set(a);
            set_team_b.set(b);
        });
    };

    view! {
        <main class="container">
            <h1>"Team Builder"</h1>
            <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>

            <form class="row" on:submit=players_event>
                <input
                    id="team-sheet-input"
                    placeholder="Enter team google sheet url..."
                    on:input=update_player_sheet_url
                />
                <button type="submit">"Refresh player list"</button>
            </form>

            <div class="row">
                <div>
                    <h3>" Participating Players: ("{ move || players_sig.get().len() }")"</h3>
                    <table id="player-listing">
                        <tr>
                            <th> Name </th><th> Rating </th><th> Gender </th><th> Team Lock </th>
                        </tr>
                        { move || players_sig.get().into_iter()
                            .map(|p| view!{ <tr>
                                <td>{p.name}</td>
                                <td>{p.rating}</td>
                                <td>{ move || if p.gender {"F"} else {"M"} }</td>
                                <td>{ move || {
                                    if let Some(fixed) = p.fixed_team {
                                        if fixed {
                                            "A"
                                        } else {
                                            "B"
                                        }
                                    } else {
                                        ""
                                    }
                                } }</td>
                            </tr> })
                            .collect_view() }
                    </table>
                </div>
                <div id="teams">
                    <ConfigProvider>
                        <ColorPicker value=team_a_color/>
                        <ColorPicker value=team_b_color/>
                    </ConfigProvider>
                    <form class="row" on:submit=team_gen_event>
                        <label for="team-delta-input">Max Team Strength Delta:</label>
                        <input
                            id="team-delta-input"
                            type="number"
                            value="1.0"
                            on:input=update_max_team_delta
                        />
                        <button type="submit">Generate Teams</button>
                    </form>
                    <div class="row">
                    <div>
                    <table id="generated-teams">
                        <tr>
                            <th> Team A </th><th> Team B </th>
                        </tr>
                        { move || get_team_a.get().into_iter()
                            .zip(get_team_b.get())
                            .map(|(a, b)| view!{ <tr>
                                <td style:background-color=move || get_color_code(team_a_color.get())>{a.name}</td>
                                <td style:background-color=move || get_color_code(team_b_color.get())>{b.name}</td>
                            </tr> })
                            .collect_view() }
                        <tr>
                            <td>"Score: "{ move || get_team_a.get().iter().map(|p| p.rating - 5.0).sum::<f32>()}</td>
                            <td>"Score: "{ move || get_team_b.get().iter().map(|p| p.rating - 5.0).sum::<f32>()}</td>
                        </tr>
                    </table>
                    </div>
                    </div>
                </div>
            </div>
        </main>
    }
}
