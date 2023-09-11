pub fn get_session_id(instance: impl Into<String>) -> String {
    // read from sessions.toml file
    // check if the session id is valid (check_session_id())
    // if valid, return the session id
    // if not valid, renew the session id (renew_session_id())
    // return the session id
    todo!()
}

fn renew_session_id(instance: impl Into<String>) -> String {
    // renew the session id
    // write to sessions.toml file
    // return the session id
    todo!()
}

fn check_session_id(instance: impl Into<String>) -> String {
    todo!()
}