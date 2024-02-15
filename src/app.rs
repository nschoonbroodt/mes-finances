use leptos::*;
use leptos_router::*;

use crate::cto::Cto;
use crate::home::Home;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="grid grid-cols-4">
                <div class="flex justify-end col-span-1">
                    <nav class="text-right">
                        <ul>
                            <li><A href="/" class="border-r-4 border-white">Home</A></li>
                            <li><A href="/CTO" class="border-r-4 border-white">CTO</A></li>
                        </ul>
                    </nav>
                </div>
                <div class="col-span-3">
                    <main class="col-span-3">
                        <Routes>
                            <Route path="/" view=Home/>
                            <Route path="/CTO" view=Cto/>
                        </Routes>
                    </main>
                </div>
            </div>
        </Router>
    }
}
