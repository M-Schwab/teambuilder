use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use gloo_net::http::Request;

use crate::teamgen::Player;

#[component]
pub fn Players(players: RwSignal<Vec<Player>>) -> impl IntoView {

    let (player_sheet_url, set_player_sheet_url) = signal(String::new());

    let update_player_sheet_url = move |ev| {
        let v = event_target_value(&ev);
        set_player_sheet_url.set(v);
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
            players.set(lines.filter_map(|l| Player::from_row(l)).collect());
        });
    };

    view! {
        <div class="col">
            <form class="row" on:submit=players_event>
                <input
                    id="team-sheet-input"
                    placeholder="Enter team google sheet url..."
                    on:input=update_player_sheet_url
                />
                <button type="submit">"Refresh player list"</button>
            </form>
            <h3>" Participating Players: ("{ move || players.get().len() }")"</h3>
            <table id="player-listing">
                <tr>
                    <th> Name </th><th> Rating </th><th> Gender </th><th> Team Lock </th><th> Position </th>
                </tr>
                { move || players.get().into_iter()
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
                        <td>{ move || match &p.position {
                            Some(arr) => serde_json::to_string(arr).unwrap(),
                            None => "".to_string(),
                        }}</td>
                    </tr> })
                    .collect_view() }
            </table>
        </div>
    }
}
