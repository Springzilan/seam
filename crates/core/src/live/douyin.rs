use std::collections::HashMap;

use async_trait::async_trait;
use regex::Regex;
use serde_json::Value;
use urlencoding::decode;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const URL: &str = "https://live.douyin.com/";

/// 抖音直播
///
/// https://live.douyin.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    // TODO 说明所需 cookie
    async fn get(&self, rid: &str, headers: Option<HashMap<String, String>>) -> Result<Node> {
        // 后续应提取到获取 cookie 的逻辑中
        // let mut headers = hash2header(headers);
        // // 更新 cookie
        // headers.insert("user-agent", USER_AGENT.parse()?);
        // let resp = CLIENT
        //     .get(format!("{URL}{rid}"))
        //     .headers(headers.clone())
        //     .send()
        //     .await?;
        // header_map.insert("cookie", resp.headers().get("set-cookie").unwrap().clone());
        // 通过网页内容获取直播地址
        let resp = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(hash2header(headers))
            .send()
            .await?;
        let resp_text = resp.text().await?;

        let re =
            Regex::new(r#"<script id="RENDER_DATA" type="application/json">([\s\S]*?)</script>"#)?;
        let json = decode(
            re.captures(&resp_text)
                .ok_or(SeamError::NeedFix("captures"))?
                .get(1)
                .ok_or(SeamError::NeedFix("json"))?
                .as_str(),
        )?;
        let json: serde_json::Value = serde_json::from_str(&json)?;

        let room_info = &json["app"]["initialState"]["roomStore"]["roomInfo"];
        match room_info["anchor"] {
            // TODO 主播不存在 这种需要额外判断吗?
            serde_json::Value::Null => Err(SeamError::None),
            _ => match &room_info["room"]["stream_url"] {
                // 未开播
                Value::Null => Err(SeamError::None),
                stream_url => {
                    let title = room_info["room"]["title"]
                        .as_str()
                        .unwrap_or("douyin")
                        .to_string();
                    // 返回最高清晰度的直播地址 flv 和 hls
                    let urls = vec![
                        parse_url(
                            stream_url["flv_pull_url"]["FULL_HD1"]
                                .as_str()
                                .ok_or(SeamError::NeedFix("flv_pull_url"))?
                                .to_owned(),
                        ),
                        parse_url(
                            stream_url["hls_pull_url_map"]["FULL_HD1"]
                                .as_str()
                                .ok_or(SeamError::NeedFix("hls_pull_url_map"))?
                                .to_owned(),
                        ),
                    ];
                    Ok(Node {
                        rid: rid.to_string(),
                        title,
                        urls,
                    })
                }
            },
        }
    }
}

#[cfg(test)]
macros::gen_test!(5893162289);
