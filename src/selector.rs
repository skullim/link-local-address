pub trait SelectIp<Ip> {
    fn select(&mut self) -> Option<&Ip>;
}

pub struct SequentialIpSelector<Ip> {
    ip_range: Vec<Ip>,
    index: usize,
}

impl<Ip> SequentialIpSelector<Ip> {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Ip>,
    {
        Self {
            ip_range: iter.into_iter().collect(),
            index: 0,
        }
    }
}

impl<Ip> SelectIp<Ip> for SequentialIpSelector<Ip> {
    fn select(&mut self) -> Option<&Ip> {
        let index = self.index;
        if index < self.ip_range.len() {
            self.index += 1;
            Some(&self.ip_range[index])
        } else {
            None
        }
    }
}
