use leptos::*;
use log::debug;
use reqwest::Url;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct Referee {
    id: String,
    name: String,
}

#[component]
pub fn RefereeList() -> impl IntoView {
    let (referees, set_referees) = create_signal(Vec::<Referee>::new());

    create_effect(move |_| {
        spawn_local(async move {
            let url = Url::parse("http://localhost:3001/referees");
            debug!("url: {:?}", url);
            let response = reqwest::Client::new()
                .get(url.unwrap())
                .fetch_mode_no_cors()
                .send()
                .await;
            debug!("Response: {:?}", response);
            let referee_list: Vec<Referee> = response.unwrap().json().await.unwrap();
            set_referees(referee_list);
        });
    });

    view! {
        <div>
            <h2>"Referee List"</h2>
            <ul>
                {move || referees.get().into_iter().map(|r| view! {
                    <li>{r.name}</li>
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
