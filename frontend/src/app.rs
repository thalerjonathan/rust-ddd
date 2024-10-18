use crate::components::{
    assignments::Assignments, availabilities::Availabilities, fixture_list::FixtureList, home::Home, referee_details::RefereeDetails, referee_list::RefereeList, team_details::TeamDetails, team_list::TeamList, venue_details::VenueDetails, venue_list::VenueList
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Home />
                <Route path="/referees" view=RefereeList />
                <Route path="/referee/:id" view=RefereeDetails />
                <Route path="/venues" view=VenueList />
                <Route path="/venue/:id" view=VenueDetails />
                <Route path="/teams" view=TeamList />
                <Route path="/team/:id" view=TeamDetails />
                <Route path="/fixtures" view=FixtureList />
                <Route path="/availabilities" view=Availabilities />
                <Route path="/assignments" view=Assignments />
            </Routes>
        </Router>
    }
}
