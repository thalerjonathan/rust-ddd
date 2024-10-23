use ev::SubmitEvent;
use leptos::*;
use log::error;
use restinterface::{create_venue, fetch_venues, VenueCreationDTO, VenueDTO};

#[component]
pub fn VenueList() -> impl IntoView {
    let (venues, set_venues) = create_signal(Vec::<VenueDTO>::new());
    let (venue_name, set_venue_name) = create_signal(String::new());
    let (venue_street, set_venue_street) = create_signal(String::new());
    let (venue_zip, set_venue_zip) = create_signal(String::new());
    let (venue_city, set_venue_city) = create_signal(String::new());
    let (venue_telephone, set_venue_telephone) = create_signal(String::new());
    let (venue_email, set_venue_email) = create_signal(String::new());

    create_effect(move |_| {
        // set venues to "use" the signal, so that leptos knows to rerun the effect when it changes after fetching
        set_venues(Vec::<VenueDTO>::new());
        spawn_local(async move {
            let res = fetch_venues().await;
            set_venues(res);
        });
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let venue_name = venue_name.get();
        let venue_street = venue_street.get();
        let venue_zip = venue_zip.get();
        let venue_city = venue_city.get();
        let venue_telephone = venue_telephone.get();
        let venue_email = venue_email.get();

        let venue_creation = VenueCreationDTO {
            name: venue_name,
            street: venue_street,
            zip: venue_zip,
            city: venue_city,
            telephone: if venue_telephone.is_empty() {
                None
            } else {
                Some(venue_telephone)
            },
            email: if venue_email.is_empty() {
                None
            } else {
                Some(venue_email)
            },
        };

        let mut venues_previous = venues.get();
        spawn_local(async move {
            let res = create_venue(&venue_creation).await;
            match res {
                Ok(v) => {
                    venues_previous.push(v);
                    set_venues(venues_previous);
                }
                Err(e) => {
                    error!("Error creating venue: {}", e);
                }
            }
        });
    };

    view! {
        <div>
            <h2>"Venues"</h2>

            <form on:submit=on_submit>
                <input
                    id="venue-name"
                    type="text"
                    placeholder="Venue name"
                    prop:value=venue_name
                    on:input=move |ev| set_venue_name.set(event_target_value(&ev))
                />
                <input
                    id="venue-street"
                    type="text"
                    placeholder="Venue street"
                    prop:value=venue_street
                    on:input=move |ev| set_venue_street.set(event_target_value(&ev))
                />
                <input
                    id="venue-zip"
                    type="text"
                    placeholder="Venue zip"
                    prop:value=venue_zip
                    on:input=move |ev| set_venue_zip.set(event_target_value(&ev))
                />
                <input
                    id="venue-city"
                    type="text"
                    placeholder="Venue city"
                    prop:value=venue_city
                    on:input=move |ev| set_venue_city.set(event_target_value(&ev))
                />
                <input
                    id="venue-telephone"
                    type="text"
                    placeholder="Venue telephone"
                    prop:value=venue_telephone
                    on:input=move |ev| set_venue_telephone.set(event_target_value(&ev))
                />
                <input
                    id="venue-email"
                    type="text"
                    placeholder="Venue email"
                    prop:value=venue_email
                    on:input=move |ev| set_venue_email.set(event_target_value(&ev))
                />
                <button type="submit">"Create Venue"</button>
            </form>

            <ul>
                {move || venues.get().into_iter().map(|v| view! {
                    <li>
                        <a href=format!("/venue/{}", v.id.0)>{v.name}</a>
                    </li>
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
