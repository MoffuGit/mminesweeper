use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

use crate::primitive::{RenderElement, RenderFn};

#[derive(Debug, Clone)]
pub struct SwitchContext {
    checked: RwSignal<bool>,
    disabled: Signal<bool>,
}

#[derive(Debug, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum SwitchState {
    Checked,
    Unchecked,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SwitchRootState {
    pub checked: RwSignal<bool>,
    pub disabled: Signal<bool>,
}

#[component]
pub fn SwitchRoot(
    #[prop(into, optional)] checked: RwSignal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into, default = Signal::from(false))] disabled: Signal<bool>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional, into)] render: Option<RenderFn<SwitchRootState>>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let spread = view! {
        <{..}
            on:click=move |_| {
                checked.update(|ch| {
                    *ch = !*ch
                });
            }
            disabled=disabled
            class=class
            data-state=move || {
                if checked.get() {
                    SwitchState::Checked.to_string()
                } else {
                    SwitchState::Unchecked.to_string()
                }
            }
        />
    };
    view! {
        <Provider value=SwitchContext { checked, disabled }>
            <RenderElement
                state=SwitchRootState { checked, disabled }
                render=render
                node_ref=node_ref
                element=html::button()
                {..spread}
            >
                {children.get_value().map(|children| children())}
            </RenderElement>
        </Provider>
    }
}

#[component]
pub fn SwitchTumb(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional, into)] render: Option<RenderFn<()>>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let SwitchContext { checked, .. } = use_context().expect("should acces to the switch context");
    let children = StoredValue::new(children);
    let spread = view! {
        <{..}
            class=class
            data-state=move || {
                if checked.get() {
                    SwitchState::Checked.to_string()
                } else {
                    SwitchState::Unchecked.to_string()
                }
            }
        />
    };
    view! {
        <RenderElement
            state=()
            render=render
            node_ref=node_ref
            element=html::span()
            {..spread}
        >
            {children.get_value().map(|children| children())}
        </RenderElement>
    }
}
