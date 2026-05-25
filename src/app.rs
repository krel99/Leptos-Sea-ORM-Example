use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use serde::{Deserialize, Serialize};

// Shared between client and server — used as server fn return type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryData {
    pub field1: String,
    pub field2: String,
}

#[server]
pub async fn get_entries() -> Result<Vec<EntryData>, ServerFnError<String>> {
    use crate::entity;
    use sea_orm::EntityTrait;

    let db = crate::db::get_db();
    let rows = entity::Entity::find()
        .all(db)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(rows
        .into_iter()
        .map(|r| EntryData {
            field1: r.field1,
            field2: r.field2,
        })
        .collect())
}

#[server]
pub async fn submit_entry(field1: String, field2: String) -> Result<(), ServerFnError<String>> {
    use crate::entity;
    use sea_orm::{ActiveModelTrait, Set};

    let db = crate::db::get_db();
    entity::ActiveModel {
        field1: Set(field1),
        field2: Set(field2),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(())
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Router>
            <Routes fallback=|| "Not found">
                <Route path=path!("/") view=HomePage/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let submit_action = ServerAction::<SubmitEntry>::new();
    let field1 = RwSignal::new(String::new());
    let field2 = RwSignal::new(String::new());

    // Re-fetches every time the action completes
    let entries = Resource::new(move || submit_action.version().get(), |_| get_entries());

    view! {
        <h1>"Simple Form"</h1>
        <div>
            <input
                type="text"
                placeholder="Field 1"
                prop:value=field1
                on:input=move |ev| field1.set(event_target_value(&ev))
            />
            <input
                type="text"
                placeholder="Field 2"
                prop:value=field2
                on:input=move |ev| field2.set(event_target_value(&ev))
            />
            <button on:click=move |_| {
                submit_action.dispatch(SubmitEntry {
                    field1: field1.get(),
                    field2: field2.get(),
                });
            }>
                "Submit"
            </button>
        </div>

        <h2>"Previous Entries"</h2>
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            {move || {
                entries.get().map(|result| match result {
                    Ok(items) if items.is_empty() => {
                        view! { <p>"No entries yet."</p> }.into_any()
                    }
                    Ok(items) => items
                        .into_iter()
                        .map(|e| {
                            view! {
                                <div>
                                    <span>{e.field1}</span>
                                    {" | "}
                                    <span>{e.field2}</span>
                                </div>
                            }
                        })
                        .collect_view()
                        .into_any(),
                    Err(e) => {
                        view! { <p>"Error: " {e.to_string()}</p> }.into_any()
                    }
                })
            }}
        </Suspense>
    }
}
