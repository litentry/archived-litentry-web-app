#[macro_use]
extern crate seed;
use seed::prelude::*;
use seed::{fetch, Method, Request};
use serde::{Serialize, Deserialize};
use futures::Future;
use seed::dom_types::MessageMapper;

mod pas;
mod pvr;
mod pga;

// Model
pub struct Model {
    // for routing
    page: Page,
    pas_model: pas::Model,
    pvr_model: pvr::Model,
    pga_model: pga::Model,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            page: Page::EventList,
            pas_model: pas::Model::default(),
            pvr_model: pvr::Model::default(),
            pga_model: pga::Model::default(),
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
    // for children pages' msgs
    Pas(pas::Msg),
    Pvr(pvr::Msg),
    Pga(pga::Msg),
    Test(String)
}


#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Init::new(Model::default()), update, view)
        .routes(routes)
        // `trigger_update_handler` is necessary,
        // because we want to process `seed::update(..)` calls.
        //.window_events(|_| vec![trigger_update_handler()])
        .finish()
        .run();
}

fn routes(url: seed::Url) -> Option<Msg> {
    if url.path.is_empty() {
        return Some(Msg::PageFowardTo(Page::EventList));
    }

    match url.path[0].as_ref() {
        "account_state" => {
            Some(Msg::PageFowardTo(Page::AccountState))
        },
        "verify_request" => {
            Some(Msg::PageFowardTo(Page::VerifyRequest))
        },
        "generate_authorization" => {
            Some(Msg::PageFowardTo(Page::GenerateAuthorization))
        },
        _ => Some(Msg::PageFowardTo(Page::EventList))
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::PageFowardTo(page) => {
            // switch page
            model.page = page.clone();
            // do child page initialization
            match page {
                Page::AccountState => pas::init(&mut model.pas_model, &mut orders.proxy(Msg::Pas)),
                Page::VerifyRequest => pvr::init(&mut model.pvr_model, &mut orders.proxy(Msg::Pvr)),
                Page::GenerateAuthorization => pga::init(&mut model.pga_model, &mut orders.proxy(Msg::Pga)),
                Page::EventList => {}
            }
        },
        Msg::Pas(msg) => {
            pas::update(msg, &mut model.pas_model, &mut orders.proxy(Msg::Pas));
        }
        Msg::Pvr(msg) => {
            pvr::update(msg, &mut model.pvr_model, &mut orders.proxy(Msg::Pvr));
        }
        Msg::Pga(msg) => {
            pga::update(msg, &mut model.pga_model, &mut orders.proxy(Msg::Pga));
        },
        Msg::Test(astr) => {
            log!(astr)
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
            pas::view(&model.pas_model)
                .map_message(Msg::Pas)
        },
        Page::VerifyRequest => {
            pvr::view(&model.pvr_model)
                .map_message(Msg::Pvr)
        },
        Page::GenerateAuthorization => {
            pga::view(&model.pga_model)
                .map_message(Msg::Pga)
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
