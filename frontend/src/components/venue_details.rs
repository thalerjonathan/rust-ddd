use leptos::*;
use leptos_router::use_params_map;
use shared::fetch_venue;

#[component]
pub fn VenueDetails() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned());

    let (venue, set_venue) = create_signal(None);

    create_effect(move |_| {
        let id = id().unwrap_or_default();
        spawn_local(async move {
            let venue_details: shared::VenueDTO = fetch_venue(id.into()).await.unwrap();
            set_venue(Some(venue_details));
        });
    });

    view! {
        <div>
            <h2>"Venue Details"</h2>
            {move || venue.get().map(|v  | view! {
                <div>
                    <p>"Name: " {v.name}</p>
                    <p>"Street: " {v.street}</p>
                    <p>"Zip: " {v.zip}</p>
                    <p>"City: " {v.city}</p>
                    <p>"Telephone: " {v.telephone}</p>
                    <p>"Email: " {v.email}</p>
                </div>
            })}
        </div>
    }
}
