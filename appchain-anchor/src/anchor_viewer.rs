use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AnchorEvents {
    /// The anchor event data map.
    events: LookupMap<u64, AnchorEvent>,
    /// The start index of valid anchor event.
    start_index: u64,
    /// The end index of valid anchor event.
    end_index: u64,
}

impl AnchorEvents {
    ///
    pub fn new() -> Self {
        Self {
            events: LookupMap::new(StorageKey::AnchorEventsMap.into_bytes()),
            start_index: 0,
            end_index: 0,
        }
    }
    ///
    pub fn get(&self, index: &u64) -> Option<AnchorEvent> {
        self.events.get(index)
    }
    ///
    pub fn index_range(&self) -> IndexRange {
        IndexRange {
            start_index: U64::from(self.start_index),
            end_index: U64::from(self.end_index),
        }
    }
}

pub trait AnchorViewer {
    /// Get anchor settings detail.
    fn get_anchor_settings(&self) -> AnchorSettings;
    /// Get appchain settings detail.
    fn get_appchain_settings(&self) -> AppchainSettings;
    /// Get protocol settings detail.
    fn get_protocol_settings(&self) -> ProtocolSettings;
    /// Get state of corresponding appchain.
    fn get_appchain_state(&self) -> AppchainState;
    /// Get current status of anchor.
    fn get_anchor_status(&self) -> AnchorStatus;
    /// Get validator set history info.
    fn get_validator_set_info_of(&self, era_number: U64) -> Option<ValidatorSetInfo>;
    /// Get processing status of validator set of era.
    fn get_processing_status_of(&self, era_number: U64) -> Option<ValidatorSetProcessingStatus>;
    /// Get the index range of staking histories stored in anchor.
    fn get_index_range_of_staking_history(&self) -> IndexRange;
    /// Get staking history by index.
    /// If the param `index `is omitted, the latest history will be returned.
    /// If the paran `index` is smaller than the start index, or bigger than the end index
    /// stored in anchor, or there is no history in anchor yet, `Option::None` will be returned.
    fn get_staking_history(&self, index: Option<U64>) -> Option<StakingHistory>;
    /// Get the index range of anchor events stored in anchor.
    fn get_index_range_of_anchor_event(&self) -> IndexRange;
    /// Get anchor event by index.
    /// If the param `index `is omitted, the latest event will be returned.
    /// If the paran `index` is smaller than the start index, or bigger than the end index
    /// stored in anchor, or there is no event in anchor yet, `Option::None` will be returned.
    fn get_anchor_event(&self, index: Option<U64>) -> Option<AnchorEvent>;
    /// Get the index range of token bridging histories stored in anchor.
    fn get_index_range_of_token_bridging_history(&self) -> IndexRange;
    /// Get token bridging history by index.
    /// If the param `index `is omitted, the latest history will be returned.
    /// If the paran `index` is smaller than the start index, or bigger than the end index
    /// stored in anchor, or there is no history in anchor yet, `Option::None` will be returned.
    fn get_token_bridging_history(&self, index: Option<U64>) -> Option<TokenBridgingHistory>;
    /// Get the validator list of a certain era.
    fn get_validator_list_of_era(&self, era_number: U64) -> Vec<AppchainValidator>;
    /// Get the delegators of a validator of a certain era.
    /// If the param `era_number` is omitted, the latest validator set will be used.
    fn get_delegators_of_validator_in_era(
        &self,
        era_number: Option<U64>,
        validator_id: AccountId,
    ) -> Vec<AppchainDelegator>;
    /// Get unbonded stakes of an account.
    fn get_unbonded_stakes_of(&self, account_id: AccountId) -> Vec<UnbondedStake>;
    /// Get validator rewards of a certain era range.
    fn get_validator_rewards_of(
        &self,
        start_era: U64,
        end_era: U64,
        validator_id: AccountId,
    ) -> Vec<RewardHistory>;
    /// Get validator rewards of a certain era range.
    fn get_delegator_rewards_of(
        &self,
        start_era: U64,
        end_era: U64,
        delegator_id: AccountId,
        validator_id: AccountId,
    ) -> Vec<RewardHistory>;
}

