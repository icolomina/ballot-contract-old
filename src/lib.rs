#![no_std]

use soroban_sdk::{contractimpl, contracttype, contracterror, symbol, Env, Symbol, Vec, Map, Address, panic_with_error};

const PARTIES: Symbol = symbol!("parties");
const VOTERS: Symbol = symbol!("voters");
const VOTES: Symbol = symbol!("votes");

fn get_parties(env: &Env) -> Vec<Symbol> {
    let parties: Vec<Symbol>= env
        .storage()
        .get(&PARTIES)
        .unwrap_or(Ok(Vec::new(&env)))
        .unwrap()
    ;

    parties
}

fn get_voters(env: &Env) -> Vec<Address> {
    let voters: Vec<Address>= env
        .storage()
        .get(&VOTERS)
        .unwrap_or(Ok(Vec::new(&env)))
        .unwrap()
    ;

    voters
}

fn get_votes(env: &Env) -> Vec<Address> {
    let votes: Vec<Address> = env
        .storage()
        .get(&VOTES)
        .unwrap_or(Ok(Vec::new(&env)))
        .unwrap()
    ;

    votes
}

fn get_delegated_votes(env: &Env, addr: &Address) -> Vec<Address> {
    let votes: Vec<Address> = env
        .storage()
        .get(addr)
        .unwrap_or(Ok(Vec::new(&env)))
        .unwrap()
    ;

    votes
}

fn is_vote_delegated(env: &Env, v_to_delegate: &Address, voters: &Vec<Address>) -> bool {
    let mut already_delegated = false;
    let mut i = 0;
    while i < voters.len() && !already_delegated {

        let voter = voters.get(i).unwrap();
        match voter {
            Ok(vot) => {
                let d_votes = get_delegated_votes(&env, &vot);
                if d_votes.contains(v_to_delegate) {
                    already_delegated = true;
                }
            },
            Err(_e) => ()
        }

        i += 1;
    }

    already_delegated
}

#[contracttype]
pub enum PartyCounter {
    Counter(Symbol),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    VoterDelegated = 1,
    VoterNotRegistered = 2,
    PartyNotRegistered = 3,
    VoterAlreadyVoted = 4
}


pub struct BallotContract;

#[contractimpl]
impl BallotContract {
    
    pub fn add_party(env: Env, admin: Address, name: Symbol) -> u32  {
        admin.require_auth();
        let mut parties: Vec<Symbol>= get_parties(&env);

        if ! parties.contains(&name) {
            parties.push_back(name);
            env.storage().set(&PARTIES, &parties);
        }

        parties.len() as u32
    }

    pub fn add_voter(env: Env, admin: Address, addr: Address) -> u32 {
        admin.require_auth();
        let mut voters: Vec<Address> = get_voters(&env);
        
        if ! voters.contains(&addr) {
            voters.push_back(addr);
            env.storage().set(&VOTERS, &voters);
        }

        voters.len() as u32
    }

    pub fn vote(env: Env, voter: Address, party: Symbol) -> bool {

        let voters: Vec<Address>      = get_voters(&env);

        if is_vote_delegated(&env, &voter, &voters) {
            panic_with_error!(&env, Error::VoterDelegated);
        }

        let mut count_sum = 1;

        let parties: Vec<Symbol>      = get_parties(&env);
        let mut votes: Vec<Address>   = get_votes(&env);

        if !parties.contains(&party) {
            panic_with_error!(&env, Error::PartyNotRegistered);
        }

        if votes.contains(&voter) {
            panic_with_error!(&env, Error::VoterAlreadyVoted);
        }

        if !voters.contains(&voter) {
            panic_with_error!(&env, Error::VoterNotRegistered);
        }

        let party_counter_key = PartyCounter::Counter(party);
        let mut count: u32 = env.storage().get(&party_counter_key).unwrap_or(Ok(0)).unwrap(); 

        let v_delegated_votes = get_delegated_votes(&env, &voter);
        if v_delegated_votes.len() > 0 {
            count_sum = v_delegated_votes.len() + 1;
        }

        count += count_sum;
        env.storage().set(&party_counter_key, &count);
        votes.push_back(voter);
        env.storage().set(&VOTES, &votes);

        true
    }
 
    pub fn count(env: Env) -> Map<Symbol, u32> {
        
        let parties = get_parties(&env);
        let mut count_map: Map<Symbol, u32>= Map::new(&env);
        for party in parties.iter() {
            match party {
                Ok(p) => {
                    let party_counter_key = PartyCounter::Counter(p);
                    let party_count: u32 = env.storage().get(&party_counter_key).unwrap_or(Ok(0)).unwrap(); 
                    count_map.set(p, party_count);
                },
                _ => ()
            }
            
        }

        count_map

    }
     
    pub fn delegate(env: Env, v_to_delegate: Address, v_delegate: Address) -> Vec<Address> {
        let voters = get_voters(&env);
        if !voters.contains(&v_to_delegate) || !voters.contains(&v_delegate) {
            panic_with_error!(&env, Error::VoterNotRegistered);
        }

        let already_delegated = is_vote_delegated(&env, &v_to_delegate, &voters);

        let mut d_votes = get_delegated_votes(&env, &v_delegate);
        if !already_delegated {
            d_votes.push_back(v_to_delegate);
            env.storage().set(&v_delegate, &d_votes);
        }
        
        d_votes
    }
}

mod test;

