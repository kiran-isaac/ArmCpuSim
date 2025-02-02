use crate::*;

#[derive(Clone, Copy)]
pub struct RunConfig {
    pub executors: usize,
    pub pipelined: bool,
}

impl RunConfig {
    pub fn no_pipeline_no_superscalar() -> Self {
        RunConfig {
            executors: 1,
            pipelined: false,
        }
    }

    pub fn pipeline_no_superscalar() -> Self {
        RunConfig {
            executors: 1,
            pipelined: false,
        }
    }
}

pub struct Runner {
    state: ProcessorState,
    executor_pool: ExecutorPool,
    config: RunConfig,
}

impl Runner {
    pub fn from_config(config: &RunConfig, state: ProcessorState) -> Runner {
        // create trace dir if it doesn't exist
        std::fs::create_dir_all("traces").unwrap();
        let tracer = Tracer::new("traces/trace.csv", &state.regs);
        let log_file_path = "traces/log.txt";
        let stack_dump_file = "traces/stack_dump.txt";

        let executor_pool =
            ExecutorPool::new(config.executors, tracer, &log_file_path, &stack_dump_file);

        Runner {
            state,
            executor_pool,
            config: *config,
        }
    }

    /// Returns PC, and the instruction count
    pub fn tick(&mut self) -> (u32, u32) {
        if !self.config.pipelined && self.config.executors == 1 {
            let instruction = self.state.mem.get_instruction(self.state.regs.pc);
            let decoded = decode(instruction);
            self.executor_pool.assign(decoded, is_32_bit(instruction));
            // Run all instructions to completion
            self.executor_pool.flush(&mut self.state);
        } else {
            unimplemented!()
        };

        (self.state.regs.pc, self.executor_pool.executed_count)
    }
}
