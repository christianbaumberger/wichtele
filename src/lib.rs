use rand::rng;
use rand::seq::SliceRandom;
use regex::Regex;
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}
#[derive(Clone, PartialEq)]
pub struct Assignment {
    pub person1: Person,
    pub person2: Person,
}
pub fn get_valid_assignment(content: &str) -> Vec<Assignment> {
    let persons = parse_names(content);
    let mut assignments;
    loop {
        assignments = generate_assignment(&persons);
        match check_assignments(&assignments) {
            Ok(()) => break,
            Err(msg) => println!("{}", msg),
        }
    }
    assignments
}

fn check_assignments(assignments: &Vec<Assignment>) -> Result<(), String> {
    let result = check_person_to_itself(&assignments);
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    let result = check_family(&assignments);
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    let result = check_full_circle(&assignments);
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    let result = check_family_to_family(&assignments);
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    Ok(())
}

fn check_person_to_itself(assignments: &Vec<Assignment>) -> Result<(), String> {
    for assignment in assignments {
        if assignment.person1 == assignment.person2 {
            return Err(format!(
                "Conflict: Itself with {}, {}",
                assignment.person1.first_name, assignment.person1.last_name
            )
            .to_string());
        }
    }
    Ok(())
}

fn check_family(assignments: &Vec<Assignment>) -> Result<(), String> {
    for assignment in assignments {
        if assignment.person1.last_name == assignment.person2.last_name {
            return Err(format!(
                "Conflict: Family {} {} with {} {}",
                assignment.person1.first_name,
                assignment.person1.last_name,
                assignment.person2.first_name,
                assignment.person2.last_name
            )
            .to_string());
        }
    }
    Ok(())
}

fn check_full_circle(assignments: &Vec<Assignment>) -> Result<(), String> {
    let first_person = assignments[0].person1.clone();
    let mut current_person = first_person.clone();
    let mut counter = 0;
    loop {
        counter += 1;
        let assignment = assignments
            .iter()
            .find(|assignemt| assignemt.person1 == current_person);
        if let Some(assignment) = assignment {
            if assignment.person2 == first_person {
                break;
            }
            current_person = assignment.person2.clone();
        }
    }
    if counter != assignments.len() {
        return Err(format!(
            "Conflict: Should have one large circle, but has len {}/{}",
            counter,
            assignments.len()
        ));
    }
    Ok(())
}

fn check_family_to_family(assignments: &Vec<Assignment>) -> Result<(), String> {
    let mut family_map: HashMap<String, i32> = HashMap::new();
    for assignment in assignments {
        let family_key =
            assignment.person1.last_name.clone() + "_" + &assignment.person2.last_name.clone();
        if family_map.contains_key(&family_key) {
            let current_value = family_map.get(&family_key).unwrap().clone();
            family_map.insert(family_key, current_value + 1);
        } else {
            family_map.insert(family_key, 1);
        }
    }
    println!("Family map: {:?}", family_map);
    let counts: Vec<&i32> = family_map.values().clone().collect();
    for count in counts {
        if *count > 1 {
            return Err("Conflict: family to family".to_string());
        }
    }
    Ok(())
}

fn generate_assignment(persons: &Vec<Person>) -> Vec<Assignment> {
    let mut persons_shuffled = persons.clone();
    persons_shuffled.shuffle(&mut rng());

    let mut result = Vec::new();
    for (person1, person2) in persons.iter().zip(persons_shuffled) {
        let assignment = Assignment {
            person1: person1.clone(),
            person2: person2.clone(),
        };
        result.push(assignment);
    }
    result
}

