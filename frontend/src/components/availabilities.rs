use leptos::*;
use log::debug;
use shared::{fetch_fixtures, fetch_referees, FixtureDTO, FixtureIdDTO, RefereeDTO};

#[component]
pub fn Availabilities() -> impl IntoView {
    let (_availabilities, _set_availabilities) = create_signal(Vec::<FixtureIdDTO>::new());
    let (referees, set_referees) = create_signal(Vec::<RefereeDTO>::new());
    let (fixtures, set_fixtures) = create_signal(Vec::<FixtureDTO>::new());

    create_effect(move |_| {
        spawn_local(async move {
            let referees = fetch_referees().await;
            let fixtures = fetch_fixtures().await;

            set_referees(referees);
            set_fixtures(fixtures);
        });
    });

    let select_referee = move |ev: ev::Event| {
        let referee_id_str = event_target_value(&ev);
        debug!("referee_id_str: {}", referee_id_str);
        // TODO: fetch availabilities for the selected referee
        // TODO: render availabilities/fixtures
    };

    view! {
        <div>
            <h1>Availabilities</h1>

            <select name="referee" id="referees" on:change=select_referee>
                {move || referees.get().into_iter().map(|r| view! {
                    <option value={r.id.0.to_string()}>{r.name}</option>
                }).collect::<Vec<_>>()}
            </select>

            // TODO: add declare/withdraw availability buttons

            {move || fixtures.get().into_iter().map(|f| view! {
                <div>
                    <b>{f.date.to_string()}</b>
                    <p>{f.venue.name}</p>
                    <p>{format!("{:?}", f.status)}</p>
                    <p>"Home: " {f.team_home.name}</p>
                    <p>"Away: " {f.team_away.name}</p>
                    <p/>
                    <hr/>
                </div>
            }).collect::<Vec<_>>()}
        </div>
    }
}
