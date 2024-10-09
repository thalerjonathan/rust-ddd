use ev::SubmitEvent;
use leptos::*;
use log::error;
use shared::{create_team, fetch_teams, TeamCreationDTO, TeamDTO};

#[component]
pub fn TeamList() -> impl IntoView {
    let (teams, set_teams) = create_signal(Vec::<TeamDTO>::new());
    let (team_name, set_team_name) = create_signal(String::new());
    let (team_club, set_team_club) = create_signal(String::new());

    create_effect(move |_| {
        // set teams to "use" the signal, so that leptos knows to rerun the effect when it changes after fetching
        set_teams(Vec::<TeamDTO>::new());
        spawn_local(async move {
            let res = fetch_teams().await;
            set_teams(res);
        });
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let team = TeamCreationDTO {
            name: team_name.get(),
            club: team_club.get(),
        };

        let mut teams_previous = teams.get();
        spawn_local(async move {
            let res = create_team(team).await;
            match res {
                Ok(t) => {
                    teams_previous.push(t);
                    set_teams(teams_previous);
                    set_team_name(String::new());
                    set_team_club(String::new());
                }
                Err(e) => {
                    error!("Error creating team: {}", e);
                }
            }
        });
    };

    view! {
        <div>
            <h2>"Teams"</h2>

            <form on:submit=on_submit>
                <input
                    id="team-name"
                    type="text"
                    placeholder="Team name"
                    prop:value=team_name
                    on:input=move |ev| set_team_name.set(event_target_value(&ev))
                />
                <input
                    id="team-club"
                    type="text"
                    placeholder="Team club"
                    prop:value=team_club
                    on:input=move |ev| set_team_club.set(event_target_value(&ev))
                />
                <button type="submit">"Create Team"</button>
            </form>

            <ul>
                {move || teams.get().into_iter().map(|t| view! {
                    <li>
                        <a href=format!("/team/{}", t.id)>{t.name}</a>
                    </li>
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