fn parse_names(content: &str) -> Vec<Person> {
    let re = Regex::new(r"(?<firstname>\w+)\s(?<lastname>\w+)").unwrap();
    let mut persons: Vec<Person> = Vec::new();
    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            let first_name = caps["firstname"].parse::<String>();
            let last_name = caps["lastname"].parse::<String>();
            let person = Person {
                first_name: first_name.unwrap(),
                last_name: last_name.unwrap(),
            };
            persons.push(person);
        }
    }
    persons
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_person_to_itself() {
        let assignment1 = Assignment {
            person1: Person {
                first_name: "FirstName1".to_string(),
                last_name: "LastName1".to_string(),
            },
            person2: Person {
                first_name: "FirstName2".to_string(),
                last_name: "LastName2".to_string(),
            },
        };
        let assignment2 = Assignment {
            person1: Person {
                first_name: "FirstName2".to_string(),
                last_name: "LastName2".to_string(),
            },
            person2: Person {
                first_name: "FirstName1".to_string(),
                last_name: "LastName1".to_string(),
            },
        };
        let assignment3 = Assignment {
            person1: Person {
                first_name: "FirstName3".to_string(),
                last_name: "LastName3".to_string(),
            },
            person2: Person {
                first_name: "FirstName3".to_string(),
                last_name: "LastName3".to_string(),
            },
        };
        let mut assignments = Vec::new();
        assignments.push(assignment1);
        assignments.push(assignment2);
        assignments.push(assignment3);

        assert_eq!(
            Err("Conflict: Itself with FirstName3, LastName3".to_string()),
            check_person_to_itself(&assignments)
        );
    }

    #[test]
    fn test_check_family() {
        let assignment1 = Assignment {
            person1: Person {
                first_name: "FirstName1".to_string(),
                last_name: "LastName1".to_string(),
            },
            person2: Person {
                first_name: "FirstName2".to_string(),
                last_name: "LastName2".to_string(),
            },
        };
        let assignment2 = Assignment {
            person1: Person {
                first_name: "FirstName2".to_string(),
                last_name: "LastName2".to_string(),
            },
            person2: Person {
                first_name: "FirstName3".to_string(),
                last_name: "LastName2".to_string(),
            },
        };
        let assignment3 = Assignment {
            person1: Person {
                first_name: "FirstName3".to_string(),
                last_name: "LastName2".to_string(),
            },
            person2: Person {
                first_name: "FirstName1".to_string(),
                last_name: "LastName1".to_string(),
            },
        };
        let mut assignments = Vec::new();
        assignments.push(assignment1);
        assignments.push(assignment2);
        assignments.push(assignment3);

        assert_eq!(
            Err("Conflict: Family FirstName2 LastName2 with FirstName3 LastName2".to_string()),
            check_family(&assignments)
        );
    }
    #[test]
    fn test_check_two_circle() {
        let assignment1 = Assignment {
            person1: Person {
                first_name: "FirstName1".to_string(),
                last_name: "LastName1".to_string(),
            },
            person2: Person {
                first_name: "FirstName2".to_string(),
                last_name: "LastName2".to_string(),
            },
        };
        let assignment2 = Assignment {
            person1: Person {
                first_name: "FirstName2".to_string(),
                last_name: "LastName2".to_string(),
            },
            person2: Person {
                first_name: "FirstName1".to_string(),
                last_name: "LastName1".to_string(),
            },
        };
        let assignment3 = Assignment {
            person1: Person {
                first_name: "FirstName3".to_string(),
                last_name: "LastName3".to_string(),
            },
            person2: Person {
                first_name: "FirstName4".to_string(),
                last_name: "LastName4".to_string(),
            },
        };
        let assignment4 = Assignment {
            person1: Person {
                first_name: "FirstName4".to_string(),
                last_name: "LastName4".to_string(),
            },
            person2: Person {
                first_name: "FirstName3".to_string(),
                last_name: "LastName3".to_string(),
            },
        };
        let mut assignments = Vec::new();
        assignments.push(assignment1);
        assignments.push(assignment2);
        assignments.push(assignment3);
        assignments.push(assignment4);

        assert_eq!(
            Err("Conflict: Should have one large circle, but has len 2/4".to_string()),
            check_full_circle(&assignments)
        );
    }
    #[test]
    fn test_check_full_circle() {
        let assignment1 = Assignment {
            person1: Person {
                first_name: "FirstName1".to_string(),
                last_name: "LastName1".to_string(),
            },
            person2: Person {
                first_name: "FirstName2".to_string(),
                last_name: "LastName2".to_string(),
            },
        };
        let assignment2 = Assignment {
            person1: Person {
                first_name: "FirstName2".to_string(),
                last_name: "LastName2".to_string(),
            },
            person2: Person {
                first_name: "FirstName3".to_string(),
                last_name: "LastName3".to_string(),
            },
        };
        let assignment3 = Assignment {
            person1: Person {
                first_name: "FirstName3".to_string(),
                last_name: "LastName3".to_string(),
            },
            person2: Person {
                first_name: "FirstName4".to_string(),
                last_name: "LastName4".to_string(),
            },
        };
        let assignment4 = Assignment {
            person1: Person {
                first_name: "FirstName4".to_string(),
                last_name: "LastName4".to_string(),
            },
            person2: Person {
                first_name: "FirstName1".to_string(),
                last_name: "LastName1".to_string(),
            },
        };
        let mut assignments = Vec::new();
        assignments.push(assignment1);
        assignments.push(assignment2);
        assignments.push(assignment3);
        assignments.push(assignment4);

        assert_eq!(Ok(()), check_full_circle(&assignments));
    }
    #[test]
    fn test_check_family_to_family() {
        let assignment1 = Assignment {
            person1: Person {
                first_name: "FirstName1".to_string(),
                last_name: "LastName1".to_string(),
            },
            person2: Person {
                first_name: "FirstName2".to_string(),
                last_name: "LastName2".to_string(),
            },
        };
        let assignment2 = Assignment {
            person1: Person {
                first_name: "FirstName2".to_string(),
                last_name: "LastName1".to_string(),
            },
            person2: Person {
                first_name: "FirstName1".to_string(),
                last_name: "LastName2".to_string(),
            },
        };
        let mut assignments = Vec::new();
        assignments.push(assignment1);
        assignments.push(assignment2);

        assert_eq!(
            Err("Conflict: family to family".to_string()),
            check_family_to_family(&assignments)
        );
    }
}
