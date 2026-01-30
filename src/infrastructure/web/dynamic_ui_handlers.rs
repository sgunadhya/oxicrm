use crate::infrastructure::web::custom_object_handlers::CustomObjectAppState;
use crate::infrastructure::web::fragments;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use maud::{html, Markup, PreEscaped};
use uuid::Uuid;

pub async fn dynamic_record_list_handler(
    State(state): State<CustomObjectAppState>,
    Path(object_id): Path<Uuid>,
) -> impl IntoResponse {
    let schema = state.manage_metadata.get_schema().await.unwrap_or_default();
    let current_object_tuple = schema.iter().find(|(obj, _)| obj.id == object_id);

    if let Some((object, fields)) = current_object_tuple {
        // Fetch records
        let records = state
            .manage_custom_object_data
            .list_records(object_id)
            .await
            .unwrap_or_default();

        let content = html! {
            div class="p-6" {
                 div class="flex justify-between items-center mb-6" {
                     h1 class="text-3xl font-bold text-gray-800" { (object.name_plural) }
                     a href=(format!("/app/objects/{}/new", object.id)) class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded shadow transition" { "New " (object.name_singular) }
                }

                div class="bg-white rounded-lg shadow overflow-x-auto" {
                    table class="min-w-full divide-y divide-gray-200" {
                        thead class="bg-gray-50" {
                            tr {
                                // Default columns
                                th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider" { "Created At" }
                                // Dynamic columns
                                @for field in fields {
                                    th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider" { (field.name) }
                                }
                                th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider" { "Actions" }
                            }
                        }
                        tbody class="bg-white divide-y divide-gray-200" {
                            @for record in records {
                                tr class="hover:bg-gray-50 transition" {
                                    td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500" { (record.created_at.format("%Y-%m-%d %H:%M")) }
                                    @for field in fields {
                                        td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900" {
                                            // Extract property value safely
                                            (record.properties.get(&field.name).and_then(|v| v.as_str()).unwrap_or("-"))
                                        }
                                    }
                                    td class="px-6 py-4 whitespace-nowrap text-sm font-medium" {
                                        a href=(format!("/app/records/{}", record.id)) class="text-indigo-600 hover:text-indigo-900 mr-4" { "Edit" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        Html(fragments::layout(content).into_string())
    } else {
        Html(fragments::layout(html! { h1 { "Object not found" } }).into_string())
    }
}

pub async fn dynamic_record_create_form_handler(
    State(state): State<CustomObjectAppState>,
    Path(object_id): Path<Uuid>,
) -> impl IntoResponse {
    let schema = state.manage_metadata.get_schema().await.unwrap_or_default();
    let current_object_tuple = schema.iter().find(|(obj, _)| obj.id == object_id);

    if let Some((object, fields)) = current_object_tuple {
        let content = html! {
             div class="max-w-2xl mx-auto mt-10 p-6 bg-white rounded-lg shadow" {
                h2 class="text-2xl font-bold mb-6 text-gray-800" { "Create " (object.name_singular) }
                form id="create-record-form" class="space-y-4" {
                    @for field in fields {
                        div {
                            label class="block text-sm font-medium text-gray-700 mb-1" { (field.name) }
                            // Simple input for all types for now, enhance later based on FieldType
                            input type="text" name=(field.name) class="w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 border p-2";
                        }
                    }

                    div class="flex justify-end space-x-3 mt-6" {
                        a href=(format!("/app/objects/{}", object.id)) class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50" { "Cancel" }
                        button type="button" onclick=(format!("submitRecordForm('{}')", object.id)) class="px-4 py-2 bg-blue-600 border border-transparent rounded-md text-sm font-medium text-white hover:bg-blue-700" { "Save" }
                    }
                }

                script {
                    (PreEscaped(r#"
                        async function submitRecordForm(objectId) {
                            const form = document.getElementById('create-record-form');
                            const formData = new FormData(form);
                            const properties = Object.fromEntries(formData.entries());

                            const response = await fetch(`/api/objects/${objectId}/records`, {
                                method: 'POST',
                                headers: { 'Content-Type': 'application/json' },
                                body: JSON.stringify({ properties })
                            });

                            if (response.ok) {
                                window.location.href = `/app/objects/${objectId}`;
                            } else {
                                alert('Failed to create record');
                            }
                        }
                    "#))
                }
            }
        };
        Html(fragments::layout(content).into_string())
    } else {
        Html(fragments::layout(html! { h1 { "Object not found" } }).into_string())
    }
}

// Handler for the sidebar navigation items (HTMX loaded)
pub async fn nav_custom_objects_handler(
    State(state): State<CustomObjectAppState>,
) -> impl IntoResponse {
    let schema = state.manage_metadata.get_schema().await.unwrap_or_default();

    let content = html! {
         @for (object, _) in &schema {
             a href=(format!("/app/objects/{}", object.id)) class="block py-2.5 px-4 rounded transition duration-200 hover:bg-gray-700 hover:text-white" {
                 (object.name_plural)
             }
         }
    };

    Html(content.into_string())
}
