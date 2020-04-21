use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_06/input.txt")?;
    let reader = BufReader::new(file);

    let direct_orbit_expressions: Vec<DirectOrbitExpression> = reader
        .lines()
        .filter_map(|sr| sr.ok().map(|s| s.parse().ok()).flatten())
        .collect();
    let space_object_result = direct_orbit_expressions.to_space_object()?;

    println!("Depth: {}", space_object_result.depth(0));
    println!(
        "Minimum number of orbital transfers: {:?}",
        space_object_result.transfers(&String::from("YOU"), &String::from("SAN"))
    );

    Ok(())
}

#[derive(Debug, PartialEq)]
struct SpaceObject {
    name: String,
    orbits: Vec<SpaceObject>,
}

impl SpaceObject {
    fn depth(&self, current: i32) -> i32 {
        self.orbits
            .iter()
            .fold(current, |sum, orbit| sum + orbit.depth(current + 1))
    }

    fn contains(&self, destination: &String) -> bool {
        self.orbits
            .iter()
            .any(|o| &o.name == destination || o.contains(destination))
    }

    fn deepest_orbit(&self, from: &String, to: &String) -> Option<&SpaceObject> {
        if self.contains(from) && self.contains(to) {
            for orbit in &self.orbits {
                if let Some(deepest_orbit) = orbit.deepest_orbit(from, to) {
                    return Some(deepest_orbit);
                }
            }

            Some(self)
        } else {
            None
        }
    }

    fn transfers(&self, from: &String, to: &String) -> Option<i32> {
        let deepest = self.deepest_orbit(from, to)?;

        if let (Some(from_distance), Some(to_distance)) =
            (deepest.distance_to(0, from), deepest.distance_to(0, to))
        {
            Some(from_distance + to_distance - 2)
        } else {
            None
        }
    }

    fn distance_to(&self, current: i32, destination: &String) -> Option<i32> {
        if self.orbits.len() == 0 {
            None
        } else {
            self.orbits.iter().find_map(|orbit| {
                if &orbit.name == destination {
                    Some(current + 1)
                } else {
                    orbit.distance_to(current + 1, destination)
                }
            })
        }
    }
}

#[derive(Debug, PartialEq)]
struct DirectOrbitExpression {
    from: String,
    to: String,
}

impl FromStr for DirectOrbitExpression {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        let split_char = trimmed
            .find(')')
            .ok_or("Direct orbit expression should contain ')'")?;

        let (from, to) = trimmed.split_at(split_char);

        Ok(DirectOrbitExpression {
            from: from.to_string(),
            to: to[1..].to_string(),
        })
    }
}

trait ToSpaceObject {
    fn to_space_object(&self) -> Result<SpaceObject, Cow<'static, str>>;
}

impl ToSpaceObject for Vec<DirectOrbitExpression> {
    fn to_space_object(&self) -> Result<SpaceObject, Cow<'static, str>> {
        let mut map = HashMap::<String, Vec<String>>::new();

        for direct_orbit in self.iter() {
            map.entry(direct_orbit.from.clone())
                .and_modify(|destinations| destinations.push(direct_orbit.to.clone()))
                .or_insert(vec![direct_orbit.to.clone()]);
        }

        build_space_object(String::from("COM"), &mut map)
    }
}

