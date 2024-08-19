use crate::app::components::dashboard::*;
use leptos::*;
use leptos_router::use_params_map;

#[component]
pub fn ServerList() -> impl IntoView {
    view! {
        <h1>"Servers"</h1>
        // FIXME: This is only for debugging
        <OwnedServers server_list = { vec![("1".to_string(), "Test Server".to_string())] } />
    }
}

#[component]
pub fn Dashboard() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    view! {
        <p>{ "serving server: " }{ id }</p>
    }
}
