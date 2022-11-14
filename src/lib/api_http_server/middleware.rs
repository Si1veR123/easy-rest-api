type Request = hyper::Request<hyper::Body>;
type Response = hyper::Response<hyper::Body>;

pub trait Middleware {
    fn process_request(&self, request: &mut Request);
    fn process_response(&self, reponse: &mut Response);
}
