#[macro_use]
extern crate seed;
use seed::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    id: u32,
    identityId: u32,
    tokenHash: String,
    cost: String,
    data: String,
    dataType: String,
    expired: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenData {
    data: TokenDataOwnedTokens,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenDataOwnedTokens {
    ownedTokens: Vec<Token>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identity {
    id: u32,
    ownerId: u32,
    identityHash: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdentityData {
    data: IdentityDataOwnedIdentities,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdentityDataOwnedIdentities {
    ownedIdentities: Vec<Identity>,
}



// Model
pub struct Model {
    // for routing
    page: Page,

    // for page_account_state
    account_value: String,
    owned_tokens: Vec<Token>,
    owned_identities: Vec<Identity>

}

impl Default for Model {
    fn default() -> Self {
        Self {
            page: Page::EventList,
            account_value: "".to_string(),
            owned_tokens: vec![],
            owned_identities: vec![]
        }
    }
}




#[derive(Clone)]
pub enum Page {
    EventList,    // as index now
    AccountState,
    VerifyRequest,
    GenerateAuthorization,
//    PageNotFound
}


#[derive(Clone)]
pub enum Msg {
    // used for routing
    PageFowardTo(Page),

    // used for page account state
    PageAccountStateAccountInput(String),
    PageAccountStateAccountInputBlur(String)
}

mod page_account_state;




#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Model::default(), update, view)
        .routes(routes)
        .finish()
        .run();
}

fn routes(url: seed::Url) -> Msg {
    if url.path.is_empty() {
        return Msg::PageFowardTo(Page::EventList);
    }

    match url.path[0].as_ref() {
        "account_state" => {
            Msg::PageFowardTo(Page::AccountState)
        },
        "verify_request" => {
            Msg::PageFowardTo(Page::VerifyRequest)
        },
        "generate_authorization" => {
            Msg::PageFowardTo(Page::GenerateAuthorization)
        },
        _ => Msg::PageFowardTo(Page::EventList)
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::PageFowardTo(page) => {
            model.page = page;
        },
        Msg::PageAccountStateAccountInput(astr) => {
            page_account_state::on_account_input(model, astr);
        },
        Msg::PageAccountStateAccountInputBlur(astr) => {
            page_account_state::on_account_input_blur(model, astr);
        }
    }
}


// View

fn view(model: &Model) -> impl View<Msg> {
    let page_content_node: Node<Msg> = match model.page {
        Page::EventList => {
            div!["This is event list page"]
        },
        Page::AccountState => {
            page_account_state::page_render(model)
        },
        Page::VerifyRequest => {
            div!["This is verify request page"]
        },
        Page::GenerateAuthorization => {
            div!["This is generateauthorization page"]
        }
    };

    div![
        div![class!["navigation"],
             div![class!["logo"],
                  "Litentry Logo"
             ],
             div![class!["navigator"],
                  div![class!["page_tab"],
                       a!["Account State", attrs!{At::Href => "/account_state"}]
                  ],
                  div![class!["page_tab"],
                       a!["Verify Request", attrs!{At::Href => "/verify_request"}]
                  ],
                  div![class!["page_tab"],
                       a!["GenerateAuthorization", attrs!{At::Href => "/generate_authorization"}]
                  ],
             ],
        ],
        div![class!["page_content"],
             page_content_node
        ],
    ]
}
