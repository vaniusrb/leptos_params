use crate::components::product_form::{mock_product, Product};
use leptos::*;

#[component]
pub fn ProductTable(cx: Scope) -> impl IntoView {
    // Fake API read
    let list_reader = create_local_resource(
        cx,
        move || {},
        move |_| async move { Some((vec![mock_product()], 1)) },
    );

    view! {
        cx,
        <Suspense fallback=|| view! { cx, "Loading..." }>
            {move || list_reader.read().map(|product| {
                match product {
                    Some((product, count)) =>  view! { cx,
                        <LoadedProductTable products={product} count={count}/>
                    }.into_view(cx),
                    None => view! { cx, "Error!" }.into_view(cx)
                }
             } )
            }
        </Suspense>
    }
}

#[component]
pub fn LoadedProductTable(cx: Scope, products: Vec<Product>, count: usize) -> impl IntoView {
    view! { cx,
        <h3 class="title is-4">{ format!("Products list {count}") }</h3>
        <ul>
            <For
                each=move || products.clone()
                key=|product| product.id
                view=move |cx, product: Product| {
                    view! { cx,
                        <div>
                            <a href=format!("/product/{}", product.id)>
                                { product.description }
                            </a>
                        </div>
                    }
                }
            />
        </ul>
    }
}
