#![allow(unexpected_cfgs)]

use {
    crate::instructions::initialize_permanent_delegate,
    pinocchio::{
        account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
        ProgramResult,
    },
    pinocchio_pubkey::declare_id,
    spl_token_2022_interface::{
        extension::{
            group_member_pointer::instruction::GroupMemberPointerInstruction,
            group_pointer::instruction::GroupPointerInstruction,
            memo_transfer::instruction::RequiredMemoTransfersInstruction,
            metadata_pointer::instruction::MetadataPointerInstruction, token_metadata,
        },
        instruction::{decode_instruction_type, TokenInstruction},
    },
    spl_token_group_interface::instruction::{
        InitializeGroup, InitializeMember, TokenGroupInstruction, UpdateGroupAuthority,
        UpdateGroupMaxSize,
    },
};

pub mod helpers;
mod instructions;

use instructions as i;

entrypoint!(process_instruction);
declare_id!("4ibrEMW5F6hKnkW4jVedswYv6H6VtwPN6ar6dvXDN1nT");

// if spl_token_metadata_interface::TokenMetadataInstruction
// use spl_token_metadata_interface;
use spl_token_metadata_interface::{
    instruction::{
        Emit, Initialize, RemoveKey, TokenMetadataInstruction, UpdateAuthority, UpdateField,
    },
    state::{Field, TokenMetadata},
};

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // debug
    // pinocchio::msg!("PROCESSOR TRIGGERED");
    // match TokenMetadataInstruction::unpack(instruction_data) {

    //         Ok(token_metadata_instruction) => match token_metadata_instruction {
    //             TokenMetadataInstruction::Initialize(Initialize) => {
    //                 pinocchio::msg!("proxy::token_metadata::initialize")
    //             }
    //             TokenMetadataInstruction::UpdateField(UpdateField) => {
    //                 pinocchio::msg!("proxy::token_metadata::update_field")
    //             }
    //             TokenMetadataInstruction::RemoveKey(RemoveKey) => {
    //                 pinocchio::msg!("proxy::token_metadata::remove_key")
    //             }
    //             TokenMetadataInstruction::UpdateAuthority(UpdateAuthority) => {
    //                 pinocchio::msg!("proxy::token_metadata::update_authority")
    //             }
    //             TokenMetadataInstruction::Emit(Emit) => {
    //                 pinocchio::msg!("proxy::token_metadata::emit")
    //             }
    //         },

    //         _ => {
    //                    // debug
    // pinocchio::msg!("metadata unpack _ err triggered");
    //             Err(ProgramError::InvalidInstructionData)?
    //         },
    //     };
    // Ok(())
    match TokenInstruction::unpack(instruction_data) {
        // try to match TokenInstruction
        Ok(token_instruction) => {
            match token_instruction {
                TokenInstruction::InitializeMint {
                    decimals,
                    mint_authority,
                    freeze_authority,
                } => i::initialize_mint(accounts, decimals, mint_authority, freeze_authority),

                // For Initializing TokenAccount
                TokenInstruction::InitializeAccount => i::initialize_token_account(accounts),

                TokenInstruction::GroupPointerExtension => {
                    let instruction_data = &instruction_data[1..]; // Remove extension discriminator
                    let ix: GroupPointerInstruction = decode_instruction_type(instruction_data)
                        .map_err(|_| ProgramError::InvalidInstructionData)?;

                    match ix {
                        GroupPointerInstruction::Initialize => {
                            i::group_pointer::initialize(accounts, instruction_data)
                        }
                        GroupPointerInstruction::Update => {
                            i::group_pointer::update(accounts, instruction_data)
                        }
                    }
                }

                TokenInstruction::GroupMemberPointerExtension => {
                    let instruction_data = &instruction_data[1..]; // Remove extension discriminator
                    let ix: GroupMemberPointerInstruction =
                        decode_instruction_type(instruction_data)
                            .map_err(|_| ProgramError::InvalidInstructionData)?;

                    match ix {
                        GroupMemberPointerInstruction::Initialize => {
                            i::group_member_pointer::initialize(accounts, instruction_data)
                        }
                        GroupMemberPointerInstruction::Update => {
                            i::group_member_pointer::update(accounts, instruction_data)
                        }
                    }
                }

                // MetadataPointer extention
                TokenInstruction::MetadataPointerExtension => {
                    let instruction_data = &instruction_data[1..]; // Remove extension discriminator
                    let ix: MetadataPointerInstruction = decode_instruction_type(instruction_data)
                        .map_err(|_| ProgramError::InvalidInstructionData)?;

                    match ix {
                        MetadataPointerInstruction::Initialize => {
                            i::metadata_pointer::initialize(accounts, instruction_data)
                        }
                        MetadataPointerInstruction::Update => {
                            i::metadata_pointer::update(accounts, instruction_data)
                        }
                    }
                }

                TokenInstruction::InitializePermanentDelegate { delegate } => {
                    initialize_permanent_delegate(accounts, delegate)
                }

                // MemoTransfer Extention
                TokenInstruction::MemoTransferExtension => {
                    let instruction_data = &instruction_data[1..]; // Remove extension discriminator
                    let ix: RequiredMemoTransfersInstruction =
                        decode_instruction_type(instruction_data)
                            .map_err(|_| ProgramError::InvalidInstructionData)?;

                    match ix {
                        RequiredMemoTransfersInstruction::Enable => {
                            i::memo_transfer::enable(accounts, instruction_data)
                        }
                        RequiredMemoTransfersInstruction::Disable => {
                            i::memo_transfer::disable(accounts, instruction_data)
                        }
                    }
                }

                _ => Err(ProgramError::InvalidInstructionData)?,
            }
        }
        Err(_) => {
            // debug
            pinocchio::msg!("tokenInstruction Err triggered");
            // try to match TokenGroupInstruction
            match TokenGroupInstruction::unpack(instruction_data) {
                // Ok(token_instruction) => match token_instruction {
                //     TokenGroupInstruction::InitializeGroup(InitializeGroup {
                //         update_authority,
                //         max_size,
                //     }) => i::token_group::initialize_group(accounts, update_authority, max_size),
                //     TokenGroupInstruction::UpdateGroupMaxSize(UpdateGroupMaxSize { max_size }) => {
                //         i::token_group::update_max_size(accounts, max_size)
                //     }
                //     TokenGroupInstruction::UpdateGroupAuthority(UpdateGroupAuthority {
                //         new_authority,
                //     }) => i::token_group::update_group_authority(accounts, new_authority),
                //     TokenGroupInstruction::InitializeMember(InitializeMember) => {
                //         i::token_group::initialize_member(accounts)
                //     }

                // } //@audit-issue :: fails all tokengroup + token_metadata tests
                Ok(token_instruction) => {
                    match token_instruction {
                        TokenGroupInstruction::InitializeGroup(InitializeGroup {
                            update_authority,
                            max_size,
                        }) => {
                            i::token_group::initialize_group(accounts, update_authority, max_size)?
                        }
                        TokenGroupInstruction::UpdateGroupMaxSize(UpdateGroupMaxSize {
                            max_size,
                        }) => i::token_group::update_max_size(accounts, max_size)?,
                        TokenGroupInstruction::UpdateGroupAuthority(UpdateGroupAuthority {
                            new_authority,
                        }) => i::token_group::update_group_authority(accounts, new_authority)?,
                        TokenGroupInstruction::InitializeMember(InitializeMember) => {
                            i::token_group::initialize_member(accounts)?
                        }
                    }
                    // Prevent metadata logic from running afterward
                    return Ok(());
                }
                // _ => Err(ProgramError::InvalidInstructionData)?,
                Err(_) => {
                    pinocchio::msg!("tokenGROUP Err triggered");

                    match TokenMetadataInstruction::unpack(instruction_data) {
                        Ok(token_metadata_instruction) => match token_metadata_instruction {
                            TokenMetadataInstruction::Initialize(Initialize) => {
                                pinocchio::msg!("proxy::token_metadata::initialize");
                                Ok(())
                            }
                            TokenMetadataInstruction::UpdateField(UpdateField) => {
                                pinocchio::msg!("proxy::token_metadata::update_field");
                                Ok(())
                            }
                            TokenMetadataInstruction::RemoveKey(RemoveKey) => {
                                pinocchio::msg!("proxy::token_metadata::remove_key");
                                Ok(())
                            }
                            TokenMetadataInstruction::UpdateAuthority(UpdateAuthority) => {
                                pinocchio::msg!("proxy::token_metadata::update_authority");
                                Ok(())
                            }
                            TokenMetadataInstruction::Emit(Emit) => {
                                pinocchio::msg!("proxy::token_metadata::emit");
                                Ok(())
                            }
                        },

                        _ => {
                            // debug
                            pinocchio::msg!("metadata unpack _ err triggered");
                            Err(ProgramError::InvalidInstructionData)?
                        }
                    }
                    // }
                    // }?;

                    //     match TokenMetadataInstruction::unpack(instruction_data) {

                    //         Ok(token_metadata_instruction) => match token_metadata_instruction {
                    //             TokenMetadataInstruction::Initialize(Initialize) => {
                    //                 pinocchio::msg!("proxy::token_metadata::initialize")
                    //             }
                    //             TokenMetadataInstruction::UpdateField(UpdateField) => {
                    //                 pinocchio::msg!("proxy::token_metadata::update_field")
                    //             }
                    //             TokenMetadataInstruction::RemoveKey(RemoveKey) => {
                    //                 pinocchio::msg!("proxy::token_metadata::remove_key")
                    //             }
                    //             TokenMetadataInstruction::UpdateAuthority(UpdateAuthority) => {
                    //                 pinocchio::msg!("proxy::token_metadata::update_authority")
                    //             }
                    //             TokenMetadataInstruction::Emit(Emit) => {
                    //                 pinocchio::msg!("proxy::token_metadata::emit")
                    //             }
                    //         },

                    //         _ => {
                    //                    // debug
                    // pinocchio::msg!("metadata unpack _ err triggered");
                    //             Err(ProgramError::InvalidInstructionData)?
                    //         },
                    //     };

                    // Ok(())
                }
            }
        }
    }
}
