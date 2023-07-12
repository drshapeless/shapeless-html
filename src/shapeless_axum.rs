use crate::*;
use axum::response::{Html, IntoResponse, Response};

impl IntoResponse for Element {
    fn into_response(self) -> Response {
        Html(self.render()).into_response()
    }
}

impl IntoResponse for Elements {
    fn into_response(self) -> Response {
        Html(self.render()).into_response()
    }
}
