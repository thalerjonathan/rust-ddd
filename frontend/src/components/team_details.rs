use leptos::*;
use leptos_router::use_params_map;
use restinterface::fetch_team;

#[component]
pub fn TeamDetails() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned());

    let (team, set_team) = create_signal(None);

    create_effect(move |_| {
        let id = id().unwrap_or_default();
        spawn_local(async move {
            let team_details: restinterface::TeamDTO = fetch_team(id.into()).await.unwrap();
            set_team(Some(team_details));
        });
    });

    view! {
        <div>
            <h2>"Team Details"</h2>
            {move || team.get().map(|t| view! {
                <div>
                    <p>"Name: " {t.name}</p>
                    <p>"Club: " {t.club}</p>
                </div>
            })}
        </div>
    }
}
