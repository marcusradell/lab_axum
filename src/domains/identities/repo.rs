#[derive(Clone)]
pub struct Repo {
    // TODO: Replace with SQLx
    _db: (),
}

impl Repo {
    pub fn new() -> Self {
        Self { _db: () }
    }
}
