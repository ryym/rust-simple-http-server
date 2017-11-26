pub enum Status {
    Ok,
    NotFound,
}

impl Status {
    pub fn code(&self) -> u16 {
        match *self {
            Status::Ok => 200,
            Status::NotFound => 404,
        }
    }

    pub fn name(&self) -> &'static str {
        match *self {
            Status::Ok => "OK",
            Status::NotFound => "NotFound",
        }
    }
}
