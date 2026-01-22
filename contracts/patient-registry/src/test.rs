#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env, String};

#[test]
fn test_register_and_get_patient() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PatientRegistry);
    let client = PatientRegistryClient::new(&env, &contract_id);

    let patient_wallet = Address::generate(&env);
    let name = String::from_str(&env, "John Doe");
    let dob = 631152000; // 1990-01-01
    let metadata = String::from_str(&env, "ipfs://some-medical-history");

    // Mock authorization
    env.mock_all_auths();

    client.register_patient(&patient_wallet, &name, &dob, &metadata);

    let patient_data = client.get_patient(&patient_wallet);
    assert_eq!(patient_data.name, name);
    assert_eq!(patient_data.dob, dob);
    assert_eq!(patient_data.metadata, metadata);
}

#[test]
fn test_update_patient() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PatientRegistry);
    let client = PatientRegistryClient::new(&env, &contract_id);

    let patient_wallet = Address::generate(&env);
    let name = String::from_str(&env, "John Doe");
    let dob = 631152000;
    let initial_metadata = String::from_str(&env, "ipfs://initial");

    env.mock_all_auths();

    client.register_patient(&patient_wallet, &name, &dob, &initial_metadata);

    let new_metadata = String::from_str(&env, "ipfs://updated-history");
    client.update_patient(&patient_wallet, &new_metadata);

    let patient_data = client.get_patient(&patient_wallet);
    assert_eq!(patient_data.metadata, new_metadata);
}

#[test]
#[should_panic(expected = "Patient already registered")]
fn test_register_already_registered() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PatientRegistry);
    let client = PatientRegistryClient::new(&env, &contract_id);

    let patient_wallet = Address::generate(&env);
    let name = String::from_str(&env, "John Doe");
    let dob = 631152000;
    let metadata = String::from_str(&env, "ipfs://test");

    env.mock_all_auths();

    client.register_patient(&patient_wallet, &name, &dob, &metadata);
    client.register_patient(&patient_wallet, &name, &dob, &metadata); // Should panic
}
