#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "NEAR Account Identifier.\n\nThis is a unique, syntactically valid, human-readable account identifier on the NEAR network.\n\n[See the crate-level docs for information about validation.](index.html#account-id-rules)\n\nAlso see [Error kind precedence](AccountId#error-kind-precedence).\n\n## Examples\n\n``` use near_account_id::AccountId;\n\nlet alice: AccountId = \"alice.near\".parse().unwrap();\n\nassert!(\"ƒelicia.near\".parse::<AccountId>().is_err()); // (ƒ is not f) ```"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"NEAR Account Identifier.\\n\\nThis is a unique, syntactically valid, human-readable account identifier on the NEAR network.\\n\\n[See the crate-level docs for information about validation.](index.html#account-id-rules)\\n\\nAlso see [Error kind precedence](AccountId#error-kind-precedence).\\n\\n## Examples\\n\\n``` use near_account_id::AccountId;\\n\\nlet alice: AccountId = \\\"alice.near\\\".parse().unwrap();\\n\\nassert!(\\\"ƒelicia.near\\\".parse::<AccountId>().is_err()); // (ƒ is not f) ```\","]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct AccountId(pub ::std::string::String);
impl ::std::ops::Deref for AccountId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AccountId> for ::std::string::String {
    fn from(value: AccountId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AccountId> for AccountId {
    fn from(value: &AccountId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for AccountId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for AccountId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for AccountId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`BandwidthRequest` describes the size of receipts that a shard would like to send to another shard. When a shard wants to send a lot of receipts to another shard, it needs to create a request and wait for a bandwidth grant from the bandwidth scheduler."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"`BandwidthRequest` describes the size of receipts that a shard would like to send to another shard. When a shard wants to send a lot of receipts to another shard, it needs to create a request and wait for a bandwidth grant from the bandwidth scheduler.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"requested_values_bitmap\","]
#[doc = "    \"to_shard\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"requested_values_bitmap\": {"]
#[doc = "      \"description\": \"Bitmap which describes what values of bandwidth are requested.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/BandwidthRequestBitmap\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"to_shard\": {"]
#[doc = "      \"description\": \"Requesting bandwidth to this shard.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint16\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BandwidthRequest {
    #[doc = "Bitmap which describes what values of bandwidth are requested."]
    pub requested_values_bitmap: BandwidthRequestBitmap,
    #[doc = "Requesting bandwidth to this shard."]
    pub to_shard: u16,
}
impl ::std::convert::From<&BandwidthRequest> for BandwidthRequest {
    fn from(value: &BandwidthRequest) -> Self {
        value.clone()
    }
}
impl BandwidthRequest {
    pub fn builder() -> builder::BandwidthRequest {
        Default::default()
    }
}
#[doc = "Bitmap which describes which values from the predefined list are being requested. The nth bit is set to 1 when the nth value from the list is being requested."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Bitmap which describes which values from the predefined list are being requested. The nth bit is set to 1 when the nth value from the list is being requested.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\","]
#[doc = "        \"format\": \"uint8\","]
#[doc = "        \"minimum\": 0.0"]
#[doc = "      },"]
#[doc = "      \"maxItems\": 5,"]
#[doc = "      \"minItems\": 5"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BandwidthRequestBitmap {
    pub data: [u8; 5usize],
}
impl ::std::convert::From<&BandwidthRequestBitmap> for BandwidthRequestBitmap {
    fn from(value: &BandwidthRequestBitmap) -> Self {
        value.clone()
    }
}
impl BandwidthRequestBitmap {
    pub fn builder() -> builder::BandwidthRequestBitmap {
        Default::default()
    }
}
#[doc = "A list of shard's bandwidth requests. Describes how much the shard would like to send to other shards."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A list of shard's bandwidth requests. Describes how much the shard would like to send to other shards.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"V1\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"V1\": {"]
#[doc = "          \"$ref\": \"#/definitions/BandwidthRequestsV1\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum BandwidthRequests {
    V1(BandwidthRequestsV1),
}
impl ::std::convert::From<&Self> for BandwidthRequests {
    fn from(value: &BandwidthRequests) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<BandwidthRequestsV1> for BandwidthRequests {
    fn from(value: BandwidthRequestsV1) -> Self {
        Self::V1(value)
    }
}
#[doc = "BandwidthRequestsV1"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"requests\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"requests\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/BandwidthRequest\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BandwidthRequestsV1 {
    pub requests: ::std::vec::Vec<BandwidthRequest>,
}
impl ::std::convert::From<&BandwidthRequestsV1> for BandwidthRequestsV1 {
    fn from(value: &BandwidthRequestsV1) -> Self {
        value.clone()
    }
}
impl BandwidthRequestsV1 {
    pub fn builder() -> builder::BandwidthRequestsV1 {
        Default::default()
    }
}
#[doc = "BlockHeaderView"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"approvals\","]
#[doc = "    \"block_merkle_root\","]
#[doc = "    \"challenges_result\","]
#[doc = "    \"challenges_root\","]
#[doc = "    \"chunk_headers_root\","]
#[doc = "    \"chunk_mask\","]
#[doc = "    \"chunk_receipts_root\","]
#[doc = "    \"chunk_tx_root\","]
#[doc = "    \"chunks_included\","]
#[doc = "    \"epoch_id\","]
#[doc = "    \"gas_price\","]
#[doc = "    \"hash\","]
#[doc = "    \"height\","]
#[doc = "    \"last_ds_final_block\","]
#[doc = "    \"last_final_block\","]
#[doc = "    \"latest_protocol_version\","]
#[doc = "    \"next_bp_hash\","]
#[doc = "    \"next_epoch_id\","]
#[doc = "    \"outcome_root\","]
#[doc = "    \"prev_hash\","]
#[doc = "    \"prev_state_root\","]
#[doc = "    \"random_value\","]
#[doc = "    \"rent_paid\","]
#[doc = "    \"signature\","]
#[doc = "    \"timestamp\","]
#[doc = "    \"timestamp_nanosec\","]
#[doc = "    \"total_supply\","]
#[doc = "    \"validator_proposals\","]
#[doc = "    \"validator_reward\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"approvals\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/Signature\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"type\": \"null\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"block_body_hash\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"block_merkle_root\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"block_ordinal\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"challenges_result\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/SlashedValidator\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"challenges_root\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"chunk_endorsements\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"array\","]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"uint8\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"chunk_headers_root\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"chunk_mask\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"boolean\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"chunk_receipts_root\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"chunk_tx_root\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"chunks_included\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"epoch_id\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"epoch_sync_data_hash\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"gas_price\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"hash\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"height\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"last_ds_final_block\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"last_final_block\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"latest_protocol_version\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"next_bp_hash\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"next_epoch_id\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"outcome_root\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"prev_hash\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"prev_height\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"prev_state_root\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"random_value\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"rent_paid\": {"]
#[doc = "      \"description\": \"TODO(2271): deprecated.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"signature\": {"]
#[doc = "      \"$ref\": \"#/definitions/Signature\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"Legacy json number. Should not be used.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"timestamp_nanosec\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"total_supply\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"validator_proposals\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ValidatorStakeView\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"validator_reward\": {"]
#[doc = "      \"description\": \"TODO(2271): deprecated.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BlockHeaderView {
    pub approvals: ::std::vec::Vec<::std::option::Option<Signature>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_body_hash: ::std::option::Option<CryptoHash>,
    pub block_merkle_root: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_ordinal: ::std::option::Option<u64>,
    pub challenges_result: ::std::vec::Vec<SlashedValidator>,
    pub challenges_root: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chunk_endorsements: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<u8>>>,
    pub chunk_headers_root: CryptoHash,
    pub chunk_mask: ::std::vec::Vec<bool>,
    pub chunk_receipts_root: CryptoHash,
    pub chunk_tx_root: CryptoHash,
    pub chunks_included: u64,
    pub epoch_id: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub epoch_sync_data_hash: ::std::option::Option<CryptoHash>,
    pub gas_price: ::std::string::String,
    pub hash: CryptoHash,
    pub height: u64,
    pub last_ds_final_block: CryptoHash,
    pub last_final_block: CryptoHash,
    pub latest_protocol_version: u32,
    pub next_bp_hash: CryptoHash,
    pub next_epoch_id: CryptoHash,
    pub outcome_root: CryptoHash,
    pub prev_hash: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub prev_height: ::std::option::Option<u64>,
    pub prev_state_root: CryptoHash,
    pub random_value: CryptoHash,
    #[doc = "TODO(2271): deprecated."]
    pub rent_paid: ::std::string::String,
    pub signature: Signature,
    #[doc = "Legacy json number. Should not be used."]
    pub timestamp: u64,
    pub timestamp_nanosec: ::std::string::String,
    pub total_supply: ::std::string::String,
    pub validator_proposals: ::std::vec::Vec<ValidatorStakeView>,
    #[doc = "TODO(2271): deprecated."]
    pub validator_reward: ::std::string::String,
}
impl ::std::convert::From<&BlockHeaderView> for BlockHeaderView {
    fn from(value: &BlockHeaderView) -> Self {
        value.clone()
    }
}
impl BlockHeaderView {
    pub fn builder() -> builder::BlockHeaderView {
        Default::default()
    }
}
#[doc = "ChunkHeaderView"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"balance_burnt\","]
#[doc = "    \"chunk_hash\","]
#[doc = "    \"encoded_length\","]
#[doc = "    \"encoded_merkle_root\","]
#[doc = "    \"gas_limit\","]
#[doc = "    \"gas_used\","]
#[doc = "    \"height_created\","]
#[doc = "    \"height_included\","]
#[doc = "    \"outcome_root\","]
#[doc = "    \"outgoing_receipts_root\","]
#[doc = "    \"prev_block_hash\","]
#[doc = "    \"prev_state_root\","]
#[doc = "    \"rent_paid\","]
#[doc = "    \"shard_id\","]
#[doc = "    \"signature\","]
#[doc = "    \"tx_root\","]
#[doc = "    \"validator_proposals\","]
#[doc = "    \"validator_reward\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"balance_burnt\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"bandwidth_requests\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/BandwidthRequests\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"chunk_hash\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"congestion_info\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/CongestionInfoView\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"encoded_length\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"encoded_merkle_root\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"gas_limit\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"gas_used\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"height_created\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"height_included\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"outcome_root\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"outgoing_receipts_root\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"prev_block_hash\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"prev_state_root\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"rent_paid\": {"]
#[doc = "      \"description\": \"TODO(2271): deprecated.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"shard_id\": {"]
#[doc = "      \"$ref\": \"#/definitions/ShardId\""]
#[doc = "    },"]
#[doc = "    \"signature\": {"]
#[doc = "      \"$ref\": \"#/definitions/Signature\""]
#[doc = "    },"]
#[doc = "    \"tx_root\": {"]
#[doc = "      \"$ref\": \"#/definitions/CryptoHash\""]
#[doc = "    },"]
#[doc = "    \"validator_proposals\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ValidatorStakeView\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"validator_reward\": {"]
#[doc = "      \"description\": \"TODO(2271): deprecated.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ChunkHeaderView {
    pub balance_burnt: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bandwidth_requests: ::std::option::Option<BandwidthRequests>,
    pub chunk_hash: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub congestion_info: ::std::option::Option<CongestionInfoView>,
    pub encoded_length: u64,
    pub encoded_merkle_root: CryptoHash,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub height_created: u64,
    pub height_included: u64,
    pub outcome_root: CryptoHash,
    pub outgoing_receipts_root: CryptoHash,
    pub prev_block_hash: CryptoHash,
    pub prev_state_root: CryptoHash,
    #[doc = "TODO(2271): deprecated."]
    pub rent_paid: ::std::string::String,
    pub shard_id: ShardId,
    pub signature: Signature,
    pub tx_root: CryptoHash,
    pub validator_proposals: ::std::vec::Vec<ValidatorStakeView>,
    #[doc = "TODO(2271): deprecated."]
    pub validator_reward: ::std::string::String,
}
impl ::std::convert::From<&ChunkHeaderView> for ChunkHeaderView {
    fn from(value: &ChunkHeaderView) -> Self {
        value.clone()
    }
}
impl ChunkHeaderView {
    pub fn builder() -> builder::ChunkHeaderView {
        Default::default()
    }
}
#[doc = "CongestionInfoView"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"allowed_shard\","]
#[doc = "    \"buffered_receipts_gas\","]
#[doc = "    \"delayed_receipts_gas\","]
#[doc = "    \"receipt_bytes\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"allowed_shard\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint16\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"buffered_receipts_gas\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"delayed_receipts_gas\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"receipt_bytes\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CongestionInfoView {
    pub allowed_shard: u16,
    pub buffered_receipts_gas: ::std::string::String,
    pub delayed_receipts_gas: ::std::string::String,
    pub receipt_bytes: u64,
}
impl ::std::convert::From<&CongestionInfoView> for CongestionInfoView {
    fn from(value: &CongestionInfoView) -> Self {
        value.clone()
    }
}
impl CongestionInfoView {
    pub fn builder() -> builder::CongestionInfoView {
        Default::default()
    }
}
#[doc = "CryptoHash"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct CryptoHash(pub ::std::string::String);
impl ::std::ops::Deref for CryptoHash {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CryptoHash> for ::std::string::String {
    fn from(value: CryptoHash) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CryptoHash> for CryptoHash {
    fn from(value: &CryptoHash) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for CryptoHash {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for CryptoHash {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for CryptoHash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "PublicKey"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct PublicKey(pub ::std::string::String);
impl ::std::ops::Deref for PublicKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PublicKey> for ::std::string::String {
    fn from(value: PublicKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PublicKey> for PublicKey {
    fn from(value: &PublicKey) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for PublicKey {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for PublicKey {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "RpcBlockResponse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"RpcBlockResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"author\","]
#[doc = "    \"chunks\","]
#[doc = "    \"header\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"author\": {"]
#[doc = "      \"$ref\": \"#/definitions/AccountId\""]
#[doc = "    },"]
#[doc = "    \"chunks\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ChunkHeaderView\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"header\": {"]
#[doc = "      \"$ref\": \"#/definitions/BlockHeaderView\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RpcBlockResponse {
    pub author: AccountId,
    pub chunks: ::std::vec::Vec<ChunkHeaderView>,
    pub header: BlockHeaderView,
}
impl ::std::convert::From<&RpcBlockResponse> for RpcBlockResponse {
    fn from(value: &RpcBlockResponse) -> Self {
        value.clone()
    }
}
impl RpcBlockResponse {
    pub fn builder() -> builder::RpcBlockResponse {
        Default::default()
    }
}
#[doc = "The shard identifier. It may be an arbitrary number - it does not need to be a number in the range 0..NUM_SHARDS. The shard ids do not need to be sequential or contiguous.\n\nThe shard id is wrapped in a new type to prevent the old pattern of using indices in range 0..NUM_SHARDS and casting to ShardId. Once the transition if fully complete it potentially may be simplified to a regular type alias."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The shard identifier. It may be an arbitrary number - it does not need to be a number in the range 0..NUM_SHARDS. The shard ids do not need to be sequential or contiguous.\\n\\nThe shard id is wrapped in a new type to prevent the old pattern of using indices in range 0..NUM_SHARDS and casting to ShardId. Once the transition if fully complete it potentially may be simplified to a regular type alias.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"uint64\","]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ShardId(pub u64);
impl ::std::ops::Deref for ShardId {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}
impl ::std::convert::From<ShardId> for u64 {
    fn from(value: ShardId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ShardId> for ShardId {
    fn from(value: &ShardId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<u64> for ShardId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for ShardId {
    type Err = <u64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for ShardId {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for ShardId {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ShardId {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for ShardId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "Signature"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct Signature(pub ::std::string::String);
impl ::std::ops::Deref for Signature {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Signature> for ::std::string::String {
    fn from(value: Signature) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Signature> for Signature {
    fn from(value: &Signature) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for Signature {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Signature {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Signature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "SlashedValidator"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account_id\","]
#[doc = "    \"is_double_sign\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account_id\": {"]
#[doc = "      \"$ref\": \"#/definitions/AccountId\""]
#[doc = "    },"]
#[doc = "    \"is_double_sign\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SlashedValidator {
    pub account_id: AccountId,
    pub is_double_sign: bool,
}
impl ::std::convert::From<&SlashedValidator> for SlashedValidator {
    fn from(value: &SlashedValidator) -> Self {
        value.clone()
    }
}
impl SlashedValidator {
    pub fn builder() -> builder::SlashedValidator {
        Default::default()
    }
}
#[doc = "ValidatorStakeView"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account_id\","]
#[doc = "        \"public_key\","]
#[doc = "        \"stake\","]
#[doc = "        \"validator_stake_struct_version\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account_id\": {"]
#[doc = "          \"$ref\": \"#/definitions/AccountId\""]
#[doc = "        },"]
#[doc = "        \"public_key\": {"]
#[doc = "          \"$ref\": \"#/definitions/PublicKey\""]
#[doc = "        },"]
#[doc = "        \"stake\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"validator_stake_struct_version\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"V1\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "validator_stake_struct_version")]
pub enum ValidatorStakeView {
    V1 {
        account_id: AccountId,
        public_key: PublicKey,
        stake: ::std::string::String,
    },
}
impl ::std::convert::From<&Self> for ValidatorStakeView {
    fn from(value: &ValidatorStakeView) -> Self {
        value.clone()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BandwidthRequest {
        requested_values_bitmap:
            ::std::result::Result<super::BandwidthRequestBitmap, ::std::string::String>,
        to_shard: ::std::result::Result<u16, ::std::string::String>,
    }
    impl ::std::default::Default for BandwidthRequest {
        fn default() -> Self {
            Self {
                requested_values_bitmap: Err(
                    "no value supplied for requested_values_bitmap".to_string()
                ),
                to_shard: Err("no value supplied for to_shard".to_string()),
            }
        }
    }
    impl BandwidthRequest {
        pub fn requested_values_bitmap<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BandwidthRequestBitmap>,
            T::Error: ::std::fmt::Display,
        {
            self.requested_values_bitmap = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for requested_values_bitmap: {}",
                    e
                )
            });
            self
        }
        pub fn to_shard<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u16>,
            T::Error: ::std::fmt::Display,
        {
            self.to_shard = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for to_shard: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BandwidthRequest> for super::BandwidthRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BandwidthRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                requested_values_bitmap: value.requested_values_bitmap?,
                to_shard: value.to_shard?,
            })
        }
    }
    impl ::std::convert::From<super::BandwidthRequest> for BandwidthRequest {
        fn from(value: super::BandwidthRequest) -> Self {
            Self {
                requested_values_bitmap: Ok(value.requested_values_bitmap),
                to_shard: Ok(value.to_shard),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BandwidthRequestBitmap {
        data: ::std::result::Result<[u8; 5usize], ::std::string::String>,
    }
    impl ::std::default::Default for BandwidthRequestBitmap {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
            }
        }
    }
    impl BandwidthRequestBitmap {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<[u8; 5usize]>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BandwidthRequestBitmap> for super::BandwidthRequestBitmap {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BandwidthRequestBitmap,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { data: value.data? })
        }
    }
    impl ::std::convert::From<super::BandwidthRequestBitmap> for BandwidthRequestBitmap {
        fn from(value: super::BandwidthRequestBitmap) -> Self {
            Self {
                data: Ok(value.data),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BandwidthRequestsV1 {
        requests:
            ::std::result::Result<::std::vec::Vec<super::BandwidthRequest>, ::std::string::String>,
    }
    impl ::std::default::Default for BandwidthRequestsV1 {
        fn default() -> Self {
            Self {
                requests: Err("no value supplied for requests".to_string()),
            }
        }
    }
    impl BandwidthRequestsV1 {
        pub fn requests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::BandwidthRequest>>,
            T::Error: ::std::fmt::Display,
        {
            self.requests = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for requests: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BandwidthRequestsV1> for super::BandwidthRequestsV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BandwidthRequestsV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                requests: value.requests?,
            })
        }
    }
    impl ::std::convert::From<super::BandwidthRequestsV1> for BandwidthRequestsV1 {
        fn from(value: super::BandwidthRequestsV1) -> Self {
            Self {
                requests: Ok(value.requests),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BlockHeaderView {
        approvals: ::std::result::Result<
            ::std::vec::Vec<::std::option::Option<super::Signature>>,
            ::std::string::String,
        >,
        block_body_hash:
            ::std::result::Result<::std::option::Option<super::CryptoHash>, ::std::string::String>,
        block_merkle_root: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        block_ordinal: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        challenges_result:
            ::std::result::Result<::std::vec::Vec<super::SlashedValidator>, ::std::string::String>,
        challenges_root: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        chunk_endorsements: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::vec::Vec<u8>>>,
            ::std::string::String,
        >,
        chunk_headers_root: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        chunk_mask: ::std::result::Result<::std::vec::Vec<bool>, ::std::string::String>,
        chunk_receipts_root: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        chunk_tx_root: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        chunks_included: ::std::result::Result<u64, ::std::string::String>,
        epoch_id: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        epoch_sync_data_hash:
            ::std::result::Result<::std::option::Option<super::CryptoHash>, ::std::string::String>,
        gas_price: ::std::result::Result<::std::string::String, ::std::string::String>,
        hash: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        height: ::std::result::Result<u64, ::std::string::String>,
        last_ds_final_block: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        last_final_block: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        latest_protocol_version: ::std::result::Result<u32, ::std::string::String>,
        next_bp_hash: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        next_epoch_id: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        outcome_root: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        prev_hash: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        prev_height: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        prev_state_root: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        random_value: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        rent_paid: ::std::result::Result<::std::string::String, ::std::string::String>,
        signature: ::std::result::Result<super::Signature, ::std::string::String>,
        timestamp: ::std::result::Result<u64, ::std::string::String>,
        timestamp_nanosec: ::std::result::Result<::std::string::String, ::std::string::String>,
        total_supply: ::std::result::Result<::std::string::String, ::std::string::String>,
        validator_proposals: ::std::result::Result<
            ::std::vec::Vec<super::ValidatorStakeView>,
            ::std::string::String,
        >,
        validator_reward: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BlockHeaderView {
        fn default() -> Self {
            Self {
                approvals: Err("no value supplied for approvals".to_string()),
                block_body_hash: Ok(Default::default()),
                block_merkle_root: Err("no value supplied for block_merkle_root".to_string()),
                block_ordinal: Ok(Default::default()),
                challenges_result: Err("no value supplied for challenges_result".to_string()),
                challenges_root: Err("no value supplied for challenges_root".to_string()),
                chunk_endorsements: Ok(Default::default()),
                chunk_headers_root: Err("no value supplied for chunk_headers_root".to_string()),
                chunk_mask: Err("no value supplied for chunk_mask".to_string()),
                chunk_receipts_root: Err("no value supplied for chunk_receipts_root".to_string()),
                chunk_tx_root: Err("no value supplied for chunk_tx_root".to_string()),
                chunks_included: Err("no value supplied for chunks_included".to_string()),
                epoch_id: Err("no value supplied for epoch_id".to_string()),
                epoch_sync_data_hash: Ok(Default::default()),
                gas_price: Err("no value supplied for gas_price".to_string()),
                hash: Err("no value supplied for hash".to_string()),
                height: Err("no value supplied for height".to_string()),
                last_ds_final_block: Err("no value supplied for last_ds_final_block".to_string()),
                last_final_block: Err("no value supplied for last_final_block".to_string()),
                latest_protocol_version: Err(
                    "no value supplied for latest_protocol_version".to_string()
                ),
                next_bp_hash: Err("no value supplied for next_bp_hash".to_string()),
                next_epoch_id: Err("no value supplied for next_epoch_id".to_string()),
                outcome_root: Err("no value supplied for outcome_root".to_string()),
                prev_hash: Err("no value supplied for prev_hash".to_string()),
                prev_height: Ok(Default::default()),
                prev_state_root: Err("no value supplied for prev_state_root".to_string()),
                random_value: Err("no value supplied for random_value".to_string()),
                rent_paid: Err("no value supplied for rent_paid".to_string()),
                signature: Err("no value supplied for signature".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                timestamp_nanosec: Err("no value supplied for timestamp_nanosec".to_string()),
                total_supply: Err("no value supplied for total_supply".to_string()),
                validator_proposals: Err("no value supplied for validator_proposals".to_string()),
                validator_reward: Err("no value supplied for validator_reward".to_string()),
            }
        }
    }
    impl BlockHeaderView {
        pub fn approvals<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::option::Option<super::Signature>>>,
            T::Error: ::std::fmt::Display,
        {
            self.approvals = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for approvals: {}", e));
            self
        }
        pub fn block_body_hash<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CryptoHash>>,
            T::Error: ::std::fmt::Display,
        {
            self.block_body_hash = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for block_body_hash: {}", e));
            self
        }
        pub fn block_merkle_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.block_merkle_root = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for block_merkle_root: {}",
                    e
                )
            });
            self
        }
        pub fn block_ordinal<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.block_ordinal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for block_ordinal: {}", e));
            self
        }
        pub fn challenges_result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SlashedValidator>>,
            T::Error: ::std::fmt::Display,
        {
            self.challenges_result = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for challenges_result: {}",
                    e
                )
            });
            self
        }
        pub fn challenges_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.challenges_root = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for challenges_root: {}", e));
            self
        }
        pub fn chunk_endorsements<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<::std::vec::Vec<u8>>>>,
            T::Error: ::std::fmt::Display,
        {
            self.chunk_endorsements = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for chunk_endorsements: {}",
                    e
                )
            });
            self
        }
        pub fn chunk_headers_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.chunk_headers_root = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for chunk_headers_root: {}",
                    e
                )
            });
            self
        }
        pub fn chunk_mask<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.chunk_mask = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chunk_mask: {}", e));
            self
        }
        pub fn chunk_receipts_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.chunk_receipts_root = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for chunk_receipts_root: {}",
                    e
                )
            });
            self
        }
        pub fn chunk_tx_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.chunk_tx_root = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chunk_tx_root: {}", e));
            self
        }
        pub fn chunks_included<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.chunks_included = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chunks_included: {}", e));
            self
        }
        pub fn epoch_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.epoch_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for epoch_id: {}", e));
            self
        }
        pub fn epoch_sync_data_hash<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CryptoHash>>,
            T::Error: ::std::fmt::Display,
        {
            self.epoch_sync_data_hash = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for epoch_sync_data_hash: {}",
                    e
                )
            });
            self
        }
        pub fn gas_price<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.gas_price = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gas_price: {}", e));
            self
        }
        pub fn hash<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.hash = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hash: {}", e));
            self
        }
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height: {}", e));
            self
        }
        pub fn last_ds_final_block<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.last_ds_final_block = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for last_ds_final_block: {}",
                    e
                )
            });
            self
        }
        pub fn last_final_block<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.last_final_block = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for last_final_block: {}",
                    e
                )
            });
            self
        }
        pub fn latest_protocol_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u32>,
            T::Error: ::std::fmt::Display,
        {
            self.latest_protocol_version = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for latest_protocol_version: {}",
                    e
                )
            });
            self
        }
        pub fn next_bp_hash<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.next_bp_hash = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_bp_hash: {}", e));
            self
        }
        pub fn next_epoch_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.next_epoch_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_epoch_id: {}", e));
            self
        }
        pub fn outcome_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.outcome_root = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for outcome_root: {}", e));
            self
        }
        pub fn prev_hash<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.prev_hash = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prev_hash: {}", e));
            self
        }
        pub fn prev_height<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.prev_height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prev_height: {}", e));
            self
        }
        pub fn prev_state_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.prev_state_root = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prev_state_root: {}", e));
            self
        }
        pub fn random_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.random_value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for random_value: {}", e));
            self
        }
        pub fn rent_paid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.rent_paid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rent_paid: {}", e));
            self
        }
        pub fn signature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Signature>,
            T::Error: ::std::fmt::Display,
        {
            self.signature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for signature: {}", e));
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp: {}", e));
            self
        }
        pub fn timestamp_nanosec<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp_nanosec = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for timestamp_nanosec: {}",
                    e
                )
            });
            self
        }
        pub fn total_supply<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.total_supply = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total_supply: {}", e));
            self
        }
        pub fn validator_proposals<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ValidatorStakeView>>,
            T::Error: ::std::fmt::Display,
        {
            self.validator_proposals = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for validator_proposals: {}",
                    e
                )
            });
            self
        }
        pub fn validator_reward<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.validator_reward = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for validator_reward: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<BlockHeaderView> for super::BlockHeaderView {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BlockHeaderView,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                approvals: value.approvals?,
                block_body_hash: value.block_body_hash?,
                block_merkle_root: value.block_merkle_root?,
                block_ordinal: value.block_ordinal?,
                challenges_result: value.challenges_result?,
                challenges_root: value.challenges_root?,
                chunk_endorsements: value.chunk_endorsements?,
                chunk_headers_root: value.chunk_headers_root?,
                chunk_mask: value.chunk_mask?,
                chunk_receipts_root: value.chunk_receipts_root?,
                chunk_tx_root: value.chunk_tx_root?,
                chunks_included: value.chunks_included?,
                epoch_id: value.epoch_id?,
                epoch_sync_data_hash: value.epoch_sync_data_hash?,
                gas_price: value.gas_price?,
                hash: value.hash?,
                height: value.height?,
                last_ds_final_block: value.last_ds_final_block?,
                last_final_block: value.last_final_block?,
                latest_protocol_version: value.latest_protocol_version?,
                next_bp_hash: value.next_bp_hash?,
                next_epoch_id: value.next_epoch_id?,
                outcome_root: value.outcome_root?,
                prev_hash: value.prev_hash?,
                prev_height: value.prev_height?,
                prev_state_root: value.prev_state_root?,
                random_value: value.random_value?,
                rent_paid: value.rent_paid?,
                signature: value.signature?,
                timestamp: value.timestamp?,
                timestamp_nanosec: value.timestamp_nanosec?,
                total_supply: value.total_supply?,
                validator_proposals: value.validator_proposals?,
                validator_reward: value.validator_reward?,
            })
        }
    }
    impl ::std::convert::From<super::BlockHeaderView> for BlockHeaderView {
        fn from(value: super::BlockHeaderView) -> Self {
            Self {
                approvals: Ok(value.approvals),
                block_body_hash: Ok(value.block_body_hash),
                block_merkle_root: Ok(value.block_merkle_root),
                block_ordinal: Ok(value.block_ordinal),
                challenges_result: Ok(value.challenges_result),
                challenges_root: Ok(value.challenges_root),
                chunk_endorsements: Ok(value.chunk_endorsements),
                chunk_headers_root: Ok(value.chunk_headers_root),
                chunk_mask: Ok(value.chunk_mask),
                chunk_receipts_root: Ok(value.chunk_receipts_root),
                chunk_tx_root: Ok(value.chunk_tx_root),
                chunks_included: Ok(value.chunks_included),
                epoch_id: Ok(value.epoch_id),
                epoch_sync_data_hash: Ok(value.epoch_sync_data_hash),
                gas_price: Ok(value.gas_price),
                hash: Ok(value.hash),
                height: Ok(value.height),
                last_ds_final_block: Ok(value.last_ds_final_block),
                last_final_block: Ok(value.last_final_block),
                latest_protocol_version: Ok(value.latest_protocol_version),
                next_bp_hash: Ok(value.next_bp_hash),
                next_epoch_id: Ok(value.next_epoch_id),
                outcome_root: Ok(value.outcome_root),
                prev_hash: Ok(value.prev_hash),
                prev_height: Ok(value.prev_height),
                prev_state_root: Ok(value.prev_state_root),
                random_value: Ok(value.random_value),
                rent_paid: Ok(value.rent_paid),
                signature: Ok(value.signature),
                timestamp: Ok(value.timestamp),
                timestamp_nanosec: Ok(value.timestamp_nanosec),
                total_supply: Ok(value.total_supply),
                validator_proposals: Ok(value.validator_proposals),
                validator_reward: Ok(value.validator_reward),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChunkHeaderView {
        balance_burnt: ::std::result::Result<::std::string::String, ::std::string::String>,
        bandwidth_requests: ::std::result::Result<
            ::std::option::Option<super::BandwidthRequests>,
            ::std::string::String,
        >,
        chunk_hash: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        congestion_info: ::std::result::Result<
            ::std::option::Option<super::CongestionInfoView>,
            ::std::string::String,
        >,
        encoded_length: ::std::result::Result<u64, ::std::string::String>,
        encoded_merkle_root: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        gas_limit: ::std::result::Result<u64, ::std::string::String>,
        gas_used: ::std::result::Result<u64, ::std::string::String>,
        height_created: ::std::result::Result<u64, ::std::string::String>,
        height_included: ::std::result::Result<u64, ::std::string::String>,
        outcome_root: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        outgoing_receipts_root: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        prev_block_hash: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        prev_state_root: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        rent_paid: ::std::result::Result<::std::string::String, ::std::string::String>,
        shard_id: ::std::result::Result<super::ShardId, ::std::string::String>,
        signature: ::std::result::Result<super::Signature, ::std::string::String>,
        tx_root: ::std::result::Result<super::CryptoHash, ::std::string::String>,
        validator_proposals: ::std::result::Result<
            ::std::vec::Vec<super::ValidatorStakeView>,
            ::std::string::String,
        >,
        validator_reward: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChunkHeaderView {
        fn default() -> Self {
            Self {
                balance_burnt: Err("no value supplied for balance_burnt".to_string()),
                bandwidth_requests: Ok(Default::default()),
                chunk_hash: Err("no value supplied for chunk_hash".to_string()),
                congestion_info: Ok(Default::default()),
                encoded_length: Err("no value supplied for encoded_length".to_string()),
                encoded_merkle_root: Err("no value supplied for encoded_merkle_root".to_string()),
                gas_limit: Err("no value supplied for gas_limit".to_string()),
                gas_used: Err("no value supplied for gas_used".to_string()),
                height_created: Err("no value supplied for height_created".to_string()),
                height_included: Err("no value supplied for height_included".to_string()),
                outcome_root: Err("no value supplied for outcome_root".to_string()),
                outgoing_receipts_root: Err(
                    "no value supplied for outgoing_receipts_root".to_string()
                ),
                prev_block_hash: Err("no value supplied for prev_block_hash".to_string()),
                prev_state_root: Err("no value supplied for prev_state_root".to_string()),
                rent_paid: Err("no value supplied for rent_paid".to_string()),
                shard_id: Err("no value supplied for shard_id".to_string()),
                signature: Err("no value supplied for signature".to_string()),
                tx_root: Err("no value supplied for tx_root".to_string()),
                validator_proposals: Err("no value supplied for validator_proposals".to_string()),
                validator_reward: Err("no value supplied for validator_reward".to_string()),
            }
        }
    }
    impl ChunkHeaderView {
        pub fn balance_burnt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.balance_burnt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for balance_burnt: {}", e));
            self
        }
        pub fn bandwidth_requests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BandwidthRequests>>,
            T::Error: ::std::fmt::Display,
        {
            self.bandwidth_requests = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for bandwidth_requests: {}",
                    e
                )
            });
            self
        }
        pub fn chunk_hash<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.chunk_hash = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chunk_hash: {}", e));
            self
        }
        pub fn congestion_info<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CongestionInfoView>>,
            T::Error: ::std::fmt::Display,
        {
            self.congestion_info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for congestion_info: {}", e));
            self
        }
        pub fn encoded_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.encoded_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for encoded_length: {}", e));
            self
        }
        pub fn encoded_merkle_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.encoded_merkle_root = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for encoded_merkle_root: {}",
                    e
                )
            });
            self
        }
        pub fn gas_limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.gas_limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gas_limit: {}", e));
            self
        }
        pub fn gas_used<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.gas_used = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gas_used: {}", e));
            self
        }
        pub fn height_created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.height_created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height_created: {}", e));
            self
        }
        pub fn height_included<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.height_included = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height_included: {}", e));
            self
        }
        pub fn outcome_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.outcome_root = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for outcome_root: {}", e));
            self
        }
        pub fn outgoing_receipts_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.outgoing_receipts_root = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for outgoing_receipts_root: {}",
                    e
                )
            });
            self
        }
        pub fn prev_block_hash<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.prev_block_hash = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prev_block_hash: {}", e));
            self
        }
        pub fn prev_state_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.prev_state_root = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prev_state_root: {}", e));
            self
        }
        pub fn rent_paid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.rent_paid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rent_paid: {}", e));
            self
        }
        pub fn shard_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ShardId>,
            T::Error: ::std::fmt::Display,
        {
            self.shard_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shard_id: {}", e));
            self
        }
        pub fn signature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Signature>,
            T::Error: ::std::fmt::Display,
        {
            self.signature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for signature: {}", e));
            self
        }
        pub fn tx_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CryptoHash>,
            T::Error: ::std::fmt::Display,
        {
            self.tx_root = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tx_root: {}", e));
            self
        }
        pub fn validator_proposals<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ValidatorStakeView>>,
            T::Error: ::std::fmt::Display,
        {
            self.validator_proposals = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for validator_proposals: {}",
                    e
                )
            });
            self
        }
        pub fn validator_reward<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.validator_reward = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for validator_reward: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ChunkHeaderView> for super::ChunkHeaderView {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChunkHeaderView,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                balance_burnt: value.balance_burnt?,
                bandwidth_requests: value.bandwidth_requests?,
                chunk_hash: value.chunk_hash?,
                congestion_info: value.congestion_info?,
                encoded_length: value.encoded_length?,
                encoded_merkle_root: value.encoded_merkle_root?,
                gas_limit: value.gas_limit?,
                gas_used: value.gas_used?,
                height_created: value.height_created?,
                height_included: value.height_included?,
                outcome_root: value.outcome_root?,
                outgoing_receipts_root: value.outgoing_receipts_root?,
                prev_block_hash: value.prev_block_hash?,
                prev_state_root: value.prev_state_root?,
                rent_paid: value.rent_paid?,
                shard_id: value.shard_id?,
                signature: value.signature?,
                tx_root: value.tx_root?,
                validator_proposals: value.validator_proposals?,
                validator_reward: value.validator_reward?,
            })
        }
    }
    impl ::std::convert::From<super::ChunkHeaderView> for ChunkHeaderView {
        fn from(value: super::ChunkHeaderView) -> Self {
            Self {
                balance_burnt: Ok(value.balance_burnt),
                bandwidth_requests: Ok(value.bandwidth_requests),
                chunk_hash: Ok(value.chunk_hash),
                congestion_info: Ok(value.congestion_info),
                encoded_length: Ok(value.encoded_length),
                encoded_merkle_root: Ok(value.encoded_merkle_root),
                gas_limit: Ok(value.gas_limit),
                gas_used: Ok(value.gas_used),
                height_created: Ok(value.height_created),
                height_included: Ok(value.height_included),
                outcome_root: Ok(value.outcome_root),
                outgoing_receipts_root: Ok(value.outgoing_receipts_root),
                prev_block_hash: Ok(value.prev_block_hash),
                prev_state_root: Ok(value.prev_state_root),
                rent_paid: Ok(value.rent_paid),
                shard_id: Ok(value.shard_id),
                signature: Ok(value.signature),
                tx_root: Ok(value.tx_root),
                validator_proposals: Ok(value.validator_proposals),
                validator_reward: Ok(value.validator_reward),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CongestionInfoView {
        allowed_shard: ::std::result::Result<u16, ::std::string::String>,
        buffered_receipts_gas: ::std::result::Result<::std::string::String, ::std::string::String>,
        delayed_receipts_gas: ::std::result::Result<::std::string::String, ::std::string::String>,
        receipt_bytes: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for CongestionInfoView {
        fn default() -> Self {
            Self {
                allowed_shard: Err("no value supplied for allowed_shard".to_string()),
                buffered_receipts_gas: Err(
                    "no value supplied for buffered_receipts_gas".to_string()
                ),
                delayed_receipts_gas: Err("no value supplied for delayed_receipts_gas".to_string()),
                receipt_bytes: Err("no value supplied for receipt_bytes".to_string()),
            }
        }
    }
    impl CongestionInfoView {
        pub fn allowed_shard<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u16>,
            T::Error: ::std::fmt::Display,
        {
            self.allowed_shard = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allowed_shard: {}", e));
            self
        }
        pub fn buffered_receipts_gas<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.buffered_receipts_gas = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for buffered_receipts_gas: {}",
                    e
                )
            });
            self
        }
        pub fn delayed_receipts_gas<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.delayed_receipts_gas = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delayed_receipts_gas: {}",
                    e
                )
            });
            self
        }
        pub fn receipt_bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.receipt_bytes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for receipt_bytes: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CongestionInfoView> for super::CongestionInfoView {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CongestionInfoView,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_shard: value.allowed_shard?,
                buffered_receipts_gas: value.buffered_receipts_gas?,
                delayed_receipts_gas: value.delayed_receipts_gas?,
                receipt_bytes: value.receipt_bytes?,
            })
        }
    }
    impl ::std::convert::From<super::CongestionInfoView> for CongestionInfoView {
        fn from(value: super::CongestionInfoView) -> Self {
            Self {
                allowed_shard: Ok(value.allowed_shard),
                buffered_receipts_gas: Ok(value.buffered_receipts_gas),
                delayed_receipts_gas: Ok(value.delayed_receipts_gas),
                receipt_bytes: Ok(value.receipt_bytes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RpcBlockResponse {
        author: ::std::result::Result<super::AccountId, ::std::string::String>,
        chunks:
            ::std::result::Result<::std::vec::Vec<super::ChunkHeaderView>, ::std::string::String>,
        header: ::std::result::Result<super::BlockHeaderView, ::std::string::String>,
    }
    impl ::std::default::Default for RpcBlockResponse {
        fn default() -> Self {
            Self {
                author: Err("no value supplied for author".to_string()),
                chunks: Err("no value supplied for chunks".to_string()),
                header: Err("no value supplied for header".to_string()),
            }
        }
    }
    impl RpcBlockResponse {
        pub fn author<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AccountId>,
            T::Error: ::std::fmt::Display,
        {
            self.author = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for author: {}", e));
            self
        }
        pub fn chunks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ChunkHeaderView>>,
            T::Error: ::std::fmt::Display,
        {
            self.chunks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chunks: {}", e));
            self
        }
        pub fn header<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BlockHeaderView>,
            T::Error: ::std::fmt::Display,
        {
            self.header = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for header: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<RpcBlockResponse> for super::RpcBlockResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RpcBlockResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                author: value.author?,
                chunks: value.chunks?,
                header: value.header?,
            })
        }
    }
    impl ::std::convert::From<super::RpcBlockResponse> for RpcBlockResponse {
        fn from(value: super::RpcBlockResponse) -> Self {
            Self {
                author: Ok(value.author),
                chunks: Ok(value.chunks),
                header: Ok(value.header),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SlashedValidator {
        account_id: ::std::result::Result<super::AccountId, ::std::string::String>,
        is_double_sign: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for SlashedValidator {
        fn default() -> Self {
            Self {
                account_id: Err("no value supplied for account_id".to_string()),
                is_double_sign: Err("no value supplied for is_double_sign".to_string()),
            }
        }
    }
    impl SlashedValidator {
        pub fn account_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AccountId>,
            T::Error: ::std::fmt::Display,
        {
            self.account_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account_id: {}", e));
            self
        }
        pub fn is_double_sign<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_double_sign = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_double_sign: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SlashedValidator> for super::SlashedValidator {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SlashedValidator,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account_id: value.account_id?,
                is_double_sign: value.is_double_sign?,
            })
        }
    }
    impl ::std::convert::From<super::SlashedValidator> for SlashedValidator {
        fn from(value: super::SlashedValidator) -> Self {
            Self {
                account_id: Ok(value.account_id),
                is_double_sign: Ok(value.is_double_sign),
            }
        }
    }
}
