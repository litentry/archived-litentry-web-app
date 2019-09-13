#[macro_use]
extern crate seed;
use seed::prelude::*;


// Model
struct Model {
    page: Page
}

impl Default for Model {
    fn default() -> Self {
        Self {
            page: Page::EventList
        }
    }
}

#[derive(Clone)]
enum Page {
    EventList,    // as index now
    AccountState,
    VerifyRequest,
    GenerateAuthorization,
//    PageNotFound
}


#[derive(Clone)]
enum Msg {
    PageFowardTo(Page),
}

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
    }
}


// View

fn view(model: &Model) -> impl View<Msg> {
    let page_content_node: Node<Msg> = match model.page {
        Page::EventList => {
            div!["This is event list page"]
        },
        Page::AccountState => {
            div!["This is account state page"]
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
