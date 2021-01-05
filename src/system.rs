#[derive(Debug,Serialize)]
pub struct State {
    pub system_uid: String,
    pub hostname: String,
    pub memory: u64
}

impl State {
    pub fn new () -> State {
        State {
            system_uid: String::new(),
            hostname: String::new(),
            memory: 0,
        }
    }
}
