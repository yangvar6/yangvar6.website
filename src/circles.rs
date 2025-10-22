use leptos::prelude::*;

#[component]
pub fn Patterns(amount: usize, radius_base: usize, radius_max: usize) -> impl IntoView {
    let _ = (amount, radius_base, radius_max);

    view! {
        <div>
            "There are " {amount} " things, just use your ImAgInAtIoN"
        </div>
    }
}
