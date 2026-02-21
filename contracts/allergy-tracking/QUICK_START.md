# Quick Start Guide - Allergy Tracking Contract

Get up and running with the Allergy Tracking smart contract in minutes.

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Soroban CLI
cargo install --locked soroban-cli

# Add WASM target
rustup target add wasm32-unknown-unknown
```

## Quick Test

```bash
# Navigate to contract directory
cd contracts/contracts/allergy-tracking

# Run tests
cargo test

# Expected output: 15 tests passed
```

## Quick Build

```bash
# Build WASM
cargo build --target wasm32-unknown-unknown --release

# Optimize (optional)
soroban contract optimize \
  --wasm ../../target/wasm32-unknown-unknown/release/allergy_tracking.wasm
```

## Quick Deploy (Testnet)

```bash
# Setup network
soroban network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

# Generate identity
soroban keys generate deployer --network testnet

# Fund account
curl "https://friendbot.stellar.org?addr=$(soroban keys address deployer)"

# Deploy
CONTRACT_ID=$(soroban contract deploy \
  --wasm ../../target/wasm32-unknown-unknown/release/allergy_tracking.wasm \
  --source deployer \
  --network testnet)

echo "Contract deployed: $CONTRACT_ID"
```

## Quick Usage Example

```bash
# Generate test addresses
PATIENT=$(soroban keys generate patient --network testnet && soroban keys address patient)
PROVIDER=$(soroban keys generate provider --network testnet && soroban keys address provider)

# Record an allergy
ALLERGY_ID=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source provider \
  --network testnet \
  -- record_allergy \
  --patient_id $PATIENT \
  --provider_id $PROVIDER \
  --allergen "Penicillin" \
  --allergen_type "medication" \
  --reaction_types '["rash", "hives"]' \
  --severity "moderate" \
  --onset_date 1640000000 \
  --verified true)

echo "Allergy recorded with ID: $ALLERGY_ID"

# Check drug interaction
soroban contract invoke \
  --id $CONTRACT_ID \
  --network testnet \
  -- check_drug_allergy_interaction \
  --patient_id $PATIENT \
  --drug_name "Penicillin"

# Get active allergies
soroban contract invoke \
  --id $CONTRACT_ID \
  --source provider \
  --network testnet \
  -- get_active_allergies \
  --patient_id $PATIENT \
  --requester $PROVIDER
```

## Common Commands

### Development
```bash
# Run tests
make test

# Build
make build

# Format code
make fmt

# Lint
make lint

# Coverage
make coverage
```

### Deployment
```bash
# Deploy to testnet
make deploy-testnet

# Build documentation
make docs
```

## Project Structure

```
allergy-tracking/
├── src/
│   ├── lib.rs              # Main contract
│   └── test.rs             # Tests
├── Cargo.toml              # Dependencies
├── Makefile                # Build automation
├── README.md               # Full documentation
├── API_REFERENCE.md        # API details
├── DEPLOYMENT.md           # Deployment guide
├── SECURITY.md             # Security info
└── QUICK_START.md          # This file
```

## Key Functions

### Record Allergy
```bash
record_allergy(
  patient_id,
  provider_id,
  allergen,
  allergen_type,    # "medication", "food", "environmental", "other"
  reaction_types,
  severity,         # "mild", "moderate", "severe", "life_threatening"
  onset_date,
  verified
)
```

### Check Drug Interaction
```bash
check_drug_allergy_interaction(
  patient_id,
  drug_name
)
```

### Update Severity
```bash
update_allergy_severity(
  allergy_id,
  provider_id,
  new_severity,
  reason
)
```

### Get Active Allergies
```bash
get_active_allergies(
  patient_id,
  requester
)
```

## Error Codes

| Code | Error | Description |
|------|-------|-------------|
| 1 | AllergyNotFound | Allergy doesn't exist |
| 2 | Unauthorized | Not authorized |
| 3 | InvalidSeverity | Invalid severity level |
| 4 | InvalidAllergenType | Invalid allergen type |
| 5 | AlreadyResolved | Allergy already resolved |
| 6 | PatientNotFound | Patient doesn't exist |
| 7 | DuplicateAllergy | Allergy already recorded |

## Troubleshooting

### Tests Failing
```bash
# Clean and rebuild
cargo clean
cargo test
```

### Build Errors
```bash
# Update dependencies
cargo update
cargo build --target wasm32-unknown-unknown --release
```

### Deployment Issues
```bash
# Check account balance
soroban keys address deployer
# Fund if needed
curl "https://friendbot.stellar.org?addr=$(soroban keys address deployer)"
```

## Next Steps

1. ✅ Read [README.md](README.md) for comprehensive documentation
2. ✅ Check [API_REFERENCE.md](API_REFERENCE.md) for detailed API docs
3. ✅ Review [SECURITY.md](SECURITY.md) for security considerations
4. ✅ Follow [DEPLOYMENT.md](DEPLOYMENT.md) for production deployment
5. ✅ See [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) for project overview

## Support

- **Documentation**: See README.md
- **Issues**: GitHub Issues
- **Security**: See SECURITY.md
- **Stellar Discord**: https://discord.gg/stellar

## Quick Reference

### Allergen Types
- `medication` or `med` - Drug allergies
- `food` - Food allergies
- `environmental` or `env` - Environmental allergies
- `other` - Other types

### Severity Levels
- `mild` - Minor reactions
- `moderate` - Significant reactions
- `severe` - Serious reactions
- `life_threatening` or `life` - Critical reactions

### Status Values
- `Active` - Currently active allergy
- `Resolved` - No longer allergic
- `Suspected` - Unverified allergy

## License

Part of the Stellar Healthcare System project.

---

**Ready to deploy?** Follow the [DEPLOYMENT.md](DEPLOYMENT.md) guide for detailed instructions.

**Need help?** Check the [README.md](README.md) for comprehensive documentation.

**Security concerns?** Review [SECURITY.md](SECURITY.md) for security information.
