use leptos::html;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

use crate::primitive::{RenderElement, RenderFn};
use crate::tabs::TabsContext;

use super::use_tabs_context;

#[component]
pub fn TabsList(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional, into)] render: Option<RenderFn<()>>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let TabsContext { orientation, .. } = use_tabs_context();
    let spread = view! { <{..} class=class data-orientation=orientation.to_string() /> };

    view! {
        <RenderElement state=() render=render node_ref=node_ref element=html::div() {..spread}>
            {children.get_value().map(|children| children())}
        </RenderElement>
    }
}
