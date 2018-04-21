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
        let worker_id_bits = 5;
        let datacenter_id_bits = 5;
        let sequence_bits = 12;
        let worker_id_shift = sequence_bits;
        let timestamp_left_shift = sequence_bits + worker_id_bits + datacenter_id_bits;
        let sequence_mask = ((-1 as i64) ^ (-1 << sequence_bits)) as u64;

        let seq = (self.sequence + 1) & sequence_mask;

        let id = ((time::precise_time_ns() - self.epoch) << timestamp_left_shift)
            | (self.worker_id << worker_id_shift) | (self.sequence);

        self.sequence = seq;

        return id;
    }
}

pub fn new(worker_id: u64) -> Generator {
    return Generator {
        worker_id: worker_id,
        epoch: 10,
        sequence: 0,
        lasttime: 0,
    };
}
