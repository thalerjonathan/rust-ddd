use leptos::*;
use log::{debug, error};
use shared::{commit_assignments, fetch_assignments, fetch_fixtures, fetch_referees, remove_committed_assignment, remove_staged_assignment, stage_assignment, AssignmentDTO, AssignmentRefereeRoleDTO, AssignmentStagingDTO, AssignmentStatusDTO, FixtureDTO, FixtureIdDTO, RefereeDTO, RefereeIdDTO};

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


    let assign_referee = move |fixture_id: FixtureIdDTO, referee_role: AssignmentRefereeRoleDTO, referee_id: RefereeIdDTO| {
        debug!("Assigning referee to fixture: {:?}, role: {:?}, referee_id: {:?}", fixture_id, referee_role, referee_id);
 
        spawn_local(async move {
            let assignment_staging = AssignmentStagingDTO {
                fixture_id,
                referee_role,
                referee_id,
            };
            let result = stage_assignment(&assignment_staging).await;
            if result.is_ok() {
                // NOTE: we are too lazy to be clever about updating the assignments list, so we simply re-fetch
                let assignments = fetch_assignments().await;
                set_assignments(assignments);
            } else {
                error!("Failed to assign referee: {}", result.err().unwrap());
            }
        });
    };

    view! {
        <div>
            <h1>Assignments</h1>
           
            <button on:click=move |_| {
                spawn_local(async move {
                    let result = commit_assignments().await;
                    if result.is_ok() {
                        let assignments = fetch_assignments().await;
                        set_assignments(assignments);
                    } else {
                        error!("Failed to commit assignments: {}", result.err().unwrap());
                    }
                });
            }>Commit Assignments</button>
            <hr/>
        </div>

        {move || fixtures.get().into_iter().map(|f| {
            let first_referee_assignment = assignments.get().into_iter().find(|a| a.fixture_id == f.id && a.referee_role == AssignmentRefereeRoleDTO::First);
            let second_referee_assignment = assignments.get().into_iter().find(|a| a.fixture_id == f.id && a.referee_role == AssignmentRefereeRoleDTO::Second);
            
            let first_referee_assignment_view = match first_referee_assignment {
                Some(a) => 
                    match a.status {
                        AssignmentStatusDTO::Staged => view! {
                        <div>
                            <p>First Referee: {referees.get().iter().find(|r| r.id == a.referee_id).unwrap().name.clone()}</p>
                            <select name="first_referee" id="first_referee" on:change=move |ev| assign_referee(f.id, AssignmentRefereeRoleDTO::First, RefereeIdDTO(event_target_value(&ev).parse().unwrap()))>
                            {move || referees.get().into_iter().map(|r| {
                                view! {
                                    <option value={r.id.0.to_string()}>{r.name}</option>
                                }
                            }).collect::<Vec<_>>()}
                        </select>
                        <button on:click=move |_| {
                            let a = a.clone();
                            spawn_local(async move {  
                                let result = remove_staged_assignment(&a).await;
                                if result.is_ok() {
                                    let assignments = fetch_assignments().await;
                                    set_assignments(assignments);
                                } else {
                                    error!("Failed to unassign first referee: {}", result.err().unwrap());
                                }
                            });
                        }>Unassign First Referee</button>
                        </div>  
                        },
                        AssignmentStatusDTO::Committed => view! {
                            <div>
                                <p>First Referee: {referees.get().iter().find(|r| r.id == a.referee_id).unwrap().name.clone()}</p>
                                <button on:click=move |_| {
                                    let a = a.clone();
                                    spawn_local(async move {
                                        let result = remove_committed_assignment(&a).await;
                                        if result.is_ok() {
                                            let assignments = fetch_assignments().await;
                                            set_assignments(assignments);
                                        } else {
                                            error!("Failed to unassign referee: {}", result.err().unwrap());
                                        }
                                    });
                                }>Remove First Referee</button>
                            </div>
                        },
                },
                None => view! {
                    <div>
                        <p>First Referee: Unassigned
                        <select name="first_referee" id="first_referee" on:change=move |ev| assign_referee(f.id, AssignmentRefereeRoleDTO::First, RefereeIdDTO(event_target_value(&ev).parse().unwrap()))>
                            {move || referees.get().into_iter().map(|r| {
                                view! {
                                    <option value={r.id.0.to_string()}>{r.name}</option>
                                }
                            }).collect::<Vec<_>>()}
                        </select>
                        </p>
                    </div>
                }   
            };

            let second_referee_assignment_view = match second_referee_assignment {
                Some(a) => 
                    match a.status {
                        AssignmentStatusDTO::Staged => view! {
                        <div>
                            <p>Second Referee: {referees.get().iter().find(|r| r.id == a.referee_id).unwrap().name.clone()}</p>
                            <select name="second_referee" id="second_referee" on:change=move |ev| assign_referee(f.id, AssignmentRefereeRoleDTO::Second, RefereeIdDTO(event_target_value(&ev).parse().unwrap()))>
                            {move || referees.get().into_iter().map(|r| {
                                view! {
                                    <option value={r.id.0.to_string()}>{r.name}</option>
                                }
                            }).collect::<Vec<_>>()}
                        </select>
                        <button on:click=move |_| {
                            let a = a.clone();
                            spawn_local(async move {  
                                let result = remove_staged_assignment(&a).await;
                                if result.is_ok() {
                                    let assignments = fetch_assignments().await;
                                    set_assignments(assignments);
                                } else {
                                    error!("Failed to unassign second referee: {}", result.err().unwrap());
                                }
                            });
                        }>Unassign Second Referee</button>
                        </div>  
                        },
                        AssignmentStatusDTO::Committed => view! {
                            <div>
                                <p>Second Referee: {referees.get().iter().find(|r| r.id == a.referee_id).unwrap().name.clone()}</p>
                                <button on:click=move |_| {
                                    let a = a.clone();
                                    spawn_local(async move {
                                        let result = remove_committed_assignment(&a).await;
                                        if result.is_ok() {
                                            let assignments = fetch_assignments().await;
                                            set_assignments(assignments);
                                        } else {
                                            error!("Failed to unassign second referee: {}", result.err().unwrap());
                                        }
                                    });
                                }>Remove Second Referee</button>
                            </div>
                        },
                },
                None => view! {
                    <div>
                        <p>Second Referee: Unassigned
                        <select name="second_referee" id="second_referees" on:change=move |ev| assign_referee(f.id, AssignmentRefereeRoleDTO::Second, RefereeIdDTO(event_target_value(&ev).parse().unwrap()))>
                            {move || referees.get().into_iter().map(|r| {
                                view! {
                                    <option value={r.id.0.to_string()}>{r.name}</option>
                                }
                            }).collect::<Vec<_>>()}
                        </select>
                        </p>
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