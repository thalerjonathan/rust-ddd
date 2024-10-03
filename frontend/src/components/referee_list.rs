use leptos::*;
use log::{debug, error};
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Referee {
    id: String,
    name: String,
}

#[component]
pub fn RefereeList() -> impl IntoView {
    let (referees, set_referees) = create_signal(Vec::<Referee>::new());
    let (referee_name, set_referee_name) = create_signal(String::new());
    let (referee_club, set_referee_club) = create_signal(String::new());

    create_effect(move |_| {
        set_referees(Vec::<Referee>::new());
        spawn_local(async move {
            let res = fetch_referees().await;
            set_referees(res);
        });
    });

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name = referee_name.get();
        let club = referee_club.get();
        let mut referees_previous = referees.get();
        spawn_local(async move {
            let res = create_referee((name, club)).await;
            match res {
                Ok(r) => {
                    debug!("Referee created: {} {}", r.name, r.id);
                    referees_previous.push(r);
                    set_referees(referees_previous);
                }
                Err(e) => {
                    error!("Error creating referee: {}", e);
                }
            }
        });

        set_referee_name(String::new());
        set_referee_club(String::new());
    };

    view! {
        <div>
            <h2>"Referee List"</h2>
            <form on:submit=on_submit>
                <input
                    id="referee-name"
                    type="text"
                    placeholder="Referee name"
                    prop:value=referee_name
                    on:input=move |ev| set_referee_name.set(event_target_value(&ev))
                />
                <input
                    id="referee-club"
                    type="text"
                    placeholder="Referee club"
                    prop:value=referee_club
                    on:input=move |ev| set_referee_club.set(event_target_value(&ev))
                />
                <button type="submit">"Add Referee"</button>
            </form>

            <ul>
                {move || referees.get().into_iter().map(|r| view! {
                    <li>
                        <a href=format!("/referee/{}", r.id)>{r.name}</a>
                    </li>
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

async fn fetch_referees() -> Vec<Referee> {
    let url = Url::parse("http://localhost:3001/referees");
    let response = reqwest::Client::new().get(url.unwrap()).send().await;
    response.unwrap().json().await.unwrap()
}

async fn create_referee(input: (String, String)) -> Result<Referee, reqwest::Error> {
    let url = Url::parse("http://localhost:3001/referee").unwrap();
    let response = reqwest::Client::new()
        .post(url)
        .json(&input.0)
        .send()
        .await?;
    response.json().await
}
