#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Symbol};

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    /// CREATE / UPDATE: Memberikan suara kepada kandidat (tambah 1 suara)
    pub fn vote(env: Env, candidate: Symbol) -> u32 {
        let mut current_votes: u32 = env.storage().instance().get(&candidate).unwrap_or(0);
        current_votes += 1;
        env.storage().instance().set(&candidate, &current_votes);
        
        return current_votes;
    }

    /// READ: Membaca total suara dari seorang kandidat
    pub fn read_votes(env: Env, candidate: Symbol) -> u32 {
        return env.storage().instance().get(&candidate).unwrap_or(0);
    }

    /// UPDATE: Mengatur ulang (reset) suara kandidat kembali ke 0
    pub fn reset_votes(env: Env, candidate: Symbol) -> String {
        if env.storage().instance().has(&candidate) {
            // Set ulang nilai (value) ke 0 bertipe u32
            env.storage().instance().set(&candidate, &0u32);
            return String::from_str(&env, "Suara kandidat berhasil direset ke 0");
        } 
        
        return String::from_str(&env, "Kandidat tidak ditemukan");
    }

    /// DELETE: Menghapus data kandidat beserta jumlah suaranya dari storage
    pub fn delete_candidate(env: Env, candidate: Symbol) -> String {
        if env.storage().instance().has(&candidate) {
            env.storage().instance().remove(&candidate);
            return String::from_str(&env, "Data kandidat berhasil dihapus");
        } 
        
        return String::from_str(&env, "Kandidat tidak ditemukan");
    }
}

mod test;