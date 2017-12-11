use std::collections::BTreeMap;

struct Team {
    name: String,
    win: u8,
    draw: u8,
    loss: u8,
    played: u8,
    score: u8,
}

impl Team {
    fn to_row(&self) -> String {
        create_row(
            &[
                &self.name,
                &self.played.to_string(),
                &self.win.to_string(),
                &self.draw.to_string(),
                &self.loss.to_string(),
                &self.score.to_string(),
            ],
        ).unwrap()
    }
}

pub fn tally(input: &str) -> String {
    let mut result = create_row(&["Team", "MP", "W", "D", "L", "P"]).unwrap();
    let mut teams: BTreeMap<String, Team> = BTreeMap::new();

    input.lines().for_each(|tournament| {
        let items: Vec<&str> = tournament.split_terminator(';').collect();
        score(&items, &mut teams);
    });

    let mut sort_by_point = teams.values().collect::<Vec<&Team>>();
    sort_by_point.sort_by(|a, b| b.score.cmp(&a.score));

    sort_by_point.iter().for_each(
        |team| result.push_str(&team.to_row()),
    );

    result.trim_right_matches('\n').to_string()
}

fn get_team<'a>(name: &str, teams: &'a mut BTreeMap<String, Team>) -> &'a mut Team {
    teams.entry(name.to_string()).or_insert(Team {
        name: name.to_string(),
        win: 0,
        draw: 0,
        loss: 0,
        played: 0,
        score: 0,
    })
}

fn score(items: &Vec<&str>, teams: &mut BTreeMap<String, Team>) {
    match items[2] {
        "win" => {
            win(items[0], teams);
            loss(items[1], teams);
        }
        "draw" => {
            draw(items[0], teams);
            draw(items[1], teams);
        }
        "loss" => {
            loss(items[0], teams);
            win(items[1], teams);
        }
        _ => (),
    }
}

fn win(name: &str, teams: &mut BTreeMap<String, Team>) {
    let team = get_team(name, teams);
    team.win += 1;
    team.played += 1;
    team.score += 3;
}

fn draw(name: &str, teams: &mut BTreeMap<String, Team>) {
    let team = get_team(name, teams);
    team.draw += 1;
    team.played += 1;
    team.score += 1;
}

fn loss(name: &str, teams: &mut BTreeMap<String, Team>) {
    let team = get_team(name, teams);
    team.loss += 1;
    team.played += 1;
}

fn create_row(lines: &[&str]) -> Result<String, &'static str> {
    if lines.len() != 6 {
        Err("error row!")
    } else {
        Ok(format!(
            "{:<31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}\n",
            lines[0],
            lines[1],
            lines[2],
            lines[3],
            lines[4],
            lines[5]
        ))
    }
}
