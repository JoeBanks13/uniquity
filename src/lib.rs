// Reference from https://github.com/twitter/snowflake:
// long nextId = ((timestamp - twepoch) << this.timestampLeftShift) | (this.workerId << this.workerIdShift) | (this.sequence);
//
// private[this] val workerIdBits = 5L
// private[this] val datacenterIdBits = 5L
// private[this] val maxWorkerId = -1L ^ (-1L << workerIdBits)
// private[this] val maxDatacenterId = -1L ^ (-1L << datacenterIdBits)
// private[this] val sequenceBits = 12L
// private[this] val workerIdShift = sequenceBits
// private[this] val datacenterIdShift = sequenceBits + workerIdBits
// private[this] val timestampLeftShift = sequenceBits + workerIdBits + datacenterIdBits
// private[this] val sequenceMask = -1L ^ (-1L << sequenceBits)
// private[this] var lastTimestamp = -1L

#![feature(test)]

const WORKER_ID_BITS: u64 = 5;
const DATACENTER_ID_BITS: u64 = 5;
const SEQUENCE_BITS: u64 = 12;
const WORKER_ID_SHIFT: u64 = SEQUENCE_BITS;
const TIMESTAMP_LEFT_SHIFT: u64 = SEQUENCE_BITS + WORKER_ID_BITS + DATACENTER_ID_BITS;
const SEQUENCE_MASK: u64 = ((-1 as i64) ^ (-1 << SEQUENCE_BITS)) as u64;

#[cfg(test)]
mod tests;

extern crate time;

#[derive(Debug, PartialEq)]
pub struct Generator {
    worker_id: u64,
    epoch: u64,
    sequence: u64,
    lasttime: u64,
}

impl Generator {
    pub fn generate(&mut self) -> u64 {
        let time = time::precise_time_ns();

        let mut seq = 0;

        if time == self.lasttime {
            seq = (self.sequence + 1) & SEQUENCE_MASK;
            self.sequence = seq;
        }

        let id = (time - self.epoch) << TIMESTAMP_LEFT_SHIFT | (self.worker_id << WORKER_ID_SHIFT)
            | (seq);

        return id;
    }
}

pub fn new(worker_id: u64) -> Generator {
    return Generator {
        worker_id: worker_id,
        epoch: 0,
        sequence: 0,
        lasttime: 0,
    };
}
