use leptos::html;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

use crate::primitive::{RenderElement, RenderFn};
use crate::tabs::{TabsContext, use_tabs_context};

#[component]
pub fn Panel(
    value: String,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional, into)] render: Option<RenderFn<()>>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let value = StoredValue::new(value);
    let children = StoredValue::new(children);
    let TabsContext { selected_tab, .. } = use_tabs_context();
    let spread = view! { <{..} class=class /> };

    view! {
        <Show when=move || selected_tab.with(|selected| selected == &value.get_value())>
            <RenderElement state=() render=render.clone() node_ref=node_ref element=html::div() {..spread.clone()}>
                {children.get_value().map(|children| children())}
            </RenderElement>
        </Show>
    }
}
