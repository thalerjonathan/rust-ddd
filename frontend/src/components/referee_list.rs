use leptos::*;
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
    let referees_resource = create_resource(referees, |_| async move { fetch_referees().await });
    let (new_referee_name, set_new_referee_name) = create_signal(String::new());
    let create_referee_action = create_action(|name: &String| create_referee(name.clone()));

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        create_referee_action.dispatch(new_referee_name.get());
        set_new_referee_name.set(String::new());
        referees_resource.refetch();
    };

    view! {
        <div>
            <h2>"Referee List"</h2>
            <form on:submit=on_submit>
                <input
                    type="text"
                    placeholder="New referee name"
                    prop:value=new_referee_name
                    on:input=move |ev| set_new_referee_name.set(event_target_value(&ev))
                />
                <button type="submit">"Add Referee"</button>
            </form>

            {move || match referees_resource.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(referees) => {
                    view! {
                        <ul>
                            {referees.into_iter().map(|r| view! {
                                <li>
                                    <a href=format!("/referee/{}", r.id)>{r.name}</a>
                                </li>
                            }).collect::<Vec<_>>()}
                        </ul>
                    }.into_view()
                }
            }}
        </div>
    }
}

async fn fetch_referees() -> Vec<Referee> {
    let url = Url::parse("http://localhost:3001/referees");
    let response = reqwest::Client::new().get(url.unwrap()).send().await;
    response.unwrap().json().await.unwrap()
}

async fn create_referee(name: String) -> Result<Referee, reqwest::Error> {
    let url = Url::parse("http://localhost:3001/referee").unwrap();
    let response = reqwest::Client::new().post(url).json(&name).send().await?;
    response.json().await
}
