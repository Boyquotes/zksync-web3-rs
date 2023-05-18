pub const EIP712_TX_TYPE: u8 = 0x71;
pub const PRIORITY_OPERATION_L2_TX_TYPE: u8 = 0xff;

// The large L2 gas per pubdata to sign. This gas is enough to ensure that
// any reasonable limit will be accepted. Note, that the operator is NOT required to
// use the honest value of gas per pubdata and it can use any value up to the one signed by the user.
// In the future releases, we will provide a way to estimate the current gasPerPubdata.
pub const DEFAULT_GAS_PER_PUBDATA_LIMIT: u16 = 50000;
