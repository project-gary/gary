use std::process::Command;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

// args specific to process executor
struct ProcessArgs<'pa> {
    cmd: &'pa str,
    args: Option<&'pa str>, // this should be an iterable
}

// commands sendable to process runner
enum ProcessRunnerCmd {
    NewCount(usize),
    Slaughter(),
}

fn main() {
    let cmds = [
        ProcessArgs {
            cmd: "/bin/echo",
            args: Some("hello world"),
        },
        ProcessArgs {
            cmd: "/bin/echoo",
            args: Some("hello world"),
        },
        ProcessArgs {
            cmd: "/bin/date",
            args: None,
        },
        ProcessArgs {
            cmd: "/bin/sleep",
            args: Some("inf"),
        },
    ];

    //let (p_runner_ctl_tx, p_runner_ctl_rx) = channel::<ProcessRunnerCmd>();
    let (p_runner_ctl_tx, p_runner_ctl_rx) = channel();
    //let (p_runner_stat_tx, p_runner_stat_rx) = channel();

    let runner = thread::spawn(move || {
        t_process_runner(1, p_runner_ctl_rx, &cmds[3]);
    });

    // TODO: make controller wait until runner has started?
    let controller = thread::spawn(move || {
        t_process_controller(p_runner_ctl_tx);
    });

    controller.join().unwrap_or_else(|e| {
        panic!("Some kind of error happened: {:?}", e);
    });
    runner.join().unwrap_or_else(|e| {
        panic!("Some kind of error happened: {:?}", e);
    });
}

/*  TODO: this should maybe watch a config file for changes and listen for
 *  commands from the cluster control APIs?
 */
fn t_process_controller(ctl_tx: Sender<ProcessRunnerCmd>) {
    thread::sleep(std::time::Duration::from_secs(2));
    ctl_tx
        .send(ProcessRunnerCmd::NewCount(4))
        .unwrap_or_else(|e| {
            panic!("Failed to send message: {:?}", e);
        });
    thread::sleep(std::time::Duration::from_secs(2));
    ctl_tx
        .send(ProcessRunnerCmd::NewCount(6))
        .unwrap_or_else(|e| {
            panic!("Failed to send message: {:?}", e);
        });
    thread::sleep(std::time::Duration::from_secs(2));
    ctl_tx
        .send(ProcessRunnerCmd::NewCount(2))
        .unwrap_or_else(|e| {
            panic!("Failed to send message: {:?}", e);
        });
    thread::sleep(std::time::Duration::from_secs(2));
    ctl_tx
        .send(ProcessRunnerCmd::Slaughter())
        .unwrap_or_else(|e| {
            panic!("Failed to send message: {:?}", e);
        });
    println!("Exiting process controllers");
}

fn t_process_runner(initial_count: usize, ctl_rx: Receiver<ProcessRunnerCmd>, args: &ProcessArgs) {
    let mut count = initial_count;
    let mut running = Vec::with_capacity(count);
    let mut dead = Vec::new();

    println!("Starting with {} instances", count);

    // TODO: use a future to check for message and check for process status
    //       That can block instead of pool loop
    //       Adjusting procs can then happen on changed count or kid death
    loop {
        // Hey Terry; are there any messages waiting for me?
        let msg = ctl_rx.try_recv();
        if !msg.is_err() {
            // handle the message
            match msg.unwrap() {
                ProcessRunnerCmd::NewCount(c) => {
                    if c != count {
                        println!("Updating count to {} instances", c);
                        count = c;
                    }
                }
                ProcessRunnerCmd::Slaughter() => {
                    println!("killing everything");
                    count = 0;
                }
            };
        }

        // increase running instances as needed
        while running.len() < count {
            // there needs to be a way to get the places to run; add to launcher
            let kid = child_launcher(args);
            println!(
                "Spawned process {} ({}/{})",
                kid.id(),
                running.len() + 1,
                count
            );
            running.push(kid);
        }

        // newest kids are on the right, so those are the ones to kill
        while running.len() > count {
            let mut kid = running.pop().unwrap();
            println!(
                "Killing process {} ({}/{})",
                kid.id(),
                running.len() + 1,
                count
            );
            kid.kill().expect("Command was already dead")
        }

        // check for dead kids
        for i in 0..running.len() {
            match running[i].try_wait() {
                Ok(Some(status)) => {
                    println!("Process {} exited w/ {}", running[i].id(), status);
                    dead.push(i);
                }
                Ok(None) => {
                    //println!("Process {} is ok", running[i].id());
                }
                Err(e) => {
                    println!("Failed checking process: {}", e);
                    dead.push(i);
                }
            };
        }

        // remove *after* iterating so we don't screw up vector length
        if dead.len() > 0 {
            // remove biggest to smallest index
            dead.reverse();
            for d in &dead {
                running.remove(*d);
            }
            dead.clear();
        }

        if count == 0 {
            // exit thread after cleanup
            println!("Exiting runner");
            break;
        }

        /* // this'd be great
        running.retain(|&mut kid| {
            match kid.try_wait() {
                Ok(Some(status)) => return true,
                Ok(None) => return false,
                Err(e) => return true,
            }
        })
        */
    }
}

// TODO: alter this to select a host to run upon
fn child_launcher(cmd_params: &ProcessArgs) -> std::process::Child {
    let mut cmd = Command::new(cmd_params.cmd);
    if cmd_params.args.is_some() {
        cmd.arg(cmd_params.args.unwrap());
    }
    let kid = cmd
        .spawn()
        .expect(&format!("failed to run '{}'", cmd_params.cmd));

    return kid;
}
