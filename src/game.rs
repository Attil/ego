use question::Question;

type Player = String;

enum GameState {
    Preparation,
    Game {
        turn: u32,
        question: u32
    },
    Finish
}

pub enum GameError {
    NotEnoughPlayers
}

pub struct Game {
    questions: Vec<Question>,
    players: Vec<Player>,
    state: GameState
}

impl Game {
    pub fn new(questions: Vec<Question>) -> Game { 
        Game {
            questions: questions,
            players: vec![],
            state: GameState::Preparation
        }
    }

    fn join(&mut self, player: Player) {
        self.players.push(player)
    }

    pub fn start(&mut self) -> Result<(), GameError> {
        if self.players.len() >= 2 {
            self.state = GameState::Game { 
                turn: 0,
                question : 0
            };
            Result::Ok(())
        } else {
            Result::Err(GameError::NotEnoughPlayers)
        }
    }
}