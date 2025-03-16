use serde::{Serialize, Deserialize};
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: String,
    pub rating: f32,
    pub gender: bool,
    pub fixed_team: Option<bool>,
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

        if attending {
            Some(Player {
                name,
                rating,
                gender,
                fixed_team,
            })
        } else {
            None
        }
    }
}

pub fn get_even_teams(players: &[Player], max_delta: f32)  -> (Vec<Player>, Vec<Player>) {
    for _ in 0..100_000 {
        let (a, b) = random_team(players);
        let a_rating: f32 = a.iter().map(|p| p.rating).sum();
        let b_rating: f32 = b.iter().map(|p| p.rating).sum();
        if (a_rating - b_rating).abs() < max_delta {
            return (a, b)
        }
    }
    panic!("Couldn't create a")
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