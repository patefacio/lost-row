use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lost-row.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { 
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=|| view! {
                         <Parent/>
                    }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.

use leptos::MaybeSignal;
use leptos::Signal;

/// Child
#[component]
pub fn Child(
    /// Analogous to forecast
    s: MaybeSignal<String>
) -> impl IntoView {
    let detail = move || {
        use leptos::SignalWith;
        s.with(|s| {
                view! {
                    <tr>
                        <td>{s.clone()}</td>
                    </tr>
                    <tr>
                        <td>"C2"</td>
                    </tr>
                }
        })
    };

    view! {
            <table class="content-table">
                <thead>
                    <th>"H1"</th>
                </thead>
                {detail}
            </table>
    }
}

/// Parent
#[component]
pub fn Parent() -> impl IntoView {
    use leptos::SignalGet;
    use leptos::SignalUpdate;
    let (s, s_write) = leptos::create_signal(String::from("initial"));
    let (count, count_write) = leptos::create_signal(0);
    let button_label = move || format!("Do Forecast {}", count.get());
    let do_forecast = move |_| {
        count_write.update(|c| *c += 1);
        s_write.update(|s| s.push_str("."));
    };
    view! {
        <button on:click=do_forecast>{button_label}</button>
        <h4>"Parent"</h4>
        <Child s=s.into()/>
    }
}
