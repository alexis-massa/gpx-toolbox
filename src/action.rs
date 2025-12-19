use std::path::PathBuf;


pub enum Action {
    ComputeDistance(PathBuf),
}

pub enum Job {
    ComputeDistance(PathBuf),
    ComputeDistances,
}

pub enum JobResult {
    Distance(PathBuf, u32),
    TotDistance(u32),
}
