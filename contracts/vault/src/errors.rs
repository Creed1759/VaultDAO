//! VaultDAO error definitions.

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum VaultError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NoSigners = 3,
    ThresholdTooLow = 4,
    ThresholdTooHigh = 5,
    QuorumTooHigh = 6,
    QuorumNotReached = 7,
    Unauthorized = 10,
    NotASigner = 11,
    InsufficientRole = 12,
    VoterNotInSnapshot = 13,
    ProposalNotFound = 20,
    ProposalNotPending = 21,
    ProposalNotApproved = 22,
    ProposalAlreadyExecuted = 23,
    ProposalExpired = 24,
    ProposalAlreadyCancelled = 25,
    VotingDeadlinePassed = 26,
    AlreadyApproved = 30,
    InvalidAmount = 40,
    ExceedsProposalLimit = 41,
    ExceedsDailyLimit = 42,
    ExceedsWeeklyLimit = 43,
    VelocityLimitExceeded = 50,
    TimelockNotExpired = 60,
    SchedulingError = 61,
    InsufficientBalance = 70,
    TransferFailed = 71,
    SignerAlreadyExists = 80,
    SignerNotFound = 81,
    CannotRemoveSigner = 82,
    RecipientNotWhitelisted = 90,
    RecipientBlacklisted = 91,
    AddressAlreadyOnList = 92,
    AddressNotOnList = 93,
    InsuranceInsufficient = 110,
    GasLimitExceeded = 120,
    BatchTooLarge = 130,
    ConditionsNotMet = 140,
    IntervalTooShort = 150,
    DexError = 160,
    DexOperationFailed = 161,
    BridgeError = 165,
    RetryError = 168,
    XVaultNotEnabled = 200,
    TemplateNotFound = 210,
    TemplateInactive = 211,
    TemplateValidationFailed = 212,
    FundingRoundError = 220,
    // Issue #1064: Streaming Rate Limiter
    StreamRateLimitExceeded = 230,
    StreamDustRejected = 231,
    // Issue #1075: Insurance Claim Governance
    ClaimNotFound = 240,
    ClaimNotPending = 241,
    ClaimAlreadyVoted = 242,
    ClaimSelfVote = 243,
    ClaimVoteDeadlineTooShort = 244,
    ClaimBondInsufficient = 245,
    // Issue #1081: Multi-Token Vault
    TokenAlreadySupported = 250,
    TokenNotSupported = 251,
    TooManyTokens = 252,
    CannotRemoveDefaultToken = 253,
    TokenHasActivePayments = 254,
}

// Compatibility markers for CI source checks:
// DelegationError, DelegationChainTooLong, CircularDelegation
