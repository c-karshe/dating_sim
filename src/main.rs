extern crate rand;
use std::io;
use rand::seq::SliceRandom;
use std::collections::HashMap;

//setting up structure for game
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Target {
    HeatingPad,
    CapriSun,
    Strawberry,
}

struct Map {
    rooms: Vec<Room>,
}

struct Room {
    description: String,
    options: Vec<Choice>,
}

struct State {
    current_room: usize,
    romance: i8,
}

struct Choice {
    choice: String,
    outcome: String,
    next_room: usize,
    targets: Vec<TargetValence>,
    special_outcomes: HashMap<Target, String>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Valence {
    Bad,
    Good,
    Neutral,
}

struct TargetValence {
    target: Target,
    valence: Valence,
}

//compatibility quiz
fn take_quiz() -> (Target, &'static str) {
    println!("Answer these questions honestly for the most accurate results.");

    let questions = [
        ("If you were a flavor of ice-cream what would you be?", vec!["Spicy Cinnamon", "Pacific Cooler", "Strawberry, obviously"]),
        ("What is your go-to karaoke song?", vec!["Hot in Here by Nelly", "Juice by Lizzo", "Strawberry Blonde by Mitski"]),
        ("If you could have any superpower what would it be?", vec!["Fire Bending", "Healing and Quenching Thirst", "Plant Growth"]),
    ];

  
    //sets up targets
    let targets = [Target::HeatingPad, Target::CapriSun, Target::Strawberry];
    let mut t_tally = vec![0, 0, 0];

    //asks questions
    for (question, answers) in &questions {
        println!("{}", question);
        for (i, answer) in answers.iter().enumerate() {
            println!("{}. {}", i + 1, answer);
        }

        
        //parses input and error checking
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line");
      
        let user_ans = match user_input.trim().parse::<usize>() {
            Ok(val) => val - 1, // zero index
            Err(_) => {
                println!("That's not allowed! Please input an answer in the form of a number");
                return take_quiz(); 
            }
        };

        if user_ans >= answers.len() {
            println!("That's not allowed! Please enter a valid number");
            return take_quiz(); 
        }

        //adds up quiz scores
        t_tally[user_ans] += 1;
    }

    let max_score = *t_tally.iter().max().unwrap();
    let max_score_indices: Vec<usize> = t_tally.iter().enumerate().filter(|&(_, &score)| score == max_score).map(|(i, _)| i).collect();

    //when there is no tie
    if max_score_indices.len() == 1 {
        let target = targets[max_score_indices[0]].clone();
        let message = match target {
            Target::HeatingPad => "You have been matched with Heating Pad! A warm and cozy date awaits.",
            Target::CapriSun => "You have been matched with CapriSun! A refreshing and cooling date.",
            Target::Strawberry => "You have been matched with Strawberry! A sweet and peppy date.",
        };
        return (target, message);
    } else {
        //randomly selects if there is a tie
        println!("You seem to be compatible with everyone! Randomly selecting a date now...");
        let random_target_index = max_score_indices.choose(&mut rand::thread_rng()).unwrap();
        let target = targets[*random_target_index].clone();
        let message = match target {
            Target::HeatingPad => "You have been matched with Heating Pad! A warm and cozy date awaits.",
            Target::CapriSun => "You have been matched with CapriSun! A refreshing and cooling date awaits.",
            Target::Strawberry => "You have been matched with Strawberry! A sweet and peppy date awaits.",
        };
        return (target, message);
    }
}

//handles the choices
fn choose_option(state: &mut State, map: &Map, option_idx: usize, p_target: &Target) {
    let current_room = &map.rooms[state.current_room];
    let choice = &current_room.options[option_idx];

    //does valence checking
    if choice.targets.iter().any(|tv| {
        match (tv.target.clone(), &tv.valence) {
            (target, Valence::Bad) if target == *p_target => true,
            _ => false,
        }
    }) {
        state.romance -= 1;
        println!("That was a super bad choice. Your romance level has decreased to: {}", state.romance);

        if state.romance <= 0 {
            println!("Your romance level reached 0. Man you suck at dating. Game over.");
            return;
        }
    }

    // handles special outcome based on player target
    if let Some(special_outcome) = choice.special_outcomes.get(p_target) {
        println!("{:?} {}", p_target, special_outcome);
    } else {
        //prints standard outcome otherwise
        println!("{:?} {}", p_target, choice.outcome);
    }

    state.current_room = choice.next_room;
}


fn main() {
    println!("Welcome to the dating sim!");
    println!("Before we get started, let's take a compatibility quiz to see who you should date.");
    let (p_target, target_message) = take_quiz();
    println!("{}", target_message); 

    //sets up date events
    let map = Map {
        rooms: vec![
            Room {
                description: "Welcome to your date! First thing's first, what are you bringing?".to_string(),
                options: vec![
                    Choice {
                        choice: "Flowers".to_string(),
                        outcome: "thinks that they are nice.".to_string(),
                        next_room: 1,
                        targets: vec![
                            TargetValence {
                                target: Target::HeatingPad,
                                valence: Valence::Bad,
                            },
                            TargetValence {
                                target: Target::Strawberry,
                                valence: Valence::Good,
                            },
                        ],
                        special_outcomes: {
                            let mut map = HashMap::new();
                            map.insert(Target::HeatingPad, "says 'Those are just gonna wilt in the heat'.".to_string());
                            map.insert(Target::Strawberry, "says 'These are so sweet just like me!'".to_string());
                            map
                        },
                    },
                    Choice {
                        choice: "Sparkling Wine".to_string(),
                        outcome: "thinks it is a thoughtful gift".to_string(),
                        next_room: 1,
                        targets: vec![
                            TargetValence {
                                target: Target::CapriSun,
                                valence: Valence::Bad,
                            },
                        ],
                        special_outcomes: {
                            let mut map = HashMap::new();
                            map.insert(Target::CapriSun, "says 'This is like juice but bad!!!!'.".to_string());
                            map
                        },
                    },
                ],
            },
            Room {
                description: "You guys decide to start at the boardwalk. What game do you play?".to_string(),
                options: vec![
                    Choice {
                        choice: "Water Shooter".to_string(),
                        outcome: "has fun playing with the water guns together".to_string(),
                        next_room: 2,
                        targets: vec![
                            TargetValence {
                                target: Target::CapriSun,
                                valence: Valence::Good,
                            },
                        ],
                        special_outcomes: {
                            let mut map = HashMap::new();
                            map.insert(Target::CapriSun, "says 'This feels so refreshing!!'".to_string());
                            map
                        }, 
                    },
                    Choice {
                        choice: "Claw Machine".to_string(),
                        outcome: "thinks this is a nice choice and is excited with the prize you won.".to_string(),
                        next_room: 2,
                        targets: vec![
                            TargetValence {
                                target: Target::Strawberry,
                                valence: Valence::Bad,
                            },
                        ],
                        special_outcomes: {
                            let mut map = HashMap::new();
                            map.insert(Target::Strawberry, "says 'I feel so bad for the little plushies.'".to_string());
                            map
                        },
                    },
                ],
            },
            Room {
                description: "As you walk leaving the boardwalk, there is a large puddle in the way. What do you do?".to_string(),
                options: vec![
                    Choice {
                        choice: "Put your coat over the puddle".to_string(),
                        outcome: "is wowed by this kind of wild act of 'chivalry'.".to_string(),
                        next_room: 3,
                        targets: vec![
                            TargetValence {
                                target: Target::HeatingPad,
                                valence: Valence::Good,
                            },
                            TargetValence {
                                target: Target::Strawberry,
                                valence: Valence::Bad,
                            },
                        ],
                        special_outcomes: {
                            let mut map = HashMap::new();
                            map.insert(Target::HeatingPad, "says 'I'm honored you would sacrifice your warm coat for me!'".to_string());
                            map.insert(Target::Strawberry, "says 'I prefer leaving the mud at the garden.'".to_string());
                            map
                        },
                    },
                    Choice {
                        choice: "Walk around the puddle".to_string(),
                        outcome: "walks around the puddle with you".to_string(),
                        next_room: 3,
                        targets: vec![
                            TargetValence {
                                target: Target::CapriSun,
                                valence: Valence::Good,
                            },
                        ],
                        special_outcomes: {
                            let mut map = HashMap::new();
                            map.insert(Target::CapriSun, "says 'Thank you for not disturbing the serene puddle.'".to_string());
                            map
                        }, 
                    },
                ],
            },
            Room {
                description: "You decide to go to the movies. What movie do you want to watch?".to_string(),
                options: vec![
                    Choice {
                        choice: "Super Action Hero Fighter".to_string(),
                        outcome: "thinks its good as far as action movies go.".to_string(),
                        next_room: 4,
                        targets: vec![
                            TargetValence {
                                target: Target::Strawberry,
                                valence: Valence::Bad,
                            },
                            TargetValence {
                                target: Target::CapriSun,
                                valence: Valence::Good,
                            },
                        ],
                        special_outcomes: {
                            let mut map = HashMap::new();
                            map.insert(Target::CapriSun, "says 'This movie is so good it makes me thirsty!'".to_string());
                            map.insert(Target::Strawberry, "says 'This is the opposite of my kind of movie.'".to_string());
                            map
                        },
                    },
                    Choice {
                        choice: "Frozen 5".to_string(),
                        outcome: "thinks its a pretty nice movie".to_string(),
                        next_room: 4,
                        targets: vec![
                            TargetValence {
                                target: Target::HeatingPad,
                                valence: Valence::Good,
                            },
                            TargetValence {
                                target: Target::CapriSun,
                                valence: Valence::Bad,
                            },
                        ],
                        special_outcomes: {
                            let mut map = HashMap::new();
                            map.insert(Target::HeatingPad, "says 'The chilly movie makes me feel even warmer somehow.'".to_string());
                            map.insert(Target::CapriSun, "says 'This makes my juice freeze up.'".to_string());
                            map
                        }, 
                    },
                ],
            },
            Room {
                description: "As you walk to get dinner, there's an opportunity to hold hands. What do you do?".to_string(),
                options: vec![
                    Choice {
                        choice: "Reach out and hold their hands".to_string(),
                        outcome: "thinks its cozy and nice".to_string(),
                        next_room: 5,
                        targets: vec![
                            TargetValence {
                                target: Target::CapriSun,
                                valence: Valence::Bad,
                            },
                            TargetValence {
                                target: Target::Strawberry,
                                valence: Valence::Good,
                            },
                        ],
                        special_outcomes: {
                            let mut map = HashMap::new();
                            map.insert(Target::CapriSun, "says 'Don't do that! I have sweaty palms so it's uncomfy.'".to_string());
                            map.insert(Target::Strawberry, "says 'This is so sweet! I've been thinking of doing this all day'".to_string());
                            map
                        },
                    },
                    Choice {
                        choice: "Offer to hold their bag instead".to_string(),
                        outcome: "thanks you for carrying their things".to_string(),
                        next_room: 5,
                        targets: vec![
                            TargetValence {
                                target: Target::HeatingPad,
                                valence: Valence::Good,
                            },
                        ],
                        special_outcomes: {
                            let mut map = HashMap::new();
                            map.insert(Target::HeatingPad, "says 'Thank you so much!! My cords always get tangled up.'".to_string());
                            map
                        }, 
                    },
                ],
            },
            Room {
                description: "Finally it's time for dinner. What restaurant do you pick?".to_string(),
                options: vec![
                    Choice {
                        choice: "Steaks R Us".to_string(),
                        outcome: "thinks it has a nice menu.".to_string(),
                        next_room: 6,
                        targets: vec![
                            TargetValence {
                                target: Target::HeatingPad,
                                valence: Valence::Bad,
                            },
                            TargetValence {
                                target: Target::CapriSun,
                                valence: Valence::Good,
                            },
                        ],
                        special_outcomes: {
                            let mut map = HashMap::new();
                            map.insert(Target::CapriSun, "says 'I love the drink and meal combos here!'".to_string());
                            map.insert(Target::HeatingPad, "says 'This kind of meal always makes me feel super hot and tired'".to_string());
                            map
                        },
                    },
                    Choice {
                        choice: "Plants and Plates".to_string(),
                        outcome: "thinks its a good restarant".to_string(),
                        next_room: 6,
                        targets: vec![
                            TargetValence {
                                target: Target::Strawberry,
                                valence: Valence::Bad,
                            },
                        ],
                        special_outcomes: {
                            let mut map = HashMap::new();
                            map.insert(Target::Strawberry, "says 'How could you take me here to eat other plants??'".to_string());
                            map
                        }, 
                    },
                ],
            },
            Room {
                description: "It's the end of the date. You both lean in for a kiss, then smile. Maybe a second date is on the horizon!".to_string(),
                options: vec![],
            },
        ],
    };

    //sets starting romance level and start room
    let mut state = State {
        current_room: 0,
        romance: 2,
    };

    //prints out room info and choices
    while state.current_room < map.rooms.len() - 1 && state.romance > 0 {
        let current_room = &map.rooms[state.current_room];
        println!("{}", current_room.description);

        for (i, option) in current_room.options.iter().enumerate() {
            println!("{}. {}", i + 1, option.choice);
        }

        //handles user input
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line");

        let user_choice = match user_input.trim().parse::<usize>() {
            Ok(val) => val - 1, // Subtract 1 to convert to zero-based index
            Err(_) => {
                println!("That's not right! Please enter a number.");
                continue; // Restart the loop for the current room
            }
        };

        choose_option(&mut state, &map, user_choice, &p_target);
    }

    // ends game
    if state.current_room == map.rooms.len() - 1 {
        let final_room = &map.rooms[state.current_room];
        println!("{}", final_room.description);
    }
}







/*
quiz notes:
    - can structure as some print and read lines, or keep count?

submission notes:
    - trailer + repository
    - repository includes readme that has a guide for reading the code
    - can waive the trailer
    - he needs to have everything by the end of thursday.
    - double check on canvas that he isn't missing anything from me. 
 */