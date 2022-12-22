use std::collections::HashMap;

use rocket::{http::RawStr, request::FromRequest, request::Outcome, Request};

pub mod page_data;
pub mod perms;

pub struct ReqParams {
    pub params: HashMap<String, String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ReqParams {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mut params = HashMap::new();
        if req.uri().query().is_none() {
            return Outcome::Success(ReqParams { params });
        }

        for (key, value) in req.uri().query().unwrap().split('&').map(|s| {
            let mut iter = s.split('=');
            (
                iter.next().unwrap_or_else(|| RawStr::new("")).to_string(),
                iter.next().unwrap_or_else(|| RawStr::new("")).to_string(),
            )
        }) {
            params.insert(key, value);
        }
        Outcome::Success(ReqParams { params })
    }
}
