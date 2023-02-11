use crate::components::product_form::*;
use crate::routes::home::*;
use crate::routes::product_table::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn AppRoutes(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Routes>
            <Route path="/" view=|cx| view! { cx,  <Home/> }/>
            <Route path="product/:id" view=|cx| view! { cx,  <ProductForm/> }/>
            <Route path="products" view=|cx| view! { cx,  <ProductTable/> }/>
        </Routes>
    }
}
