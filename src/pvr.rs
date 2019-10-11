use seed::prelude::*;
use seed::{fetch, Method, Request};
use serde::{Serialize, Deserialize};
use serde_json::json;
use futures::Future;

use crate::pas::DataPassed;

#[derive(Default, Clone, Debug)]
pub struct Model {
    token_info: TokenInfo
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TokenInfo {
    tokenHash: String,
    identityHash: String,
    ownerAddress: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TokenInfoDataGet {
    getTokenInfo: TokenInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TokenInfoData {
    data: TokenInfoDataGet,
}



#[derive(Serialize, Deserialize, Clone)]
pub enum Msg {
    PageLoaded(String),
    TokenInfoData(Option<TokenInfoData>),
    OnGetInfoErr,
    TakeSnapshot,
}


fn get_token_info(tokenHash: String) -> impl Future<Item = Msg, Error = Msg> {
    let url = "http://112.125.25.18:3000/graphql";
    let mut body = String::new();
    body.push_str("{getTokenInfo (tokenHash: \"");
    body.push_str(&tokenHash);
    body.push_str("\") {
    tokenHash
    identityHash
    ownerAddress}}");

    let message = json!({
        "query": &body
    });


    // send account to server, to get
    Request::new(url)
        .method(Method::Post)
        .send_json(&message)
        .fetch_json_data(|r: fetch::ResponseDataResult<TokenInfoData>| r)
        .map(|p| {
            match p {
                Ok(data) => {
                    Msg::TokenInfoData(Some(data))
                },
                Err(err) => {
                    log!(err);
                    Msg::TokenInfoData(None)
                }
            }
        })
        .map_err( |_| {
            Msg::OnGetInfoErr
        })
}



pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::PageLoaded(astr) => {
            log!("PageLoaded - ", astr);
            orders.skip().perform_cmd(get_token_info(astr));

            // open the camera
            let constraints = web_sys::MediaStreamConstraints::new()
                .audio(&JsValue::from_bool(false))
                .video(&JsValue::from_bool(true));

            // XXX: This return promise? Do it like a future?
            web_sys::MediaDevices.get_user_media_with_constraints(&constraints)
                .map_err(|e| log!(e) )
                .and_then(|stream| {
                    log!(stream);

                    // get video element, and assign src to video element
                    let video = doc.get_element_by_id("video")
                        .and_then(|element| element.dyn_into::<web_sys::HtmlVideoElement>().ok()).unwrap();
                    let url = web_sys::Url.create_object_url_with_source(stream).unwrap();
                    video.set_attribute("src", &url);
                    video.play();
                });
        },
        Msg::TokenInfoData(Some(data)) => {
            log!(format!("in token info data handler {:?}", data));
            model.token_info = data.data.getTokenInfo;
        },
        Msg::TokenInfoData(None) => {
            log!("TokenInfoData None");
        },
        Msg::OnGetInfoErr => {
            let err = "";
            log!(format!("Get Token Info error: {:?}", err));
        },
        Msg::TakeSnapshot => {
            log!("TakeSnapshot action");
            let doc = seed::document();
            let video = doc.get_element_by_id("video")
                .and_then(|element| element.dyn_into::<web_sys::HtmlVideoElement>().ok()).unwrap();
            let canvas = doc.get_element_by_id("canvas")
                .and_then(|element| element.dyn_into::<web_sys::HtmlCanvasElement>().ok()).unwrap();
            let img = doc.get_element_by_id("img")
                .and_then(|element| element.dyn_into::<web_sys::HtmlImageElement>().ok()).unwrap();

            // XXX: Convert type?
            canvas.get_context("2d").unwrap().unwrap()
                .draw_image_with_html_video_element_and_dw_and_dh(&video, 0, 0, 400, 300);

            img.set_attribute("src", &canvas.to_data_url_with_type("image/png").unwrap());


        }
    }
}



pub fn view(model: &Model) -> Node<Msg> {
    let storage = seed::storage::get_storage().unwrap();
    let loaded_serialized = storage.get_item("data-passed").unwrap().unwrap();
    let data: DataPassed = serde_json::from_str(&loaded_serialized).unwrap();
    log!("data from local storage: ", data);

    div![id!("pvr"),
         div![class!["account"],
              span!["Your Account: "],
              span![model.token_info.ownerAddress],
         ],
         div![class!["token_info"],
              div![class!["item", "token"], format!("Token Id: {}", model.token_info.tokenHash)],
              div![class!["item", "identity"], format!("Issued Identity: {}", model.token_info.identityHash)]
         ],
         div![class!["content webscan"],
              div![class!["title"], "Webcan Scan QR Code"],
              video![attrs! {
                  At::Id => "video",
                  At::Width => "400",
                  At::Height => "300"
              }],
              button![id("action"),
                      "Take Snapshot",
                      simple_ev(Ev::Click, Msg::TakeSnapshot)
              ],
              canvas![attrs!{
                  At::Id => "canvas",
                  At::Width => "400",
                  At::Height => "300"
              }],
              img![attrs!{
                  At::Id => "img",
                  At::Src => ""
              }]
         ],
         div![class!["action"], "Success or Not!"],

         // use this empty div to act as a event placeholder
         div![
             did_mount(move |_| {
                 let tokenHash = data.tokenHash.clone();
                 log!("execute did_update", tokenHash);
                 seed::update(crate::Msg::Pvr(Msg::PageLoaded(tokenHash)));
             })
         ]
    ]
}
