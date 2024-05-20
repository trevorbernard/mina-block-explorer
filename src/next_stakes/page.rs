use super::functions::*;
use crate::{
    common::{components::*, models::TableMetadata, table::*},
    stakes::{components::EpochButton, models::EpochStyleVariant},
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

#[component]
pub fn NextStakesPage() -> impl IntoView {
    view! {
        <Title text="Next Staking Ledger | Search For Stakers"/>
        <PageContainer>
            <NextStakesPageContents/>
        </PageContainer>
    }
}

#[component]
fn NextStakesPageContents() -> impl IntoView {
    let (metadata, set_metadata) = create_signal(Some(TableMetadata::default()));
    let query_params_map = use_query_map();
    let resource = create_resource(
        move || query_params_map.get(),
        |params_map| async move {
            let public_key = params_map.get("q-key").cloned();
            let delegate = params_map.get("q-delegate").cloned();
            load_data(public_key, delegate).await
        },
    );

    let get_data = move || resource.get().and_then(|res| res.ok());

    let table_columns = vec![
        TableColumn {
            column: "Key".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Stake".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Delegate".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Delegators".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Ledger Hash".to_string(),
            is_searchable: false,
        },
    ];
    let table_cols_length = table_columns.len();

    create_effect(move |_| {
        get_data().map(|data| {
            set_metadata.set(Some(TableMetadata {
                total_records: "all".to_string(),
                displayed_records: data.nextstakes.len() as i64,
            }))
        });
    });

    view! {
        <ErrorBoundary fallback=move |_| ().into_view()>
            <TableSection
                metadata
                section_heading="Next Staking Ledger"
                controls=move || {
                    view! {
                        <EpochButton
                            href="/staking-ledgers"
                            text="Previous"
                            style_variant=EpochStyleVariant::Secondary
                        />
                        <EpochButton
                            text="Next"
                            disabled=true
                            style_variant=EpochStyleVariant::Primary
                        />
                    }
                }
            >

                <TableContainer>
                    <Table>
                        <TableHeader columns=table_columns/>
                        <Suspense fallback=move || {
                            view! {
                                <TableRows data=vec![
                                    vec![LoadingPlaceholder; table_cols_length];
                                    10
                                ]/>
                            }
                        }>
                            {move || {
                                get_data()
                                    .map(|data| {
                                        view! { <TableRows data=data.nextstakes/> }
                                    })
                            }}

                        </Suspense>
                    </Table>
                </TableContainer>
            </TableSection>
        </ErrorBoundary>
    }
}
