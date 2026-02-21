# Allergy Tracking Contract - API Reference

Quick reference guide for all contract functions and data structures.

## Table of Contents
- [Data Structures](#data-structures)
- [Functions](#functions)
- [Error Codes](#error-codes)
- [Events](#events)
- [Examples](#examples)

## Data Structures

### AllergyRecord

Complete allergy record with all metadata.

```rust
pub struct AllergyRecord {
    pub allergy_id: u64,              // Unique identifier
    pub patient_id: Address,          // Patient's blockchain address
    pub provider_id: Address,         // Recording provider's address
    pub allergen: String,             // Name of allergen
    pub allergen_type: AllergenType,  // Type classification
    pub reaction_types: Vec<String>,  // List of reactions
    pub severity: Severity,           // Current severity level
    pub onset_date: Option<u64>,      // When allergy first appeared
    pub verified: bool,               // Confirmed vs suspected
    pub status: AllergyStatus,        // Active/Resolved/Suspected
    pub recorded_date: u64,           // When record was created
    pub last_updated: u64,            // Last modification timestamp
    pub resolution_date: Option<u64>, // When resolved (if applicable)
    pub resolution_reason: Option<String>, // Why resolved
}
```

### AllergenType

Classification of allergen types.

```rust
pub enum AllergenType {
    Medication,    // Drug allergies
    Food,          // Food allergies
    Environmental, // Pollen, dust, etc.
    Other,         // Other types
}
```

**Symbol Mappings:**
- `"medication"` or `"med"` → Medication
- `"food"` → Food
- `"environmental"` or `"env"` → Environmental
- `"other"` → Other

### Severity

Severity classification for allergic reactions.

```rust
pub enum Severity {
    Mild,            // Minor reactions
    Moderate,        // Significant reactions
    Severe,          // Serious reactions
    LifeThreatening, // Anaphylaxis, critical reactions
}
```

**Symbol Mappings:**
- `"mild"` → Mild
- `"moderate"` → Moderate
- `"severe"` → Severe
- `"life_threatening"` or `"life"` → LifeThreatening

### AllergyStatus

Current status of an allergy record.

```rust
pub enum AllergyStatus {
    Active,    // Currently active allergy
    Resolved,  // No longer allergic
    Suspected, // Unverified allergy
}
```

### SeverityUpdate

Audit record for severity changes.

```rust
pub struct SeverityUpdate {
    pub allergy_id: u64,
    pub provider_id: Address,
    pub old_severity: Severity,
    pub new_severity: Severity,
    pub reason: String,
    pub timestamp: u64,
}
```

### InteractionWarning

Warning returned when checking drug interactions.

```rust
pub struct InteractionWarning {
    pub allergy_id: u64,
    pub allergen: String,
    pub severity: Severity,
    pub reaction_types: Vec<String>,
}
```

## Functions

### record_allergy

Record a new allergy for a patient.

**Signature:**
```rust
fn record_allergy(
    env: Env,
    patient_id: Address,
    provider_id: Address,
    allergen: String,
    allergen_type: Symbol,
    reaction_types: Vec<String>,
    severity: Symbol,
    onset_date: Option<u64>,
    verified: bool
) -> Result<u64, Error>
```

**Parameters:**
- `patient_id`: Patient's blockchain address
- `provider_id`: Healthcare provider's address (requires auth)
- `allergen`: Name of the allergen (e.g., "Penicillin")
- `allergen_type`: Type symbol ("medication", "food", "environmental", "other")
- `reaction_types`: Vector of reaction descriptions
- `severity`: Severity symbol ("mild", "moderate", "severe", "life_threatening")
- `onset_date`: Optional timestamp when allergy first appeared
- `verified`: true for confirmed, false for suspected

**Returns:** Unique allergy ID (u64)

**Errors:**
- `DuplicateAllergy`: Allergy already exists for this patient
- `InvalidAllergenType`: Invalid allergen type symbol
- `InvalidSeverity`: Invalid severity symbol

**Example:**
```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source provider \
  -- record_allergy \
  --patient_id $PATIENT \
  --provider_id $PROVIDER \
  --allergen "Penicillin" \
  --allergen_type "medication" \
  --reaction_types '["rash", "hives"]' \
  --severity "moderate" \
  --onset_date 1640000000 \
  --verified true
```

---

### update_allergy_severity

Update the severity level of an existing allergy.

**Signature:**
```rust
fn update_allergy_severity(
    env: Env,
    allergy_id: u64,
    provider_id: Address,
    new_severity: Symbol,
    reason: String
) -> Result<(), Error>
```

**Parameters:**
- `allergy_id`: ID of the allergy to update
- `provider_id`: Healthcare provider's address (requires auth)
- `new_severity`: New severity symbol
- `reason`: Explanation for the severity change

**Returns:** `Ok(())` on success

**Errors:**
- `AllergyNotFound`: Allergy ID does not exist
- `AlreadyResolved`: Cannot update resolved allergies
- `InvalidSeverity`: Invalid severity symbol

**Example:**
```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source provider \
  -- update_allergy_severity \
  --allergy_id 0 \
  --provider_id $PROVIDER \
  --new_severity "severe" \
  --reason "Patient had severe reaction during treatment"
```

---

### resolve_allergy

Mark an allergy as resolved (no longer active).

**Signature:**
```rust
fn resolve_allergy(
    env: Env,
    allergy_id: u64,
    provider_id: Address,
    resolution_date: u64,
    resolution_reason: String
) -> Result<(), Error>
```

**Parameters:**
- `allergy_id`: ID of the allergy to resolve
- `provider_id`: Healthcare provider's address (requires auth)
- `resolution_date`: Timestamp when allergy was resolved
- `resolution_reason`: Explanation for resolution

**Returns:** `Ok(())` on success

**Errors:**
- `AllergyNotFound`: Allergy ID does not exist
- `AlreadyResolved`: Allergy already resolved

**Example:**
```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source provider \
  -- resolve_allergy \
  --allergy_id 0 \
  --provider_id $PROVIDER \
  --resolution_date 1650000000 \
  --resolution_reason "Completed desensitization therapy"
```

---

### check_drug_allergy_interaction

Check for potential drug interactions based on patient allergies.

**Signature:**
```rust
fn check_drug_allergy_interaction(
    env: Env,
    patient_id: Address,
    drug_name: String
) -> Result<Vec<InteractionWarning>, Error>
```

**Parameters:**
- `patient_id`: Patient's blockchain address
- `drug_name`: Name of drug to check

**Returns:** Vector of interaction warnings (empty if no interactions)

**Errors:** None (returns empty vector if no allergies)

**Example:**
```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  -- check_drug_allergy_interaction \
  --patient_id $PATIENT \
  --drug_name "Amoxicillin"
```

---

### get_active_allergies

Retrieve all active allergies for a patient.

**Signature:**
```rust
fn get_active_allergies(
    env: Env,
    patient_id: Address,
    requester: Address
) -> Result<Vec<AllergyRecord>, Error>
```

**Parameters:**
- `patient_id`: Patient's blockchain address
- `requester`: Address requesting the data (requires auth)

**Returns:** Vector of active allergy records

**Errors:** None (returns empty vector if no allergies)

**Example:**
```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source requester \
  -- get_active_allergies \
  --patient_id $PATIENT \
  --requester $REQUESTER
```

---

### get_allergy

Get a specific allergy record by ID.

**Signature:**
```rust
fn get_allergy(
    env: Env,
    allergy_id: u64
) -> Result<AllergyRecord, Error>
```

**Parameters:**
- `allergy_id`: ID of the allergy to retrieve

**Returns:** Complete allergy record

**Errors:**
- `AllergyNotFound`: Allergy ID does not exist

**Example:**
```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  -- get_allergy \
  --allergy_id 0
```

---

### get_severity_history

Get the complete severity update history for an allergy.

**Signature:**
```rust
fn get_severity_history(
    env: Env,
    allergy_id: u64
) -> Vec<SeverityUpdate>
```

**Parameters:**
- `allergy_id`: ID of the allergy

**Returns:** Vector of severity updates (empty if no updates)

**Example:**
```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  -- get_severity_history \
  --allergy_id 0
```

---

### register_cross_sensitivity

Register a cross-sensitivity relationship between two drugs (admin only).

**Signature:**
```rust
fn register_cross_sensitivity(
    env: Env,
    admin: Address,
    drug1: String,
    drug2: String
) -> Result<(), Error>
```

**Parameters:**
- `admin`: Admin address (requires auth)
- `drug1`: First drug name
- `drug2`: Second drug name

**Returns:** `Ok(())` on success

**Errors:** None

**Example:**
```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source admin \
  -- register_cross_sensitivity \
  --admin $ADMIN \
  --drug1 "Penicillin" \
  --drug2 "Amoxicillin"
```

## Error Codes

| Code | Name | Description |
|------|------|-------------|
| 1 | AllergyNotFound | Allergy ID does not exist |
| 2 | Unauthorized | Caller not authorized for operation |
| 3 | InvalidSeverity | Invalid severity level provided |
| 4 | InvalidAllergenType | Invalid allergen type provided |
| 5 | AlreadyResolved | Allergy already marked as resolved |
| 6 | PatientNotFound | Patient ID does not exist |
| 7 | DuplicateAllergy | Allergy already recorded for patient |

## Events

The contract emits events for key operations:

### Allergy Recorded
```rust
("allergy", patient_id, allergy_id) => allergen
```

### Severity Updated
```rust
("sev_upd", allergy_id) => (old_severity, new_severity)
```

### Allergy Resolved
```rust
("resolved", allergy_id) => resolution_reason
```

## Usage Patterns

### Complete Workflow Example

```bash
# 1. Setup
CONTRACT_ID="your_contract_id"
PATIENT="patient_address"
PROVIDER="provider_address"
ADMIN="admin_address"

# 2. Register cross-sensitivities
soroban contract invoke \
  --id $CONTRACT_ID \
  --source admin \
  -- register_cross_sensitivity \
  --admin $ADMIN \
  --drug1 "Penicillin" \
  --drug2 "Amoxicillin"

# 3. Record allergy
ALLERGY_ID=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source provider \
  -- record_allergy \
  --patient_id $PATIENT \
  --provider_id $PROVIDER \
  --allergen "Penicillin" \
  --allergen_type "medication" \
  --reaction_types '["hives", "swelling"]' \
  --severity "moderate" \
  --onset_date 1640000000 \
  --verified true)

# 4. Check drug interaction
soroban contract invoke \
  --id $CONTRACT_ID \
  -- check_drug_allergy_interaction \
  --patient_id $PATIENT \
  --drug_name "Amoxicillin"

# 5. Update severity if needed
soroban contract invoke \
  --id $CONTRACT_ID \
  --source provider \
  -- update_allergy_severity \
  --allergy_id $ALLERGY_ID \
  --provider_id $PROVIDER \
  --new_severity "severe" \
  --reason "Patient had severe reaction"

# 6. Get patient's active allergies
soroban contract invoke \
  --id $CONTRACT_ID \
  --source provider \
  -- get_active_allergies \
  --patient_id $PATIENT \
  --requester $PROVIDER

# 7. Resolve allergy if appropriate
soroban contract invoke \
  --id $CONTRACT_ID \
  --source provider \
  -- resolve_allergy \
  --allergy_id $ALLERGY_ID \
  --provider_id $PROVIDER \
  --resolution_date 1650000000 \
  --resolution_reason "False positive confirmed"
```

## Integration Tips

### JavaScript/TypeScript

```typescript
import { Contract } from '@stellar/stellar-sdk';

const contract = new Contract(contractId);

// Record allergy
const result = await contract.call(
  'record_allergy',
  patientId,
  providerId,
  'Penicillin',
  'medication',
  ['rash', 'hives'],
  'moderate',
  1640000000,
  true
);
```

### Python

```python
from stellar_sdk import Soroban

soroban = Soroban(rpc_url)

# Check drug interaction
warnings = soroban.invoke_contract(
    contract_id=contract_id,
    function_name="check_drug_allergy_interaction",
    parameters=[patient_id, "Amoxicillin"]
)
```

### Rust

```rust
use allergy_tracking::AllergyTrackingContractClient;

let client = AllergyTrackingContractClient::new(&env, &contract_id);

// Get active allergies
let allergies = client.get_active_allergies(&patient_id, &requester);
```

## Performance Considerations

- **Gas Costs**: Recording allergies costs more than reading
- **Storage**: Each allergy record uses ~500 bytes
- **Batch Operations**: Consider batching multiple reads
- **Caching**: Cache frequently accessed data off-chain

## Best Practices

1. **Always check drug interactions** before prescribing
2. **Update severity** when patient experiences new reactions
3. **Resolve false positives** to maintain data accuracy
4. **Use verified=true** only for confirmed allergies
5. **Provide detailed reasons** for severity updates and resolutions
6. **Register cross-sensitivities** for drug classes
7. **Monitor events** for real-time updates

## Support

For questions or issues:
- Documentation: See README.md
- Security: See SECURITY.md
- Deployment: See DEPLOYMENT.md
- GitHub: [Repository URL]
