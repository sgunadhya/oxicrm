use maud::{html, Markup};

pub fn oob_swap_counter(count: i32) -> Markup {
    html! {
        div id="opportunity-count" hx-swap-oob="true" {
            (format!("Total Opportunities: {}", count))
        }
    }
}
