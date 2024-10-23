use leptos::*;
use log::debug;
use restinterface::{
    declare_availability, fetch_availabilities_for_referee, fetch_fixtures, fetch_referees,
    withdraw_availability, FixtureDTO, FixtureIdDTO, RefereeDTO, RefereeIdDTO,
};

#[component]
pub fn Availabilities() -> impl IntoView {
    let (availabilities, set_availabilities) = create_signal(Vec::<FixtureIdDTO>::new());
    let (referees, set_referees) = create_signal(Vec::<RefereeDTO>::new());
    let (fixtures, set_fixtures) = create_signal(Vec::<FixtureDTO>::new());
    let (selected_referee_id, set_selected_referee_id) =
        create_signal(Option::<RefereeIdDTO>::None);

    create_effect(move |_| {
        spawn_local(async move {
            let referees = fetch_referees().await;
            let fixtures = fetch_fixtures().await;
            let selected_referee_id: RefereeIdDTO = referees.first().unwrap().id;
            let availabilities = fetch_availabilities_for_referee(selected_referee_id)
                .await
                .unwrap();

            set_referees(referees);
            set_fixtures(fixtures);
            set_availabilities(availabilities);
            set_selected_referee_id(Some(selected_referee_id));
        });
    });

    let select_referee = move |ev: ev::Event| {
        let referee_id_str = event_target_value(&ev);
        let referee_id = RefereeIdDTO(referee_id_str.parse().unwrap());
        debug!("referee_id_str: {}", referee_id_str);

        spawn_local(async move {
            let availabilities = fetch_availabilities_for_referee(referee_id).await.unwrap();
            set_availabilities(availabilities);
            set_selected_referee_id(Some(referee_id));
        });
    };

    let on_withdraw_availability = move |fixture_id: FixtureIdDTO| {
        debug!("withdrawing availability for fixture_id: {}", fixture_id.0);
        spawn_local(async move {
            let result =
                withdraw_availability(fixture_id, selected_referee_id.get().unwrap()).await;
            match result {
                Ok(_) => {
                    let updated_availabilities = availabilities
                        .get()
                        .into_iter()
                        .filter(|a| *a != fixture_id)
                        .collect();
                    set_availabilities(updated_availabilities);
                }
                Err(e) => {
                    debug!("withdraw_availability error: {:?}", e);
                }
            }
        });
    };

    let on_declare_availability = move |fixture_id: FixtureIdDTO| {
        debug!("declaring availability for fixture_id: {}", fixture_id.0);
        spawn_local(async move {
            let result = declare_availability(fixture_id, selected_referee_id.get().unwrap()).await;
            match result {
                Ok(_) => {
                    let mut updated_availabilities = availabilities.get();
                    updated_availabilities.push(fixture_id);
                    set_availabilities(updated_availabilities);
                }
                Err(e) => {
                    debug!("declare_availability error: {:?}", e);
                }
            }
        });
    };

    view! {
        <div>
            <h1>Availabilities</h1>

            <select name="referee" id="referees" on:change=select_referee>
                {move || referees.get().into_iter().map(|r| view! {
                    <option value={r.id.0.to_string()}>{r.name}</option>
                }).collect::<Vec<_>>()}
            </select>

            {move || fixtures.get().into_iter().map(|f| {
                let ret = if availabilities.get().contains(&f.id) {
                    view! {
                        <div>
                            <p>Availability: Declared</p>
                            <button on:click= move |_ev: ev::MouseEvent| { on_withdraw_availability(f.id) }>Withdraw Availability</button>
                        </div>
                    }
                } else {
                    view! {
                        <div>
                            <p>Availability: Not declared</p>
                            <button on:click=move |_ev: ev::MouseEvent| { on_declare_availability(f.id) }>Declare Availability</button>
                        </div>
                    }
                };

                view! {
                    <div>
                        <b>{f.date.to_string()}</b>
                        <p>{f.venue.name}</p>
                        <p>{format!("{:?}", f.status)}</p>
                        <p>"Home: " {f.team_home.name}</p>
                        <p>"Away: " {f.team_away.name}</p>
                        <p/>
                        {ret}
                        <hr/>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
