use crate::components::{home::Home, referee_list::RefereeList};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Home />
                <Route path="/refereelist" view=RefereeList />
            </Routes>
        </Router>
    }
}
