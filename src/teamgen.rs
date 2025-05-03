use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};
use rand::thread_rng;
use rand::seq::SliceRandom;


#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: String,
    pub rating: f32,
    pub gender: bool,
    pub fixed_team: Option<bool>,
    pub position: Option<Vec<String>>,
}

impl Player {
    pub fn from_row(value: &'_ str) -> Option<Self> {
        let mut player_fields = value.split(",");

        let mut name = player_fields.next().unwrap().to_string();
        name.push(' ');
        name.push_str(player_fields.next().unwrap());
        let gender = player_fields.next().unwrap() == "F";
        let rating = player_fields.next().unwrap().parse().unwrap();

        let attending = player_fields.next().unwrap() == "y";

        let fixed_team = if let Some(s) = player_fields.next() {
            if s == "A" {
                Some(true)
            } else if s == "B" {
                Some(false)
            } else {
                None
            }
        } else {
            None
        };

        let position = if let Some(s) = player_fields.next() {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                Some(
                    s.split('/')
                        .into_iter()
                        .map(|s| s.trim().to_lowercase())
                        .collect()
                )
            }
        } else {
            None
        };

        if attending {
            Some(Player {
                name,
                rating,
                gender,
                fixed_team,
                position,
            })
        } else {
            None
        }
    }
}

fn rating(team: &Vec<Player>) -> f32 {
    team.iter().map(|p| p.rating).sum()
}

fn min_pos_met(team: &Vec<Player>, min_positions: &BTreeMap<String, usize>) -> bool {
    min_positions.iter()
        .all(|(pos, req)| {
            let gk_ok = if pos == "gk" {
                !team.iter().any(|p| match &p.position {
                    Some(v) => v.iter().any(|p_pos| p_pos == pos) && p.name.ends_with("(1st half)"),
                    None => false
                })
            } else {
                true
            };
            let pos_count = team.iter().filter(|p| match &p.position {
                Some(v) => v.iter().any(|p_pos| p_pos == pos),
                None => false,
            })
            .count();

            pos_count >= *req && gk_ok
        })
}

pub fn get_even_teams(players: &[Player], max_delta: f32, min_positions: &BTreeMap<String, usize>)  -> Result<(Vec<Player>, Vec<Player>), String> {
    for _ in 0..50_000 {
        let (a, b) = random_team(players);
        if (rating(&a) - rating(&b)).abs() < max_delta {
            if min_pos_met(&a, min_positions) && min_pos_met(&b, min_positions) {
                return Ok((a, b))
            }
        }
    }
    Err(format!("Unable to generate even teams.  You may need to loosen the requirements for each team."))
}


pub fn random_team(players: &[Player]) -> (Vec<Player>, Vec<Player>) {

    let player_count = players.len();

    let mut team_a = players.iter()
        .filter(|p| p.fixed_team.is_some_and(|f| f))
        .map(|p| p.clone())
        .collect::<Vec<_>>();

    let mut team_b = players.iter()
        .filter(|p| p.fixed_team.is_some_and(|f| !f))
        .map(|p| p.clone())
        .collect::<Vec<_>>();

    let mut players = players.iter()
        .filter(|p| p.fixed_team.is_none())
        .map(|p| p.clone())
        .collect::<Vec<_>>();

    players.shuffle(&mut thread_rng());
    
    while team_a.len() < player_count / 2 {
        team_a.push(players.pop().unwrap())
    }

    while team_b.len() < player_count / 2 {
        team_b.push(players.pop().unwrap())
    }

    if players.len() == 1 {
        let mut shared_player = players.pop().unwrap();
        let player_name = shared_player.name.clone();

        let mut copy =  shared_player.clone();
        copy.name = format!("{player_name} (1st half)");
        shared_player.name = format!("{player_name} (2nd half)");
        team_a.push(copy);
        team_b.push(shared_player);
    }

    (team_a, team_b)
}