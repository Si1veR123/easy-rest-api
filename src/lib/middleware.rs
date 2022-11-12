type Request = hyper::Request<hyper::Body>;
type Response = hyper::Response<hyper::Body>;

pub trait Middleware {
    fn process_request(&self, request: Request) -> Request;
    fn process_response(&self, reponse: Response) -> Response;
}
