use crate::{Error, Proposal, ProposalType, Result, User};
use uuid::Uuid;

/// Use case for creating DAO proposals
pub struct CreateProposalUseCase {
    user_repository: Box<dyn UserRepository>,
    proposal_repository: Box<dyn ProposalRepository>,
}

/// Repository trait for user operations
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
}

/// Repository trait for proposal operations
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait ProposalRepository: Send + Sync {
    async fn create(&self, proposal: &Proposal) -> Result<()>;
}

const MIN_TOKENS_FOR_PROPOSAL: u64 = 1000;

impl CreateProposalUseCase {
    pub fn new(
        user_repository: Box<dyn UserRepository>,
        proposal_repository: Box<dyn ProposalRepository>,
    ) -> Self {
        Self {
            user_repository,
            proposal_repository,
        }
    }

    /// Create a new DAO proposal
    pub async fn execute(
        &self,
        proposer_id: Uuid,
        title: String,
        description: String,
        proposal_type: ProposalType,
        voting_duration_hours: i64,
    ) -> Result<Proposal> {
        // Validate inputs
        if title.is_empty() {
            return Err(Error::Validation("Title cannot be empty".to_string()));
        }

        if description.is_empty() {
            return Err(Error::Validation(
                "Description cannot be empty".to_string(),
            ));
        }

        if voting_duration_hours < 24 || voting_duration_hours > 168 {
            return Err(Error::Validation(
                "Voting duration must be between 24 and 168 hours".to_string(),
            ));
        }

        // Find proposer
        let proposer = self
            .user_repository
            .find_by_id(proposer_id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("User {} not found", proposer_id)))?;

        // Check token threshold
        if proposer.token_balance < MIN_TOKENS_FOR_PROPOSAL {
            return Err(Error::InsufficientTokens {
                required: MIN_TOKENS_FOR_PROPOSAL,
                available: proposer.token_balance,
            });
        }

        // Check user is verified
        if !proposer.is_verified {
            return Err(Error::Unauthorized(
                "Only verified users can create proposals".to_string(),
            ));
        }

        // Create proposal
        let proposal = Proposal::new(
            proposer_id,
            title,
            description,
            proposal_type,
            voting_duration_hours,
        );

        // Persist proposal
        self.proposal_repository.create(&proposal).await?;

        Ok(proposal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_create_proposal_success() {
        let proposer_id = Uuid::new_v4();
        let mut user = User::new(
            "test@example.com".to_string(),
            "testuser".to_string(),
            "hash".to_string(),
        );
        user.id = proposer_id;
        user.token_balance = 1500;
        user.is_verified = true;

        let mut user_repo = Box::new(MockUserRepository::new());
        user_repo
            .expect_find_by_id()
            .with(eq(proposer_id))
            .returning(move |_| Ok(Some(user.clone())));

        let mut proposal_repo = Box::new(MockProposalRepository::new());
        proposal_repo.expect_create().returning(|_| Ok(()));

        let use_case = CreateProposalUseCase::new(user_repo, proposal_repo);
        let result = use_case
            .execute(
                proposer_id,
                "Test Proposal".to_string(),
                "Description".to_string(),
                ProposalType::ParameterChange,
                48,
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_proposal_insufficient_tokens() {
        let proposer_id = Uuid::new_v4();
        let mut user = User::new(
            "test@example.com".to_string(),
            "testuser".to_string(),
            "hash".to_string(),
        );
        user.id = proposer_id;
        user.token_balance = 500; // Below threshold
        user.is_verified = true;

        let mut user_repo = Box::new(MockUserRepository::new());
        user_repo
            .expect_find_by_id()
            .returning(move |_| Ok(Some(user.clone())));

        let proposal_repo = Box::new(MockProposalRepository::new());

        let use_case = CreateProposalUseCase::new(user_repo, proposal_repo);
        let result = use_case
            .execute(
                proposer_id,
                "Test".to_string(),
                "Desc".to_string(),
                ProposalType::ParameterChange,
                48,
            )
            .await;

        assert!(result.is_err());
    }
}
