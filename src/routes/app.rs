use crate::components::app_routes::*;
use crate::components::nav::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/leptos_study.css"/>
        <Meta name="description" content="Leptos study app"/>

        <header class="header">
            <section class="hero is-primary">
            <div class="hero-body">
                <p class="title">{"Rust"}</p>
                <p class="subtitle">{"Leptos study app"}</p>
            </div>
            </section>
        </header>

        <div class="columns">
            <div class="column is-full mx-4">
                <Router>
                    <Nav />
                    <main>
                    <AppRoutes/>
                    </main>
                </Router>
            </div>
        </div>

        <footer class="footer">
                <div class="content has-text-right">
                <p><strong>{"Leptos"}</strong> {" powered by Rust"}</p>
            </div>
        </footer>
    }
}
