use super::functions::*;
use crate::common::{components::*, functions::*, table::*};
use leptos::*;

#[component]
pub fn BlockInternalCommandsTable(block_state_hash: Option<String>) -> impl IntoView {
    let (bsh_signal, _set_bsh) = create_signal(block_state_hash);
    let resource = create_resource(
        move || bsh_signal.get(),
        move |block_state_hash_opt| async move { load_data(50, block_state_hash_opt).await },
    );

    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => {
                view! {
                    {match data.feetransfers.len() {
                        0 => {
                            view! {
                                <EmptyTable message="No internal commands for this block"
                                    .to_string()/>
                            }
                        }
                        _ => {
                            let pag = build_pagination(
                                data.feetransfers.len(),
                                records_per_page,
                                current_page.get(),
                                set_current_page,
                            );
                            let subset = get_subset(
                                &data.feetransfers,
                                records_per_page,
                                current_page.get() - 1,
                            );
                            view! { <Table data=subset pagination=pag/> }
                        }
                    }}
                }
            }
            _ => view! { <NullView/> },
        }}
    }
}
