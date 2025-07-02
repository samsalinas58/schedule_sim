
use crate::ps::Ps;

#[derive(Debug, Clone, Copy)]
pub struct FinishedPs {
    pub ps: Ps,
    pub turnaround_time: u8,
    pub response_time: u8,
}

pub trait NewFinished {
    fn new(ps: Ps, turnaround_time: u8, response_time: u8) -> FinishedPs;
}

impl NewFinished for FinishedPs {
    fn new(ps: Ps, turnaround_time: u8, response_time: u8) -> FinishedPs {
        FinishedPs { ps, turnaround_time, response_time }
    }
}

//SORT BY ID
pub fn sort(finished_ps: &Vec<FinishedPs>) -> Vec<FinishedPs> {
    let mut result_vec: Vec<FinishedPs> = Vec::with_capacity(5);
    unsafe { result_vec.set_len(5) };
    for ps in finished_ps.iter() {
        result_vec[ps.ps.id as usize] = *ps; 
    }

    Vec::from(result_vec)
}