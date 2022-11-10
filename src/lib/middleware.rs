

pub trait Middleware {
    fn process_request(&self, request: i32) -> i32;
    fn process_response(&self, reponse: i32) -> i32;
}
