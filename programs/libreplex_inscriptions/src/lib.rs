use anchor_lang::prelude::*;
use instructions::*;


use anchor_lang::{AnchorDeserialize, AnchorSerialize};

declare_id!("inscokhJarcjaEs59QbQ7hYjrKz25LEPRfCbP8EmdUp");

pub mod instructions;
pub mod state;

pub mod errors;
pub use errors::*;

pub use state::*;

#[program]
pub mod libreplex_inscriptions {

    
    use super::*;

    pub fn create_inscription_rank_page(
        ctx: Context<CreateInscriptionRank>,
        input: CreateInscriptionRankInput,
    ) -> Result<()> {
        instructions::create_inscription_rank_page::handler(
            ctx,
            input
        )
    }

    pub fn make_inscription_immutable(
        ctx: Context<MakeInscriptionImmutable>
    ) -> Result<()> {
        instructions::make_inscription_immutable::handler(
            ctx
        )
    }

    pub fn create_inscription(
        ctx: Context<CreateInscription>,
        inscription_input: CreateInscriptionInput,
    ) -> Result<()> {
        instructions::create_inscription::handler(
            ctx,
            inscription_input
        )
    }

    // pub fn delete_inscription(
    //     ctx: Context<DeleteInscription>
    // ) -> Result<()> {
    //     instructions::delete_inscription::handler(
    //         ctx
    //     )
    // }

    pub fn resize_inscription(
        ctx: Context<ResizeInscription>,
        input: ResizeInscriptionInput,
    ) -> Result<()> {
        instructions::resize_inscription::handler(
            ctx,
            input
        )
    }

    pub fn claim_excess_rent(
        ctx: Context<ClaimExcessRent>,
    ) -> Result<()> {
        instructions::claim_excess_rent::handler(
            ctx
        )
    }


    pub fn write_to_inscription(
        ctx: Context<WriteToInscription>,
        input: WriteToInscriptionInput,
    ) -> Result<()> {
        instructions::write_to_inscription::handler(
            ctx,
            input
        )
    }

    pub fn migrate_to_v3(
        ctx: Context<MigrateToV3>,
    ) -> Result<()> {
        instructions::migrate_to_v3::handler(
            ctx
        )
    }

    pub fn set_validation_hash(
        ctx: Context<SetValidationHash>,
        validation_hash: Option<String>
    ) -> Result<()> {
        instructions::set_validation_hash::handler(
            ctx,
            validation_hash
        )
    }



}