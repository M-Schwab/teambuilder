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

pub struct Team<'a> {
    pub players: Vec<&'a Player>,
    pub half_player: Option<&'a Player>,
}

impl<'a> Team<'a> {
    pub fn owned(&'a self, half: usize) -> Vec<Player> {
        let mut players: Vec<Player> = self.players.iter()
            .map(|&p| p.clone())
            .collect();

        match self.half_player {
            Some(p) => {
                let name = if half == 1 {
                    format!("{} (1st half)", p.name)
                } else if half == 2 {
                    format!("{} (2nd half)", p.name)
                } else {
                    panic!("???? got half {half}");
                };
                let p = Player {
                    name,
                    fixed_team: p.fixed_team,
                    gender: p.gender,
                    rating: p.rating,
                    position: p.position.clone(),
                };
                players.push(p);
            },
            None => {},
        };
        players
    }
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

fn rating(team: &Team<'_>) -> f32 {
    let rating: f32 = team.players.iter().map(|p| p.rating).sum();
    rating + team.half_player.map(|p| p.rating).unwrap_or(0.0)
}

fn min_pos_met(team: &Vec<&Player>, min_positions: &BTreeMap<String, usize>) -> bool {
    min_positions.iter()
        .all(|(pos, req)| {
            let pos_count = team.iter().filter(|p| match &p.position {
                Some(v) => v.iter().any(|p_pos| p_pos == pos),
                None => false,
            })
            .count();
            pos_count >= *req
        })
}

pub fn get_even_teams(players: &[Player], max_delta: f32, min_positions: &BTreeMap<String, usize>)  -> Result<(Vec<Player>, Vec<Player>), String> {
    for _ in 0..100_000 {
        let (a, b) = random_team(players);
        if (rating(&a) - rating(&b)).abs() < max_delta {
            // The shared player doesn't count for meeting the minimum position counts.
            if min_pos_met(&a.players, min_positions) && min_pos_met(&b.players, min_positions) {
                return Ok((a.owned(1), b.owned(2)))
            }
        }
    }
    Err(format!("Unable to generate even teams.  You may need to loosen the requirements for each team."))
}


pub fn random_team<'a>(players: &'a [Player]) -> (Team<'a>, Team<'a>) {

    let player_count = players.len();

    let mut team_a = players.iter()
        .filter(|p| p.fixed_team.is_some_and(|f| f))
        .collect::<Vec<_>>();

    let mut team_b = players.iter()
        .filter(|p| p.fixed_team.is_some_and(|f| !f))
        .collect::<Vec<_>>();

    let mut players = players.iter()
        .filter(|p| p.fixed_team.is_none())
        .collect::<Vec<_>>();

    players.shuffle(&mut thread_rng());
    
    while team_a.len() < player_count / 2 {
        team_a.push(players.pop().unwrap())
    }

    while team_b.len() < player_count / 2 {
        team_b.push(players.pop().unwrap())
    }

    
    let half_player = players.pop();
    (
        Team { players: team_a, half_player },
        Team { players: team_b, half_player }
    )
}