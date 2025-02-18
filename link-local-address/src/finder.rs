use crate::{batcher::IpBatcher, error::Result, probe::ProbeHost, selector::SelectIp};

#[derive(Debug, Clone)]
pub(super) struct FreeIpFinder<Ip, S, P> {
    ip_batcher: IpBatcher<Ip, S>,
    host_prober: P,
}

impl<Ip, S, P: ProbeHost<Ip>> FreeIpFinder<Ip, S, P> {
    pub(super) fn new(ip_batcher: IpBatcher<Ip, S>, host_prober: P) -> Self {
        Self {
            ip_batcher,
            host_prober,
        }
    }
}

/// Client is expected to handle the response in following way:
/// Some(vec![]) -> there are no free ips from the last ip batch, call function again to try subsequent batch
/// None -> iterator has been exhausted, all batches has been processed
impl<Ip: Clone, S: SelectIp<Ip>, P: ProbeHost<Ip>> FreeIpFinder<Ip, S, P> {
    pub(super) async fn find_next(&mut self) -> Result<Option<Vec<Ip>>> {
        if let Some(batch) = self.ip_batcher.next_batch() {
            let outcomes = self.host_prober.probe(batch).await?;
            Ok(Some(
                outcomes
                    .into_iter()
                    .filter(|outcome| outcome.is_free())
                    .map(|free_outcome| free_outcome.target_ip().clone())
                    .collect(),
            ))
        } else {
            Ok(None)
        }
    }
}
