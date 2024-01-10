use leptos::*;
use leptos_router::*;

use super::functions::*;
use super::models::*;

use crate::blocks::components::AccountOverviewBlocksTable;
use crate::common::components::*;
use crate::common::models::MyError;
use crate::common::spotlight::*;
use crate::icons::WalletIcon;
use crate::snarks::components::AccountOverviewSnarkJobTable;
use crate::transactions::components::*;

#[component]
pub fn AccountSummaryPage() -> impl IntoView {
    let memo_params_map = use_params_map();

    let resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            if let Some(id) = value.get("id").cloned() {
                let id_clone = id.clone();
                load_data(&id_clone).await
            } else {
                Err(MyError::ParseError(String::from(
                    "Could not parse id parameter from url",
                )))
            }
        },
    );

    view! {
        {move || match resource.get() {
            Some(Ok(res)) =>{
                let pk =res.account.public_key.clone();
                view! {
                    <SpotlightSection header="Account Spotlight".to_string()
                        spotlight_items=get_spotlight_data(res.account.clone())
                        meta=format!("Username: {}",res.account.username)
                        id=pk.clone()>
                        <WalletIcon width=40/>
                    </SpotlightSection>
                    <TransactionsSection public_key=Some(pk.clone()) with_link=true/>
                    <div class="md:col-start-2 md:col-end-3 grid grid-cols-1 md:grid-cols-2 gap-4">
                        <section class="md:col-start-1 md:col-end-2 md:rounded-lg bg-table-section">
                            <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">"SNARK Jobs"</h1>
                            <AccountOverviewSnarkJobTable public_key=Some(pk.clone())/>
                        </section>
                        <section class="md:col-start-2 md:col-end-3 md:rounded-lg bg-table-section">
                            <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">"Block Production"</h1>
                            <AccountOverviewBlocksTable public_key=Some(pk) />
                        </section>
                    </div>
                }.into_view()
            },
            _ => view! { <span/>  }.into_view()
        }}
    }
}

#[component]
fn AccountSummarySection(summary: AccountResponse) -> impl IntoView {
    view! {
        <section class="grid grid-cols-2 gap-1">
            <SummaryItem id="publicKey".to_string() label="Public Key".to_string() value={SummaryItemKind::Str(summary.account.public_key)} />
            <SummaryItem id="username".to_string() label="Username".to_string() value={SummaryItemKind::Str(summary.account.username)} />
            <SummaryItem id="balance".to_string() label="Balance".to_string() value={SummaryItemKind::Float64(summary.account.balance.total())} />
            <SummaryItem id="nonce".to_string() label="Nonce".to_string() value={SummaryItemKind::Int32(summary.account.nonce)} />
            <SummaryItem id="receiptChainHash".to_string() label="Receipt Chain Hash".to_string() value={SummaryItemKind::Str(summary.account.receipt_chain_hash)} />
            <SummaryItem id="delegate".to_string() label="Delegate".to_string() value={SummaryItemKind::Str(summary.account.delegate)} />
            <SummaryItem id="votingFor".to_string() label="Voting For".to_string() value={SummaryItemKind::Str(summary.account.voting_for)} />
            <SummaryItem id="pendingTransactions".to_string() label="Pending Transactions".to_string() value={SummaryItemKind::Int32(summary.account.count_pending_transactions)} />
        </section>
    }
}