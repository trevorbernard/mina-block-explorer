// use chrono::Utc;
use self::next_staking_ledgers_query::NextstakeQueryInput;
use graphql_client::GraphQLQuery;

// type DateTime = chrono::DateTime<Utc>;
// type Long = i32;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/next_staking_ledgers.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone",
    skip_serializing_none
)]
pub struct NextStakingLedgersQuery;

#[allow(clippy::derivable_impls)]
impl Default for NextstakeQueryInput {
    fn default() -> Self {
        NextstakeQueryInput {
            receipt_chain_hash_gte: None,
            public_key_in: None,
            receipt_chain_hash_exists: None,
            public_key_exists: None,
            balance_in: None,
            nonce_gte: None,
            ledger_hash_in: None,
            pk_lte: None,
            receipt_chain_hash_lt: None,
            permissions_exists: None,
            pk_lt: None,
            timing: None,
            voting_for: None,
            delegate_in: None,
            pk_ne: None,
            public_key_nin: None,
            delegate_lt: None,
            pk_gt: None,
            ledger_hash_gte: None,
            nonce_in: None,
            voting_for_in: None,
            delegate_lte: None,
            delegate_gt: None,
            receipt_chain_hash_ne: None,
            pk_nin: None,
            public_key_lt: None,
            nonce_nin: None,
            nonce_ne: None,
            delegate: None,
            voting_for_exists: None,
            voting_for_lt: None,
            ledger_hash: None,
            voting_for_lte: None,
            pk_exists: None,
            receipt_chain_hash_nin: None,
            delegate_exists: None,
            public_key_lte: None,
            balance_gte: None,
            voting_for_gte: None,
            token_lt: None,
            timing_exists: None,
            delegate_nin: None,
            receipt_chain_hash_gt: None,
            ledger_hash_gt: None,
            token_lte: None,
            pk_in: None,
            nonce_lte: None,
            delegate_ne: None,
            public_key_gte: None,
            balance_exists: None,
            ledger_hash_lte: None,
            delegate_gte: None,
            token: None,
            receipt_chain_hash_in: None,
            ledger_hash_nin: None,
            token_in: None,
            nonce_exists: None,
            ledger_hash_exists: None,
            nonce_gt: None,
            public_key: None,
            balance: None,
            receipt_chain_hash_lte: None,
            balance_gt: None,
            ledger_hash_ne: None,
            public_key_ne: None,
            token_ne: None,
            ledger_hash_lt: None,
            balance_lt: None,
            nonce_lt: None,
            voting_for_nin: None,
            pk_gte: None,
            voting_for_ne: None,
            token_nin: None,
            voting_for_gt: None,
            token_exists: None,
            balance_lte: None,
            receipt_chain_hash: None,
            balance_nin: None,
            or: None,
            balance_ne: None,
            permissions: None,
            public_key_gt: None,
            token_gte: None,
            and: None,
            nonce: None,
            pk: None,
            token_gt: None,
        }
    }
}
