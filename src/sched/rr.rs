
use crate::ps::{Ps, New};
use crate::ps::PsId;
use crate::sched::sched_ps::{SchedPs, NewSched};
use crate::sched::finished_ps::{FinishedPs, NewFinished};
use std::collections::VecDeque;


const TIME_SLICE: u8 = 1;

//round robin scheduling
pub fn rr(ps: [Ps; 5]) -> (u8, f64, Vec<FinishedPs>) {
    //track the total time it takes for the algorithm to run
    let mut t: u8 = 0;
    //convert the slice we are given in the function to a mutable vector
    let mut ps_vec: Vec<Ps> = Vec::from(ps);

    //explicitly denote that no process is running
    let none = Ps::new(PsId::None, 255, 255, 255, 255);
    let no_running_ps = SchedPs::new(none, 255, 255, 255, 0, 255);

    let (mut service_running, mut io_running): (bool, bool) = (false, false);
    let (mut service_running_ps, mut io_running_ps): (SchedPs, SchedPs) = (no_running_ps, no_running_ps);

    //create a ready queue and an IO queue of scheduled processes
    let (mut rdy_q, mut io_q): (VecDeque<SchedPs>, VecDeque<SchedPs>) = (VecDeque::new(), VecDeque::new());
    let mut finished: Vec<FinishedPs> = Vec::new();

    loop {
        // if there's nothing to do, end the loop
        if ps_vec.is_empty() && io_q.is_empty() && rdy_q.is_empty() && !service_running && !io_running { break; };


        //when process arrives, move it to the ready queue
        for &ps in (&ps_vec).into_iter(){
            if ps.arr_time == t {
                let mut session_time_remaining = TIME_SLICE; //default to global time slice
                if ps.disk_io_act == 0 { 
                    if TIME_SLICE > ps.serv_time { session_time_remaining = ps.serv_time; }
                    else { session_time_remaining = TIME_SLICE; }
                }
                else if TIME_SLICE > ps.serv_time / (ps.disk_io_act + 1) { session_time_remaining = ps.serv_time / (ps.disk_io_act + 1); }
                let process_arrived = SchedPs::new(ps, ps.serv_time, session_time_remaining, ps.disk_io_act, 0, 0); 
                rdy_q.push_back(process_arrived);
            }
        }

        //after process arrives, remove it from original process list.
        for &sched_ps in (&rdy_q).into_iter(){
            for i in 0..ps_vec.len(){
                if sched_ps.ps.id == ps_vec[i].id {
                    ps_vec.remove(i);
                    break; 
                }
            }
        }

        //once the process has finished it's session, determine if we need to push it to the IO queue or the ready queue
        if service_running && service_running_ps.session_time_remaining == 0 {

            //if it still has io activities remaining and we've reached a time stamp to put the process into the IO queue
            if service_running_ps.io_activities_remaining != 0 && 
            (service_running_ps.ps.serv_time - service_running_ps.total_service_time_remaining) %
            (service_running_ps.ps.serv_time / (service_running_ps.ps.disk_io_act + 1)) == 0 {
                //recalculate session time for IO, it does not use the same time slice the processor uses.
                let session_time_remaining = service_running_ps.ps.disk_io_time / service_running_ps.ps.disk_io_act;
                service_running_ps.session_time_remaining = session_time_remaining;
                io_q.push_back(service_running_ps); 
            }

            //otherwise, push it to the back of the ready queue
            else if service_running_ps.total_service_time_remaining != 0 { 

                //recalculate session time
                let session_time_remaining: u8;

                if TIME_SLICE > service_running_ps.total_service_time_remaining {
                    session_time_remaining = service_running_ps.total_service_time_remaining;
                }
                else { session_time_remaining = TIME_SLICE; }
                service_running_ps.session_time_remaining = session_time_remaining;

                //push the process to the back of the ready queue
                rdy_q.push_back(service_running_ps);
            }
            else {
                let turnaround_time = t - service_running_ps.ps.arr_time;
                let response_time = service_running_ps.response_time;
                let finished_ps = FinishedPs::new(service_running_ps.ps, turnaround_time, response_time);
                finished.push(finished_ps);
            }
            //now that we've pushed to one of the queues (or done nothing), no process is running.
            service_running = false;
            service_running_ps = no_running_ps;
        }

        //once the IO bound process has finished it's session, push it back to the ready queue
        if io_running && io_running_ps.session_time_remaining == 0 {
            //recalculate the session time
            io_running_ps.io_activities_remaining -= 1;
            //this most likely will never happen... but this is here. just in case.
            if io_running_ps.total_service_time_remaining != 0 {
                if TIME_SLICE > io_running_ps.total_service_time_remaining { 
                    io_running_ps.session_time_remaining = io_running_ps.total_service_time_remaining; 
                }
                else {
                    io_running_ps.session_time_remaining = TIME_SLICE; 
                }
                rdy_q.push_back(io_running_ps);
            }

            io_running_ps = no_running_ps;
            io_running = false;
        }

        //if there's no process running, run the process at the front of the ready queue if there is one
        if !service_running {
            let popped_ps = rdy_q.pop_front();
            match popped_ps {

                //if we find something at the beginning of the queue, then run the process
                Some(sched_ps) => {
                    service_running = true;
                    service_running_ps = sched_ps;
                    if service_running_ps.ps.serv_time == service_running_ps.total_service_time_remaining {
                        service_running_ps.response_time = service_running_ps.waiting_time;
                    }
                    service_running_ps.waiting_time = 0;
                }
                //if there's no processes in the queue, continue as usual. 
                None => {
                    ();
                }
            }
        }

        // run the next process in the IO queue. if there's nothing then do nothing!
        if !io_running {
            let popped_ps = io_q.pop_front();
            match popped_ps {
                Some(sched_ps) => {
                    io_running = true;
                    io_running_ps = sched_ps;
                }
                None => {
                    ();
                }
            }
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