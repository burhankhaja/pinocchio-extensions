# Pinocchio Token-2022 Extensions - Project Status

## Overview
This project works on creating CPI calls in Pinocchio for Token 2022 extensions. It also includes comprehensive tests.

---

## CPI Program Structure

### Location
- **Main Program**: `pinocchio-extensions/programs/token-2022/src/extension/`
- **Proxy Program**: `pinocchio-extensions/cpi-tests/programs/token-2022-proxy/src/instructions/`

### CPI Instruction Format (Example: Scaled UI Amount)

```
pinocchio-extensions/programs/token-2022/src/extension/scaled_ui_amount/
├── mod.rs                          # Module exports
├── state.rs                        # State structs, enums, instruction data builders
└── instructions/
    ├── mod.rs                      # Instruction exports
    ├── initialize.rs               # Initialize instruction
    └── update.rs                   # Update instruction
```

**State File** (`state.rs`):
- Extension state structs (e.g., `ScaledUiAmountConfig`)
- `from_bytes()` / `from_account_info()` methods
- Instruction data builder functions
- Instruction enum (e.g., `ScaledUiAmountInstruction`)

**Instruction File** (e.g., `initialize.rs`):
- Wrapper struct with fields for accounts and parameters
- `invoke()` and `invoke_signed()` methods
- Uses fixed-size arrays (no Vec due to no_std)
- Builds `AccountMeta` array and instruction data
- Calls `pinocchio::cpi::invoke_signed()`

---

## Test Structure

### Location
- **Tests**: `pinocchio-extensions/cpi-tests/tests/src/`
- **Helpers**: `pinocchio-extensions/cpi-tests/tests/src/helpers/extensions/token_2022/`

### Test File Format (Example: Scaled UI Amount)

```
pinocchio-extensions/cpi-tests/tests/src/
├── scaled_ui_amount.rs             # Test cases
└── helpers/extensions/token_2022/
    └── scaled_ui_amount.rs         # Helper trait implementation
```

**Helper File** (`helpers/.../scaled_ui_amount.rs`):
- Trait definition (e.g., `Token2022ScaledUiAmountExtension`)
- `try_*` methods for instructions (e.g., `try_initialize`, `try_update`)
- `query_*` methods for state (e.g., `query_scaled_ui_amount_config`)
- Handles both `Target::Spl` and `Target::Proxy`

**Test File** (`scaled_ui_amount.rs`):
- Import helper trait
- Test both SPL and Proxy implementations
- Use `Target::Spl` for SPL Token-2022
- Use `Target::Proxy` for Pinocchio implementation
- Compare results with `assert_eq!()`

**Test Naming Convention**:
- `{instruction_name}` - Tests SPL implementation
- `proxy_{instruction_name}` - Tests Pinocchio implementation

---

## Completed Extensions

### 1. **CPI Guard**

### 2. **Default Account State**

### 3. **Group Member Pointer**

### 4. **Group Pointer**

### 5. **Interest Bearing Mint**

### 6. **Pausable**

### 7. **Permanent Delegate**

### 8. **Scaled UI Amount**

### 9. **Token Group**

### 10. **Token Group Member**

### 11. **Transfer Hook**

---

## In Progress ⏳

### 12. **Transfer Fee**
- By Zubayr

### 13. **Memo Transfer**
- By Burhan

### 14. **Metadata Pointer**
- By Burhan

---

## Not Started

### 13. **Confidential Mint Burn**
- Not implemented
- **Complexity**: High - requires zkSNARK operations

### 14. **Confidential Transfer**
- Not implemented
- **Complexity**: High - requires zkSNARK operations

### 15. **Confidential Transfer Fee**
- Not implemented
- **Complexity**: High - requires zkSNARK operations

### 17. **Token Metadata**

---

## State-Only Extensions (No Instructions Needed)

These extensions are set during mint/account creation and have no separate CPI instructions:

- **Immutable Owner** - Marker extension, no state
- **Mint Close Authority** - State set during mint creation
- **Non-Transferable** - Marker extension, no state

---

## Development Notes

- Always check existing working extensions (e.g., `pausable`, `scaled_ui_amount`) as reference
- Use `cargo test --lib {extension}::proxy -- --nocapture` to run proxy tests
- Use `cargo test --lib {extension} -- --nocapture` to run all tests
- Prefer step-by-step implementation: state → instruction → proxy → helper → tests
- Remember: Pinocchio is `no_std` - no Vec, no String, no allocations!
- Have patience with this. It has a learning curve involved!

