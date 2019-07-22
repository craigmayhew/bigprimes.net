use seed::prelude::*;
use crate::Msg;

pub fn render() -> seed::dom_types::Node<Msg> {
    div![
        h1!["Contact"],
    ]
}