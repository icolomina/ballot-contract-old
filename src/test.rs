#![cfg(test)]

use super::{BallotContract, BallotContractClient, Party, Voter };
use soroban_sdk::{Env, symbol, vec, testutils::Address as _, Address};

extern crate std;


#[test]
fn add_party_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BallotContract);
    let client = BallotContractClient::new(&env, &contract_id);
    
    // Testing 
    assert_eq!(client.add_party(&symbol!("Laborist")), 1 );
    assert_eq!(client.add_party(&symbol!("Conserv")), 2);
    assert_eq!(client.add_party(&symbol!("Conserv")), 2);

}

#[test]
fn add_voter_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BallotContract);
    let client = BallotContractClient::new(&env, &contract_id);

    let addr1 = Address::random(&env);
    let addr2 = Address::random(&env);


    assert_eq!(client.add_voter(&addr1), 1 );
    assert_eq!(client.add_voter(&addr2), 2 );
}

#[test]
fn vote_test() {

    let env = Env::default();
    let contract_id = env.register_contract(None, BallotContract);
    let client = BallotContractClient::new(&env, &contract_id);

    let addr1 = Address::random(&env);
    let addr2 = Address::random(&env);
    let addr3 = Address::random(&env);

    client.add_party(&symbol!("Laborist"));
    client.add_party(&symbol!("Conserv"));

    client.add_voter(&addr1);
    client.add_voter(&addr2);
    client.add_voter(&addr3);

    assert_eq!(client.vote(&addr1, &symbol!("Laborist")), true);
    assert_eq!(client.vote(&addr2, &symbol!("Laborist")), true);
    assert_eq!(client.vote(&addr3, &symbol!("Conserv")), true);
    assert_eq!(client.vote(&addr3, &symbol!("Conserv")), false);

    let result = client.count();

    assert_eq!(result.get(symbol!("Laborist")).unwrap().ok(), Some(2));
    assert_eq!(result.get(symbol!("Conserv")).unwrap().ok(), Some(1));
    
}