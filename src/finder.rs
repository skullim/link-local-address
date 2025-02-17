use crate::{batcher::IpBatcher, probe::HostProber, selector::SelectIp};

pub struct FreeIpFinder<Ip, S> {
    ip_batcher: IpBatcher<Ip, S>,
    host_prober: HostProber<Ip>,
}

impl<Ip, S> FreeIpFinder<Ip, S> {
    pub fn new(ip_batcher: IpBatcher<Ip, S>, host_prober: HostProber<Ip>) -> Self {
        Self {
            ip_batcher,
            host_prober,
        }
    }
}

/// Client is expected to handle the response in following way:
/// Some(vec![]) -> there are no free ips from the last ip batch, call function again to try subsequent batch
/// None -> iterator has been exhausted, all batches has been processed
impl<Ip: Clone, S: SelectIp<Ip>> FreeIpFinder<Ip, S> {
    pub async fn find_next(&mut self) -> Option<Vec<Ip>> {
        if let Some(batch) = self.ip_batcher.next_batch() {
            let outcomes = self.host_prober.probe(batch).await;
            Some(
                outcomes
                    .into_iter()
                    .filter_map(|outcome| outcome.ok())
                    .filter(|ok_outcome| ok_outcome.is_free())
                    .map(|free_outcome| free_outcome.target_ip().clone())
                    .collect(),
            )
        } else {
            None
        }
    }
}
