use core::data::{Deployment, ProcessSpec, Spec};
use std::process::Command;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
// use executor; // someday

/*
  Figure out which nodes support process
  Elect a manager from that list
  Provide deployment info to manager
*/
pub fn deploy(d: Deployment) {
    println!("Deploy!");

    // something like this:
    //let manager = executor::elect_manager("type=process");
    //manager.send(manager_command::new_deployment(d))
    //
    // Then the elected manager starts deployment_manage()
    // and waits for manger commands that it forwards to the
    // manage channel, which forwards to the nodes

    let (p_runner_ctl_tx, p_runner_ctl_rx) = channel();
    let runner = thread::spawn(move || {
        deployment_manage(d, p_runner_ctl_rx);
    });
    let controller = thread::spawn(move || {
        dummy_controller(p_runner_ctl_tx);
    });

    controller.join().unwrap_or_else(|e| {
        panic!("Some kind of error happened: {:?}", e);
    });
    runner.join().unwrap_or_else(|e| {
        panic!("Some kind of error happened: {:?}", e);
    });
}

pub fn dummy_controller(ctl_tx: Sender<ProcessRunnerCmd>) {
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
    println!("Exiting process controller");
}

/***************************************/
/* manage a deployment */
pub enum ProcessRunnerCmd {
    NewCount(usize),
    Slaughter(),
}

// TODO: this should ask each involved node to local_manage a process
// then the kid.alive check shoud be listening from a status heartbeat
fn deployment_manage(d: Deployment, ctl_rx: Receiver<ProcessRunnerCmd>) {
    let mut count = d.spec.replicas;
    let mut running = Vec::with_capacity(count);
    let mut dead = Vec::new();

    println!("Starting with {} instances", count);

    // TODO: use a future to check for message and check for process status
    //       That can block instead of poll loop
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
            if let Spec::ProcessSpec(spec) = &d.spec.template.spec {
                let kid = local_manage(&spec);
                println!(
                    "Spawned process {} ({}/{})",
                    kid.id(),
                    running.len() + 1,
                    count
                );
                running.push(kid);
            } else {
                println!("random failure"); //TODO: better errors
            }
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

/***************************************/
/* manage a single process             */
/* formerly known as child_launcher()  */
fn local_manage(cmd_params: &ProcessSpec) -> std::process::Child {
    let mut cmd = Command::new(cmd_params.cmd.clone());
    if cmd_params.args.is_some() {
        cmd.arg(cmd_params.args.clone().unwrap());
    }
    let kid = cmd
        .spawn()
        .expect(&format!("failed to run '{}'", cmd_params.cmd));

    return kid;
}
