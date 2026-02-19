use leptos::html::button;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

use crate::primitive::{RenderElement, RenderFn};

#[component]
pub fn Button(
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None)] render: Option<RenderFn<ButtonState>>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <RenderElement
            state=ButtonState {
                disabled,
            }
            render=render
            node_ref=node_ref
            element=button()
            {..}
            data-disabled=move || disabled.get()
        >
            {children()}
        </RenderElement>

    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ButtonState {
    pub disabled: Signal<bool>,
}
