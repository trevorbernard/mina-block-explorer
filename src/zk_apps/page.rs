use super::functions::*;
use crate::{
    account_activity::models::AccountActivityQueryDirectionalTransactions,
    common::{components::*, functions::*, models::*, spotlight::*, table::*},
    icons::*,
};
use indoc::indoc;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn ZkAppSpotlight() -> impl IntoView {
    let records_per_page = 5;
    let (current_txn_page, set_current_txn_page) = create_signal(1);
    let (current_fees_page, set_current_fees_page) = create_signal(1);
    let txn = stub_zk_app_txn_data(10);
    let fees = vec![AccountActivityQueryDirectionalTransactions {
        fee: Some(0.01_f64),
        counterparty: Some("B62qmQsEHcsPUs5xdtHKjEmWqqhUPRSF2GNmdguqnNvpEZpKftPC69f".to_string()),
        direction: Some("IN".to_string()),
        hash: Some("5JunUf7Niybx1d2CdLLZWL1D9wwtce5dBFM7nXsQ9GtiyopSh1Ee".to_string()),
        amount: Some(0.01_f64),
        date_time: Some(chrono::Utc::now()),
        height: Some(5822_i64),
        kind: Some("PAYMENT".to_string()),
        nonce: Some(1),
        failure_reason: None,
        memo: None,
        canonical: Some(true),
    }];
    view! {
        <Title text="ZK App Spotlight"/>
        <PageContainer>
            <SpotlightSection
                header="ZK App Spotlight".to_string()
                spotlight_items=vec![
                    SpotlightEntry {
                        label: String::from("Balance"),
                        any_el: Some(
                            decorate_with_currency_tag(
                                "1324.593847562".to_string(),
                                "mina".to_string(),
                            ),
                        ),
                        ..Default::default()
                    },
                    SpotlightEntry {
                        label: String::from("Total Txn"),
                        any_el: Some(convert_to_pill("52".to_string(), ColorVariant::Blue)),
                        ..Default::default()
                    },
                    SpotlightEntry {
                        label: String::from("Voting For"),
                        any_el: Some(convert_to_span(generate_base58_string(44))),
                        copiable: true,
                    },
                    SpotlightEntry {
                        label: String::from("Ver. Key Hash"),
                        any_el: Some(convert_to_span(generate_base58_string(44))),
                        copiable: true,
                    },
                ]

                meta=Some(
                    format!(
                        "Last Active: {}",
                        print_time_since(&generate_random_datetime_within_days(1).to_string()),
                    ),
                )

                id=Some(generate_base58_string(44))
            >
                <ZKAppSymbol width=40/>
            </SpotlightSection>
            <TableSection section_heading="ZK App Details".to_string() controls=|| ().into_view()>
                <SpotlightTable>
                    <ZkAppDetailTr>
                        <ZkAppDetailTh>"Permissions :"</ZkAppDetailTh>
                        <ZkAppDetailTd>
                            <CodeBlock>

                                {
                                    indoc! {
                                        r#"{
    "access":"none"
    "editActionState":"proof"
    "editState":"proof"
    "incrementNonce":"signature"
    "receive":"none"
    "send":"proof"
    "setDelegate":"signature"
    "setPermissions":"signature"
    "setTiming":"signature"
    "setTokenSymbol":"signature"
    "setVerificationKey":"signature"
    "setVotingFor":"signature"
    "setZkAppUri":"signature"
}"#
                                    }
                                }

                            </CodeBlock>
                        </ZkAppDetailTd>
                    </ZkAppDetailTr>
                    <ZkAppDetailTr>
                        <ZkAppDetailTh>"Events :"</ZkAppDetailTh>
                        <ZkAppDetailTd>
                            <CodeBlock>

                                {
                                    indoc! {
                                        r#"[
    0:"25079927036070901246064867767436987657692091363973573142121686150614948079097"
    1:"25079927036070901246064867767436987657692091363973573142121686150614948079097"
    2:"25079927036070901246064867767436987657692091363973573142121686150614948079097"
    3:"25079927036070901246064867767436987657692091363973573142121686150614948079097"
    4:"25079927036070901246064867767436987657692091363973573142121686150614948079097"
]"#
                                    }
                                }

                            </CodeBlock>
                        </ZkAppDetailTd>
                    </ZkAppDetailTr>
                    <ZkAppDetailTr>
                        <ZkAppDetailTh>"App State :"</ZkAppDetailTh>
                        <ZkAppDetailTd>
                            <CodeBlock>

                                {
                                    indoc! {
                                        r#"[
    0:"1"
    1:"0"
    2:"0"
    3:"0"
    4:"0"
    5:"0"
    6:"0"
    7:"0"
]"#
                                    }
                                }

                            </CodeBlock>
                        </ZkAppDetailTd>
                    </ZkAppDetailTr>
                </SpotlightTable>
            </TableSection>
            <SubSectionContainer>
                <AppSubSection
                    heading="zkApp Commands".to_string()
                    position=SubSectionPosition::Left
                >
                    {move || {
                        let data = txn.clone();
                        let pag = build_pagination(
                            data.len(),
                            records_per_page,
                            current_txn_page.get(),
                            set_current_txn_page,
                            None,
                            None,
                        );
                        let subset = get_subset(
                            &data.into_iter().map(Some).collect::<Vec<_>>(),
                            records_per_page,
                            current_txn_page.get() - 1,
                        );
                        view! { <Table data=subset pagination=pag/> }
                    }}

                </AppSubSection>
                <AppSubSection
                    heading="Fee Payments".to_string()
                    position=SubSectionPosition::Right
                >
                    {move || {
                        let data = fees.clone();
                        let pag = build_pagination(
                            data.len(),
                            records_per_page,
                            current_fees_page.get(),
                            set_current_fees_page,
                            None,
                            None,
                        );
                        let subset = get_subset(
                            &data.into_iter().map(Some).collect::<Vec<_>>(),
                            records_per_page,
                            current_fees_page.get() - 1,
                        );
                        view! { <Table data=subset pagination=pag/> }
                    }}

                </AppSubSection>
            </SubSectionContainer>
        </PageContainer>
    }
}

