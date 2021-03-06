// Copyright 2015-2016 the slack-rs authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;

use super::{ApiResult, SlackWebRequestSender, parse_slack_response};

/// Gets the access logs for the current team.
///
/// Wraps https://api.slack.com/methods/team.accessLogs
pub fn access_logs<R: SlackWebRequestSender>(client: &R, token: &str, count: Option<u32>, page: Option<u32>) -> ApiResult<AccessLogsResponse> {
    let count = count.map(|c| c.to_string());
    let page = page.map(|p| p.to_string());
    let mut params: HashMap<&str, &str> = HashMap::new();
    if let Some(ref count) = count {
        params.insert("count", count);
    }
    if let Some(ref page) = page {
        params.insert("page", page);
    }
    let response = try!(client.send_authed("team.accessLogs", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct LoginInfo {
    pub user_id: String,
    pub username: String,
    pub date_first: u32,
    pub date_last: u32,
    pub count: u32,
    pub ip: String,
    pub user_agent: String,
    pub isp: String,
    pub country: String,
    pub region: String,
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct AccessLogsResponse {
    pub logins: Vec<LoginInfo>,
    pub paging: super::Pagination,
}

/// Gets information about the current team.
///
/// Wraps https://api.slack.com/methods/team.info
pub fn info<R: SlackWebRequestSender>(client: &R, token: &str) -> ApiResult<InfoResponse> {
    let response = try!(client.send_authed("team.info", token, HashMap::new()));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct IconInfo {
    pub image_34: String,
    pub image_44: String,
    pub image_68: String,
    pub image_88: String,
    pub image_102: String,
    pub image_132: String,
    pub image_default: bool,
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct TeamInfo {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub email_domain: String,
    pub icon: IconInfo,
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct InfoResponse {
    pub team: TeamInfo,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_helpers::*;

    #[test]
    fn general_api_error_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": false, "err": "some_error"}"#);
        let result = access_logs(&client, "TEST_TOKEN", None, None);
        assert!(result.is_err());
    }

    #[test]
    fn access_logs_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "logins": [
                {
                    "user_id": "U12345",
                    "username": "bob",
                    "date_first": 1422922864,
                    "date_last": 1422922864,
                    "count": 1,
                    "ip": "127.0.0.1",
                    "user_agent": "SlackWeb Mozilla\/5.0 (Macintosh; Intel Mac OS X 10_10_2) AppleWebKit\/537.36 (KHTML, like Gecko) Chrome\/41.0.2272.35 Safari\/537.36",
                    "isp": "BigCo ISP",
                    "country": "US",
                    "region": "CA"
                },
                {
                    "user_id": "U45678",
                    "username": "alice",
                    "date_first": 1422922493,
                    "date_last": 1422922493,
                    "count": 1,
                    "ip": "127.0.0.1",
                    "user_agent": "SlackWeb Mozilla\/5.0 (iPhone; CPU iPhone OS 8_1_3 like Mac OS X) AppleWebKit\/600.1.4 (KHTML, like Gecko) Version\/8.0 Mobile\/12B466 Safari\/600.1.4",
                    "isp": "BigCo ISP",
                    "country": "US",
                    "region": "CA"
                }
            ],
            "paging": {
                "count": 100,
                "total": 2,
                "page": 1,
                "pages": 1
            }
        }"#);
        let result = access_logs(&client, "TEST_TOKEN", None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.logins[0].username, "bob");
        assert_eq!(result.logins[1].username, "alice");
    }

    #[test]
    fn info_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "team": {
                "id": "T12345",
                "name": "My Team",
                "domain": "example",
                "email_domain": "",
                "icon": {
                    "image_34": "https:\/\/...",
                    "image_44": "https:\/\/...",
                    "image_68": "https:\/\/...",
                    "image_88": "https:\/\/...",
                    "image_102": "https:\/\/...",
                    "image_132": "https:\/\/...",
                    "image_default": true
                }
            }
        }"#);
        let result = info(&client, "TEST_TOKEN");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.team.name, "My Team");
        assert_eq!(result.team.icon.image_default, true);
    }
}
