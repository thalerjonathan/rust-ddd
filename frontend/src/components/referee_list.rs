use leptos::*;
use log::{debug, error};
use shared::{create_referee, fetch_referees, RefereeCreationDTO, RefereeDTO};

#[component]
pub fn RefereeList() -> impl IntoView {
    let (referees, set_referees) = create_signal(Vec::<RefereeDTO>::new());
    let (referee_name, set_referee_name) = create_signal(String::new());
    let (referee_club, set_referee_club) = create_signal(String::new());

    create_effect(move |_| {
        // set referees to "use" the signal, so that leptos knows to rerun the effect when it changes after fetching
        set_referees(Vec::<RefereeDTO>::new());
        spawn_local(async move {
            let res = fetch_referees().await;
            set_referees(res);
        });
    });

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        // get the values from the form fields outside of the async block, otherwise leptos complains in browser console
        let name = referee_name.get();
        let club = referee_club.get();
        let ref_creation = RefereeCreationDTO { name, club };
        let mut referees_previous = referees.get();
        spawn_local(async move {
            let res = create_referee(ref_creation).await;
            match res {
                Ok(r) => {
                    // update the list of referees in the UI, which will result in re-rendering
                    debug!("Referee created: {} {}", r.name, r.id);
                    referees_previous.push(r);
                    set_referees(referees_previous);
                }
                Err(e) => {
                    error!("Error creating referee: {}", e);
                }
            }
        });

        // reset the form fields
        set_referee_name(String::new());
        set_referee_club(String::new());
    };

    view! {
        <div>
            <h2>"Referees"</h2>
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
