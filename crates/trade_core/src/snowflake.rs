use parking_lot::Mutex;
use std::sync::Arc;

use crate::util::current_timestamp_ms;

const EPOCH: u64 = 1_700_000_000_000; // Some Custom epoch
const MACHINE_ID_BITS: u8 = 10;
const SEQUENCE_BITS: u8 = 12;
const MAX_MACHINE_ID: u16 = (1 << MACHINE_ID_BITS) - 1;
const MAX_SEQUENCE: u16 = (1 << SEQUENCE_BITS) - 1;

pub type SnowflakeId = u64;

#[derive(Debug)]
struct State {
    last_timestamp: u64,
    sequence: u16,
}

/// Thread-safe snowflake generator
#[derive(Debug, Clone)]
pub struct SnowflakeIdGenerator {
    machine_id: u16,
    state: Arc<Mutex<State>>,
}

impl SnowflakeIdGenerator {
    pub fn new(machine_id: u16) -> Self {
        assert!(machine_id <= MAX_MACHINE_ID, "machine_id out of range");
        // TODO change assert ot be a critical error using our app_core error

        Self {
            machine_id,
            state: Arc::new(Mutex::new(State {
                last_timestamp: 0,
                sequence: 0,
            })),
        }
    }

    pub fn generate(&self) -> SnowflakeId {
        let mut state = self.state.lock();

        let mut timestamp = current_timestamp_ms();

        if timestamp < state.last_timestamp {
            // Clock moved backwards — super rare
            timestamp = state.last_timestamp;
        }

        if timestamp == state.last_timestamp {
            state.sequence = (state.sequence + 1) & MAX_SEQUENCE;
            if state.sequence == 0 {
                // Sequence rollover in same ms — wait for next ms
                while timestamp <= state.last_timestamp {
                    timestamp = current_timestamp_ms();
                }
                state.last_timestamp = timestamp;
            }
        } else {
            state.sequence = 0;
            state.last_timestamp = timestamp;
        }

        let time_part = (timestamp - EPOCH) << (MACHINE_ID_BITS + SEQUENCE_BITS);
        let machine_part = (self.machine_id as u64) << SEQUENCE_BITS;
        let seq_part = state.sequence as u64;

        time_part | machine_part | seq_part
    }
}