#[component]
fn ZkAppDetailTd(children: Children) -> impl IntoView {
    view! {
        <td class="flex justify-start items-center m-1 p-1 text-left text-xs md:text-sm whitespace-nowrap">
            {children()}
        </td>
    }
}

#[component]
fn ZkAppDetailTr(children: Children) -> impl IntoView {
    view! { <tr class="w-full flex flex-col lg:flex-row justify-start">{children()}</tr> }
}

#[component]
fn ZkAppDetailTh(children: Children) -> impl IntoView {
    view! {
        <th class="flex justify-start items-start m-1 p-1 text-xs md:text-sm whitespace-nowrap w-36 md:w-40 min-w-36 md:min-w-40 font-normal text-slate-400">
            {children()}
        </th>
    }
}

#[component]
pub fn ZkAppTransactionsPage() -> impl IntoView {
    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);
    let data = stub_zk_app_txn_data(1000);
    view! {
        <Title text="Commands | ZK Apps"/>
        <PageContainer>
            <TableSection section_heading="ZK App Commands" controls=|| ().into_view()>

                {move || {
                    let data = data.clone();
                    let pag = build_pagination(
                        data.len(),
                        records_per_page,
                        current_page.get(),
                        set_current_page,
                        None,
                        None,
                    );
                    let subset = get_subset(
                        &data.into_iter().map(Some).collect::<Vec<_>>(),
                        records_per_page,
                        current_page.get() - 1,
                    );
                    view! { <Table data=subset pagination=pag/> }
                }}

            </TableSection>
        </PageContainer>
    }
}

#[component]
pub fn ZkAppsPage() -> impl IntoView {
    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);
    let data = stub_zk_apps_data(9000);
    view! {
        <Title text="ZK Apps | Search For ZK Apps"/>
        <PageContainer>
            <TableSection section_heading="ZK Apps" controls=|| ().into_view()>

                {move || {
                    let data = data.clone();
                    let pag = build_pagination(
                        data.len(),
                        records_per_page,
                        current_page.get(),
                        set_current_page,
                        None,
                        None,
                    );
                    let subset = get_subset(
                        &data.into_iter().map(Some).collect::<Vec<_>>(),
                        records_per_page,
                        current_page.get() - 1,
                    );
                    view! { <Table data=subset pagination=pag/> }
                }}

            </TableSection>
        </PageContainer>
    }
}

