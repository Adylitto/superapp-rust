use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// DAO proposal entity for governance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: Uuid,
    pub proposer_id: Uuid,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub status: ProposalStatus,
    pub votes: VotingResults,
    pub execution_data: Option<String>, // Serialized execution parameters
    pub created_at: DateTime<Utc>,
    pub voting_starts_at: DateTime<Utc>,
    pub voting_ends_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    ParameterChange,
    TreasuryAllocation,
    FeatureToggle,
    MiniAppApproval,
    Custom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus {
    Draft,
    Active,
    Passed,
    Rejected,
    Executed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingResults {
    pub votes_for: u64,
    pub votes_against: u64,
    pub votes_abstain: u64,
    pub total_voting_power: u64,
    pub voters: HashMap<Uuid, Vote>,
    pub quadratic_voting_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter_id: Uuid,
    pub vote_type: VoteType,
    pub voting_power: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum VoteType {
    For,
    Against,
    Abstain,
}

impl Proposal {
    /// Create a new proposal
    pub fn new(
        proposer_id: Uuid,
        title: String,
        description: String,
        proposal_type: ProposalType,
        voting_duration_hours: i64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            proposer_id,
            title,
            description,
            proposal_type,
            status: ProposalStatus::Draft,
            votes: VotingResults {
                votes_for: 0,
                votes_against: 0,
                votes_abstain: 0,
                total_voting_power: 0,
                voters: HashMap::new(),
                quadratic_voting_enabled: false,
            },
            execution_data: None,
            created_at: now,
            voting_starts_at: now,
            voting_ends_at: now + chrono::Duration::hours(voting_duration_hours),
            executed_at: None,
        }
    }

    /// Activate proposal for voting
    pub fn activate(&mut self) -> Result<(), crate::Error> {
        if self.status != ProposalStatus::Draft {
            return Err(crate::Error::BusinessRule(
                "Only draft proposals can be activated".to_string(),
            ));
        }
        self.status = ProposalStatus::Active;
        Ok(())
    }

    /// Cast a vote
    pub fn cast_vote(
        &mut self,
        voter_id: Uuid,
        vote_type: VoteType,
        voting_power: u64,
    ) -> Result<(), crate::Error> {
        if self.status != ProposalStatus::Active {
            return Err(crate::Error::BusinessRule(
                "Can only vote on active proposals".to_string(),
            ));
        }

        let now = Utc::now();
        if now > self.voting_ends_at {
            return Err(crate::Error::BusinessRule(
                "Voting period has ended".to_string(),
            ));
        }

        // Remove previous vote if exists
        if let Some(prev_vote) = self.votes.voters.get(&voter_id) {
            match prev_vote.vote_type {
                VoteType::For => self.votes.votes_for -= prev_vote.voting_power,
                VoteType::Against => self.votes.votes_against -= prev_vote.voting_power,
                VoteType::Abstain => self.votes.votes_abstain -= prev_vote.voting_power,
            }
        }

        // Apply quadratic voting if enabled
        let adjusted_power = if self.votes.quadratic_voting_enabled {
            (voting_power as f64).sqrt().floor() as u64
        } else {
            voting_power
        };

        // Add new vote
        match vote_type {
            VoteType::For => self.votes.votes_for += adjusted_power,
            VoteType::Against => self.votes.votes_against += adjusted_power,
            VoteType::Abstain => self.votes.votes_abstain += adjusted_power,
        }

        self.votes.voters.insert(
            voter_id,
            Vote {
                voter_id,
                vote_type,
                voting_power: adjusted_power,
                timestamp: now,
            },
        );

        Ok(())
    }

    /// Finalize voting and determine outcome
    pub fn finalize_voting(&mut self, quorum_percentage: f64) -> Result<(), crate::Error> {
        if self.status != ProposalStatus::Active {
            return Err(crate::Error::BusinessRule(
                "Can only finalize active proposals".to_string(),
            ));
        }

        let total_votes = self.votes.votes_for + self.votes.votes_against + self.votes.votes_abstain;
        let participation_rate = if self.votes.total_voting_power > 0 {
            total_votes as f64 / self.votes.total_voting_power as f64
        } else {
            0.0
        };

        // Check quorum
        if participation_rate < quorum_percentage {
            self.status = ProposalStatus::Rejected;
            return Ok(());
        }

        // Determine outcome
        if self.votes.votes_for > self.votes.votes_against {
            self.status = ProposalStatus::Passed;
        } else {
            self.status = ProposalStatus::Rejected;
        }

        Ok(())
    }

    /// Mark proposal as executed
    pub fn mark_executed(&mut self) -> Result<(), crate::Error> {
        if self.status != ProposalStatus::Passed {
            return Err(crate::Error::BusinessRule(
                "Only passed proposals can be executed".to_string(),
            ));
        }
        self.status = ProposalStatus::Executed;
        self.executed_at = Some(Utc::now());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proposal_lifecycle() {
        let proposer = Uuid::new_v4();
        let mut proposal = Proposal::new(
            proposer,
            "Test Proposal".to_string(),
            "Description".to_string(),
            ProposalType::ParameterChange,
            48,
        );

        assert_eq!(proposal.status, ProposalStatus::Draft);
        assert!(proposal.activate().is_ok());
        assert_eq!(proposal.status, ProposalStatus::Active);

        let voter1 = Uuid::new_v4();
        proposal.votes.total_voting_power = 1000;
        assert!(proposal.cast_vote(voter1, VoteType::For, 600).is_ok());

        assert!(proposal.finalize_voting(0.1).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Passed);
    }
}
