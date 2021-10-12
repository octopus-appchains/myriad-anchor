use near_sdk::AccountId;

/// Storage keys for collections of sub-struct in main contract
pub enum StorageKey {
    OctToken,
    WrappedAppchainToken,
    NearFungibleTokenSymbols,
    NearFungibleTokens,
    NearFungibleTokensMap,
    NextValidatorSet,
    ValidatorSetHistories,
    ValidatorSetHistoriesMap,
    UnwithdrawnValidatorRewards,
    UnwithdrawnDelegatorRewards,
    UnbondedStakes,
    LookupMapOfValidatorIdsInAppchain,
    AppchainSettings,
    AnchorSettings,
    ProtocolSettings,
    StakingHistories,
    StakingHistoriesMap,
    TokenBridgingHistories,
    TokenBridgingHistoriesMap,
    AnchorEvents,
    AnchorEventsMap,
    PermissionlessActionsStatus,
    ValidatorIdsOfEra(u64),
    ValidatorToDelegatorsMapOfEra(u64),
    DelegatorToValidatorsMapOfEra(u64),
    ValidatorsOfEra(u64),
    DelegatorsOfEra(u64),
    UnprofitableValidatorIdsOfEra(u64),
    ValidatorListOfEra(u64),
    ValidatorRewardsOfEra(u64),
    DelegatorRewardsOfEra(u64),
    DelegatorIdsInMapOfVToDOfEra {
        era_number: u64,
        validator_id: AccountId,
    },
    ValidatorIdsInMapOfDToVOfEra {
        era_number: u64,
        delegator_id: AccountId,
    },
}

impl StorageKey {
    pub fn to_string(&self) -> String {
        match self {
            StorageKey::OctToken => "oct".to_string(),
            StorageKey::WrappedAppchainToken => "wat".to_string(),
            StorageKey::NearFungibleTokenSymbols => "fts".to_string(),
            StorageKey::NearFungibleTokens => "ft".to_string(),
            StorageKey::NearFungibleTokensMap => "ftm".to_string(),
            StorageKey::NextValidatorSet => "nvs".to_string(),
            StorageKey::ValidatorSetHistories => "vsh".to_string(),
            StorageKey::ValidatorSetHistoriesMap => "vshm".to_string(),
            StorageKey::UnwithdrawnValidatorRewards => "uwvrs".to_string(),
            StorageKey::UnwithdrawnDelegatorRewards => "uwdrs".to_string(),
            StorageKey::UnbondedStakes => "ubss".to_string(),
            StorageKey::LookupMapOfValidatorIdsInAppchain => "via".to_string(),
            StorageKey::AppchainSettings => "acs".to_string(),
            StorageKey::AnchorSettings => "ans".to_string(),
            StorageKey::ProtocolSettings => "pcs".to_string(),
            StorageKey::StakingHistories => "skh".to_string(),
            StorageKey::StakingHistoriesMap => "skhm".to_string(),
            StorageKey::TokenBridgingHistories => "tbh".to_string(),
            StorageKey::TokenBridgingHistoriesMap => "tbhm".to_string(),
            StorageKey::AnchorEvents => "aes".to_string(),
            StorageKey::AnchorEventsMap => "aesm".to_string(),
            StorageKey::PermissionlessActionsStatus => "pas".to_string(),
            StorageKey::ValidatorIdsOfEra(era_number) => format!("{}vis", era_number),
            StorageKey::ValidatorToDelegatorsMapOfEra(era_number) => format!("{}lmvtd", era_number),
            StorageKey::DelegatorToValidatorsMapOfEra(era_number) => format!("{}lmdtv", era_number),
            StorageKey::ValidatorsOfEra(era_number) => format!("{}vs", era_number),
            StorageKey::DelegatorsOfEra(era_number) => format!("{}ds", era_number),
            StorageKey::UnprofitableValidatorIdsOfEra(era_number) => {
                format!("{}upvis", era_number)
            }
            StorageKey::ValidatorListOfEra(era_number) => format!("{}vl", era_number),
            StorageKey::ValidatorRewardsOfEra(era_number) => format!("{}vrs", era_number),
            StorageKey::DelegatorRewardsOfEra(era_number) => format!("{}drs", era_number),
            StorageKey::DelegatorIdsInMapOfVToDOfEra {
                era_number,
                validator_id,
            } => format!("{}lmvtd{}", era_number, validator_id),
            StorageKey::ValidatorIdsInMapOfDToVOfEra {
                era_number,
                delegator_id,
            } => format!("{}lmdtv{}", era_number, delegator_id),
        }
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}