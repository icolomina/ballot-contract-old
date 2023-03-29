#![cfg(test)]

use super::{BallotContract, BallotContractClient};
use soroban_sdk::{Env, symbol, testutils::Address as _, Address};

extern crate std;


#[test]
fn add_party_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BallotContract);
    let client = BallotContractClient::new(&env, &contract_id);

    let admin_addr = Address::random(&env);
    
    // Testing 
    assert_eq!(client.add_party(&admin_addr, &symbol!("Laborist")), 1 );
    assert_eq!(client.add_party(&admin_addr, &symbol!("Conserv")), 2);
    assert_eq!(client.add_party(&admin_addr, &symbol!("Conserv")), 2);

}

#[test]
fn add_voter_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BallotContract);
    let client = BallotContractClient::new(&env, &contract_id);

    let addr1 = Address::random(&env);
    let addr2 = Address::random(&env);
    let admin_addr = Address::random(&env);


    assert_eq!(client.add_voter(&admin_addr, &addr1), 1 );
    assert_eq!(client.add_voter(&admin_addr, &addr2), 2 );
    assert_eq!(client.add_voter(&admin_addr, &addr2), 2 );
}

#[test]
fn vote_test() {

    let env = Env::default();
    let contract_id = env.register_contract(None, BallotContract);
    let client = BallotContractClient::new(&env, &contract_id);

    let addr1 = Address::random(&env);
    let addr2 = Address::random(&env);
    let addr3 = Address::random(&env);
    let addr4 = Address::random(&env);
    let addr5 = Address::random(&env);

    let admin_addr = Address::random(&env);

    client.add_party(&admin_addr, &symbol!("Laborist"));
    client.add_party(&admin_addr, &symbol!("Conserv"));

    client.add_voter(&admin_addr, &addr1);
    client.add_voter(&admin_addr, &addr2);
    client.add_voter(&admin_addr, &addr3);
    client.add_voter(&admin_addr, &addr4);
    client.add_voter(&admin_addr, &addr5);

    client.delegate(&addr5, &addr4);

    assert_eq!(client.vote(&addr1, &symbol!("Laborist")), true);
    assert_eq!(client.vote(&addr2, &symbol!("Laborist")), true);
    assert_eq!(client.vote(&addr3, &symbol!("Conserv")), true);
    assert_eq!(client.vote(&addr4, &symbol!("Conserv")), true);

    let result = client.count();

    assert_eq!(result.get(symbol!("Laborist")).unwrap().ok(), Some(2));
    assert_eq!(result.get(symbol!("Conserv")).unwrap().ok(), Some(3));
    
}

#[test]
#[should_panic(expected = "Error(1)")]
fn vote_delegated() {

    let env = Env::default();
    let contract_id = env.register_contract(None, BallotContract);
    let client = BallotContractClient::new(&env, &contract_id);

    let addr1 = Address::random(&env);
    let addr2 = Address::random(&env);
    let admin_addr = Address::random(&env);

    client.add_party(&admin_addr, &symbol!("Laborist"));
    client.add_party(&admin_addr, &symbol!("Conserv"));

    client.add_voter(&admin_addr, &addr1);
    client.add_voter(&admin_addr, &addr2);

    client.delegate(&addr1, &addr2);
    client.vote(&addr1, &symbol!("Laborist"));
}

#[test]
#[should_panic(expected = "Error(3)")]
fn vote_party_not_registered() {

    let env = Env::default();
    let contract_id = env.register_contract(None, BallotContract);
    let client = BallotContractClient::new(&env, &contract_id);

    let addr1 = Address::random(&env);
    let admin_addr = Address::random(&env);

    client.add_party(&admin_addr, &symbol!("Laborist"));
    client.add_voter(&admin_addr, &addr1);

    client.vote(&addr1, &symbol!("Conserv"));
}

#[test]
#[should_panic(expected = "Error(4)")]
fn vote_voter_already_voted() {

    let env = Env::default();
    let contract_id = env.register_contract(None, BallotContract);
    let client = BallotContractClient::new(&env, &contract_id);

    let addr1 = Address::random(&env);
    let admin_addr = Address::random(&env);

    client.add_party(&admin_addr, &symbol!("Laborist"));
    client.add_voter(&admin_addr, &addr1);

    client.vote(&addr1, &symbol!("Laborist"));
    client.vote(&addr1, &symbol!("Laborist"));
}

#[test]
#[should_panic(expected = "Error(2)")]
fn vote_voter_not_registered() {

    let env = Env::default();
    let contract_id = env.register_contract(None, BallotContract);
    let client = BallotContractClient::new(&env, &contract_id);

    let addr1 = Address::random(&env);
    let admin_addr = Address::random(&env);

    client.add_party(&admin_addr, &symbol!("Laborist"));
    client.vote(&addr1, &symbol!("Laborist"));
}

#[test]
fn delegate_test() {

    let env = Env::default();
    let contract_id = env.register_contract(None, BallotContract);
    let client = BallotContractClient::new(&env, &contract_id);

    let addr1 = Address::random(&env);
    let addr2 = Address::random(&env);
    let admin_addr = Address::random(&env);

    client.add_voter(&admin_addr, &addr1);
    client.add_voter(&admin_addr, &addr2);

    let d_votes = client.delegate(&addr1, &addr2);
    let d_votes_2 = client.delegate(&addr1, &addr2);
    
    assert_eq!(d_votes.len(), 1);
    assert_eq!(d_votes_2.len(), 1);
    
}


#[test]
#[should_panic(expected = "Error(2)")]
fn delegate_fail() {

    let env = Env::default();
    let contract_id = env.register_contract(None, BallotContract);
    let client = BallotContractClient::new(&env, &contract_id);

    let addr1 = Address::random(&env);
    let addr2 = Address::random(&env);

    client.delegate(&addr1, &addr2);
}

#[test]
#[should_panic(expected = "Error(5)")]
fn delegate_fail_voter() {

    let env = Env::default();
    let contract_id = env.register_contract(None, BallotContract);
    let client = BallotContractClient::new(&env, &contract_id);   

    let addr1 = Address::random(&env);
    let addr2 = Address::random(&env);
    let addr3 = Address::random(&env);
    let admin_addr = Address::random(&env);

    client.add_voter(&admin_addr, &addr1);
    client.add_voter(&admin_addr, &addr2);
    client.add_voter(&admin_addr, &addr3);

    client.delegate(&addr1, &addr2);
    client.delegate(&addr2, &addr3);

}