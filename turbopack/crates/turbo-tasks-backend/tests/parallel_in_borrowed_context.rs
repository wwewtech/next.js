#![allow(clippy::needless_return)] // tokio macro-generated code doesn't respect this

//! `turbo_tasks::parallel` helpers must work while the turbo-tasks task-local is borrowed, e.g.
//! inside `with_turbo_tasks`, where backend code such as a batched task restore can run.
//!
//! The calling thread drains jobs of its own scope inline, so a job that re-entered
//! `turbo_tasks_scope` there would panic with "cannot enter a task-local scope while the
//! task-local storage is borrowed".

mod util;

use turbo_tasks::{parallel, with_turbo_tasks};

use crate::util::create_tt;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn parallel_helpers_inside_with_turbo_tasks() {
    let (tt, _persistence_dir) = create_tt("parallel_helpers_inside_with_turbo_tasks");
    let result = turbo_tasks::run_once(tt.clone(), async move {
        // Plenty of items so that several jobs are queued and the calling thread runs some of
        // them itself.
        let items: Vec<u32> = (0..10_000).collect();
        let doubled = with_turbo_tasks(|_| {
            parallel::map_collect::<_, _, Vec<_>>(&items, |&item| {
                // The context is available inside every job, inline or on a helper.
                with_turbo_tasks(|_| item * 2)
            })
        });
        assert_eq!(
            doubled,
            items.iter().map(|item| item * 2).collect::<Vec<_>>()
        );

        let sum = with_turbo_tasks(|_| {
            parallel::map_collect_owned::<_, _, Vec<_>>(items.clone(), u64::from)
                .into_iter()
                .sum::<u64>()
        });
        assert_eq!(sum, items.iter().map(|&item| u64::from(item)).sum::<u64>());
        anyhow::Ok(())
    })
    .await;
    tt.stop_and_wait().await;
    result.unwrap();
}
