pub enum Action {
    ComputeDistances,
}

pub enum Job {
    ComputeDistances,
}

pub enum JobResult {
    Distance(u32),
    TotDistance(u32),
}
