use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// SuperApp DAO Program for decentralized governance
#[program]
pub mod superapp_dao {
    use super::*;

    /// Initialize the DAO
    pub fn initialize_dao(
        ctx: Context<InitializeDao>,
        quorum_percentage: u8,
        voting_duration: i64,
    ) -> Result<()> {
        require!(quorum_percentage <= 100, DaoError::InvalidQuorum);

        let dao = &mut ctx.accounts.dao;
        dao.authority = ctx.accounts.authority.key();
        dao.token_mint = ctx.accounts.token_mint.key();
        dao.quorum_percentage = quorum_percentage;
        dao.voting_duration = voting_duration;
        dao.proposal_count = 0;
        dao.total_supply = 0;

        msg!("DAO initialized with quorum: {}%", quorum_percentage);
        Ok(())
    }

    /// Mint tokens to a user (rewards mechanism)
    pub fn mint_tokens(
        ctx: Context<MintTokens>,
        amount: u64,
        reason: RewardReason,
    ) -> Result<()> {
        require!(amount > 0, DaoError::InvalidAmount);

        let dao = &mut ctx.accounts.dao;
        dao.total_supply = dao.total_supply.checked_add(amount)
            .ok_or(DaoError::Overflow)?;

        // Mint tokens to user's account
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::mint_to(cpi_ctx, amount)?;

        emit!(TokensMinted {
            user: ctx.accounts.user.key(),
            amount,
            reason,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Minted {} tokens for {:?}", amount, reason);
        Ok(())
    }

    /// Create a governance proposal
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        proposal_type: ProposalType,
    ) -> Result<()> {
        require!(title.len() <= 100, DaoError::TitleTooLong);
        require!(description.len() <= 1000, DaoError::DescriptionTooLong);

        let dao = &mut ctx.accounts.dao;
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;

        // Check proposer has minimum tokens (1000)
        require!(
            ctx.accounts.proposer_token_account.amount >= 1000,
            DaoError::InsufficientTokens
        );

        proposal.dao = dao.key();
        proposal.proposer = ctx.accounts.proposer.key();
        proposal.title = title.clone();
        proposal.description = description;
        proposal.proposal_type = proposal_type;
        proposal.status = ProposalStatus::Active;
        proposal.votes_for = 0;
        proposal.votes_against = 0;
        proposal.votes_abstain = 0;
        proposal.created_at = clock.unix_timestamp;
        proposal.voting_ends_at = clock.unix_timestamp + dao.voting_duration;
        proposal.executed_at = 0;

        dao.proposal_count = dao.proposal_count.checked_add(1)
            .ok_or(DaoError::Overflow)?;

        emit!(ProposalCreated {
            proposal: proposal.key(),
            proposer: proposal.proposer,
            title,
            timestamp: clock.unix_timestamp,
        });

        msg!("Proposal created");
        Ok(())
    }

    /// Cast a vote on a proposal
    pub fn cast_vote(
        ctx: Context<CastVote>,
        vote_type: VoteType,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;

        // Verify proposal is active
        require!(
            proposal.status == ProposalStatus::Active,
            DaoError::ProposalNotActive
        );

        // Check voting period hasn't ended
        require!(
            clock.unix_timestamp <= proposal.voting_ends_at,
            DaoError::VotingEnded
        );

        let voting_power = ctx.accounts.voter_token_account.amount;
        require!(voting_power > 0, DaoError::NoVotingPower);

        // Apply quadratic voting: sqrt(tokens)
        let adjusted_power = (voting_power as f64).sqrt() as u64;

        match vote_type {
            VoteType::For => {
                proposal.votes_for = proposal.votes_for.checked_add(adjusted_power)
                    .ok_or(DaoError::Overflow)?;
            }
            VoteType::Against => {
                proposal.votes_against = proposal.votes_against.checked_add(adjusted_power)
                    .ok_or(DaoError::Overflow)?;
            }
            VoteType::Abstain => {
                proposal.votes_abstain = proposal.votes_abstain.checked_add(adjusted_power)
                    .ok_or(DaoError::Overflow)?;
            }
        }

        emit!(VoteCast {
            proposal: proposal.key(),
            voter: ctx.accounts.voter.key(),
            vote_type,
            voting_power: adjusted_power,
            timestamp: clock.unix_timestamp,
        });

        msg!("Vote cast: {:?} with power {}", vote_type, adjusted_power);
        Ok(())
    }

    /// Finalize voting and determine outcome
    pub fn finalize_proposal(ctx: Context<FinalizeProposal>) -> Result<()> {
        let dao = &ctx.accounts.dao;
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;

        // Verify voting has ended
        require!(
            clock.unix_timestamp > proposal.voting_ends_at,
            DaoError::VotingNotEnded
        );

        require!(
            proposal.status == ProposalStatus::Active,
            DaoError::ProposalNotActive
        );

        let total_votes = proposal.votes_for
            .checked_add(proposal.votes_against)
            .and_then(|v| v.checked_add(proposal.votes_abstain))
            .ok_or(DaoError::Overflow)?;

        // Check quorum
        let participation_rate = if dao.total_supply > 0 {
            (total_votes as f64 / dao.total_supply as f64) * 100.0
        } else {
            0.0
        };

        if participation_rate < dao.quorum_percentage as f64 {
            proposal.status = ProposalStatus::Rejected;
            msg!("Proposal rejected: quorum not met");
        } else if proposal.votes_for > proposal.votes_against {
            proposal.status = ProposalStatus::Passed;
            msg!("Proposal passed");
        } else {
            proposal.status = ProposalStatus::Rejected;
            msg!("Proposal rejected: more against than for");
        }

        emit!(ProposalFinalized {
            proposal: proposal.key(),
            status: proposal.status,
            votes_for: proposal.votes_for,
            votes_against: proposal.votes_against,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    /// Execute a passed proposal
    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;

        require!(
            proposal.status == ProposalStatus::Passed,
            DaoError::ProposalNotPassed
        );

        // Execute proposal logic based on type
        match proposal.proposal_type {
            ProposalType::ParameterChange => {
                msg!("Executing parameter change");
                // TODO: Implement parameter change logic
            }
            ProposalType::TreasuryAllocation => {
                msg!("Executing treasury allocation");
                // TODO: Implement treasury allocation logic
            }
            ProposalType::MiniAppApproval => {
                msg!("Executing mini-app approval");
                // TODO: Implement mini-app approval logic
            }
        }

        proposal.status = ProposalStatus::Executed;
        proposal.executed_at = clock.unix_timestamp;

        emit!(ProposalExecuted {
            proposal: proposal.key(),
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }
}

// Account structures
#[derive(Accounts)]
pub struct InitializeDao<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Dao::INIT_SPACE
    )]
    pub dao: Account<'info, Dao>,
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    /// CHECK: User receiving tokens
    pub user: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    #[account(
        init,
        payer = proposer,
        space = 8 + Proposal::INIT_SPACE
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    #[account(
        constraint = proposer_token_account.owner == proposer.key()
    )]
    pub proposer_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub voter: Signer<'info>,
    #[account(
        constraint = voter_token_account.owner == voter.key()
    )]
    pub voter_token_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct FinalizeProposal<'info> {
    pub dao: Account<'info, Dao>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub authority: Signer<'info>,
}

