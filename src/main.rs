use leptos::*;
mod api_models;
mod nav;
mod root;
mod summary_item;
mod summary_page;
mod latest_block_page;
mod transactions_page;
mod table;
mod snarks_page;
mod stakes_page;
mod header;
mod table_section;
mod accounts;

use root::Root;

fn main() {
    leptos::mount_to_body(|| view! { <Root/> })
}
