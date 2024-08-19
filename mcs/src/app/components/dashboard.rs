use leptos::*;

#[component]
pub fn OwnedServers(server_list: Vec<(String, String)>) -> impl IntoView {
    view! {
        <ul>
            {
                server_list.iter().map(|(id, name)| {
                    view! {
                        <li>
                            <a href={format!("/servers/{}", id)}>{name}</a>
                        </li>
                    }
                }).collect::<Vec<_>>()
            }
        </ul>
    }
}