// Data structures
#[account]
pub struct Dao {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub quorum_percentage: u8,
    pub voting_duration: i64,
    pub proposal_count: u64,
    pub total_supply: u64,
}

impl Dao {
    pub const INIT_SPACE: usize = 32 + 32 + 1 + 8 + 8 + 8;
}

#[account]
pub struct Proposal {
    pub dao: Pubkey,
    pub proposer: Pubkey,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub status: ProposalStatus,
    pub votes_for: u64,
    pub votes_against: u64,
    pub votes_abstain: u64,
    pub created_at: i64,
    pub voting_ends_at: i64,
    pub executed_at: i64,
}

impl Proposal {
    pub const INIT_SPACE: usize = 32 + 32 + 100 + 1000 + 1 + 1 + 8 + 8 + 8 + 8 + 8 + 8;
}

// Enums
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ProposalType {
    ParameterChange,
    TreasuryAllocation,
    MiniAppApproval,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum VoteType {
    For,
    Against,
    Abstain,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum RewardReason {
    PostEngagement,
    RideCompletion,
    Referral,
    DailyBonus,
}

// Events
#[event]
pub struct TokensMinted {
    pub user: Pubkey,
    pub amount: u64,
    pub reason: RewardReason,
    pub timestamp: i64,
}

#[event]
pub struct ProposalCreated {
    pub proposal: Pubkey,
    pub proposer: Pubkey,
    pub title: String,
    pub timestamp: i64,
}

#[event]
pub struct VoteCast {
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub vote_type: VoteType,
    pub voting_power: u64,
    pub timestamp: i64,
}

#[event]
pub struct ProposalFinalized {
    pub proposal: Pubkey,
    pub status: ProposalStatus,
    pub votes_for: u64,
    pub votes_against: u64,
    pub timestamp: i64,
}

#[event]
pub struct ProposalExecuted {
    pub proposal: Pubkey,
    pub timestamp: i64,
}

// Errors
#[error_code]
pub enum DaoError {
    #[msg("Invalid quorum percentage")]
    InvalidQuorum,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Insufficient tokens")]
    InsufficientTokens,
    #[msg("Title too long")]
    TitleTooLong,
    #[msg("Description too long")]
    DescriptionTooLong,
    #[msg("Proposal not active")]
    ProposalNotActive,
    #[msg("Voting period has ended")]
    VotingEnded,
    #[msg("No voting power")]
    NoVotingPower,
    #[msg("Voting not ended")]
    VotingNotEnded,
    #[msg("Proposal not passed")]
    ProposalNotPassed,
}
