// content-type: application/octet-stream
// status: 204 No Content
// ratelimit-limit: 1200
// ratelimit-remaining: 771
// ratelimit-reset: 1415984218

use std::borrow::Cow;
use std::fmt;

use crate::response::{self, NamedResponse};
use reqwest::{blocking::Response, header, StatusCode};

//#[derive(Deserialize)]
pub struct HeaderOnly {
    //#[serde(rename = "content-type")]
    pub content_type: String,
    pub status: StatusCode,
    //#[serde(rename = "ratelimit-limit")]
    pub ratelimit_limit: f64,
    //#[serde(rename = "ratelimit-remaining")]
    pub ratelimit_remaining: f64,
    //#[serde(rename = "ratelimit-reset")]
    pub ratelimit_reset: f64,
}

impl response::NotArray for HeaderOnly {}

impl HeaderOnly {
    pub fn from_response(response: Response) -> Result<HeaderOnly, String> {
        let content_type = match response.headers().get(header::CONTENT_TYPE) {
            Some(content_type) => match content_type.to_str() {
                Ok(content_type) => String::from(content_type),
                Err(error) => return Err(error.to_string()),
            },
            None => String::new(),
        };

        let status = response.status();

        // let raw_status = response.status_raw();
        // let status = format!("{} {}", raw_status.0, raw_status.1);

        // let rl_limit_raw = match r.headers.get_raw("ratelimit-limit") {
        //     Some(c) => c,
        //     None => return Err("No ratelimit-limit provided".to_owned()),
        // };

        // let rl_remain_raw = match r.headers.get_raw("ratelimit-remaining") {
        //     Some(c) => c,
        //     None => return Err("No ratelimit-remaining provided".to_owned()),
        // };

        // let rl_reset_raw = match r.headers.get_raw("ratelimit-reset") {
        //     Some(c) => c,
        //     None => return Err("No ratelimit-reset provided".to_owned()),
        // };

        // let rl_limit_str = String::from_utf8_lossy(&rl_limit_raw[0][..]);
        // let rl_remain_str = String::from_utf8_lossy(&rl_remain_raw[0][..]);
        // let rl_reset_str = String::from_utf8_lossy(&rl_reset_raw[0][..]);
        // let rl_limit = match rl_limit_str.as_ref().parse::<f64>() {
        //     Ok(n) => n,
        //     Err(e) => return Err(e.to_string()),
        // };

        // let rl_remain = match rl_remain_str.as_ref().parse::<f64>() {
        //     Ok(n) => n,
        //     Err(e) => return Err(e.to_string()),
        // };
        // let rl_reset = match rl_reset_str.as_ref().parse::<f64>() {
        //     Ok(n) => n,
        //     Err(e) => return Err(e.to_string()),
        // };
        
        Ok(HeaderOnly {
            content_type,
            status,
            ratelimit_limit: 0.0,
            ratelimit_remaining: 0.0,
            ratelimit_reset: 0.0,
        })
    }
}

impl NamedResponse for HeaderOnly {
    fn name<'a>() -> Cow<'a, str> {
        "header".into()
    }
}

impl fmt::Display for HeaderOnly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Status: {}\n\
                   Request Limit: {:.0}\n\
                   Requests Remaining: {:.0}\n\
                   Time to Reset: {:.0}\n",
            self.status, self.ratelimit_limit, self.ratelimit_remaining, self.ratelimit_reset
        )
    }
}

impl fmt::Debug for HeaderOnly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "content-type: {:?}\n\
                   status: {:?}\n\
                   ratelimit-limit: {:.0}\n\
                   ratelimit-remaining: {:.0}\n\
                   ratelimit_reset: {:.0}\n",
            self.content_type,
            self.status,
            self.ratelimit_limit,
            self.ratelimit_remaining,
            self.ratelimit_reset
        )
    }
}
