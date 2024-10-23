use leptos::*;
use leptos_router::use_params_map;
use log::{debug, error};
use restinterface::{change_referee_club, fetch_referee, RefereeDTO};

#[component]
pub fn RefereeDetails() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned());

    let (referee, set_referee) = create_signal(None);
    let (referee_club, set_referee_club) = create_signal(None);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let club: Option<String> = referee_club.get();
        match club {
            Some(club) => {
                let referee_id = id().unwrap_or_default();
                let referee_old_club = referee.get().unwrap();
                spawn_local(async move {
                    let res = change_referee_club(referee_id.into(), &club).await;
                    match res {
                        Ok(club) => {
                            // update referee which will result in re-rendering
                            debug!("Referee club changed to {}", club);
                            set_referee(Some(RefereeDTO {
                                club: club.to_string(),
                                ..referee_old_club
                            }));

                            // reset the form fields
                            set_referee_club(Some(String::new()));
                        }
                        Err(e) => {
                            error!("Error changing referee club: {}", e);
                            // reset the form fields
                            set_referee_club(Some(String::new()));
                        }
                    }
                });
            }
            None => {
                error!("Club is required");
            }
        }
    };

    create_effect(move |_| {
        let id = id().unwrap_or_default();
        spawn_local(async move {
            let referee_details = fetch_referee(id.into()).await;
            set_referee_club(Some(String::new()));
            set_referee(referee_details.ok());
        });
    });

    view! {
        <div>
            <h2>"Referee Details"</h2>
            {move || referee.get().map(|r| view! {
                <div>
                    <p>"Name: " {r.name}</p>
                    <p>"Club: " {r.club}</p>
                </div>
            })}

            <form on:submit=on_submit>
            <input
                    id="referee-club"
                    type="text"
                    placeholder="Changed referee club"
                    prop:value=referee_club
                    on:input=move |ev| set_referee_club.set(Some(event_target_value(&ev)))
                />
                <button type="submit">"Change Club"</button>
            </form>
        </div>
    }
}
