use parking_lot::Mutex;
use std::sync::Arc;

use crate::util::current_timestamp_ms;

/// Custom epoch to reduce timestamp size in the ID
const EPOCH: u64 = 1_700_000_000_000; // e.g. corresponds to specific UTC time

/// Number of bits allocated for machine ID
const MACHINE_ID_BITS: u8 = 10;

/// Number of bits allocated for the per-millisecond sequence
const SEQUENCE_BITS: u8 = 12;

/// Maximum values derived from bit allocation
const MAX_MACHINE_ID: u16 = (1 << MACHINE_ID_BITS) - 1;
const MAX_SEQUENCE: u16 = (1 << SEQUENCE_BITS) - 1;

/// Alias for clarity
pub type SnowflakeId = u64;

/// Internal generator state, protected by a mutex for thread safety
#[derive(Debug)]
struct State {
    last_timestamp: u64, // Last timestamp Snowflake ID was generated
    sequence: u16,       // Sequence number for the current millisecond
}

/// Thread-safe unique ID generator based on the Snowflake pattern
#[derive(Debug, Clone)]
pub struct SnowflakeIdGenerator {
    machine_id: u16,          // Unique machine identifier
    state: Arc<Mutex<State>>, // Shared mutable state
}

impl SnowflakeIdGenerator {
    /// Create new instance with the given machine ID
    ///
    /// # Panics if the machine ID exceeds the allowed bit space.
    pub fn new(machine_id: u16) -> Self {
        assert!(machine_id <= MAX_MACHINE_ID, "machine_id out of range");
        // TODO: better error handling

        Self {
            machine_id,
            state: Arc::new(Mutex::new(State {
                last_timestamp: 0,
                sequence: 0,
            })),
        }
    }

    /// Generate a unique Snowflake ID (unique to this gen instance)
    ///
    /// Ensures uniqueness across threads and time, by combining:
    /// - timestamp (relative to custom epoch)
    /// - machine ID
    /// - per-millisecond sequence number
    pub fn generate(&self) -> SnowflakeId {
        let mut state = self.state.lock();

        let mut timestamp = current_timestamp_ms();

        // Handle clock rollback: fallback to last known timestamp
        if timestamp < state.last_timestamp {
            timestamp = state.last_timestamp;
        }

        if timestamp == state.last_timestamp {
            // Same millisecond: increment the sequence
            state.sequence = (state.sequence + 1) & MAX_SEQUENCE;

            // If sequence overflows, wait for the next millisecond
            if state.sequence == 0 {
                while timestamp <= state.last_timestamp {
                    timestamp = current_timestamp_ms();
                }
                state.last_timestamp = timestamp;
            }
        } else {
            // New millisecond: reset sequence
            state.sequence = 0;
            state.last_timestamp = timestamp;
        }

        // Compose ID: timestamp | machine_id | sequence
        let time_part = (timestamp - EPOCH) << (MACHINE_ID_BITS + SEQUENCE_BITS);
        let machine_part = (self.machine_id as u64) << SEQUENCE_BITS;
        let seq_part = state.sequence as u64;

        time_part | machine_part | seq_part
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::thread;
    use std::time::Duration;

    fn decompose_id(id: SnowflakeId) -> (u64, u16, u16) {
        let timestamp = (id >> (MACHINE_ID_BITS + SEQUENCE_BITS)) + EPOCH;
        let machine_id = ((id >> SEQUENCE_BITS) & ((1 << MACHINE_ID_BITS) - 1)) as u16;
        let sequence = (id & ((1 << SEQUENCE_BITS) - 1)) as u16;
        (timestamp, machine_id, sequence)
    }

    #[test]
    fn test_single_id_generation() {
        let gen = SnowflakeIdGenerator::new(1);
        let id = gen.generate();
        let (timestamp, machine_id, sequence) = decompose_id(id);

        assert!(timestamp >= EPOCH);
        assert_eq!(machine_id, 1);
        assert!(sequence <= MAX_SEQUENCE);
    }

    #[test]
    fn test_monotonic_ids() {
        let gen = SnowflakeIdGenerator::new(2);
        let mut prev = gen.generate();
        for _ in 0..1000 {
            let current = gen.generate();
            assert!(current > prev, "IDs should be monotonic");
            prev = current;
        }
    }

    #[test]
    fn test_thread_safety() {
        let gen = Arc::new(SnowflakeIdGenerator::new(3));
        let mut handles = vec![];
        let id_count = 10_000;
        let thread_count = 4;

        let results = Arc::new(Mutex::new(Vec::with_capacity(id_count * thread_count)));

        for _ in 0..thread_count {
            let g = Arc::clone(&gen);
            let r = Arc::clone(&results);
            let handle = thread::spawn(move || {
                for _ in 0..id_count {
                    let id = g.generate();
                    r.lock().push(id);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let ids = results.lock();
        let unique_ids: HashSet<_> = ids.iter().cloned().collect();
        assert_eq!(unique_ids.len(), ids.len(), "Duplicate IDs found!");
    }

    #[test]
    fn test_machine_id_boundary() {
        let valid = SnowflakeIdGenerator::new(MAX_MACHINE_ID); // Should not panic

        let result = std::panic::catch_unwind(|| {
            SnowflakeIdGenerator::new(MAX_MACHINE_ID + 1); // Should panic
        });

        assert!(result.is_err(), "Expected panic on invalid machine ID");
        drop(valid); // avoid warning
    }

    #[test]
    fn test_sequence_rollover_behavior() {
        let gen = SnowflakeIdGenerator::new(4);

        {
            // Lock state and simulate same timestamp repeatedly to force sequence wrap
            let mut state = gen.state.lock();
            state.last_timestamp = current_timestamp_ms();
            state.sequence = MAX_SEQUENCE;
        }

        // Next generate should trigger rollover wait (if it happens in same ms)
        let id = gen.generate();
        let (_, _, sequence) = decompose_id(id);

        assert_eq!(sequence, 0); // After wrap, sequence should reset
    }

    #[test]
    fn test_unique_ids_across_milliseconds() {
        let gen = SnowflakeIdGenerator::new(5);
        let id1 = gen.generate();
        thread::sleep(Duration::from_millis(2));
        let id2 = gen.generate();

        let (ts1, _, _) = decompose_id(id1);
        let (ts2, _, _) = decompose_id(id2);

        assert!(ts2 > ts1, "Later ID should have greater timestamp");
    }
}
