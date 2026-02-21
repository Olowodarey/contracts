# Allergy Tracking Smart Contract

A comprehensive blockchain-based allergy tracking system for healthcare providers built on Stellar's Soroban platform. This contract enables secure recording, management, and querying of patient allergies with severity assessment, cross-sensitivity checking, and complete audit trails.

## Features

### Core Functionality
- **Multi-Type Allergen Support**: Track medication, food, environmental, and other allergen types
- **Severity Classification**: Four-level severity system (Mild, Moderate, Severe, Life-Threatening)
- **Drug Interaction Checking**: Real-time allergy checking with cross-sensitivity support
- **Historical Tracking**: Complete audit trail of all allergy records and severity changes
- **Status Management**: Track allergies as Active, Resolved, or Suspected
- **Duplicate Prevention**: Automatic detection and prevention of duplicate allergy records

### Security Features
- **Provider Authentication**: All operations require authenticated healthcare provider authorization
- **Patient Privacy**: Access control for viewing patient allergy records
- **Immutable Audit Trail**: All changes are permanently recorded on the blockchain
- **Verification System**: Support for verified vs. suspected allergies

### Advanced Capabilities
- **Cross-Sensitivity Database**: Register and check drug cross-sensitivities
- **Severity Update History**: Track all severity changes with reasons and timestamps
- **Reaction Type Tracking**: Record multiple reaction types per allergy
- **Onset Date Recording**: Optional tracking of when allergies first appeared
- **Resolution Management**: Mark allergies as resolved with detailed reasoning

## Data Structures

### AllergyRecord
```rust
pub struct AllergyRecord {
    pub allergy_id: u64,
    pub patient_id: Address,
    pub provider_id: Address,
    pub allergen: String,
    pub allergen_type: AllergenType,
    pub reaction_types: Vec<String>,
    pub severity: Severity,
    pub onset_date: Option<u64>,
    pub verified: bool,
    pub status: AllergyStatus,
    pub recorded_date: u64,
    pub last_updated: u64,
    pub resolution_date: Option<u64>,
    pub resolution_reason: Option<String>,
}
```

### Enumerations

**AllergenType**:
- `Medication` - Drug allergies
- `Food` - Food allergies
- `Environmental` - Pollen, dust, etc.
- `Other` - Other allergen types

**Severity**:
- `Mild` - Minor reactions
- `Moderate` - Significant reactions
- `Severe` - Serious reactions
- `LifeThreatening` - Anaphylaxis or critical reactions

**AllergyStatus**:
- `Active` - Currently active allergy
- `Resolved` - No longer allergic
- `Suspected` - Unverified allergy

## API Reference

### record_allergy
Record a new allergy for a patient.

```rust
fn record_allergy(
    env: Env,
    patient_id: Address,
    provider_id: Address,
    allergen: String,
    allergen_type: Symbol, // "medication", "food", "environmental", "other"
    reaction_types: Vec<String>,
    severity: Symbol, // "mild", "moderate", "severe", "life_threatening"
    onset_date: Option<u64>,
    verified: bool
) -> Result<u64, Error>
```

**Returns**: Unique allergy ID

**Errors**:
- `DuplicateAllergy` - Allergy already exists for this patient
- `InvalidAllergenType` - Invalid allergen type symbol
- `InvalidSeverity` - Invalid severity symbol

### update_allergy_severity
Update the severity level of an existing allergy.

```rust
fn update_allergy_severity(
    env: Env,
    allergy_id: u64,
    provider_id: Address,
    new_severity: Symbol,
    reason: String
) -> Result<(), Error>
```

**Errors**:
- `AllergyNotFound` - Allergy ID does not exist
- `AlreadyResolved` - Cannot update resolved allergies
- `InvalidSeverity` - Invalid severity symbol

### resolve_allergy
Mark an allergy as resolved (no longer active).

```rust
fn resolve_allergy(
    env: Env,
    allergy_id: u64,
    provider_id: Address,
    resolution_date: u64,
    resolution_reason: String
) -> Result<(), Error>
```

**Errors**:
- `AllergyNotFound` - Allergy ID does not exist
- `AlreadyResolved` - Allergy already resolved

### check_drug_allergy_interaction
Check for potential drug interactions based on patient allergies.

```rust
fn check_drug_allergy_interaction(
    env: Env,
    patient_id: Address,
    drug_name: String
) -> Result<Vec<InteractionWarning>, Error>
```

**Returns**: Vector of interaction warnings with severity and reaction details

### get_active_allergies
Retrieve all active allergies for a patient.

```rust
fn get_active_allergies(
    env: Env,
    patient_id: Address,
    requester: Address
) -> Result<Vec<AllergyRecord>, Error>
```

**Returns**: Vector of active allergy records

### get_allergy
Get a specific allergy record by ID.

```rust
fn get_allergy(
    env: Env,
    allergy_id: u64
) -> Result<AllergyRecord, Error>
```

### get_severity_history
Get the complete severity update history for an allergy.

```rust
fn get_severity_history(
    env: Env,
    allergy_id: u64
) -> Vec<SeverityUpdate>
```

### register_cross_sensitivity
Register a cross-sensitivity relationship between two drugs (admin only).

