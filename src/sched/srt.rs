//shortest remaining time

use crate::ps::{Ps, New};
use crate::ps::PsId;
use crate::sched::sched_ps::{SchedPs, NewSched};
use crate::sched::finished_ps::{FinishedPs, NewFinished};
use std::collections::VecDeque;

pub fn srt(ps: [Ps; 5]) -> (u8, f64, Vec<FinishedPs>) {
    //track the total time it takes for the algorithm to run
    let mut t: u8 = 0;
    //convert the slice we are given in the function to a mutable vector
    let mut ps_vec: Vec<Ps> = Vec::from(ps);

    //explicitly denote that no process is running
    let none = Ps::new(PsId::None, 255, 255, 255, 255);
    let no_running_ps = SchedPs::new(none, 255, 255, 255, 0, 255);

    let (mut service_running, mut io_running): (bool, bool) = (false, false);
    let (mut service_running_ps, mut io_running_ps): (SchedPs, SchedPs) = (no_running_ps, no_running_ps);

    //create a ready queue and IO queue
    let (mut rdy_q, mut io_q): (VecDeque<SchedPs>, VecDeque<SchedPs>) = (VecDeque::new(), VecDeque::new());
    let mut finished: Vec<FinishedPs> = Vec::new();
    
    //determine when a new process has arrived to call the scheduler
    let mut arrived: bool = false;

    loop {
        if ps_vec.is_empty() && io_q.is_empty() && rdy_q.is_empty() && !service_running && !io_running { break };

        //when process arrives, move it to the ready queue
        for &ps in (&ps_vec).into_iter(){
            if ps.arr_time == t { 
                //calculate burst time
                let session_time_remaining: u8;
                if ps.disk_io_act == 0 { session_time_remaining = ps.serv_time; }
                else { session_time_remaining = ps.serv_time / (ps.disk_io_act + 1); }
                let process_arrived = SchedPs::new(ps, 
                    ps.serv_time, 
                    session_time_remaining, 
                    ps.disk_io_act,
                    0,
                    0);
                rdy_q.push_back(process_arrived);
                arrived = true;
            }
        }

        //after process arrives, remove it from original process list.
        for &sched_ps in rdy_q.iter(){
            for i in 0..ps_vec.len(){
                if sched_ps.ps.id == ps_vec[i].id {
                    ps_vec.remove(i);
                    break; 
                }
            }
        }

        //decide whether to move running process to IO or completely done, if there is no IO left to perform then the process is done.
        if service_running && service_running_ps.session_time_remaining == 0 {
            // if the process needs to perform IO, push it to the IO queue
            if service_running_ps.io_activities_remaining != 0 {
                service_running_ps.session_time_remaining = service_running_ps.ps.disk_io_time / service_running_ps.ps.disk_io_act;
                io_q.push_back(service_running_ps);
            }
            else { 
                let turnaround_time = t - service_running_ps.ps.arr_time;
                let response_time = service_running_ps.response_time;
                let finished_ps = FinishedPs::new(service_running_ps.ps, turnaround_time, response_time);
                finished.push(finished_ps);
            }

            //running process will be overwritten with the default process;
            service_running = false;
            service_running_ps = no_running_ps;
        }

        // if the process is done using IO, then move it back to the ready queue
        if io_running && io_running_ps.session_time_remaining == 0 {
            io_running_ps.io_activities_remaining -= 1;
            
            //with the way this project is designed, the process should always have some time remaining after finishing IO, but just in case...
            if io_running_ps.io_activities_remaining == 0 { io_running_ps.session_time_remaining = io_running_ps.total_service_time_remaining; }

            else { io_running_ps.session_time_remaining = io_running_ps.ps.serv_time / (io_running_ps.ps.disk_io_act + 1) }
            
            rdy_q.push_back(io_running_ps);

            arrived = true;

            io_running = false;
            io_running_ps = no_running_ps;
        }

        //when a new process has arrived, reschedule the running process (or leave it running)
        if arrived || (!service_running && !rdy_q.is_empty()) {
            let mut shortest_remaining_time: u8 = 255;
            let mut i = 0;
            for (index, &sched_ps) in rdy_q.iter().enumerate(){
                let s: u8 = sched_ps.ps.serv_time;
                let e: u8 = sched_ps.ps.serv_time - sched_ps.total_service_time_remaining;
                if (s - e) < shortest_remaining_time {
                    shortest_remaining_time = s - e;
                    i = index;
                }
            }
            //compare with the current running process
            if service_running {
                let s: u8 = service_running_ps.ps.serv_time;
                let e: u8 = service_running_ps.total_service_time_remaining;
                //if the current running process does not have the shortest remaining time
                //then swap the running process with the process that has the shortest remaining time
                if !( (s - e) <= shortest_remaining_time) { 
                    let new_running_ps = rdy_q.remove(i).unwrap();
                    rdy_q.push_back(service_running_ps);
                    service_running_ps = new_running_ps;
                }
            }

            //run the process if there's no service running
            else { 
                service_running_ps = rdy_q.remove(i).unwrap();
                service_running = true;
            }
            if service_running_ps.ps.serv_time == service_running_ps.total_service_time_remaining {
                service_running_ps.response_time = service_running_ps.waiting_time;
            }
            service_running_ps.waiting_time = 0;
            arrived = false;
        }

        if !io_running && !io_q.is_empty(){
            let popped_ps = io_q.pop_front();
            io_running = true;
            io_running_ps = popped_ps.unwrap();
        }

        if service_running {
            service_running_ps.session_time_remaining -= 1;
            service_running_ps.total_service_time_remaining -= 1;
        }

        if io_running {
            io_running_ps.session_time_remaining -= 1;
        }

        t += 1;
        for ps in rdy_q.iter_mut() { ps.waiting_time += 1; }
    }
    t -= 1;
    let throughput: f64 = 5.0 / t as f64;
    (t, throughput, finished)
}   