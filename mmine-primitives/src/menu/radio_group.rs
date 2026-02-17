use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::primitive::{RenderElement, RenderFn};

#[derive(Clone)]
pub struct MenuRadioGroupContext {
    pub value: RwSignal<String>,
}

#[derive(Clone)]
pub struct MenuRadioItemContext {
    pub item_value: Signal<String>,
}

#[component]
pub fn MenuRadioGroupProvider(
    children: Children,
    #[prop(into)] value: RwSignal<String>,
) -> impl IntoView {
    view! {
        <Provider value=MenuRadioGroupContext { value }>
            {children()}
        </Provider>
    }
}

#[component]
pub fn MenuRadioGroupItem(
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into)] value: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] render: Option<RenderFn<()>>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let radio_group_context = use_context::<MenuRadioGroupContext>()
        .expect("MenuRadioGroupItem must be used within a MenuRadioGroupProvider");

    let children = StoredValue::new(children);
    let spread = view! {
        <{..}
            class=class
            on:click=move |_| {
                radio_group_context.value.set(value.get());
            }
        />
    };
    view! {
        <Provider value=MenuRadioItemContext { item_value: value }>
            <RenderElement
                state=()
                render=render
                node_ref=node_ref
                element=html::div()
                {..spread}
            >
                {children.get_value().map(|children| children())}
            </RenderElement>
        </Provider>
    }
}

#[component]
pub fn MenuRadioGroupItemIndicator(children: ChildrenFn) -> impl IntoView {
    let radio_group_context = use_context::<MenuRadioGroupContext>()
        .expect("MenuRadioGroupItemIndicator must be used within a MenuRadioGroupProvider");
    let radio_item_context = use_context::<MenuRadioItemContext>()
        .expect("MenuRadioGroupItemIndicator must be used within a MenuRadioGroupItem");

    let is_selected =
        Memo::new(move |_| radio_group_context.value.get() == radio_item_context.item_value.get());

    view! {
        <Show when=is_selected>
            {children()}
        </Show>
    }
}
