use leptos::html::button;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

use crate::primitive::{RenderElement, RenderFn};

#[component]
pub fn Button(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into, default = Signal::from(false))] disabled: Signal<bool>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional, into)] render: Option<RenderFn<ButtonState>>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let spread = view! {
        <{..}
            disabled=disabled
            class=class
        />
    };
    view! {
        <RenderElement
            state=ButtonState {
                disabled,
            }
            render=render
            node_ref=node_ref
            element=button()

            {..spread}
        >
            {children.get_value().map(|children| children())}
        </RenderElement>

    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ButtonState {
    pub disabled: Signal<bool>,
}
