struct BoardGame {
    name: String,
    players: u8,
    playtime: u8
}

enum Game {
    SkipRope,
    BoardGame(BoardGame),
    ComputerGame {name: String, platform: String, size: u64 }
}

fn main() {
    let dixit = Game::BoardGame(BoardGame {
        name: String::from("Dixit"),
        players: 6,
        playtime: 40
    });
    let skip_rope = Game::SkipRope;
    let portal = Game::ComputerGame {
        name: String::from("Portal"),
        platform: String::from("macOS"),
        size: 2560
    };

    let games = [dixit, skip_rope, portal];

    for game in games.iter() {
        match game {
            Game::SkipRope => println!("Skip Rope"),
            Game::BoardGame(board_game) => println!(
                "Board Game: {}, Players: {}, Playtime: {}",
            board_game.name, board_game.players, board_game.playtime),
            Game::ComputerGame {name, platform, size} => println!("name: {}, platform: {}, size: {}", name, platform, size)
        }
    }
    // You do not need to use the fully-defined Option::Some and Option::None
    // because they're used so frequently
    let cypress = Some(String::from("Cypress"));
    let none: Option<String> = None;
    let strings = [cypress, none];

    for string in strings.iter() {
        match string {
            Some(string) => println!("{}", string),
            None => println!("Nothing")
        }
    }
}
