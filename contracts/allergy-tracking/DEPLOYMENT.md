# Allergy Tracking Contract - Deployment Guide

This guide provides step-by-step instructions for deploying the Allergy Tracking smart contract to Stellar's Soroban platform.

## Prerequisites

Before deploying, ensure you have:

1. **Rust and Cargo** (1.74.0 or later)
   ```bash
   rustup --version
   cargo --version
   ```

2. **Soroban CLI** (21.0.0 or later)
   ```bash
   cargo install --locked soroban-cli
   soroban --version
   ```

3. **Stellar CLI** (optional, for network management)
   ```bash
   cargo install --locked stellar-cli
   ```

4. **WASM target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

## Environment Setup

### 1. Configure Networks

Add Stellar networks to your Soroban configuration:

```bash
# Testnet
soroban network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

# Mainnet (when ready)
soroban network add \
  --global mainnet \
  --rpc-url https://soroban-mainnet.stellar.org:443 \
  --network-passphrase "Public Global Stellar Network ; September 2015"
```

### 2. Create or Import Identity

```bash
# Generate new identity
soroban keys generate deployer --network testnet

# Or import existing secret key
soroban keys add deployer --secret-key <YOUR_SECRET_KEY>

# Get the public address
soroban keys address deployer
```

### 3. Fund Your Account (Testnet Only)

```bash
# Get testnet XLM from friendbot
curl "https://friendbot.stellar.org?addr=$(soroban keys address deployer)"
```

## Build Process

### 1. Clean Build

```bash
cd contracts/contracts/allergy-tracking
cargo clean
```

### 2. Run Tests

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html --output-dir coverage

# Verify >85% coverage
cargo tarpaulin | grep "coverage"
```

### 3. Build WASM Binary

```bash
# Build for release
cargo build --target wasm32-unknown-unknown --release

# The WASM file will be at:
# ../../target/wasm32-unknown-unknown/release/allergy_tracking.wasm
```

### 4. Optimize WASM

```bash
# Optimize the WASM binary
soroban contract optimize \
  --wasm ../../target/wasm32-unknown-unknown/release/allergy_tracking.wasm

# This creates: allergy_tracking.optimized.wasm
```

### 5. Verify Build

```bash
# Check WASM size (should be < 100KB for optimal performance)
ls -lh ../../target/wasm32-unknown-unknown/release/allergy_tracking.optimized.wasm

# Inspect contract
soroban contract inspect \
  --wasm ../../target/wasm32-unknown-unknown/release/allergy_tracking.optimized.wasm
```

## Deployment

### Testnet Deployment

```bash
# Deploy to testnet
CONTRACT_ID=$(soroban contract deploy \
  --wasm ../../target/wasm32-unknown-unknown/release/allergy_tracking.optimized.wasm \
  --source deployer \
  --network testnet)

echo "Contract deployed at: $CONTRACT_ID"

# Save contract ID for later use
echo $CONTRACT_ID > .contract-id-testnet
```

### Mainnet Deployment

```bash
# IMPORTANT: Only deploy to mainnet after thorough testing on testnet

# Deploy to mainnet
CONTRACT_ID=$(soroban contract deploy \
  --wasm ../../target/wasm32-unknown-unknown/release/allergy_tracking.optimized.wasm \
  --source deployer \
  --network mainnet)

echo "Contract deployed at: $CONTRACT_ID"

# Save contract ID
echo $CONTRACT_ID > .contract-id-mainnet
```

## Post-Deployment Verification

### 1. Verify Contract Deployment

```bash
# Read contract ID
CONTRACT_ID=$(cat .contract-id-testnet)

# Verify contract exists
soroban contract inspect --id $CONTRACT_ID --network testnet
```

### 2. Test Basic Functionality

```bash
# Generate test addresses
PATIENT=$(soroban keys generate patient --network testnet && soroban keys address patient)
PROVIDER=$(soroban keys generate provider --network testnet && soroban keys address provider)

# Fund test accounts
curl "https://friendbot.stellar.org?addr=$PATIENT"
curl "https://friendbot.stellar.org?addr=$PROVIDER"

# Test recording an allergy
soroban contract invoke \
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
  --verified true
```

### 3. Monitor Contract

```bash
# Check contract events
soroban events \
  --id $CONTRACT_ID \
  --network testnet \
  --start-ledger <LEDGER_NUMBER>
```

## Integration

### JavaScript/TypeScript Integration

```typescript
import { Contract, SorobanRpc } from '@stellar/stellar-sdk';

const contractId = 'YOUR_CONTRACT_ID';
const rpcUrl = 'https://soroban-testnet.stellar.org:443';

const server = new SorobanRpc.Server(rpcUrl);
const contract = new Contract(contractId);

