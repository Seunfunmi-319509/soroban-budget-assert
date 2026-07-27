//! Minimal stand-in for `soroban_sdk::Env` for the UI tests.
//!
//! The macros emit `env.cost_estimate().budget().<metric>()` and/or
//! `env.cost_estimate().resources().<field>`, so the UI tests can exercise
//! every body shape against this mock instead of pulling in the SDK and
//! compiling a contract. Reported costs are fixed per instance so a test can
//! decide up front whether the injected assertion should pass or panic.

#![allow(dead_code)]

pub struct Env {
    cpu: u64,
    mem: u64,
    read_bytes: u32,
    write_bytes: u32,
}

pub struct CostEstimate<'a> {
    env: &'a Env,
}

pub struct Budget<'a> {
    env: &'a Env,
}

#[derive(Clone, Copy)]
pub struct Resources {
    pub read_bytes: u32,
    pub write_bytes: u32,
}

impl Env {
    pub fn new(cpu: u64, mem: u64) -> Self {
        Env {
            cpu,
            mem,
            read_bytes: 0,
            write_bytes: 0,
        }
    }

    pub fn with_read_write(mut self, read_bytes: u32, write_bytes: u32) -> Self {
        self.read_bytes = read_bytes;
        self.write_bytes = write_bytes;
        self
    }

    pub fn cost_estimate(&self) -> CostEstimate<'_> {
        CostEstimate { env: self }
    }
}

impl<'a> CostEstimate<'a> {
    pub fn budget(&self) -> Budget<'a> {
        Budget { env: self.env }
    }

    pub fn resources(&self) -> Resources {
        Resources {
            read_bytes: self.env.read_bytes,
            write_bytes: self.env.write_bytes,
        }
    }
}

impl Budget<'_> {
    pub fn cpu_instruction_cost(&self) -> u64 {
        self.env.cpu
    }

    pub fn memory_bytes_cost(&self) -> u64 {
        self.env.mem
    }
}

/// Runs `f`, returning the budget assertion's panic message if it panicked.
pub fn budget_panic<F: FnOnce() -> R + std::panic::UnwindSafe, R>(f: F) -> Option<String> {
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(f);
    std::panic::set_hook(previous_hook);

    match result {
        Ok(_) => None,
        Err(payload) => Some(
            payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default(),
        ),
    }
}
