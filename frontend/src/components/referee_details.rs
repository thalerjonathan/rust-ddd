use leptos::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct RefereeDetails {
    id: i32,
    name: String,
    age: i32,
    experience: i32,
}

#[component]
pub fn RefereeDetails() -> impl IntoView {
    let (referee_id, set_referee_id) = create_signal(0);
    let (referee, set_referee) = create_signal(None);

    create_effect(move |_| {
        let id = referee_id.get();
        if id > 0 {
            spawn_local(async move {
                let response = reqwest::get(&format!("http://localhost:3000/refereessss/{}", id))
                    .await
                    .unwrap();
                let referee_details: RefereeDetails = response.json().await.unwrap();
                set_referee(Some(referee_details));
            });
        }
    });

    view! {
        <div>
            <h2>"Referee Details"</h2>
            <input
                type="number"
                on:input=move |ev| {
                    set_referee_id(event_target_value(&ev).parse::<i32>().unwrap_or(0))
                }
                prop:value=move || referee_id.get().to_string()
            />
            {move || referee.get().map(|r| view! {
                <div>
                    <p>"Name: " {r.name}</p>
                    <p>"Age: " {r.age}</p>
                    <p>"Experience: " {r.experience} " years"</p>
                </div>
            })}
        </div>
    }
}
