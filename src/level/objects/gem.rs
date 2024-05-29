use super::{Level, Obj, Request, State};

pub struct Gem;

impl Obj for Gem {
    fn char(&self) -> char {
        '+'
    }
    fn emoji(&self) -> char {
        '💎'
    }
    fn name(&self) -> &str {
        "gem"
    }

    fn breakable(&self) -> bool {
        true
    }

    fn init(&self) -> Vec<Request> {
        vec![Request::AddMaxScore]
    }
    fn on_broken(&self, level: &Level) -> Vec<Request> {
        let mut requests = vec![Request::AddScore];
        if level.score + 1 == level.max_score {
            requests.push(Request::UpdateState(State::Win));
        }

        requests
    }
}
