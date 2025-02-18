use std::num::NonZeroUsize;

use crate::selector::SelectIp;

#[derive(Debug, Clone)]
pub(super) struct IpBatcher<Ip, S> {
    batch: Vec<Ip>,
    size: usize,
    selector: S,
}

impl<Ip: Clone, S: SelectIp<Ip>> IpBatcher<Ip, S> {
    pub(super) fn new(size: NonZeroUsize, selector: S) -> Self {
        let batch = Vec::with_capacity(size.into());
        Self {
            batch,
            size: size.into(),
            selector,
        }
    }

    pub(super) fn next_batch(&mut self) -> Option<&[Ip]> {
        self.batch.clear();

        for _ in 0..self.size {
            if let Some(next_ip) = self.selector.select() {
                self.batch.push(next_ip.clone());
            } else {
                return Some(&self.batch);
            }
        }
        Some(&self.batch)
    }
}
