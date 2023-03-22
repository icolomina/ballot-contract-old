#![no_std]
use core::{panic};

use soroban_sdk::{contractimpl, contracttype, symbol, Env, Symbol, Vec, Map, Address};

const PARTIES: Symbol = symbol!("parties");
const VOTERS: Symbol = symbol!("voters");
const VOTES: Symbol = symbol!("votes");

pub fn get_parties(env: &Env) -> Vec<Party> {
    let parties: Vec<Party>= env
        .storage()
        .get(&PARTIES)
        .unwrap_or(Ok(Vec::new(&env)))
        .unwrap()
    ;

    parties
}

pub fn get_voters(env: &Env) -> Vec<Voter> {
    let voters: Vec<Voter>= env
        .storage()
        .get(&VOTERS)
        .unwrap_or(Ok(Vec::new(&env)))
        .unwrap()
    ;

    voters
}

pub fn get_voter(voters: Vec<Voter>, addr: Address) -> Option<Voter> {
    let voter = voters
        .iter()
        .find(|r1| r1.as_ref().unwrap().addr == addr)
        .unwrap()
        .ok()
    ;

    voter
}

#[derive(Debug, Clone)]
#[contracttype]
pub struct Voter {
    addr: Address
}

#[derive(Debug, Clone)]
#[contracttype]
pub struct Vote {
    party: Party,
    voter: Voter
}

#[derive(Debug, Clone)]
#[contracttype]
pub struct Party {
    name: Symbol
}

pub struct BallotContract;

#[contractimpl]
impl BallotContract {
    
    pub fn add_party(env: Env, name: Symbol) -> u32  {
        let mut parties: Vec<Party>= get_parties(&env);

        if ! parties.contains(&Party { name }) {
            parties.push_back(Party { name });
            env.storage().set(&PARTIES, &parties);
        }

        parties.len() as u32
    }

    pub fn add_voter(env: Env, addr: Address) -> u32{
        let mut voters: Vec<Voter>= get_voters(&env);
        let voter = Voter { addr };
        
        if ! voters.contains(&voter) {
            voters.push_back(voter);
            env.storage().set(&VOTERS, &voters);
        }

        voters.len() as u32
    }

    pub fn vote(env: Env, voter: Address, party: Symbol) -> bool {

        let mut vote_added = false;
        let v: Voter;
        let p: Party;

        let parties: Vec<Party>= get_parties(&env);
        let voters: Vec<Voter>= get_voters(&env);


        let voter = voters
            .iter()
            .find(|r1| r1.as_ref().unwrap().addr == voter)
            .unwrap()
            .ok()
        ;

        let party = parties
            .iter()
            .find(|r2| r2.as_ref().unwrap().name == party)
            .unwrap()
            .ok()
        ;


        match voter {
            Some(x) => v = x,
            _ => panic!()
        };

        match party {
            Some(x) => p = x,
            _ => panic!()
        };

        let mut votes: Vec<Vote> = env
            .storage()
            .get(&VOTES)
            .unwrap_or(Ok(Vec::new(&env)))
            .unwrap()
            ;

        let vot = Vote { party: p, voter: v };
        if ! votes.contains(&vot) {
            votes.push_back(vot);
            env.storage().set(&VOTES, &votes);
            vote_added = true;
        }

        vote_added

        
    }
 
    pub fn count(env: Env) -> Map<Symbol, u32> {
        let parties = get_parties(&env);
        let votes: Vec<Vote> = env
            .storage()
            .get(&VOTES)
            .unwrap_or(Ok(Vec::new(&env)))
            .unwrap()
        ;

        let mut map = Map::new(&env);
        for party in parties  {
            map.set(party.unwrap().name, 0);
        }

        for vote in votes  {
            let party = vote.unwrap().party.name;
            let current = map.get(party).unwrap().ok();

            match current {
                Some(tot) => map.set(party, tot + 1),
                _ => ()
            };
        }

        map

    }

}

mod test;

