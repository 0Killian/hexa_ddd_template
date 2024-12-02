use axum::{extract::Request, http::HeaderName};
use tower_http::request_id::{
    MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};

#[derive(Clone, Default, Debug)]
pub struct MyMakeRequestId {}

static REQUEST_ID: &str = "x-request-id";

impl MakeRequestId for MyMakeRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        Some(RequestId::new(
            uuid::Uuid::now_v7().to_string().parse().unwrap(),
        ))
    }
}

pub fn set_request_id_layer() -> SetRequestIdLayer<MyMakeRequestId> {
    let x_request_id = HeaderName::from_static(REQUEST_ID);
    SetRequestIdLayer::new(x_request_id, MyMakeRequestId::default())
}

pub fn propagate_request_id_layer() -> PropagateRequestIdLayer {
    let x_request_id = HeaderName::from_static(REQUEST_ID);
    PropagateRequestIdLayer::new(x_request_id)
}