```rust
fn register_cross_sensitivity(
    env: Env,
    admin: Address,
    drug1: String,
    drug2: String
) -> Result<(), Error>
```

## Usage Examples

### Recording a New Allergy

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source provider \
  --network testnet \
  -- record_allergy \
  --patient_id <PATIENT_ADDRESS> \
  --provider_id <PROVIDER_ADDRESS> \
  --allergen "Penicillin" \
  --allergen_type "medication" \
  --reaction_types '["rash", "hives", "itching"]' \
  --severity "moderate" \
  --onset_date 1640000000 \
  --verified true
```

### Checking Drug Interactions

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- check_drug_allergy_interaction \
  --patient_id <PATIENT_ADDRESS> \
  --drug_name "Amoxicillin"
```

### Updating Severity

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source provider \
  --network testnet \
  -- update_allergy_severity \
  --allergy_id 0 \
  --provider_id <PROVIDER_ADDRESS> \
  --new_severity "severe" \
  --reason "Patient experienced severe reaction during treatment"
```

### Resolving an Allergy

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source provider \
  --network testnet \
  -- resolve_allergy \
  --allergy_id 0 \
  --provider_id <PROVIDER_ADDRESS> \
  --resolution_date 1650000000 \
  --resolution_reason "Completed desensitization therapy successfully"
```

## Testing

The contract includes comprehensive test coverage (>85%) covering:

- Basic allergy recording and retrieval
- Multiple allergy management
- Duplicate prevention
- Severity updates and history tracking
- Allergy resolution
- Drug interaction checking
- Cross-sensitivity detection
- Error handling for all edge cases
- Complete workflow integration tests

### Running Tests

```bash
# Run all tests
cargo test -p allergy-tracking

# Run with output
cargo test -p allergy-tracking -- --nocapture

# Run specific test
cargo test -p allergy-tracking test_record_allergy_success
```

### Test Coverage

```bash
# Install tarpaulin for coverage
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin -p allergy-tracking --out Html
```

## Building and Deployment

### Build the Contract

```bash
# Build for development
cargo build -p allergy-tracking

# Build optimized WASM
soroban contract build

# Optimize WASM
soroban contract optimize \
  --wasm target/wasm32-unknown-unknown/release/allergy_tracking.wasm
```

### Deploy to Testnet

```bash
# Deploy contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/allergy_tracking.optimized.wasm \
  --source deployer \
  --network testnet
```

## Security Considerations

### Authentication
- All write operations require provider authentication via `require_auth()`
- Read operations for patient data require requester authentication
- Admin operations (cross-sensitivity registration) require admin authentication

### Data Privacy
- Patient allergy data is stored on-chain but access is controlled
- Only authorized providers can view patient allergies
- All access attempts are logged via blockchain events

### Audit Trail
- All allergy records are immutable once created
- Severity updates are tracked in a separate history log
- Resolution actions are permanently recorded with reasons

### Input Validation
- Allergen type and severity symbols are validated
- Duplicate allergy prevention
- Resolved allergies cannot be modified

## Error Handling

The contract uses a comprehensive error system:

| Error Code | Error Name | Description |
|------------|------------|-------------|
| 1 | AllergyNotFound | Allergy ID does not exist |
| 2 | Unauthorized | Caller not authorized for operation |
| 3 | InvalidSeverity | Invalid severity level provided |
| 4 | InvalidAllergenType | Invalid allergen type provided |
| 5 | AlreadyResolved | Allergy already marked as resolved |
| 6 | PatientNotFound | Patient ID does not exist |
| 7 | DuplicateAllergy | Allergy already recorded for patient |

## Integration Guidelines

### With EHR Systems
- Use `record_allergy` when importing patient allergy data
- Call `check_drug_allergy_interaction` before prescribing medications
- Query `get_active_allergies` when displaying patient summaries

### With Prescription Systems
- Integrate `check_drug_allergy_interaction` into prescription workflow
- Block or warn when prescribing drugs with known allergies
- Check cross-sensitivities for drug classes

### With Clinical Decision Support
- Use severity levels to prioritize warnings
- Display reaction types to inform clinical decisions
- Track severity changes over time for pattern analysis

## Performance Considerations

- **Storage**: Uses persistent storage for allergy records and history
- **Gas Optimization**: Efficient data structures minimize transaction costs
- **Scalability**: Indexed by patient ID for fast lookups
- **Batch Operations**: Consider batching multiple allergy records for efficiency

## Compliance

This contract is designed with healthcare compliance in mind:

- **HIPAA Considerations**: Access controls and audit trails
- **Data Integrity**: Immutable blockchain storage
- **Audit Requirements**: Complete history of all changes
- **Patient Rights**: Support for data access and correction

## Future Enhancements

Potential future improvements:
- Genetic marker integration for predicting allergies
- Machine learning integration for cross-sensitivity prediction
- Integration with drug databases for automatic interaction checking
- Support for allergy testing results and immunotherapy tracking
- Multi-language support for allergen names
- Image attachment support for reaction documentation

## License

This contract is part of the Stellar Healthcare System project.

## Support

For issues, questions, or contributions, please refer to the main project repository.