#[near_bindgen]
impl AnchorViewer for AppchainAnchor {
    //
    fn get_anchor_settings(&self) -> AnchorSettings {
        self.anchor_settings.get().unwrap()
    }
    //
    fn get_appchain_settings(&self) -> AppchainSettings {
        self.appchain_settings.get().unwrap()
    }
    //
    fn get_protocol_settings(&self) -> ProtocolSettings {
        self.protocol_settings.get().unwrap()
    }
    //
    fn get_appchain_state(&self) -> AppchainState {
        self.appchain_state.clone()
    }
    //
    fn get_anchor_status(&self) -> AnchorStatus {
        AnchorStatus {
            total_stake_in_next_era: self.next_validator_set.get().unwrap().total_stake.into(),
            validator_count_in_next_era: self
                .next_validator_set
                .get()
                .unwrap()
                .validator_id_set
                .len()
                .into(),
            index_range_of_anchor_event: self.anchor_events.get().unwrap().index_range(),
            index_range_of_staking_history: self.staking_histories.get().unwrap().index_range(),
            index_range_of_token_bridging_history: self
                .token_bridging_histories
                .get()
                .unwrap()
                .index_range(),
            permissionless_actions_status: self.permissionless_actions_status.get().unwrap(),
        }
    }
    //
    fn get_validator_set_info_of(&self, era_number: U64) -> Option<ValidatorSetInfo> {
        let validator_set_histories = self.validator_set_histories.get().unwrap();
        if validator_set_histories.contains(&era_number.0) {
            let validator_set = validator_set_histories.get(&era_number.0).unwrap();
            Some(validator_set.to_validator_set_info())
        } else {
            None
        }
    }
    //
    fn get_processing_status_of(&self, era_number: U64) -> Option<ValidatorSetProcessingStatus> {
        let validator_set_histories = self.validator_set_histories.get().unwrap();
        if validator_set_histories.contains(&era_number.0) {
            let validator_set = validator_set_histories.get(&era_number.0).unwrap();
            Some(validator_set.processing_status.clone())
        } else {
            None
        }
    }
    //
    fn get_index_range_of_staking_history(&self) -> IndexRange {
        self.staking_histories.get().unwrap().index_range()
    }
    //
    fn get_staking_history(&self, index: Option<U64>) -> Option<StakingHistory> {
        let index = match index {
            Some(index) => index,
            None => {
                self.staking_histories
                    .get()
                    .unwrap()
                    .index_range()
                    .end_index
            }
        };
        self.staking_histories.get().unwrap().get(&index.0)
    }
    //
    fn get_index_range_of_anchor_event(&self) -> IndexRange {
        self.anchor_events.get().unwrap().index_range()
    }
    //
    fn get_anchor_event(&self, index: Option<U64>) -> Option<AnchorEvent> {
        let index = match index {
            Some(index) => index,
            None => self.anchor_events.get().unwrap().index_range().end_index,
        };
        self.anchor_events.get().unwrap().get(&index.0)
    }
    //
    fn get_index_range_of_token_bridging_history(&self) -> IndexRange {
        self.token_bridging_histories.get().unwrap().index_range()
    }
    //
    fn get_token_bridging_history(&self, index: Option<U64>) -> Option<TokenBridgingHistory> {
        let index = match index {
            Some(index) => index,
            None => {
                self.token_bridging_histories
                    .get()
                    .unwrap()
                    .index_range()
                    .end_index
            }
        };
        self.token_bridging_histories.get().unwrap().get(&index.0)
    }
    //
    fn get_validator_list_of_era(&self, era_number: U64) -> Vec<AppchainValidator> {
        match self
            .validator_set_histories
            .get()
            .unwrap()
            .get(&era_number.0)
        {
            Some(validator_set_of_era) => validator_set_of_era.validator_list.to_vec(),
            None => Vec::new(),
        }
    }
    //
    fn get_delegators_of_validator_in_era(
        &self,
        era_number: Option<U64>,
        validator_id: AccountId,
    ) -> Vec<AppchainDelegator> {
        let mut result = Vec::<AppchainDelegator>::new();
        match era_number {
            Some(era_number) => {
                let validator_set_histories = self.validator_set_histories.get().unwrap();
                match validator_set_histories.get(&era_number.0) {
                    Some(validator_set) => {
                        match validator_set
                            .validator_set
                            .validator_id_to_delegator_id_set
                            .get(&validator_id)
                        {
                            Some(delegator_id_set) => {
                                let delegator_ids = delegator_id_set.to_vec();
                                delegator_ids.iter().for_each(|delegator_id| {
                                    let delegator = validator_set
                                        .validator_set
                                        .delegators
                                        .get(&(delegator_id.clone(), validator_id.clone()))
                                        .unwrap();
                                    result.push(AppchainDelegator {
                                        delegator_id: delegator_id.clone(),
                                        delegation_amount: U128::from(delegator.deposit_amount),
                                    });
                                });
                            }
                            None => (),
                        }
                    }
                    None => (),
                }
            }
            None => {
                let next_validator_set = self.next_validator_set.get().unwrap();
                match next_validator_set
                    .validator_id_to_delegator_id_set
                    .get(&validator_id)
                {
                    Some(delegator_id_set) => {
                        let delegator_ids = delegator_id_set.to_vec();
                        delegator_ids.iter().for_each(|delegator_id| {
                            let delegator = next_validator_set
                                .delegators
                                .get(&(delegator_id.clone(), validator_id.clone()))
                                .unwrap();
                            result.push(AppchainDelegator {
                                delegator_id: delegator_id.clone(),
                                delegation_amount: U128::from(delegator.deposit_amount),
                            });
                        });
                    }
                    None => (),
                }
            }
        };
        result
    }
    //
    fn get_unbonded_stakes_of(&self, account_id: AccountId) -> Vec<UnbondedStake> {
        let protocol_settings = self.protocol_settings.get().unwrap();
        let mut result = Vec::<UnbondedStake>::new();
        if let Some(unbonded_stake_references) = self.unbonded_stakes.get(&account_id) {
            unbonded_stake_references.iter().for_each(|reference| {
                let validator_set = self
                    .validator_set_histories
                    .get()
                    .unwrap()
                    .get(&reference.era_number)
                    .unwrap();
                let staking_history = self
                    .staking_histories
                    .get()
                    .unwrap()
                    .get(&reference.staking_history_index)
                    .unwrap();
                match staking_history.staking_fact {
                    StakingFact::StakeDecreased {
                        validator_id,
                        amount,
                    }
                    | StakingFact::ValidatorUnbonded {
                        validator_id,
                        amount,
                    } => result.push(UnbondedStake {
                        era_number: U64::from(reference.era_number),
                        account_id: validator_id,
                        amount,
                        unlock_time: validator_set.start_timestamp
                            + protocol_settings.unlock_period_of_validator_deposit.0
                                * SECONDS_OF_A_DAY
                                * NANO_SECONDS_MULTIPLE,
                    }),
                    StakingFact::DelegationDecreased {
                        delegator_id,
                        validator_id: _,
                        amount,
                    }
                    | StakingFact::DelegatorUnbonded {
                        delegator_id,
                        validator_id: _,
                        amount,
                    } => result.push(UnbondedStake {
                        era_number: U64::from(reference.era_number),
                        account_id: delegator_id,
                        amount,
                        unlock_time: validator_set.start_timestamp
                            + protocol_settings.unlock_period_of_delegator_deposit.0
                                * SECONDS_OF_A_DAY
                                * NANO_SECONDS_MULTIPLE,
                    }),
                    _ => (),
                };
            });
        }
        result
    }
    //
    fn get_validator_rewards_of(
        &self,
        start_era: U64,
        end_era: U64,
        validator_id: AccountId,
    ) -> Vec<RewardHistory> {
        let validator_set_histories = self.validator_set_histories.get().unwrap();
        let mut reward_histories = Vec::<RewardHistory>::new();
        for era_number in start_era.0..end_era.0 + 1 {
            if let Some(validator_set) = validator_set_histories.get(&era_number) {
                if let Some(reward) = validator_set.validator_rewards.get(&validator_id) {
                    reward_histories.push(RewardHistory {
                        era_number: U64::from(era_number),
                        reward: U128::from(reward),
                        is_withdrawn: !self
                            .unwithdrawn_validator_rewards
                            .contains_key(&(era_number, validator_id.clone())),
                    });
                }
            }
        }
        reward_histories
    }
    //
    fn get_delegator_rewards_of(
        &self,
        start_era: U64,
        end_era: U64,
        delegator_id: AccountId,
        validator_id: AccountId,
    ) -> Vec<RewardHistory> {
        let validator_set_histories = self.validator_set_histories.get().unwrap();
        let mut reward_histories = Vec::<RewardHistory>::new();
        for era_number in start_era.0..end_era.0 + 1 {
            if let Some(validator_set) = validator_set_histories.get(&era_number) {
                if let Some(reward) = validator_set
                    .delegator_rewards
                    .get(&(delegator_id.clone(), validator_id.clone()))
                {
                    reward_histories.push(RewardHistory {
                        era_number: U64::from(era_number),
                        reward: U128::from(reward),
                        is_withdrawn: !self.unwithdrawn_delegator_rewards.contains_key(&(
                            era_number,
                            delegator_id.clone(),
                            validator_id.clone(),
                        )),
                    });
                }
            }
        }
        reward_histories
    }
}