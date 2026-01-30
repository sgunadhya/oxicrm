use crate::domain::{Opportunity, OpportunityStage, Person};
use maud::{html, Markup, DOCTYPE};

pub fn layout(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Twenty CRM" }
                script src="https://unpkg.com/htmx.org@1.9.10" {}
                script src="https://unpkg.com/alpinejs@3.13.3" defer {}
                // Tailwind CSS for styling (CDN for scaffolding)
                script src="https://cdn.tailwindcss.com" {}
            }
            body class="bg-gray-100" {
                (content)
            }
        }
    }
}

pub fn board_card(opportunity: &Opportunity) -> Markup {
    let stage_class = match opportunity.stage {
        OpportunityStage::New => "bg-blue-100 border-l-4 border-blue-500",
        OpportunityStage::Meeting => "bg-yellow-100 border-l-4 border-yellow-500",
        OpportunityStage::Proposal => "bg-purple-100 border-l-4 border-purple-500",
        OpportunityStage::Customer => "bg-green-100 border-l-4 border-green-500",
        OpportunityStage::Lost => "bg-red-100 border-l-4 border-red-500",
    };

    html! {
        div
            id=(format!("card-{}", opportunity.id))
            class=(format!("p-4 mb-2 rounded shadow cursor-move bg-white {}", stage_class))
            x-data=(format!("{{ dragging: false, id: '{}' }}", opportunity.id))
            draggable="true"
            "x-on:dragstart"=("dragging = true; $event.dataTransfer.setData('text/plain', id)")
            "x-on:dragend"=("dragging = false")
            ":class"="{ 'opacity-50': dragging }"
        {
            h3 class="font-bold text-gray-800" { (opportunity.name) }
            div class="text-sm text-gray-600 mt-1" {
                (format!("${}", opportunity.amount_micros.unwrap_or(0) as f64 / 100.0))
            }
        }
    }
}

