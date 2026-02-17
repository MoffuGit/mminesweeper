mod backdrop;
mod content;
mod group;
mod group_label;
mod item;
mod portal;
mod radio_group;
mod separator;
mod sub_menu;
mod trigger;
use leptos::context::Provider;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

pub use super::common::Align as MenuAlign;
pub use super::common::Side as MenuSide;
pub use backdrop::*;
pub use content::*;
pub use group::*;
pub use group_label::*;
pub use item::*;
pub use portal::*;
pub use radio_group::*;
pub use separator::*;
pub use sub_menu::*;
pub use trigger::*;

use crate::common::dismissible::DismissibleOptions;
use crate::common::floating::{FloatingContext, use_floating};
use crate::common::floating_tree::{FloatingNode, use_floating_node_id};
use crate::common::status::{TransitionStatusState, use_transition_status};

#[derive(Clone)]
pub struct MenuProviderContext {
    pub open: RwSignal<bool>,
    pub dismissible: bool,
    pub dismiss_opts: DismissibleOptions,
    pub modal: bool,
    pub mount_ref: AnyNodeRef,
    pub trigger_ref: AnyNodeRef,
    pub floating: FloatingContext,
    pub content_ref: AnyNodeRef,
    pub transition_status: TransitionStatusState,
}

#[component]
pub fn MenuProvider(
    children: Children,
    #[prop(optional, default = true)] modal: bool,
    #[prop(optional, into)] open: RwSignal<bool>,
    #[prop(optional, into)] trigger_ref: AnyNodeRef,
    #[prop(optional, into)] content_ref: AnyNodeRef,
    #[prop(optional, into)] mount_ref: AnyNodeRef,
    #[prop(optional)] dismissible: bool,
    #[prop(optional)] dismiss_opts: DismissibleOptions,
    #[prop(into)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    let transition_status = use_transition_status(open.into(), content_ref);
    let id = use_floating_node_id();
    let floating = use_floating(trigger_ref, mount_ref, open, Some(id));
    Effect::new(move |_| {
        if let Some(on_close) = on_close
            && !transition_status.mounted.get()
        {
            on_close.run(());
        }
    });
    view! {
        <FloatingNode id=id.get_value()>
            <Provider
                value=MenuProviderContext {
                    dismiss_opts,
                    mount_ref,
                    transition_status,
                    dismissible,
                    open,
                    modal,
                    trigger_ref,
                    content_ref,
                    floating
                }
            >
                {children()}
            </Provider>
        </FloatingNode>
    }
}
