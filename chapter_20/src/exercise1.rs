use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Player {
    first_name: String,
    last_name: String,
    _team: String,
}

type FullName = String;

impl Player {
    pub fn full_name(&self) -> FullName {
        format!("{} {}", self.first_name, self.last_name)
    }
}

pub fn find_same_players(p1: &[Player], p2: &[Player]) -> Vec<FullName> {
    let mut names1 = HashSet::<FullName>::with_capacity(p1.len());
    for p in p1 {
        names1.insert(p.full_name());
    }

    let mut result = vec![];
    for p in p2 {
        let full_name = p.full_name();
        if names1.contains(&full_name) {
            result.push(full_name)
        }
    }

    result
}

pub(crate) mod sample {
    use super::*;

    pub fn basketball_players() -> Vec<Player> {
        vec![
            Player {
                first_name: "Jill".into(),
                last_name: "Huang".into(),
                _team: "Gators".into(),
            },
            Player {
                first_name: "Janko".into(),
                last_name: "Barton".into(),
                _team: "Sharks".into(),
            },
            Player {
                first_name: "Wanda".into(),
                last_name: "Vakulskas".into(),
                _team: "Sharks".into(),
            },
            Player {
                first_name: "Jill".into(),
                last_name: "Moloney".into(),
                _team: "Gators".into(),
            },
            Player {
                first_name: "Luuk".into(),
                last_name: "Watkins".into(),
                _team: "Gators".into(),
            },
        ]
    }
    pub fn football_players() -> Vec<Player> {
        vec![
            Player {
                first_name: "Hanzla".into(),
                last_name: "Radosti".into(),
                _team: "32ers".into(),
            },
            Player {
                first_name: "Tina".into(),
                last_name: "Watkins".into(),
                _team: "Barleycorns".into(),
            },
            Player {
                first_name: "Alex".into(),
                last_name: "Patel".into(),
                _team: "32ers".into(),
            },
            Player {
                first_name: "Jill".into(),
                last_name: "Huang".into(),
                _team: "Barleycorns".into(),
            },
            Player {
                first_name: "Wanda".into(),
                last_name: "Vakulskas".into(),
                _team: "Barleycorns".into(),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_same_players() {
        let p1 = Player {
            first_name: "A".into(),
            last_name: "B".into(),
            _team: "C".into(),
        };
        let p2 = Player {
            first_name: "M".into(),
            last_name: "N".into(),
            _team: "O".into(),
        };
        let p3 = Player {
            first_name: "X".into(),
            last_name: "Y".into(),
            _team: "Z".into(),
        };

        let basketball_players = vec![p1, p2.clone()];
        let football_players = vec![p2.clone(), p3];

        let same = find_same_players(&basketball_players, &football_players);
        assert_eq!(same.len(), 1);

        assert_eq!(same.first().unwrap(), &p2.full_name());
    }
}