// Record allergy
const tx = await contract.call(
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

### Python Integration

```python
from stellar_sdk import Soroban, Keypair, Network

contract_id = "YOUR_CONTRACT_ID"
rpc_url = "https://soroban-testnet.stellar.org:443"

soroban = Soroban(rpc_url)
provider_keypair = Keypair.from_secret("YOUR_SECRET")

# Record allergy
result = soroban.invoke_contract(
    contract_id=contract_id,
    function_name="record_allergy",
    parameters=[
        patient_id,
        provider_id,
        "Penicillin",
        "medication",
        ["rash", "hives"],
        "moderate",
        1640000000,
        True
    ],
    source_account=provider_keypair
)
```

## Upgrade Process

### 1. Deploy New Version

```bash
# Build new version
cargo build --target wasm32-unknown-unknown --release

# Optimize
soroban contract optimize \
  --wasm ../../target/wasm32-unknown-unknown/release/allergy_tracking.wasm

# Deploy new version (gets new contract ID)
NEW_CONTRACT_ID=$(soroban contract deploy \
  --wasm ../../target/wasm32-unknown-unknown/release/allergy_tracking.optimized.wasm \
  --source deployer \
  --network testnet)
```

### 2. Data Migration (if needed)

```bash
# Implement data migration script to transfer data from old to new contract
# This depends on your specific migration needs
```

## Monitoring and Maintenance

### Health Checks

```bash
# Check contract is responsive
soroban contract invoke \
  --id $CONTRACT_ID \
  --network testnet \
  -- get_allergy \
  --allergy_id 0
```

### Performance Monitoring

- Monitor transaction costs
- Track contract invocation frequency
- Monitor storage usage
- Set up alerts for errors

### Backup Strategy

- Regularly export contract state
- Maintain off-chain backups of critical data
- Document all contract IDs and deployment dates

## Security Considerations

### Pre-Deployment Checklist

- [ ] All tests passing (>85% coverage)
- [ ] Security audit completed
- [ ] Code review by multiple developers
- [ ] Dependency audit clean (`cargo audit`)
- [ ] No hardcoded secrets or keys
- [ ] Access controls properly implemented
- [ ] Error handling comprehensive
- [ ] Gas optimization reviewed

### Post-Deployment Security

- [ ] Monitor for unusual activity
- [ ] Set up alerting for errors
- [ ] Regular security audits
- [ ] Keep dependencies updated
- [ ] Maintain incident response plan

## Troubleshooting

### Common Issues

**Issue: "insufficient balance"**
```bash
# Solution: Fund your account
curl "https://friendbot.stellar.org?addr=$(soroban keys address deployer)"
```

**Issue: "contract not found"**
```bash
# Solution: Verify contract ID
soroban contract inspect --id $CONTRACT_ID --network testnet
```

**Issue: "transaction failed"**
```bash
# Solution: Check transaction details
soroban contract invoke --help
# Verify all parameters are correct
```

### Getting Help

- Stellar Discord: https://discord.gg/stellar
- Soroban Documentation: https://soroban.stellar.org
- GitHub Issues: [Your repository]

## Cost Estimation

### Testnet (Free)
- Deployment: Free
- Transactions: Free
- Storage: Free

### Mainnet (Estimated)
- Deployment: ~1-5 XLM
- Transaction: ~0.00001 XLM per operation
- Storage: ~0.001 XLM per KB per year

## Rollback Procedure

If issues are discovered post-deployment:

1. **Immediate**: Direct traffic to previous contract version
2. **Short-term**: Fix issues and deploy patch
3. **Long-term**: Implement proper upgrade mechanism

## Compliance and Auditing

### Audit Trail

All contract interactions are permanently recorded on the blockchain:
- Transaction hashes
- Timestamps
- Caller addresses
- Function parameters
- Results

### Compliance Reports

Generate compliance reports using:
```bash
# Export all contract events
soroban events \
  --id $CONTRACT_ID \
  --network testnet \
  --start-ledger <START> \
  --end-ledger <END> \
  > audit-report.json
```

## Support and Maintenance

### Regular Maintenance Tasks

- Weekly: Review error logs
- Monthly: Security audit
- Quarterly: Performance optimization review
- Annually: Full security audit

### Emergency Contacts

- DevOps Team: [Contact]
- Security Team: [Contact]
- Stellar Support: support@stellar.org

## Changelog

Track all deployments:

```
v1.0.0 - 2024-02-21
- Initial deployment to testnet
- Contract ID: [CONTRACT_ID]
- Features: Basic allergy tracking, cross-sensitivity checking
```

## License

This deployment guide is part of the Stellar Healthcare System project.