fn build_space_object(
    name: String,
    map: &mut HashMap<String, Vec<String>>,
) -> Result<SpaceObject, Cow<'static, str>> {
    if let Some(destinations) = map.remove(&name) {
        let mut space_object = SpaceObject {
            name,
            orbits: Vec::with_capacity(destinations.len()),
        };

        for destination in destinations {
            space_object
                .orbits
                .push(build_space_object(destination, map)?);
        }

        Ok(space_object)
    } else {
        Ok(SpaceObject {
            name,
            orbits: Vec::with_capacity(0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_direct_orbit_correct() {
        let input = "COM)B";

        let direct_orbit_result = input.parse();

        assert_eq!(
            direct_orbit_result,
            Ok(DirectOrbitExpression {
                from: String::from("COM"),
                to: String::from("B")
            })
        );
    }

    #[test]
    fn parse_direct_orbit_incorrect() {
        let input = "COMB";

        let direct_orbit_result = input.parse::<DirectOrbitExpression>();

        assert!(direct_orbit_result.is_err());
    }

    #[test]
    fn parse_example_map() {
        let input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ];

        let direct_orbit_expressions: Vec<DirectOrbitExpression> =
            input.iter().filter_map(|s| s.parse().ok()).collect();
        let space_object_result = direct_orbit_expressions.to_space_object();

        assert_eq!(
            space_object_result,
            Ok(SpaceObject {
                name: String::from("COM"),
                orbits: vec![SpaceObject {
                    name: String::from("B"),
                    orbits: vec![
                        SpaceObject {
                            name: String::from("C"),
                            orbits: vec![SpaceObject {
                                name: String::from("D"),
                                orbits: vec![
                                    SpaceObject {
                                        name: String::from("E"),
                                        orbits: vec![
                                            SpaceObject {
                                                name: String::from("F"),
                                                orbits: vec![]
                                            },
                                            SpaceObject {
                                                name: String::from("J"),
                                                orbits: vec![SpaceObject {
                                                    name: String::from("K"),
                                                    orbits: vec![SpaceObject {
                                                        name: String::from("L"),
                                                        orbits: vec![]
                                                    }]
                                                }]
                                            }
                                        ]
                                    },
                                    SpaceObject {
                                        name: String::from("I"),
                                        orbits: vec![]
                                    }
                                ]
                            }]
                        },
                        SpaceObject {
                            name: String::from("G"),
                            orbits: vec![SpaceObject {
                                name: String::from("H"),
                                orbits: vec![]
                            }]
                        }
                    ]
                }]
            })
        );
    }

    #[test]
    fn example_depth() {
        let space_object = SpaceObject {
            name: String::from("COM"),
            orbits: vec![SpaceObject {
                name: String::from("B"),
                orbits: vec![
                    SpaceObject {
                        name: String::from("C"),
                        orbits: vec![SpaceObject {
                            name: String::from("D"),
                            orbits: vec![
                                SpaceObject {
                                    name: String::from("E"),
                                    orbits: vec![
                                        SpaceObject {
                                            name: String::from("F"),
                                            orbits: vec![],
                                        },
                                        SpaceObject {
                                            name: String::from("J"),
                                            orbits: vec![SpaceObject {
                                                name: String::from("K"),
                                                orbits: vec![SpaceObject {
                                                    name: String::from("L"),
                                                    orbits: vec![],
                                                }],
                                            }],
                                        },
                                    ],
                                },
                                SpaceObject {
                                    name: String::from("I"),
                                    orbits: vec![],
                                },
                            ],
                        }],
                    },
                    SpaceObject {
                        name: String::from("G"),
                        orbits: vec![SpaceObject {
                            name: String::from("H"),
                            orbits: vec![],
                        }],
                    },
                ],
            }],
        };

        let expected = 42;
        let actual = space_object.depth(0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn part_2_example_distance_to() {
        let input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];

        let direct_orbit_expressions: Vec<DirectOrbitExpression> =
            input.iter().filter_map(|s| s.parse().ok()).collect();
        let space_object = direct_orbit_expressions
            .to_space_object()
            .expect("Should be valid");

        assert_eq!(space_object.distance_to(0, &String::from("B")), Some(1));
        assert_eq!(space_object.distance_to(0, &String::from("C")), Some(2));
        assert_eq!(space_object.distance_to(0, &String::from("YOU")), Some(7));
    }

    #[test]
    fn part_2_example_contains() {
        let input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];

        let direct_orbit_expressions: Vec<DirectOrbitExpression> =
            input.iter().filter_map(|s| s.parse().ok()).collect();
        let space_object = direct_orbit_expressions
            .to_space_object()
            .expect("Should be valid");

        assert!(space_object.contains(&String::from("B")));
        assert!(space_object.contains(&String::from("YOU")));
        assert!(!space_object.contains(&String::from("COM")));
    }

    #[test]
    fn part_2_example_deepest_orbit_you_san() {
        let input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];

        let direct_orbit_expressions: Vec<DirectOrbitExpression> =
            input.iter().filter_map(|s| s.parse().ok()).collect();
        let space_object = direct_orbit_expressions
            .to_space_object()
            .expect("Should be valid");

        let deepest_orbit = space_object
            .deepest_orbit(&String::from("YOU"), &String::from("SAN"))
            .expect("Should find an orbit");

        assert_eq!(deepest_orbit.name, String::from("D"));
    }

    #[test]
    fn part_2_example() {
        let input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];

        let direct_orbit_expressions: Vec<DirectOrbitExpression> =
            input.iter().filter_map(|s| s.parse().ok()).collect();
        let space_object = direct_orbit_expressions
            .to_space_object()
            .expect("Should be valid");

        assert_eq!(
            space_object.transfers(&String::from("YOU"), &String::from("SAN")),
            Some(4)
        );
    }
}
