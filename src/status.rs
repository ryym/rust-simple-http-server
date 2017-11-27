pub enum Status {
    Ok,
    NotFound,
    ServerErr,
}

impl Status {
    pub fn code(&self) -> u16 {
        match *self {
            Status::Ok => 200,
            Status::NotFound => 404,
            Status::ServerErr => 500,
        }
    }

    pub fn name(&self) -> &'static str {
        match *self {
            Status::Ok => "OK",
            Status::NotFound => "NotFound",
            Status::ServerErr => "Internal Server Error",
        }
    }
}
