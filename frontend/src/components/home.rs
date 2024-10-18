use leptos::{component, view, IntoView};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>
            <h1>"Referee Management with Rust, Leptos, DDD and Microservices"</h1>

            <p><a href="/fixtures">"Fixtures"</a></p>
            <p><a href="/referees">"Referees"</a></p>
            <p><a href="/venues">"Venues"</a></p>
            <p><a href="/teams">"Teams"</a></p>
            <p><a href="/availabilities">"Availabilities"</a></p>
        </div>
    }
}
