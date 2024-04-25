use axum_test_helper::RequestBuilder;
use nsuns_server::auth::token::COOKIE_NAME;
use serde::Serialize;
use tower_cookies::Cookie;

use crate::world::NsunsWorld;

pub trait JsonBody {
    fn json_body<T>(self, body: &T) -> Self
    where
        T: ?Sized + Serialize;
}

impl JsonBody for RequestBuilder {
    fn json_body<T>(self, body: &T) -> Self
    where
        T: ?Sized + Serialize,
    {
        self.body(serde_json::to_string(body).unwrap())
            .header("Content-Type", "application/json")
    }
}

pub trait Auth {
    fn authed(self, cookie: &NsunsWorld) -> Self;
}

impl Auth for RequestBuilder {
    fn authed(self, world: &NsunsWorld) -> Self {
        if let Some(cookie) = &world.auth_cookie {
            self.header(
                "Cookie",
                Cookie::new(COOKIE_NAME, cookie).encoded().to_string(),
            )
        } else {
            self
        }
    }
}
