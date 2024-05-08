use cairo_lang_sierra::program::VersionedProgram;
use forge_runner::compiled_runnable::{CrateLocation, FuzzerConfig, TestDetails};
use forge_runner::expected_result::ExpectedTestResult;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CompiledTestCrateRaw {
    pub sierra_program: VersionedProgram,
    pub test_cases: Vec<TestCaseRaw>,
    pub tests_location: CrateLocation,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct TestCaseRaw {
    pub name: String,
    pub available_gas: Option<usize>,
    pub ignored: bool,
    pub expected_result: ExpectedTestResult,
    pub fork_config: Option<RawForkConfig>,
    pub fuzzer_config: Option<FuzzerConfig>,
    pub test_details: TestDetails,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum RawForkConfig {
    Id(String),
    Params(RawForkParams),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RawForkParams {
    pub url: String,
    pub block_id_type: String,
    pub block_id_value: String,
}
