use geo::{Haversine, algorithm::vincenty_distance::VincentyDistance, prelude::Distance};
use std::{fs::File, io::Read, path::PathBuf, time::Instant}; // still needed

use gpx::{Gpx, errors::GpxError, read};
use thiserror::Error;

use crate::action::{Job, JobResult};

pub fn work(job: Job, full_path: &PathBuf) -> Result<JobResult, JobError> {
    match job {
        Job::ComputeDistance(path) => {
            let file_path = full_path.join(&path);
            let mut dist_m: u32 = 0;

            // Read the GPX file
            let gpx = read_gpx(&file_path)?;
            
            // For each track, segment, point, sum dist to next
            for track in &gpx.tracks {
                for segment in &track.segments {
                    let points = &segment.points;
                    for w in points.windows(2) {
                        let p1 = w[0].point();
                        let p2 = w[1].point();
                        let d_m: f64 = match p1.vincenty_distance(&p2) {
                            Ok(d) => d,
                            // Use haversine if vicenty fails
                            Err(_) => Haversine.distance(p1, p2),
                        };
                        dist_m += d_m as u32;
                    }
                }
            }

            Ok(JobResult::Distance(path, dist_m))
        }
        Job::ComputeDistances => {
            // Compute total distance (placeholder)
            let tot_dist: u32 = 0;
            Ok(JobResult::TotDistance(tot_dist))
        }
    }
}

/// Read Gpx and map errors
fn read_gpx(file_path: &PathBuf) -> Result<Gpx, JobError> {
    let mut buf = String::new();

    File::open(&file_path)
        .map_err(|e| JobError::ReadFile(e.into()))?
        .read_to_string(&mut buf)
        .map_err(|e| JobError::ReadFile(e.into()))?;

    let cleaned = buf.strip_prefix("\u{feff}").unwrap_or(&buf);

    let gpx: Gpx = read(cleaned.as_bytes()).map_err(|e| JobError::ReadFile(e.into()))?;
    Ok(gpx)
}

#[derive(Debug, Error)]
pub enum JobError {
    #[error("Read file error: {0}")]
    ReadFile(#[from] ReadFileError),
}

#[derive(Debug, Error)]
pub enum ReadFileError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Gpx error: {0}")]
    Gpx(#[from] GpxError),
}
