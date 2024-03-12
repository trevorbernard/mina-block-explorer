use crate::{
    account_dialog::dialog::AccountDialogView,
    addresses::page::{AccountSpotlightPage, AccountsPage, AddressesTabbedPage},
    blocks::page::{
        BlockAnalyticsTab, BlockFeeTransfersTab, BlockSnarkJobsTab, BlockSpotlightTab, BlockTabbedPage,
        BlockUserCommandsTab, LatestBlocksPage,
    },
    broadcast::page::{
        BroadcastDelegationPage, BroadcastFromLedgerPage, BroadcastTransactionPage,
        DelegationTabbedPage,
    },
    common::components::NullView,
    footer::Footer,
    header::navigation::Header,
    next_stakes::page::NextStakesPage,
    snarks::page::SnarksPage,
    stakes::page::StakesPage,
    summary::page::SummaryPage,
    transactions::page::{TransactionSpotlightPage, TransactionTabbedPage, TransactionsPage},
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Root() -> impl IntoView {
    view! {
        <Router>
            <Header/>
            <main>
                <Routes>
                    <Route path="/summary" view=SummaryPage>
                        <Route path="accounts/:id" view=AccountDialogView/>
                        <Route path="/*any" view=NullView/>
                    </Route>
                    <Route path="/addresses" view=AddressesTabbedPage>
                        <Route path="/accounts/:id" view=AccountSpotlightPage/>
                        <Route path="/*any" view=AccountsPage/>
                    </Route>
                    <Route path="/blocks" view=LatestBlocksPage>
                        <Route path="accounts/:id" view=AccountDialogView/>
                        <Route path="/*any" view=NullView/>
                    </Route>
                    <Route path="/blocks/:id" view=BlockTabbedPage>
                        <Route path="/spotlight" view=BlockSpotlightTab/>
                        <Route path="/user-commands" view=BlockUserCommandsTab/>
                        <Route path="/snark-jobs" view=BlockSnarkJobsTab/>
                        <Route path="/fee-transfers" view=BlockFeeTransfersTab/>
                        <Route path="/analytics" view=BlockAnalyticsTab/>
                        <Route path="/*any" view=BlockSpotlightTab/>
                    </Route>
                    <Route path="/transactions" view=TransactionTabbedPage>
                        <Route path="/" view=TransactionsPage/>
                        <Route path="/zk-trnx" view=NullView/>
                        <Route path="/token-trnx" view=NullView/>
                    </Route>
                    <Route path="/transactions/:id" view=TransactionSpotlightPage/>
                    <Route path="/snarks" view=SnarksPage/>
                    <Route path="/stakes" view=StakesPage/>
                    <Route path="/next-stakes" view=NextStakesPage/>
                    <Route path="/broadcast" view=DelegationTabbedPage>
                        <Route path="/transaction" view=BroadcastTransactionPage/>
                        <Route path="/delegation" view=BroadcastDelegationPage/>
                        <Route path="/ledger" view=BroadcastFromLedgerPage/>
                        <Route path="/*any" view=BroadcastTransactionPage/>
                    </Route>
                    <Route path="/*any" view=SummaryPage/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
