use seed::prelude::*;
use seed::{fetch, Method, Request};
use serde_json::json;

use crate::{
    Model,
    Msg
};


pub fn on_account_input(model: &mut Model, input_value: String) {
    model.account_value = input_value
}

pub fn on_account_input_blur(model: &mut Model, astr: String) {
    log!("aset is ", astr);
    log!("now account value is: ", model.account_value);
}

fn make_request_owned_identities() -> impl Future<Item = Msg, Error = Msg> {
    let url = "http://112.125.25.18:3000/graphql";
    let message = json!{
        "query": r#"
{
  ownedIdentities (address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY") {
    id
    ownerId
    identityHash
  }
}"#
    };

    // send account to server, to get
    Request::new(url)
        .method(Method::Post)
        .send_json(&message)
        .fetch_json_data(Msg::OwnedIdentitiesRequestFetched)
}

fn make_request_owned_tokens() -> impl Future<Item = Msg, Error = Msg> {
    let url = "http://112.125.25.18:3000/graphql";
    let message = json!{
        "query": r#"
{
  ownedTokens (address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY") {
    id
    identityId
    ownerId
    tokenHash
    cost
    data
    dataType
    expired
  }
}"#
    };

    // send account to server, to get
    Request::new(url)
        .method(Method::Post)
        .send_json(&message)
        .fetch_json_data(Msg::OwnedTokensRequestFetched)
}


fn render_tokens(model: &Model) -> Vec<Node<Msg>> {
    model.owned_tokens.iter().map(|item| {
        div![class!["item"],
             span![class!["caption"], item.tokenHash],
             span![class!["action"], "Verify"],
        ]
    }).collect()
}

fn render_identities(model: &Model) -> Vec<Node<Msg>> {
    model.owned_identities.iter().map(|item| {
        div![class!["item"],
             span![class!["caption"], item.identityHash],
             span![class!["action"], "Generate"],
        ]
    }).collect()
}


pub fn page_render(model: &Model) -> Node<Msg> {
    div![id!("page_account_state"),
         div![class!["account"],
              span!["Your Account: "],
              input![
                  attrs!{
                      At::Placeholder => "Please input your account here",
                      At::Value => "",
                  },
                  input_ev(Ev::Input, Msg::PageAccountStateAccountInput),
                  input_ev(Ev::Blur, Msg::PageAccountStateAccountInputBlur)
              ]
         ],
         div![class!["owned_token_list"],
              div![class!["title"], "Owned Token List"],
              div![class!["content"], render_tokens(model)]
         ],
         div![class!["owned_identity_list"],
              div![class!["title"], "Owned Identity List"],
              div![class!["content"], render_identities(model)]
         ],

    ],


}
