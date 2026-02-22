use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

use crate::primitive::{RenderElement, RenderFn};

#[derive(Debug, Clone)]
pub struct CheckboxContext {
    checked: RwSignal<bool>,
    disabled: Signal<bool>,
}

#[derive(Debug, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum CheckboxState {
    Checked,
    Unchecked,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CheckboxRootState {
    pub checked: RwSignal<bool>,
    pub disabled: Signal<bool>,
}

#[component]
pub fn CheckboxRoot(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] checked: RwSignal<bool>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] default_checked: MaybeProp<bool>,
    #[prop(default = None)] render: Option<RenderFn<CheckboxRootState>>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(optional)] on_checked_change: Option<Callback<bool, ()>>,
    children: ChildrenFn,
    #[prop(optional)] readonly: MaybeProp<bool>,
    #[prop(optional)] required: MaybeProp<bool>,
    #[prop(optional)] input_ref: AnyNodeRef,
    #[prop(optional)] value: MaybeProp<String>,
) -> impl IntoView {
    //State
    //checked
    //disabled
    //readonly
    //required

    view! {
        //Provider
        //children
        <input
            checked=move || checked.get()
            disabled=disabled
            required=required
            readonly=readonly
            name=name
            value=value
            id=id
            type="checkbox"
            aria_hidden=true
            tabindex=-1
            node_ref=input_ref
        />
    }
    // let spread = view! {
    //     <{..}
    //         role="checkbox"
    //         aria-checked=move || checked.get().to_string()
    //         on:click=move |_| {
    //             if !disabled.get() {
    //                 checked.update(|ch| {
    //                     *ch = !*ch
    //                 });
    //             }
    //         }
    //         disabled=disabled
    //         data-state=move || {
    //             if checked.get() {
    //                 CheckboxState::Checked.to_string()
    //             } else {
    //                 CheckboxState::Unchecked.to_string()
    //             }
    //         }
    //     />
    // };
    // view! {
    //     <Provider value=CheckboxContext { checked, disabled }>
    //         <RenderElement
    //             state=CheckboxRootState { checked, disabled }
    //             render=render
    //             node_ref=node_ref
    //             element=html::button()
    //             {..spread}
    //         >
    //             {children()};
    //         </RenderElement>
    //     </Provider>
    // }
}

#[component]
pub fn CheckboxIndicator(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional, into)] render: Option<RenderFn<()>>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let CheckboxContext { checked, .. } =
        use_context().expect("should access the checkbox context");
    let children = StoredValue::new(children);
    let render = StoredValue::new(render);
    view! {
        <Show when=move || checked.get()>
            <RenderElement
                state=()
                render=render.get_value()
                node_ref=node_ref
                element=html::div()
                {..}
                class=class
                data-state=move || {
                    if checked.get() {
                        CheckboxState::Checked.to_string()
                    } else {
                        CheckboxState::Unchecked.to_string()
                    }
                }
            >
                {children.get_value().map(|children| children())}
            </RenderElement>
        </Show>
    }
}
