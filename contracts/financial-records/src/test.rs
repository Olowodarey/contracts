#![cfg(test)]
use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{Env};

#[test]
fn test_add_and_get_records() {
    let e = Env::default();
    e.mock_all_auths();

    let contract_id = e.register_contract(None, FinancialRecordContract);
    let client = FinancialRecordContractClient::new(&e, &contract_id);

    let owner = Address::generate(&e);
    let ipfs_hash = String::from_str(&e, "QmXoypizj2Madv6NthR75ce451F33968F9e1XF3D8xS288");
    let description = String::from_str(&e, "Test Tax Document");

    client.add_financial_record(&owner, &RecordType::TaxDocument, &ipfs_hash, &description);

    let records = client.get_financial_records(&owner, &owner);
    assert_eq!(records.len(), 1);
    let record = records.get(0).unwrap();
    assert_eq!(record.owner, owner);
    assert_eq!(record.record_type, RecordType::TaxDocument);
    assert_eq!(record.ipfs_hash, ipfs_hash);
    assert_eq!(record.description, description);
}

#[test]
fn test_access_control() {
    let e = Env::default();
    e.mock_all_auths();

    let contract_id = e.register_contract(None, FinancialRecordContract);
    let client = FinancialRecordContractClient::new(&e, &contract_id);

    let owner = Address::generate(&e);
    let auditor = Address::generate(&e);
    let stranger = Address::generate(&e);

    let ipfs_hash = String::from_str(&e, "hash");
    let description = String::from_str(&e, "desc");

    client.add_financial_record(&owner, &RecordType::Invoice, &ipfs_hash, &description);

    // Auditor cannot see yet
    let result = e.as_contract(&contract_id, || {
        std::panic::catch_unwind(|| {
            client.get_financial_records(&auditor, &owner)
        })
    });
    assert!(result.is_err());

    // Grant access
    client.grant_access(&owner, &auditor);

    // Auditor can see now
    let records = client.get_financial_records(&auditor, &owner);
    assert_eq!(records.len(), 1);

    // Stranger still cannot see
    let result = e.as_contract(&contract_id, || {
        std::panic::catch_unwind(|| {
            client.get_financial_records(&stranger, &owner)
        })
    });
    assert!(result.is_err());

    // Revoke access
    client.revoke_access(&owner, &auditor);
    let result = e.as_contract(&contract_id, || {
        std::panic::catch_unwind(|| {
            client.get_financial_records(&auditor, &owner)
        })
    });
    assert!(result.is_err());
}

#[test]
fn test_filtering() {
    let e = Env::default();
    e.mock_all_auths();

    let contract_id = e.register_contract(None, FinancialRecordContract);
    let client = FinancialRecordContractClient::new(&e, &contract_id);

    let owner = Address::generate(&e);

    // Add multiple records with different types and timestamps
    e.ledger().set_timestamp(100);
    client.add_financial_record(&owner, &RecordType::Invoice, &String::from_str(&e, "h1"), &String::from_str(&e, "d1"));
    
    e.ledger().set_timestamp(200);
    client.add_financial_record(&owner, &RecordType::TaxDocument, &String::from_str(&e, "h2"), &String::from_str(&e, "d2"));

    e.ledger().set_timestamp(300);
    client.add_financial_record(&owner, &RecordType::Invoice, &String::from_str(&e, "h3"), &String::from_str(&e, "d3"));

    // Filter by type
    let invoices = client.get_records_by_type(&owner, &owner, &RecordType::Invoice);
    assert_eq!(invoices.len(), 2);

    // Filter by date range
    let range_records = client.get_records_by_date_range(&owner, &owner, &150, &250);
    assert_eq!(range_records.len(), 1);
    assert_eq!(range_records.get(0).unwrap().timestamp, 200);
}
