use seed::prelude::*;
use seed::{fetch, Method, Request};
use serde::{Serialize, Deserialize};
use serde_json::json;
use futures::Future;
use futures::future;
use seed::{document, window};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlMediaElement, MediaStream, MediaStreamConstraints};

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

#[derive(Clone)]
pub enum Msg {
    TokenInfoData(Option<TokenInfoData>),
    OnGetInfoErr,
    TakeSnapshot,
    UserMedia(Result<MediaStream, JsValue>),
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

pub fn open_camera() -> impl Future<Item = Msg, Error = Msg> {
    future::ok::<(), ()>(()).then(|_| {
	// open the camera
	let mut constraints = web_sys::MediaStreamConstraints::new();
	constraints.audio(&JsValue::from_bool(false));
	constraints.video(&JsValue::from_bool(true));
	log!(constraints);

	let media_stream_promise = window()
	    .navigator()
	    .media_devices()
	    .unwrap()
	    .get_user_media_with_constraints(&constraints)
	    .unwrap();

	JsFuture::from(media_stream_promise)
	    .map(MediaStream::from)
	    .then(|result| Ok(Msg::UserMedia(result)))

    })
}


pub fn init(model: &mut Model, orders: &mut impl Orders<Msg>) {
    let storage = seed::storage::get_storage().unwrap();
    let loaded_serialized = storage.get_item("data-passed").unwrap().unwrap();
    let data: DataPassed = serde_json::from_str(&loaded_serialized).unwrap();
    log!("data from local storage: ", data);

    orders
	.perform_cmd(get_token_info(data.tokenHash));

    orders
	.perform_cmd(open_camera());

}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
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
	Msg::UserMedia(Ok(media_stream)) => {
	    let video_el = document()
		.get_element_by_id("video")
		.unwrap()
		//.unwrap()
		.dyn_into::<HtmlMediaElement>()
		.unwrap();
	    log!(video_el);
	    log!(media_stream);

	    video_el.set_src_object(Some(&media_stream));
	    video_el.play();
	},
	Msg::UserMedia(Err(error)) => {
	    log!(error);
	},
	Msg::TakeSnapshot => {
	    log!("TakeSnapshot action");
	    let video = document().get_element_by_id("video")
		.and_then(|element| element.dyn_into::<web_sys::HtmlVideoElement>().ok()).unwrap();
	    let canvas = document().get_element_by_id("canvas")
		.and_then(|element| element.dyn_into::<web_sys::HtmlCanvasElement>().ok()).unwrap();
	    let img = document().get_element_by_id("img")
		.and_then(|element| element.dyn_into::<web_sys::HtmlImageElement>().ok()).unwrap();

	    // XXX: Convert type?
	    canvas.get_context("2d").unwrap().unwrap()
		.dyn_into::<web_sys::CanvasRenderingContext2d>().ok().unwrap()
		.draw_image_with_html_video_element_and_dw_and_dh(&video, 0.0, 0.0, 400.0, 300.0);


	    let img_base64_str = canvas.to_data_url_with_type("image/png").unwrap();
	    img.set_attribute("src", &img_base64_str.clone());

	    let offset = img_base64_str.find(',').unwrap_or(img_base64_str.len()) + 1;
	    let mut value = img_base64_str;
	    value.drain(..offset);
	    log!(value);
	    let bytes = base64::decode(&value).unwrap();
	    log!(bytes.len());

	    let img_obj = image::load_from_memory(&bytes).unwrap();
	    //log!(img_obj);

	    // Use default decoder
	    let decoder = bardecoder::default_decoder();

	    let results = decoder.decode(img_obj);
	    log!(results);
	    for result in results {
		println!("{}", result.unwrap());
	    }


	    //canvas.to_blob( &js_sys::Function::new_with_args("blob", "console.log(blob);") ).unwrap();

	},

    }
}



pub fn view(model: &Model) -> Node<Msg> {
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
	      button![id!("action"),
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
    ]
}
