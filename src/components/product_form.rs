use chrono::{NaiveDateTime, Utc};
use leptos::*;
use leptos_router::use_params_map;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct Product {
    pub id: Uuid,
    pub description: String,
    pub category: Uuid,
    pub price: Decimal,
    pub created_at: NaiveDateTime,
}

pub fn mock_product() -> Product {
    Product {
        id: Uuid::new_v4(),
        description: String::from("description"),
        category: Uuid::new_v4(),
        price: dec!(100),
        created_at: Utc::now().naive_utc(),
    }
}

#[component]
pub fn ProductForm(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);

    // Fake API read
    let product = create_local_resource(
        cx,
        move || params().get("id").cloned().unwrap_or_default(),
        move |id| async move {
            if id.is_empty() {
                None
            } else {
                Some(mock_product())
            }
        },
    );

    view! {
        cx,
        <Suspense fallback=|| view! { cx, "Loading..." }>
            {move || product.read().map(|product| {
                match product {
                    Some(product) =>  view! { cx,
                        <LoadedProductForm product={product}/>
                    }.into_view(cx),
                    None => view! { cx, "Error!" }.into_view(cx)
                }
             } )
            }
        </Suspense>
    }
}

#[component]
pub fn LoadedProductForm(cx: Scope, product: Product) -> impl IntoView {
    let (product_read, _product_write) = create_signal(cx, product);
    view! { cx,
        <div>
            {product_read().id.to_string()}
        </div>
        <div>
            {product_read().description}
        </div>
        <div>
            {product_read().price.to_string()}
        </div>
        <button class="button"
            on:click=move |_| {
                let navigator = window().history().unwrap();
                navigator.back().unwrap()
            }>"Back"</button>
    }
}
