#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AccessModel {
    Ownable,
    AccessControl,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FungibleType {
    Base,
    AllowList,
    BlockList,
    Votes,
    Vault,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NftType {
    Base,
    Enumerable,
    Consecutive,
    Votes,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ContractVariant {
    Fungible(FungibleType),
    NonFungible(NftType),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Feature {
    Capped,
    Pausable,
    Royalties,
}

pub struct CombinationConfig {
    pub name: &'static str,
    pub description: &'static str,
    pub variant: ContractVariant,
    pub access: AccessModel,
    pub features: &'static [Feature],
}

impl CombinationConfig {
    pub fn has(&self, feature: Feature) -> bool {
        self.features.contains(&feature)
    }

    pub fn fungible_type(&self) -> Option<FungibleType> {
        match self.variant {
            ContractVariant::Fungible(ft) => Some(ft),
            _ => None,
        }
    }

    pub fn nft_type(&self) -> Option<NftType> {
        match self.variant {
            ContractVariant::NonFungible(nt) => Some(nt),
            _ => None,
        }
    }

    pub fn is_ownable(&self) -> bool {
        self.access == AccessModel::Ownable
    }
}

pub const COMBINATIONS: &[CombinationConfig] = &[
    // === Fungible Base ===
    CombinationConfig {
        name: "ft-standard",
        description: "Standard SEP-41 fungible token with Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::Base),
        access: AccessModel::Ownable,
        features: &[],
    },
    CombinationConfig {
        name: "ft-capped",
        description: "Capped SEP-41 fungible token with maximum supply, Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::Base),
        access: AccessModel::Ownable,
        features: &[Feature::Capped],
    },
    CombinationConfig {
        name: "ft-pausable",
        description: "Pausable SEP-41 fungible token with emergency stop, Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::Base),
        access: AccessModel::Ownable,
        features: &[Feature::Pausable],
    },
    CombinationConfig {
        name: "ft-capped-pausable",
        description: "Capped and pausable SEP-41 fungible token with Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::Base),
        access: AccessModel::Ownable,
        features: &[Feature::Capped, Feature::Pausable],
    },
    // === Fungible AllowList ===
    CombinationConfig {
        name: "ft-allowlist",
        description: "SEP-41 fungible token with AllowList transfer restriction, AccessControl, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::AllowList),
        access: AccessModel::AccessControl,
        features: &[],
    },
    CombinationConfig {
        name: "ft-allowlist-capped",
        description: "Capped SEP-41 fungible token with AllowList, AccessControl, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::AllowList),
        access: AccessModel::AccessControl,
        features: &[Feature::Capped],
    },
    CombinationConfig {
        name: "ft-allowlist-pausable",
        description: "Pausable SEP-41 fungible token with AllowList, AccessControl, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::AllowList),
        access: AccessModel::AccessControl,
        features: &[Feature::Pausable],
    },
    CombinationConfig {
        name: "ft-allowlist-capped-pausable",
        description: "Capped and pausable SEP-41 fungible token with AllowList, AccessControl, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::AllowList),
        access: AccessModel::AccessControl,
        features: &[Feature::Capped, Feature::Pausable],
    },
    // === Fungible BlockList ===
    CombinationConfig {
        name: "ft-blocklist",
        description: "SEP-41 fungible token with BlockList transfer restriction, AccessControl, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::BlockList),
        access: AccessModel::AccessControl,
        features: &[],
    },
    CombinationConfig {
        name: "ft-blocklist-capped",
        description: "Capped SEP-41 fungible token with BlockList, AccessControl, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::BlockList),
        access: AccessModel::AccessControl,
        features: &[Feature::Capped],
    },
    CombinationConfig {
        name: "ft-blocklist-pausable",
        description: "Pausable SEP-41 fungible token with BlockList, AccessControl, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::BlockList),
        access: AccessModel::AccessControl,
        features: &[Feature::Pausable],
    },
    CombinationConfig {
        name: "ft-blocklist-capped-pausable",
        description: "Capped and pausable SEP-41 fungible token with BlockList, AccessControl, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::BlockList),
        access: AccessModel::AccessControl,
        features: &[Feature::Capped, Feature::Pausable],
    },
    // === Fungible Votes ===
    CombinationConfig {
        name: "ft-votes",
        description: "Governance fungible token with voting power tracking, Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::Votes),
        access: AccessModel::Ownable,
        features: &[],
    },
    CombinationConfig {
        name: "ft-votes-capped",
        description: "Capped governance fungible token with voting, Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::Votes),
        access: AccessModel::Ownable,
        features: &[Feature::Capped],
    },
    CombinationConfig {
        name: "ft-votes-pausable",
        description: "Pausable governance fungible token with voting, Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::Votes),
        access: AccessModel::Ownable,
        features: &[Feature::Pausable],
    },
    CombinationConfig {
        name: "ft-votes-capped-pausable",
        description: "Capped and pausable governance fungible token with voting, Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::Votes),
        access: AccessModel::Ownable,
        features: &[Feature::Capped, Feature::Pausable],
    },
    // === Fungible Vault ===
    CombinationConfig {
        name: "ft-vault",
        description: "ERC-4626 tokenized vault with Ownable access and Upgradeable.",
        variant: ContractVariant::Fungible(FungibleType::Vault),
        access: AccessModel::Ownable,
        features: &[],
    },
    // === NFT Base ===
    CombinationConfig {
        name: "nft-standard",
        description: "Standard NFT with sequential minting, Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::NonFungible(NftType::Base),
        access: AccessModel::Ownable,
        features: &[],
    },
    CombinationConfig {
        name: "nft-pausable",
        description: "Pausable NFT with emergency stop, Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::NonFungible(NftType::Base),
        access: AccessModel::Ownable,
        features: &[Feature::Pausable],
    },
    CombinationConfig {
        name: "nft-royalties",
        description: "NFT with ERC-2981 royalties, AccessControl, Burnable, and Upgradeable.",
        variant: ContractVariant::NonFungible(NftType::Base),
        access: AccessModel::AccessControl,
        features: &[Feature::Royalties],
    },
    CombinationConfig {
        name: "nft-royalties-pausable",
        description: "Pausable NFT with royalties, AccessControl, Burnable, and Upgradeable.",
        variant: ContractVariant::NonFungible(NftType::Base),
        access: AccessModel::AccessControl,
        features: &[Feature::Royalties, Feature::Pausable],
    },
    // === NFT Enumerable ===
    CombinationConfig {
        name: "nft-enumerable",
        description: "Enumerable NFT with on-chain token enumeration, Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::NonFungible(NftType::Enumerable),
        access: AccessModel::Ownable,
        features: &[],
    },
    CombinationConfig {
        name: "nft-enumerable-pausable",
        description: "Pausable enumerable NFT with Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::NonFungible(NftType::Enumerable),
        access: AccessModel::Ownable,
        features: &[Feature::Pausable],
    },
    CombinationConfig {
        name: "nft-enumerable-royalties",
        description: "Enumerable NFT with royalties, AccessControl, Burnable, and Upgradeable.",
        variant: ContractVariant::NonFungible(NftType::Enumerable),
        access: AccessModel::AccessControl,
        features: &[Feature::Royalties],
    },
    CombinationConfig {
        name: "nft-enumerable-royalties-pausable",
        description: "Pausable enumerable NFT with royalties, AccessControl, Burnable, and Upgradeable.",
        variant: ContractVariant::NonFungible(NftType::Enumerable),
        access: AccessModel::AccessControl,
        features: &[Feature::Royalties, Feature::Pausable],
    },
    // === NFT Consecutive ===
    CombinationConfig {
        name: "nft-consecutive",
        description: "Consecutive NFT with efficient batch minting, Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::NonFungible(NftType::Consecutive),
        access: AccessModel::Ownable,
        features: &[],
    },
    CombinationConfig {
        name: "nft-consecutive-pausable",
        description: "Pausable consecutive NFT with Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::NonFungible(NftType::Consecutive),
        access: AccessModel::Ownable,
        features: &[Feature::Pausable],
    },
    CombinationConfig {
        name: "nft-consecutive-royalties",
        description: "Consecutive NFT with royalties, AccessControl, Burnable, and Upgradeable.",
        variant: ContractVariant::NonFungible(NftType::Consecutive),
        access: AccessModel::AccessControl,
        features: &[Feature::Royalties],
    },
    CombinationConfig {
        name: "nft-consecutive-royalties-pausable",
        description: "Pausable consecutive NFT with royalties, AccessControl, Burnable, and Upgradeable.",
        variant: ContractVariant::NonFungible(NftType::Consecutive),
        access: AccessModel::AccessControl,
        features: &[Feature::Royalties, Feature::Pausable],
    },
    // === NFT Votes ===
    CombinationConfig {
        name: "nft-votes",
        description: "NFT with voting power tracking, Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::NonFungible(NftType::Votes),
        access: AccessModel::Ownable,
        features: &[],
    },
    CombinationConfig {
        name: "nft-votes-pausable",
        description: "Pausable NFT with voting power tracking, Ownable access, Burnable, and Upgradeable.",
        variant: ContractVariant::NonFungible(NftType::Votes),
        access: AccessModel::Ownable,
        features: &[Feature::Pausable],
    },
];
