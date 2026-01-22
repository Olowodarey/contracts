#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatientData {
    pub name: String,
    pub dob: u64,
    pub metadata: String, // Can include IPFS links to insurance/medical history
}

#[contracttype]
pub enum DataKey {
    Patient(Address),
}

#[contract]
pub struct PatientRegistry;

#[contractimpl]
impl PatientRegistry {
    /// Registers a new patient with their wallet address, name, date of birth, and metadata.
    pub fn register_patient(env: Env, wallet: Address, name: String, dob: u64, metadata: String) {
        // Ensure the person calling this is the owner of the wallet
        wallet.require_auth();

        let key = DataKey::Patient(wallet.clone());
        if env.storage().persistent().has(&key) {
            panic!("Patient already registered");
        }

        let data = PatientData {
            name,
            dob,
            metadata,
        };

        // Store patient data persistently
        env.storage().persistent().set(&key, &data);

        // Emit an event for tracking
        env.events()
            .publish((symbol_short!("reg_pat"), wallet), symbol_short!("success"));
    }

    /// Updates metadata for an existing patient.
    pub fn update_patient(env: Env, wallet: Address, metadata: String) {
        wallet.require_auth();

        let key = DataKey::Patient(wallet.clone());
        let mut data: PatientData = env
            .storage()
            .persistent()
            .get(&key)
            .expect("Patient not found");

        data.metadata = metadata;
        env.storage().persistent().set(&key, &data);

        env.events()
            .publish((symbol_short!("upd_pat"), wallet), symbol_short!("success"));
    }

    /// Retrieves patient data for a given wallet address.
    pub fn get_patient(env: Env, wallet: Address) -> PatientData {
        let key = DataKey::Patient(wallet);
        env.storage()
            .persistent()
            .get(&key)
            .expect("Patient not found")
    }
}

#[cfg(test)]
mod test;
