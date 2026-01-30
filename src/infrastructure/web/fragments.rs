use crate::domain::{Opportunity, OpportunityStage};
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
