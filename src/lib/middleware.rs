type Request = hyper::Request<hyper::Body>;
type Response = hyper::Response<hyper::Body>;

pub trait Middleware {
    fn process_request(&mut self, request: Request) -> Request;
    fn process_response(&mut self, reponse: Response) -> Response;
}
