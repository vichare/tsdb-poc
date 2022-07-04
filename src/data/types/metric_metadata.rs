use std::vec::Vec;
// The metadata of a metric.
// A metric could be either Gauge or Cumulative
// Gauge metric:
// The value of each point represent the value of a gauge metric.
// Cumulative metric:
// The value of the metric is represented by the difference between two adjacent points.

pub struct MetricMetadata {
    value_type: MetricValueType,
}

impl MetricMetadata {}

// Ideally I'd like to assign int values to each enum,
// but rust doesn't allow explicit discriminants for enums with tuple variants.
// TODO: MetricValueType should be comparable.
// TODO: MetricValueType should be able to cast to a i32 hash
#[derive(PartialEq)]
pub enum MetricValueType {
    Int64,
    Double,
    String,
    Bool,
    CumulativeInt64,
    CumulativeDouble,
    Distribution(Vec<f64>),
    Tuple(Vec<MetricValueType>),
}

impl Eq for MetricValueType {}

impl MetricValueType {
    fn is_cumulative(&self) -> bool {
        match self {
            MetricValueType::CumulativeInt64 => true,
            MetricValueType::CumulativeDouble => true,
            MetricValueType::Distribution(_) => true,
            _ => false,
        }
    }
}
