use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct SpaceObject {
    name: String,
    orbits: Vec<SpaceObject>,
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
}
