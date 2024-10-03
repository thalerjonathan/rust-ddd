use leptos::*;
use leptos_router::use_params_map;
use reqwest::Url;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct RefereeDetails {
    id: String,
    name: String,
}

#[component]
pub fn RefereeDetails() -> impl IntoView {
    let (referee, set_referee) = create_signal(None);

    let params = use_params_map();

    let id = move || params.with(|params| params.get("id").cloned());

    create_effect(move |_| {
        let id = id().unwrap_or_default();
        spawn_local(async move {
            let referee_details = fetch_referee(&id).await;
            set_referee(Some(referee_details));
        });
    });

    view! {
        <div>
            <h2>"Referee Details"</h2>
            {move || referee.get().map(|r| view! {
                <div>
                    <p>"Name: " {r.name}</p>
                </div>
            })}
        </div>
    }
}

async fn fetch_referee(id: &str) -> RefereeDetails {
    let url = Url::parse(&format!("http://localhost:3001/referee/{}", id));
    let response = reqwest::Client::new().get(url.unwrap()).send().await;
    response.unwrap().json().await.unwrap()
}
