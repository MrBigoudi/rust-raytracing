#[derive(Debug)]
pub enum ErrorCode {
    InitializationFailure,
    CleaningFailure,
    Unknown,
    AccessFailure,
    VulkanFailure,
}