#[component]
pub fn ZkAppTransactionSpotlightPage() -> impl IntoView {
    let spotlight_items = vec![
        SpotlightEntry {
            label: "Transaction Type".to_string(),
            any_el: Some(convert_to_pill("ZK".to_string(), ColorVariant::Green)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Transaction Hash".to_string(),
            any_el: Some(convert_to_span(
                "5JvEERgjGA3dYZSKNAz7DnDVNgERvJQrek3tCdMhTUsacov6LUzy".to_string(),
            )),
            copiable: true,
        },
        SpotlightEntry {
            label: "Block Height".to_string(),
            any_el: Some(convert_to_pill("7,326".to_string(), ColorVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Block State Hash".to_string(),
            any_el: Some(convert_to_span(
                "3NLoTnbvaSwU6zNwxVQd8vR6UcuEDrf9YuQusbjkgNzjEAHjwboG".to_string(),
            )),
            copiable: true,
        },
        SpotlightEntry {
            label: "Fee Payer".to_string(),
            any_el: Some(convert_to_span(
                "B62qpGSaBUHzKExDXp2N3ZPNPtFMFFXjSuAB84h4DSks12PWsRq5SEB".to_string(),
            )),
            copiable: true,
        },
        SpotlightEntry {
            label: "Account Updates".to_string(),
            any_el: Some(convert_to_pill("3".to_string(), ColorVariant::Grey)),
            ..Default::default()
        },
    ];

    let account_updates_1 = vec![
        SpotlightEntry {
            label: "Balance Change".to_string(),
            any_el: Some(convert_to_pill("+20 MINA".to_string(), ColorVariant::Green)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Increment Nonce".to_string(),
            any_el: Some(convert_to_pill("true".to_string(), ColorVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Token ID".to_string(),
            any_el: Some(convert_to_span(
                "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf".to_string(),
            )),
            copiable: true,
        },
        SpotlightEntry {
            label: "Call Data".to_string(),
            any_el: Some(convert_to_pill("0".to_string(), ColorVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Call Depth".to_string(),
            any_el: Some(convert_to_pill("1".to_string(), ColorVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Use Full Com.".to_string(),
            any_el: Some(convert_to_pill("No".to_string(), ColorVariant::Grey)),
            ..Default::default()
        },
    ];

    view! {
        <PageContainer>
            <SpotlightSection
                header="[zk] Command Spotlight"
                spotlight_items
                id=Some("3NK8nzfotTNSUopF4oEzJUHJ2EeLATBDnMRRgaqaTfR3zpfHK2yo".to_string())
                meta=Some("2024-02-28 20:45:00 UTC (8 minutes ago)".to_string())
            >
                <ZKAppSymbol width=40/>
            </SpotlightSection>
            <TableSection section_heading="Account Update #1" controls=|| ().into_view()>
                <SpotlightTable>
                    {account_updates_1
                        .into_iter()
                        .map(|entry| {
                            view! {
                                <ZkAppDetailTr>
                                    <ZkAppDetailTh>{entry.label}</ZkAppDetailTh>
                                    <ZkAppDetailTd>{entry.any_el}</ZkAppDetailTd>
                                </ZkAppDetailTr>
                            }
                        })
                        .collect::<Vec<_>>()} <ZkAppDetailTr>
                        <ZkAppDetailTh>"App State :"</ZkAppDetailTh>
                        <ZkAppDetailTd>
                            <CodeBlock>

                                {
                                    indoc! {
                                        r#"[
    "13085319543788982998999669060227968584120410722425376027756703205043792631731",
    "88814049655838941284774570817345763621809698732252711808042102595406818641",
    "525481201097986652723544789857104441"
]"#
                                    }
                                }

                            </CodeBlock>
                        </ZkAppDetailTd>
                    </ZkAppDetailTr> <ZkAppDetailTr>
                        <ZkAppDetailTh>"Permissions: "</ZkAppDetailTh>
                        <ZkAppDetailTd>
                            <CodeBlock>

                                {
                                    indoc! {
                                        r#"{
    "access":"",
    "editActionState":"proof",
    "editState":"proof",
    "incrementNonce":"signature",
    "receive":"none",
    "send":"proof",
    "setDelegate":"signature",
    "setPermissions":"signature",
    "setTiming":"",
    "setTokenSymbol":"signature",
    "setVerificationKey":"signature",
    "setVotingFor":"signature",
    "setZkAppUri":"signature"
}"#
                                    }
                                }

                            </CodeBlock>
                        </ZkAppDetailTd>
                    </ZkAppDetailTr>
                </SpotlightTable>
            </TableSection>
        </PageContainer>
    }
}
