


pub trait MetricMeta {
    fn query(&self) -> String;
    fn name(&self) -> String;
    fn exported_job(&self) -> String;
    fn program_id(&self) -> String;
}