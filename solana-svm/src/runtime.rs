use grid_core::GridRuntime;

/*
 * Steps to have a working Runtime SVM
 * 1. Define Processing Callback
 * 2. Define Processing Environment
 * 3. (Optional) Configure Processor */

pub struct SvmRuntime {}

impl GridRuntime for SvmRuntime {}
