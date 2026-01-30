use crate::application::use_cases::manage_metadata::ManageMetadata;
use crate::infrastructure::web::custom_object_handlers::CustomObjectAppState;
use crate::infrastructure::web::fragments;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use maud::{html, Markup};
use std::sync::Arc;
use uuid::Uuid;

pub async fn settings_objects_list_handler(
    State(state): State<CustomObjectAppState>,
) -> impl IntoResponse {
    let schema = state.manage_metadata.get_schema().await.unwrap_or_default();
    // In a real app, we'd filter or handle errors better

    let content = html! {
        div class="p-6" {
            div class="flex justify-between items-center mb-6" {
                 h1 class="text-3xl font-bold text-gray-800" { "Object Settings" }
                 a href="/settings/objects/new" class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded shadow transition" { "Create Custom Object" }
            }

            div class="bg-white rounded-lg shadow overflow-hidden" {
                table class="min-w-full divide-y divide-gray-200" {
                    thead class="bg-gray-50" {
                        tr {
                            th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider" { "Name (Singular)" }
                            th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider" { "Name (Plural)" }
                            th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider" { "Description" }
                            th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider" { "Actions" }
                        }
                    }
                    tbody class="bg-white divide-y divide-gray-200" {
                        @for (object, _) in &schema {
                            tr class="hover:bg-gray-50 transition" {
                                td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900" {
                                    a href=(format!("/settings/objects/{}", object.id)) class="text-blue-600 hover:underline" { (object.name_singular) }
                                }
                                td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500" { (object.name_plural) }
                                td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500" { (object.description.clone().unwrap_or_default()) }
                                td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500" {
                                    a href=(format!("/settings/objects/{}", object.id)) class="text-indigo-600 hover:text-indigo-900 mr-4" { "Edit" }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    Html(fragments::layout(content).into_string())
}

pub async fn settings_create_object_form_handler(
    State(_state): State<CustomObjectAppState>,
) -> impl IntoResponse {
    let content = html! {
        div class="max-w-2xl mx-auto mt-10 p-6 bg-white rounded-lg shadow" {
            h2 class="text-2xl font-bold mb-6 text-gray-800" { "Create Custom Object" }
            form action="/api/objects" method="POST" class="space-y-4" {
                // HTMX would be better here to handle response and redirect
                // For now, using standard form for simplicity, but we need JS to handle the JSON API expectation
                // or updated the API to handle Form data.
                // Let's use vanilla JS to submit as JSON for now to reuse the API.

                div {
                    label class="block text-sm font-medium text-gray-700 mb-1" { "Singular Name" }
                    input type="text" id="name_singular" name="name_singular" required class="w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 border p-2";
                }

                div {
                    label class="block text-sm font-medium text-gray-700 mb-1" { "Plural Name" }
                    input type="text" id="name_plural" name="name_plural" required class="w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 border p-2";
                }

                div {
                    label class="block text-sm font-medium text-gray-700 mb-1" { "Description" }
                    textarea id="description" name="description" rows="3" class="w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 border p-2" {};
                }

                div class="flex justify-end space-x-3 mt-6" {
                    a href="/settings/objects" class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50" { "Cancel" }
                    button type="button" onclick="submitObjectForm()" class="px-4 py-2 bg-blue-600 border border-transparent rounded-md text-sm font-medium text-white hover:bg-blue-700" { "Create Object" }
                }
            }
            script {
                (maud::PreEscaped(r#"
                    async function submitObjectForm() {
                        const name_singular = document.getElementById('name_singular').value;
                        const name_plural = document.getElementById('name_plural').value;
                        const description = document.getElementById('description').value;

                        const response = await fetch('/api/objects', {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify({ name_singular, name_plural, description })
                        });

                        if (response.ok) {
                            window.location.href = '/settings/objects';
                        } else {
                            alert('Failed to create object');
                        }
                    }
                "#))
            }
        }
    };

    Html(fragments::layout(content).into_string())
}

pub async fn settings_object_detail_handler(
    State(state): State<CustomObjectAppState>,
    Path(object_id): Path<Uuid>,
) -> impl IntoResponse {
    let schema = state.manage_metadata.get_schema().await.unwrap_or_default();
    let current_object_tuple = schema.iter().find(|(obj, _)| obj.id == object_id);

    if let Some((object, fields)) = current_object_tuple {
        let content = html! {
            div class="p-6" {
                div class="mb-8" {
                    a href="/settings/objects" class="text-blue-600 hover:text-blue-800 mb-4 inline-block" { "← Back to Objects" }
                    div class="flex justify-between items-center" {
                        div {
                            h1 class="text-3xl font-bold text-gray-800" { (object.name_plural) }
                            p class="text-gray-600 mt-1" { (object.description.clone().unwrap_or_default()) }
                        }
                    }
                }

                div class="bg-white rounded-lg shadow overflow-hidden mb-8" {
                    div class="px-6 py-4 border-b border-gray-200 flex justify-between items-center bg-gray-50" {
                        h2 class="text-lg font-medium text-gray-900" { "Fields" }
                        // Button to add field (simplified JS implementation for now)
                         button onclick="document.getElementById('create-field-modal').classList.remove('hidden')" class="bg-indigo-600 hover:bg-indigo-700 text-white px-3 py-1 rounded text-sm transition" { "Add Field" }
                    }
                    table class="min-w-full divide-y divide-gray-200" {
                        thead class="bg-gray-50" {
                            tr {
                                th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider" { "Name" }
                                th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider" { "Type" }
                            }
                        }
                        tbody class="bg-white divide-y divide-gray-200" {
                            @for field in fields {
                                tr {
                                    td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900" { (field.name) }
                                    td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500" { (format!("{:?}", field.field_type)) }
                                }
                            }
                        }
                    }
                }

                // Modal for Creating Field
                div id="create-field-modal" class="hidden fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full" {
                    div class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white" {
                        div class="mt-3 text-center" {
                            h3 class="text-lg leading-6 font-medium text-gray-900" { "Add New Field" }
                            div class="mt-2 px-7 py-3" {
                                input type="text" id="field_name" placeholder="Field Name" class="mb-3 px-3 py-2 border rounded-md w-full focus:outline-none focus:ring focus:border-blue-300";
                                select id="field_type" class="mb-3 px-3 py-2 border rounded-md w-full focus:outline-none focus:ring focus:border-blue-300" {
                                    option value="Text" { "Text" }
                                    option value="Number" { "Number" }
                                    option value="Date" { "Date" }
                                    option value="Boolean" { "Boolean" }
                                    // Add other types as needed
                                }
                            }
                            div class="items-center px-4 py-3" {
                                button id="create-field-btn" onclick=(format!("submitFieldForm('{}')", object.id)) class="px-4 py-2 bg-blue-500 text-white text-base font-medium rounded-md w-full shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300" { "Create Field" }
                                button onclick="document.getElementById('create-field-modal').classList.add('hidden')" class="mt-2 px-4 py-2 bg-gray-300 text-gray-700 text-base font-medium rounded-md w-full shadow-sm hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-300" { "Cancel" }
                            }
                        }
                    }
                }

                script {
                    (maud::PreEscaped(r#"
                        async function submitFieldForm(objectId) {
                            const name = document.getElementById('field_name').value;
                            const field_type = document.getElementById('field_type').value;

                            const response = await fetch(`/api/objects/${objectId}/fields`, {
                                method: 'POST',
                                headers: { 'Content-Type': 'application/json' },
                                body: JSON.stringify({ name, field_type, settings: {} })
                            });

                            if (response.ok) {
                                window.location.reload();
                            } else {
                                alert('Failed to create field');
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
