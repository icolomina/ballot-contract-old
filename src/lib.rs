#![no_std]

use soroban_sdk::{contractimpl, contracttype, symbol, Env, Symbol, Vec, Map, Address};

const PARTIES: Symbol = symbol!("parties");
const VOTERS: Symbol = symbol!("voters");
const VOTES: Symbol = symbol!("votes");

pub fn get_parties(env: &Env) -> Vec<Symbol> {
    let parties: Vec<Symbol>= env
        .storage()
        .get(&PARTIES)
        .unwrap_or(Ok(Vec::new(&env)))
        .unwrap()
    ;

    parties
}

pub fn get_voters(env: &Env) -> Vec<Address> {
    let voters: Vec<Address>= env
        .storage()
        .get(&VOTERS)
        .unwrap_or(Ok(Vec::new(&env)))
        .unwrap()
    ;

    voters
}

pub fn get_votes(env: &Env) -> Vec<Address> {
    let votes: Vec<Address> = env
        .storage()
        .get(&VOTES)
        .unwrap_or(Ok(Vec::new(&env)))
        .unwrap()
    ;

    votes
}

#[contracttype]
pub enum PartyCounter {
    Counter(Symbol),
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

        let mut vote_added = false;

        let parties: Vec<Symbol>      = get_parties(&env);
        let voters: Vec<Address>      = get_voters(&env);
        let mut votes: Vec<Address>   = get_votes(&env);

        if voters.contains(&voter) && parties.contains(&party) && !votes.contains(&voter) {
            let party_counter_key = PartyCounter::Counter(party);
            let mut count: u32 = env.storage().get(&party_counter_key).unwrap_or(Ok(0)).unwrap(); 

            count += 1;
            env.storage().set(&party_counter_key, &count);
            vote_added = true;
            votes.push_back(voter);
            env.storage().set(&VOTES, &votes);
        }


        vote_added
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
     

}

mod test;

