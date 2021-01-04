use crate::{local_run_loop_executor, run_loop_executor, Reaper};

pub struct FutureSupport {
    main_thread_future_spawner: crate::run_loop_executor::Spawner,
    local_main_thread_future_spawner: crate::local_run_loop_executor::Spawner,
}

impl FutureSupport {
    pub fn new(
        main_thread_future_spawner: crate::run_loop_executor::Spawner,
        local_main_thread_future_spawner: crate::local_run_loop_executor::Spawner,
    ) -> FutureSupport {
        FutureSupport {
            main_thread_future_spawner,
            local_main_thread_future_spawner,
        }
    }

    /// Spawns a future for execution in main thread.
    pub fn spawn_in_main_thread(
        &self,
        future: impl std::future::Future<Output = ()> + 'static + Send,
    ) {
        let spawner = &self.main_thread_future_spawner;
        spawner.spawn(future);
    }

    /// Spawns a future for execution in main thread.
    ///
    /// Panics if not in main thread. The difference to `spawn_in_main_thread()` is that `Send` is
    /// not required. Perfect for capturing `Rc`s.
    pub fn spawn_in_main_thread_from_main_thread(
        &self,
        future: impl std::future::Future<Output = ()> + 'static,
    ) {
        Reaper::get().require_main_thread();
        let spawner = &self.local_main_thread_future_spawner;
        spawner.spawn(future);
    }
}

#[derive(Debug)]
pub struct FutureMiddleware {
    logger: slog::Logger,
    main_thread_executor: run_loop_executor::RunLoopExecutor,
    local_main_thread_executor: local_run_loop_executor::RunLoopExecutor,
}

impl FutureMiddleware {
    pub fn new(
        logger: slog::Logger,
        executor: run_loop_executor::RunLoopExecutor,
        local_executor: local_run_loop_executor::RunLoopExecutor,
    ) -> FutureMiddleware {
        FutureMiddleware {
            logger: logger.clone(),
            main_thread_executor: executor,
            local_main_thread_executor: local_executor,
        }
    }

    pub fn reset(&self) {
        self.discard_tasks();
    }

    fn discard_tasks(&self) {
        let shared_task_count = self.main_thread_executor.discard_tasks();
        let local_task_count = self.local_main_thread_executor.discard_tasks();
        let total_task_count = shared_task_count + local_task_count;
        if total_task_count > 0 {
            slog::warn!(self.logger, "Discarded future tasks on reactivation";
                "task_count" => total_task_count,
            );
        }
    }

    pub fn run(&mut self) {
        self.main_thread_executor.run();
        self.local_main_thread_executor.run();
    }
}
