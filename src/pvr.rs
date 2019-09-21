use seed::prelude::*;
use seed::{fetch, Method, Request};
use serde::{Serialize, Deserialize};
use serde_json::json;
use futures::Future;

use crate::pas::DataPassed;

pub struct Model {
}

impl Default for Model {
    fn default() -> Self {
        Self {
        }
    }
}


#[derive(Clone)]
pub enum Msg {
}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {

    }
}



pub fn view(model: &Model) -> Node<Msg> {
    let storage = seed::storage::get_storage().unwrap();
    let loaded_serialized = storage.get_item("data-passed").unwrap().unwrap();
    let data: DataPassed = serde_json::from_str(&loaded_serialized).unwrap();
    log!(data);


    div![id!("pvr"),
         div![class!["account"],
              span!["Your Account: "],
              span!["0x5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
         ],
         div![class!["token_info"],
              div![class!["item", "token"], format!("Token Id: {}", data.tokenHash)],
              div![class!["item", "identity"], "Issued Identity: 0x4343b341f24a9999999"]
         ],
         div![class!["content webscan"],
              div![class!["title"], "Webcan Scan QR Code"],
         ],
         div![class!["action"], "Success or Not!"]
    ]
}
