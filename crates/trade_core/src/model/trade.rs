use crate::model::*;
use chrono::{DateTime, Utc};

pub type TradeId = u64;
pub type SnapshotId = usize;
pub type UserId = String;
pub type HistoryTable = Vec<(SnapshotId, UserId, TradeState, TradeState, DateTime<Utc>)>;

// TODO do these all need to be public - probably not. getters should be enough
#[derive(Debug, Clone)]
pub struct TradeEventSnapshot {
    pub snapshot_id: SnapshotId,
    pub user_id: UserId,
    pub timestamp: DateTime<Utc>,
    pub from_state: TradeState,
    pub to_state: TradeState,
    pub details: TradeDetails,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub id: TradeId,
    pub created_at: DateTime<Utc>,        // When the trade was first created
    pub history: Vec<TradeEventSnapshot>, // Current state is the last entry
}

impl Trade {
    pub fn current_state(&self) -> TradeState {
        self.history.last().map(|s| s.to_state).unwrap_or(TradeState::Draft)
    }

    pub fn latest_details(&self) -> Option<&TradeDetails> {
        self.history.last().map(|s| &s.details)
    }

    pub fn new(id: TradeId, initial_details: TradeDetails, user_id: UserId) -> Self {
        let now = Utc::now(); // TODO - discuss, time can be taken from request entry in our network
        let initial_snapshot = TradeEventSnapshot {
            snapshot_id: 0,
            user_id,
            timestamp: now,
            from_state: TradeState::Draft, // Debatable whether we need this, it can be inferred
            to_state: TradeState::Draft,
            details: initial_details,
        };

        Trade {
            id,
            created_at: now,
            history: vec![initial_snapshot],
        }
    }

    /// Add a new versioned snapshot to the trade
    pub fn add_snapshot(
        &mut self,
        user_id: impl Into<UserId>,
        to_state: TradeState,
        details: TradeDetails,
    ) -> &TradeEventSnapshot {
        self.history.push(TradeEventSnapshot {
            snapshot_id: self.history.len(),
            user_id: user_id.into(),
            timestamp: Utc::now(),
            from_state: self.current_state(),
            to_state,
            details,
        });

        self.history.last().unwrap()
    }

    /// Get a specific snapshot by version ID
    pub fn get_snapshot(&self, version: SnapshotId) -> Option<&TradeEventSnapshot> {
        self.history.get(version)
    }

    /// Get the very latest snapshot
    pub fn get_snapshot_last(&self) -> Option<&TradeEventSnapshot> {
        self.history.last()
    }

    /// Get the very first snapshot
    pub fn get_snapshot_first(&self) -> Option<&TradeEventSnapshot> {
        self.history.first()
    }

    /// Get the original requester of the trade
    /// Do not confuse with get_first_approver (user who FIRST approved the trade)
    pub fn get_requester(&self) -> UserId {
        self.history
            .get(0)
            .map(|s| s.user_id.clone())
            .unwrap_or(String::default())
    }

    /// Get user_id of the first approver
    /// Do not confuse this with get_requester (user who posted/drafted the trade)
    pub fn get_first_approver(&self) -> Option<UserId> {
        self.history
            .iter()
            .find(|snapshot| snapshot.to_state == TradeState::PendingApproval)
            .map(|snapshot| snapshot.user_id.clone())
    }

    /// Check if the most recent state is "NeedsReapproval"
    /// This is abstracted away into a function in case it needs special logic later
    /// or the rule changes, or it's used in multiple places. Just best practice
    pub fn needs_re_approval(&self) -> bool {
        self.current_state() == TradeState::NeedsReapproval
    }

    /// History in tabular-friendly form (vector of tuple)
    pub fn history_table(&self) -> HistoryTable {
        self.history
            .iter()
            .map(|s| (s.snapshot_id, s.user_id.clone(), s.from_state, s.to_state, s.timestamp))
            .collect()
    }

    // In future post MVP, could add methods to get by date and so on
}
