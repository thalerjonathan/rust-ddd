use crate::components::{
    home::Home, referee_details::RefereeDetails, referee_list::RefereeList,
    venue_details::VenueDetails, venue_list::VenueList,
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Home />
                <Route path="/refereelist" view=RefereeList />
                <Route path="/referee/:id" view=RefereeDetails />
                <Route path="/venuelist" view=VenueList />
                <Route path="/venue/:id" view=VenueDetails />
            </Routes>
        </Router>
    }
}
