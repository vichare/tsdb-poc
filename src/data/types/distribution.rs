use std::vec::Vec;

use crate::data::types::basic_types::Value;

pub struct Distribution<'a> {
    bucket: &'a DistributionBucket,
    stats: Vec<DistributionBucketStats>,
}

pub struct DistributionBucket {
    bounds: Vec<f64>,
}

struct DistributionBucketStats {
    // The number of points in this bucket.
    count: i64,

    // The sum of all the points in this bucket.
    sum: f64,

    // The sum of squared deviation.
    // E.g., if there are 3 points: x1, x2 and x3 in this bucket,
    // avg = (x1 + x2 + x3) / 3;
    // var = (x1 - avg) ^ 2 + (x2 - avg) ^ 2 + (x3 - avg) ^ 2
    // To calculate this value in an accumulative manner, we know that:
    // sum = x1 + x2 + x3
    // var = x1 ^ 2 + x2 ^ 2 + x3 ^ 2 - sum ^ 2 / 3
    var: f64,
}

impl Value for Distribution<'_> {}

#[cfg(test)]
mod tests {
    use super::DistributionBucket;

    #[test]
    fn create_distribution() -> Result<(), String> {
        let bucket_bounds = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
        let _bucket = DistributionBucket {
            bounds: bucket_bounds,
        };
        Ok(())
    }
}
