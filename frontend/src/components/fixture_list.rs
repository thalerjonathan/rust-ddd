use chrono::{NaiveDateTime, Utc};
use leptos::*;
use log::{debug, error};

use shared::{
    create_fixture, fetch_fixtures, fetch_teams, fetch_venues, FixtureCreationDTO, FixtureDTO,
    FixtureStatusDTO, TeamDTO, TeamIdDTO, VenueDTO, VenueIdDTO,
};
use uuid::Uuid;

#[component]
pub fn FixtureList() -> impl IntoView {
    let (fixtures, set_fixtures) = create_signal(Vec::<FixtureDTO>::new());
    let (venues, set_venues) = create_signal(Vec::<VenueDTO>::new());
    let (teams, set_teams) = create_signal(Vec::<TeamDTO>::new());

    let (new_fixture_date, set_new_fixture_date) = create_signal(Utc::now());
    let (new_fixture_venue_id, set_new_fixture_venue_id) =
        create_signal(VenueIdDTO(Uuid::new_v4()));
    let (new_fixture_home_team_id, set_new_fixture_home_team_id) =
        create_signal(TeamIdDTO(Uuid::new_v4()));
    let (new_fixture_away_team_id, set_new_fixture_away_team_id) =
        create_signal(TeamIdDTO(Uuid::new_v4()));

    create_effect(move |_| {
        spawn_local(async move {
            let fixtures = fetch_fixtures().await;
            let venues = fetch_venues().await;
            let teams = fetch_teams().await;

            set_new_fixture_away_team_id(teams[0].id.clone().into());
            set_new_fixture_home_team_id(teams[0].id.clone().into());
            set_new_fixture_venue_id(venues[0].id.clone().into());

            set_fixtures(fixtures);
            set_venues(venues);
            set_teams(teams);
        });
    });

    let add_fixture = move |ev: ev::MouseEvent| {
        ev.prevent_default();

        // TODO: check if Team home and Team away are different

        let fixture = FixtureCreationDTO {
            date: new_fixture_date.get(),
            team_home_id: new_fixture_home_team_id.get().into(),
            team_away_id: new_fixture_away_team_id.get().into(),
            venue_id: new_fixture_venue_id.get().into(),
        };

        let mut fixtures_previous = fixtures.get();

        spawn_local(async move {
            let res = create_fixture(&fixture).await;
            match res {
                Ok(f) => {
                    debug!("Fixture created: {} {}", f.id.0, f.date);
                    fixtures_previous.push(f);
                    set_fixtures(fixtures_previous);
                }
                Err(e) => {
                    error!("Error creating fixture: {}", e);
                }
            }
        });
    };

    let select_new_fixture_venue_id = move |ev: ev::Event| {
        let venue_id_str = event_target_value(&ev);
        debug!("venue_id_str: {}", venue_id_str);
        set_new_fixture_venue_id(venue_id_str.into());
    };

    let select_new_fixture_home_team_id = move |ev: ev::Event| {
        let home_team_id_str = event_target_value(&ev);
        debug!("home_team_id_str: {}", home_team_id_str);
        set_new_fixture_home_team_id(home_team_id_str.into());
    };

    let select_new_fixture_away_team_id = move |ev: ev::Event| {
        let away_team_id_str = event_target_value(&ev);
        debug!("away_team_id_str: {}", away_team_id_str);
        set_new_fixture_away_team_id(TeamIdDTO(Uuid::parse_str(&away_team_id_str).unwrap()));
    };

    let select_new_fixture_date = move |ev: ev::Event| {
        let date_str = event_target_value(&ev);
        debug!("date_str: '{}'", date_str);
        let date_native = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M").unwrap();
        let date_utc = date_native.and_local_timezone(Utc).earliest().unwrap();
        debug!("date_utc: {}", date_utc);
        set_new_fixture_date(date_utc);
    };

    view! {
        <div>
            <h2>"Fixtures"</h2>

            <form>
                <input type="datetime-local" on:change=select_new_fixture_date />
                <select name="venue" id="venues" on:change=select_new_fixture_venue_id>
                    {move || venues.get().into_iter().map(|v| view! {
                        <option value={v.id.0.to_string()}>{v.name}</option>
                    }).collect::<Vec<_>>()}
                </select>
                <select name="home_team" id="home_teams" on:change=select_new_fixture_home_team_id>
                    {move || teams.get().into_iter().map(|t| view! {
                        <option value={t.id.0.to_string()}>{t.name}</option>
                    }).collect::<Vec<_>>()}
                </select>
                <select name="away_team" id="away_teams" on:change=select_new_fixture_away_team_id>
                    {move || teams.get().into_iter().map(|t| view! {
                        <option value={t.id.0.to_string()}>{t.name}</option>
                    }).collect::<Vec<_>>()}
                </select>

                <button on:click=add_fixture>"Add Fixture"</button>
            </form>

            <hr/>

            <ul>
                {move || fixtures.get().into_iter().map(|f| view! {
                    <div>
                        <b>{f.date.to_string()}</b>
                        <p>{f.venue.name}</p>
                        <p>{format!("{:?}", f.status)}</p>
                        <p>"Home: " {f.team_home.name}</p>
                        <p>"Away: " {f.team_away.name}</p>
                        <p>"First Referee: " {f.first_referee.map(|r| r.name).unwrap_or("Unassigned".to_string())}</p>
                        <p>"Second Referee: " {f.second_referee.map(|r| r.name).unwrap_or("Unassigned".to_string())}</p>
                        <button disabled=f.status != FixtureStatusDTO::Scheduled>"Change Venue"</button>
                        <button disabled=f.status != FixtureStatusDTO::Scheduled>"Change Date"</button>
                        <button disabled=f.status != FixtureStatusDTO::Scheduled>"Cancel Fixture"</button>
                        <p/>
                        <hr/>
                    </div>
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
