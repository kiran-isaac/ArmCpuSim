use crate::*;
use std::collections::VecDeque;

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

    instr_queue: VecDeque<(I, u8)>,
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

            instr_queue: VecDeque::new(),
        }
    }

    /// Returns PC, and the instruction count
    pub fn tick(&mut self) -> (u32, u32) {
        if !self.config.pipelined && self.config.executors == 1 {
            if self.state.regs.pc == 0x20 {
                print!("");
            }
            if self.instr_queue.len() <= 4 {
                let instruction = self.state.mem.get_instruction(self.state.regs.pc);
                for i in decode2(decode(instruction)) {
                    self.instr_queue.push_back((i, false));
                }
            }

            let (i, i_is_32_bit) = self.instr_queue.pop_front().unwrap();

            self.executor_pool.assign(i, i_is_32_bit);
            // Run all instructions to completion
            self.executor_pool.flush(&mut self.state)
        } else {
            unimplemented!()
        };

        (self.state.regs.pc, self.executor_pool.executed_count)
    }
}
