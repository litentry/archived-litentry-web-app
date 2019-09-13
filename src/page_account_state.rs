use seed::prelude::*;

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
         ]

    ]
}
