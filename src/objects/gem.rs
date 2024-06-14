use super::{Behaviour, Labels, Level, Properties, Request, State};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gem;

impl Labels for Gem {
    fn char(&self) -> char {
        '+'
    }
    fn emoji(&self) -> char {
        '💎'
    }
}

impl Properties for Gem {
    fn can_be_broken(&self) -> bool {
        true
    }
}

impl Behaviour for Gem {
    fn init(&self) -> Vec<Request> {
        vec![Request::AddMaxScore]
    }
    fn on_broken(&self, level: &Level) -> Vec<Request> {
        let mut requests = vec![Request::AddScore];
        if level.get_score() + 1 == *level.get_max_score() {
            requests.push(Request::UpdateState(State::Win));
        }

        requests
    }
}