pub fn kanban_board(opportunities: &[Opportunity]) -> Markup {
    let stages = vec![
        OpportunityStage::New,
        OpportunityStage::Meeting,
        OpportunityStage::Proposal,
        OpportunityStage::Customer,
        OpportunityStage::Lost,
    ];

    html! {
        div class="flex h-screen p-8 overflow-x-auto gap-4" id="board-container" {
            @for stage in stages {
                div class="flex-shrink-0 w-80 flex flex-col" {
                    div class="font-bold text-gray-700 mb-4 uppercase tracking-wide flex justify-between items-center" {
                        (format!("{:?}", stage))
                        span class="bg-gray-200 text-gray-600 px-2 py-1 rounded text-xs" {
                            (opportunities.iter().filter(|o| o.stage == stage).count())
                        }
                    }
                    div class="bg-gray-200 rounded-lg p-4 flex-1 overflow-y-auto min-h-[500px]"
                        "ondragover"="event.preventDefault()"
                        "ondrop"=(format!("
                            let id = event.dataTransfer.getData('text/plain');
                            htmx.ajax('POST', '/cards/' + id + '/move', {{ 
                                target: '#board-container', 
                                swap: 'outerHTML', 
                                values: {{ new_stage: '{:?}' }} 
                            }}).then(() => window.location.reload())", stage)) 
                    {
                        @for opp in opportunities.iter().filter(|o| o.stage == stage) {
                            (board_card(opp))
                        }
                    }
                }
            }
        }
    }
}

pub fn register_form() -> Markup {
    html! {
        div class="max-w-md mx-auto mt-10" {
            form hx-post="/register" hx-target="#result" {
                h2 class="text-2xl font-bold mb-4" { "Register" }

                label class="block mb-2" { "Email" }
                input type="email" name="email" class="border p-2 w-full mb-4" required;

                label class="block mb-2" { "Password" }
                input type="password" name="password" class="border p-2 w-full mb-4" required;

                button type="submit" class="bg-blue-500 text-white p-2 rounded" { "Register" }
            }
            div id="result" class="mt-4" {}
        }
    }
}

pub fn create_workspace_form() -> Markup {
    html! {
        div class="max-w-md mx-auto mt-10" {
            form hx-post="/workspaces" hx-target="#result" {
                h2 class="text-2xl font-bold mb-4" { "Create Workspace" }

                label class="block mb-2" { "Subdomain" }
                input type="text" name="subdomain" class="border p-2 w-full mb-4" required placeholder="acme-corp";

                button type="submit" class="bg-blue-500 text-white p-2 rounded" { "Create" }
            }
            div id="result" class="mt-4" {}
        }
    }
}

pub fn person_list(people: &[Person]) -> Markup {
    html! {
        div class="max-w-4xl mx-auto mt-10" {
            div class="flex justify-between items-center mb-6" {
                 h2 class="text-2xl font-bold" { "People" }
                 a href="/people/new" class="bg-blue-500 text-white px-4 py-2 rounded" { "Add Person" }
            }

            table class="min-w-full bg-white border" {
                thead {
                    tr {
                        th class="p-4 border-b text-left" { "Name" }
                        th class="p-4 border-b text-left" { "Email" }
                        th class="p-4 border-b text-left" { "Position" }
                        th class="p-4 border-b text-left" { "Actions" }
                    }
                }
                tbody {
                    @for person in people {
                        tr class="hover:bg-gray-50" {
                            td class="p-4 border-b" { (person.name) }
                            td class="p-4 border-b" { (person.email) }
                            td class="p-4 border-b" { (person.position) }
                            td class="p-4 border-b" {
                                button
                                    hx-delete=(format!("/people/{}", person.id))
                                    hx-target="closest tr"
                                    hx-swap="outerHTML"
                                    class="text-red-500 hover:text-red-700"
                                { "Delete" }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn person_form() -> Markup {
    html! {
        div class="max-w-md mx-auto mt-10" {
            form hx-post="/people" hx-target="body" {
                h2 class="text-2xl font-bold mb-4" { "Add New Person" }

                label class="block mb-2" { "Name" }
                input type="text" name="name" class="border p-2 w-full mb-4" required;

                label class="block mb-2" { "Email" }
                input type="email" name="email" class="border p-2 w-full mb-4" required;

                label class="block mb-2" { "Position (Integer)" }
                input type="number" name="position" value="0" class="border p-2 w-full mb-4";

                div class="flex justify-between items-center" {
                     a href="/people" class="text-gray-500" { "Cancel" }
                     button type="submit" class="bg-blue-500 text-white px-4 py-2 rounded" { "Save" }
                }
            }
        }
    }
}

pub fn company_list(companies: &[crate::domain::Company]) -> Markup {
    html! {
        div class="max-w-4xl mx-auto mt-10" {
            div class="flex justify-between items-center mb-6" {
                 h2 class="text-2xl font-bold" { "Companies" }
                 a href="/companies/new" class="bg-blue-500 text-white px-4 py-2 rounded" { "Add Company" }
            }

            table class="min-w-full bg-white border" {
                thead {
                    tr {
                        th class="p-4 border-b text-left" { "Name" }
                        th class="p-4 border-b text-left" { "Domain" }
                        th class="p-4 border-b text-left" { "Employees" }
                        th class="p-4 border-b text-left" { "Actions" }
                    }
                }
                tbody {
                    @for company in companies {
                        tr class="hover:bg-gray-50" {
                            td class="p-4 border-b" { (company.name) }
                            td class="p-4 border-b" { (company.domain_name) }
                            td class="p-4 border-b" { (company.employees_count) }
                            td class="p-4 border-b" {
                                button
                                    hx-delete=(format!("/companies/{}", company.id))
                                    hx-target="closest tr"
                                    hx-swap="outerHTML" // UI Sync fix
                                    class="text-red-500 hover:text-red-700"
                                { "Delete" }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn company_form() -> Markup {
    html! {
        div class="max-w-md mx-auto mt-10" {
            form hx-post="/companies" hx-target="body" {
                h2 class="text-2xl font-bold mb-4" { "Add New Company" }

                label class="block mb-2" { "Name" }
                input type="text" name="name" class="border p-2 w-full mb-4" required;

                label class="block mb-2" { "Domain Name" }
                input type="text" name="domain_name" class="border p-2 w-full mb-4" required placeholder="example.com";

                label class="block mb-2" { "Address" }
                textarea name="address" class="border p-2 w-full mb-4" {};

                label class="block mb-2" { "Employees Count" }
                input type="number" name="employees_count" value="0" class="border p-2 w-full mb-4";

                div class="flex justify-between items-center" {
                     a href="/companies" class="text-gray-500" { "Cancel" }
                     button type="submit" class="bg-blue-500 text-white px-4 py-2 rounded" { "Save" }
                }
            }
        }
    }
}
