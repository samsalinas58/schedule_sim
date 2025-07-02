
use crate::ps::Ps;

#[derive(Debug, Clone, Copy)]
pub struct SchedPs { 
    pub ps: Ps,
    //total processor time remaining
    pub total_service_time_remaining: u8,
    //the session time remaining. this is decremented upon every iteration and recalculated once put into a new queue.
    pub session_time_remaining: u8,
    //the amount of times that this process still needs to use IO. this is changed every time a process finishes an I/O session. 
    pub io_activities_remaining: u8,
    //the total waiting time of the process. the IO queue does not increment this value at all. it  will always operate on an FCFS basis. 
    pub waiting_time: u8,
    //the response time from initial submission to the first time it is put on to the cpu
    pub response_time: u8
}

pub trait NewSched {
    fn new(ps: Ps, total_service_time_remaining: u8, session_time_remaining: u8, io_activities_remaining: u8, waiting_time: u8, response_time: u8) -> SchedPs;
}

impl NewSched for SchedPs {
    fn new(ps: Ps, total_service_time_remaining: u8, session_time_remaining: u8, io_activities_remaining: u8, waiting_time: u8, response_time: u8) -> SchedPs {
        return SchedPs { ps, total_service_time_remaining, session_time_remaining, io_activities_remaining, waiting_time, response_time }
    }
}