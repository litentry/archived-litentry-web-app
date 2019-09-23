use seed::prelude::*;
use seed::{fetch, Method, Request};
use serde::{Serialize, Deserialize};
use serde_json::json;
use futures::Future;
use seed::dom_types::MessageMapper;
use wasm_bindgen::JsCast;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Token {
    id: u32,
    identityId: u32,
    tokenHash: String,
    cost: String,
    data: String,
    dataType: String,
    expired: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TokenDataOwnedTokens {
    ownedTokens: Vec<Token>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TokenData {
    data: TokenDataOwnedTokens,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Identity {
    id: u32,
    ownerId: u32,
    identityHash: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IdentityDataOwnedIdentities {
    ownedIdentities: Vec<Identity>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IdentityData {
    data: IdentityDataOwnedIdentities,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPassed {
    pub tokenHash: String,
}


pub struct Model {
    account_value: String,
    owned_tokens: Vec<Token>,
    owned_identities: Vec<Identity>
}

impl Default for Model {
    fn default() -> Self {
        Self {
            account_value: "".to_string(),
            owned_tokens: vec![],
            owned_identities: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Msg {
    AccountInput(String),
    AccountInputBlur(String),
    TokenData(Option<TokenData>),
    IdentityData(Option<IdentityData>),
    VerifyToken(Option<String>),
    OnFetchErr
}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::AccountInput(astr) => {
            on_account_input(model, astr);
        },
        Msg::AccountInputBlur(astr) => {
            on_account_input_blur(model, astr);
            orders.skip().perform_cmd(make_request_owned_tokens());
            orders.skip().perform_cmd(make_request_owned_identities());
        },
        Msg::IdentityData(Some(data)) => {
            model.owned_identities = data.data.ownedIdentities;
        },
        Msg::IdentityData(None) => {
            //
        },
        Msg::TokenData(Some(data)) => {
            model.owned_tokens = data.data.ownedTokens;
        },
        Msg::TokenData(None) => {
            //
        },
        Msg::VerifyToken(tokenHash) => {
            log!("tokenHash is: ", tokenHash);

            // store to local storage
            let data = DataPassed {
                tokenHash: tokenHash.unwrap()
            };
            let storage = seed::storage::get_storage().unwrap();
            seed::storage::store_data(&storage, "data-passed", &data);
        },
        Msg::OnFetchErr => {
            log!("OnFetcherr");
        }
    }

}


pub fn on_account_input(model: &mut Model, input_value: String) {
    model.account_value = input_value
}

pub fn on_account_input_blur(model: &mut Model, astr: String) {
    log!("aset is ", astr);
    log!("now account value is: ", model.account_value);
}

fn make_request_owned_identities() -> impl Future<Item = Msg, Error = Msg> {
    let url = "http://112.125.25.18:3000/graphql";
    let message = json!({
        "query": r#"
{
  ownedIdentities (address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY") {
    id
    ownerId
    identityHash
  }
}"#
    });

    // send account to server, to get
    Request::new(url)
        .method(Method::Post)
        .send_json(&message)
        .fetch_json_data(|r: fetch::ResponseDataResult<IdentityData>| r)
        .map(|r| {
            match r {
                Ok(data) => {
                    Msg::IdentityData(Some(data))
                },
                Err(err) => {
                    log!(err);
                    Msg::IdentityData(None)
                },
            }
        })
        .map_err(|err| {
            log!("OnFetchErr {:?}", err);
            Msg::OnFetchErr
        })
}

fn make_request_owned_tokens() -> impl Future<Item = Msg, Error = Msg> {
    let url = "http://112.125.25.18:3000/graphql";
    let message = json!({
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
    });

    // send account to server, to get
    Request::new(url)
        .method(Method::Post)
        .send_json(&message)
        .fetch_json_data(|r: fetch::ResponseDataResult<TokenData>| r)
        .map(|p| {
            match p {
                Ok(data) => {
                    Msg::TokenData(Some(data))
                },
                Err(err) => {
                    log!(err);
                    Msg::TokenData(None)
                },
            }
        })
        .map_err( |_| {
            Msg::OnFetchErr
        })

}


fn render_tokens(model: &Model) -> Vec<Node<Msg>> {
    model.owned_tokens.iter().map(|item| {
        div![class!["item"],
             span![class!["caption"], item.tokenHash],
             a![
                 attrs!{
                     At::Class => "action",
                     At::Href => "/verify_request",
                     At::Custom("tokenHash".to_string()) => item.tokenHash
                 },
                 raw_ev(Ev::Click, move |event| {
                     let tokenHash: Option<String> = event.target()
                         .and_then(|et| et.dyn_into::<web_sys::Element>().ok())
                         .and_then(|el| el.get_attribute("tokenHash"));
                     //event.prevent_default();
                     Msg::VerifyToken(tokenHash)
                 }),
                 "Verify"
             ],
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


pub fn view(model: &Model) -> Node<Msg> {
    div![id!("page_account_state"),
         div![class!["account"],
              span!["Your Account: "],
              input![
                  attrs!{
                      At::Placeholder => "Please input your account here",
                      At::Value => "",
                  },
                  input_ev(Ev::Input, Msg::AccountInput),
                  input_ev(Ev::Blur, Msg::AccountInputBlur)
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
    ]
}
