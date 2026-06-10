use std::io::{Write, stdin, stdout};
mod functions;
use functions::cat::cat_process;
use functions::cd::cd_process;
use functions::clear::clear_process;
use functions::command::command_functions;
use functions::echo::echoing;
use functions::jobs::JobManager;
use functions::kill::kill_process;
use functions::ls::ls_process;
use functions::memory::access::access_page;
use functions::memory::alloc::alloc_pages;
use functions::memory::print::print_mem;
use functions::memory::set_replace::set_page_replace;
use functions::memory::MemoryManager;
use functions::mkdir::mkdir_process;
use functions::pwd::pwd_process;
use functions::rm::rm_process;
use functions::rmdir::rmdir_process;
use functions::scheduler::add_process::add_process;
use functions::scheduler::set_scheduler::set_scheduler;
use functions::scheduler::start_scheduler::start_scheduler;
use functions::scheduler::SchedulerManager;
use functions::sync::dining::run_dining_philosophers;
use functions::sync::producer_consumer::run_producer_consumer;
use functions::sync::SyncManager;
use functions::touch::touch_process;

fn main() {
    let mut job_manager = JobManager::new();
    let mut scheduler_manager = SchedulerManager::new();
    let mut memory_manager = MemoryManager::new(8, 4096);
    let mut sync_manager = SyncManager::new();

    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut input = input.trim();
        let is_background = if input.ends_with('&') {
            input = input[..input.len() - 1].trim_end();
            true
        } else {
            false
        };

        // must be peekable so we know when we are on the last command
        let mut commands = input.split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let mut args = parts;
            // args consumed by command handlers below

            match command {
                "addproc" => add_process(&mut scheduler_manager, args),
                "allocpages" => alloc_pages(&mut memory_manager, args),
                "accesspage" => access_page(&mut memory_manager, args),
                "bg" => job_manager.bg_job(args),
                "cat" => cat_process(args),
                "cd" => cd_process(&mut args.peekable(), &mut previous_command),
                "clear" => clear_process(),
                "clearproc" => scheduler_manager.clear_processes(),
                "createmutex" => {
                    let name = args.next().unwrap_or("");
                    if !name.is_empty() {
                        sync_manager.create_mutex(name);
                    } else {
                        println!("Usage: createmutex <name>");
                    }
                }
                "createsem" => {
                    let name = args.next().unwrap_or("");
                    let initial: isize = args.next().and_then(|v| v.parse().ok()).unwrap_or(1);
                    if !name.is_empty() {
                        sync_manager.create_semaphore(name, initial);
                    } else {
                        println!("Usage: createsem <name> [initial_count]");
                    }
                }
                "dining" => run_dining_philosophers(&mut sync_manager),
                "echo" => echoing(args),
                "exit" => return,
                "fg" => job_manager.fg_job(args),
                "jobs" => job_manager.list_jobs(),
                "kill" => kill_process(args),
                "listproc" => scheduler_manager.list_processes(),
                "listsync" => sync_manager.list(),
                "lockmutex" => {
                    let name = args.next().unwrap_or("");
                    let pid: usize = args.next().and_then(|v| v.parse().ok()).unwrap_or(0);
                    if !name.is_empty() {
                        sync_manager.mutex_lock(name, pid);
                    } else {
                        println!("Usage: lockmutex <name> <process_id>");
                    }
                }
                "ls" => ls_process(args),
                "mkdir" => mkdir_process(args),
                "printmem" => print_mem(&memory_manager),
                "procon" => run_producer_consumer(&mut sync_manager),
                "pwd" => pwd_process(),
                "rm" => rm_process(args),
                "rmdir" => rmdir_process(args),
                "semwait" => {
                    let name = args.next().unwrap_or("");
                    let pid: usize = args.next().and_then(|v| v.parse().ok()).unwrap_or(0);
                    if !name.is_empty() {
                        sync_manager.sem_wait(name, pid);
                    } else {
                        println!("Usage: semwait <name> <process_id>");
                    }
                }
                "semsignal" => {
                    let name = args.next().unwrap_or("");
                    if !name.is_empty() {
                        sync_manager.sem_signal(name);
                    } else {
                        println!("Usage: semsignal <name>");
                    }
                }
                "setpagereplace" => set_page_replace(&mut memory_manager, args),
                "setscheduler" => set_scheduler(&mut scheduler_manager, args),
                "startscheduler" => start_scheduler(&mut scheduler_manager),
                "touch" => touch_process(args),
                "unlockmutex" => {
                    let name = args.next().unwrap_or("");
                    let pid: usize = args.next().and_then(|v| v.parse().ok()).unwrap_or(0);
                    if !name.is_empty() {
                        sync_manager.mutex_unlock(name, pid);
                    } else {
                        println!("Usage: unlockmutex <name> <process_id>");
                    }
                }
                command => command_functions(
                    command,
                    args,
                    &mut previous_command,
                    &mut commands,
                ),
            }
        }

        if is_background {
            if let Some(final_command) = previous_command {
                job_manager.add_job(final_command, input.to_string());
            }
        } else if let Some(mut final_command) = previous_command {
            let _ = final_command.wait();
        }
    }
}
