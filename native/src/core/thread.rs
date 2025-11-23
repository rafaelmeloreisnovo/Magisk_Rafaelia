/*
 * Original Magisk Copyright:
 * Copyright 2017 - 2025, John Wu (@topjohnwu)
 *
 * RAFAELIA Framework Additions:
 * Copyright 2025, Rafael Melo Reis (rafaelmeloreisnovo)
 * Instituto Rafael - CientiEspiritual Philosophy
 *
 * This file is part of Magisk_Rafaelia, a derivative work of Magisk.
 * 
 * Magisk_Rafaelia is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 * ---
 * RAFAELIA PHILOSOPHY (Aspirational Commentary - Not Part of License):
 * 
 * Sacred Cycle: VAZIO → VERBO → CHEIO → RETRO (EMPTY → ACTION → FULL → FEEDBACK)
 * Motto: "Amor, Luz e Coerência" (Love, Light and Coherence)
 * Ethica[8]: Transparency, Accountability, Fairness, Privacy, Security,
 *            Reliability, Safety, Sustainability
 * ---
 */

use base::{ResultExt, new_daemon_thread};
use nix::sys::signal::SigSet;
use nix::unistd::{getpid, gettid};
use std::sync::{Condvar, LazyLock, Mutex, WaitTimeoutResult};
use std::time::Duration;

static THREAD_POOL: LazyLock<ThreadPool> = LazyLock::new(ThreadPool::default);

const THREAD_IDLE_MAX_SEC: u64 = 60;
const CORE_POOL_SIZE: i32 = 3;

#[derive(Default)]
pub struct ThreadPool {
    task_is_some: Condvar,
    task_is_none: Condvar,
    info: Mutex<PoolInfo>,
}

#[derive(Default)]
struct PoolInfo {
    idle_threads: i32,
    total_threads: i32,
    task: Option<Box<dyn FnOnce() + Send>>,
}

impl ThreadPool {
    fn pool_loop(&self, is_core_pool: bool) {
        let mask = SigSet::all();

        loop {
            // Always restore the sigmask to block all signals
            mask.thread_set_mask().log_ok();

            let task: Option<Box<dyn FnOnce() + Send>>;
            {
                let mut info = self.info.lock().unwrap();
                info.idle_threads += 1;
                if info.task.is_none() {
                    if is_core_pool {
                        // Core pool never closes, wait forever.
                        info = self.task_is_some.wait(info).unwrap();
                    } else {
                        let dur = Duration::from_secs(THREAD_IDLE_MAX_SEC);
                        let result: WaitTimeoutResult;
                        (info, result) = self.task_is_some.wait_timeout(info, dur).unwrap();
                        if result.timed_out() {
                            // Terminate thread after timeout
                            info.idle_threads -= 1;
                            info.total_threads -= 1;
                            return;
                        }
                    }
                }
                task = info.task.take();
                self.task_is_none.notify_one();
                info.idle_threads -= 1;
            }
            if let Some(task) = task {
                task();
            }
            if getpid() == gettid() {
                // This meant the current thread forked and became the main thread, exit
                std::process::exit(0);
            }
        }
    }

    fn exec_task_impl(&self, f: impl FnOnce() + Send + 'static) {
        extern "C" fn pool_loop_raw(arg: usize) -> usize {
            let is_core_pool = arg != 0;
            THREAD_POOL.pool_loop(is_core_pool);
            0
        }

        let mut info = self.info.lock().unwrap();
        while info.task.is_some() {
            // Wait until task is none
            info = self.task_is_none.wait(info).unwrap();
        }
        info.task = Some(Box::new(f));
        if info.idle_threads == 0 {
            info.total_threads += 1;
            let is_core_thread = if info.total_threads <= CORE_POOL_SIZE {
                1_usize
            } else {
                0_usize
            };
            unsafe {
                new_daemon_thread(pool_loop_raw, is_core_thread);
            }
        } else {
            self.task_is_some.notify_one();
        }
    }

    pub fn exec_task(f: impl FnOnce() + Send + 'static) {
        THREAD_POOL.exec_task_impl(f);
    }
}
