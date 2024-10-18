use leptos::*;
use log::debug;
use shared::{fetch_assignments, fetch_fixtures, fetch_referees, AssignmentDTO, AssignmentRefereeRoleDTO, FixtureDTO, RefereeDTO};

#[component]
pub fn Assignments() -> impl IntoView {
    let (assignments, set_assignments) = create_signal(Vec::<AssignmentDTO>::new());
    let (referees, set_referees) = create_signal(Vec::<RefereeDTO>::new());
    let (fixtures, set_fixtures) = create_signal(Vec::<FixtureDTO>::new());
    
    create_effect(move |_| {
        spawn_local(async move {
            let assignments = fetch_assignments().await;
            let referees = fetch_referees().await;
            let fixtures = fetch_fixtures().await;

            set_assignments(assignments);
            set_referees(referees);
            set_fixtures(fixtures);
        });
    });

    view! {
        <div>
            <h1>Assignments</h1>
        </div>

        {move || fixtures.get().into_iter().map(|f| {
            let first_referee_assignment = assignments.get().into_iter().find(|a| a.fixture_id == f.id && a.referee_role == AssignmentRefereeRoleDTO::First);
            let second_referee_assignment = assignments.get().into_iter().find(|a| a.fixture_id == f.id && a.referee_role == AssignmentRefereeRoleDTO::Second);
            
            let first_referee_assignment_view = match first_referee_assignment {
                Some(a) => view! {
                    <div>
                        <p>First Referee: {referees.get().iter().find(|r| r.id == a.referee_id).unwrap().name.clone()}</p>
                        <p>Assignment Status: {format!("{:?}", a.status)}</p>
                    </div>
                },
                None => view! {
                    <div>
                        <p>First Referee: Unassigned</p>
                    </div>
                }   
            };

            let second_referee_assignment_view = match second_referee_assignment {
                Some(a) => view! {
                    <div>
                        <p>Second Referee: {referees.get().iter().find(|r| r.id == a.referee_id).unwrap().name.clone()}</p>
                        <p>Assignment Status: {format!("{:?}", a.status)}</p>
                    </div>
                },
                None => view! {
                    <div>
                        <p>Second Referee: Unassigned</p>
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
                    {first_referee_assignment_view}
                    {second_referee_assignment_view}
                    <hr/>
                </div>
            }
        }).collect::<Vec<_>>()}
    }
}