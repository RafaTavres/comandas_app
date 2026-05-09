use leptos::prelude::*;

use leptos::children::ChildrenFn;
use crate::components::common::Navbar;

#[component]
pub fn PageLayout(
    title: String,
    #[prop(optional)] max_width: Option<String>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    children: Children,
) -> impl IntoView {
    let max_width_class = match max_width.as_deref() {
        Some("sm") => "max-w-3xl",
        Some("md") => "max-w-5xl",
        Some("lg") => "max-w-7xl",
        _ => "max-w-7xl",
    };

    let actions_view = actions.map(|actions| view! {
        <div class="flex flex-wrap items-center gap-3 justify-start sm:justify-end">
            {actions()}
        </div>
    });

    view! {
        <Navbar />
        <main class="w-full px-4 py-6 sm:px-6 lg:px-8">
            <div class=format!("mx-auto w-full {max_width_class}")>
                <div class="rounded-3xl overflow-hidden shadow-xl">
                    <section class="bg-gradient-to-r from-sky-600 via-blue-600 to-indigo-700 px-6 py-6 sm:px-8 sm:py-8 text-white">
                        <div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
                            <div>
                                <h1 class="text-2xl font-semibold sm:text-3xl">{title}</h1>
                            </div>
                            {actions_view}
                        </div>
                    </section>

                    <section class="bg-white border border-slate-200 p-6 sm:p-8">
                        {children()}
                    </section>
                </div>
            </div>
        </main>
    }
}
