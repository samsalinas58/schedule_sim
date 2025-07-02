mod sched;
mod ps;

use crate::sched::fcfs::{fcfs};
use crate::sched::rr::rr;
use crate::sched::hrrn::hrrn;
use crate::sched::srt::srt;
use ps::{gen_given_processes, gen_random_processes, Format};
use sched::finished_ps::{FinishedPs, sort};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    env::set_var("RUST_BACKTRACE", "1"); //debugging purposes

    let path = Path::new("random_results.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    //THE REPORT
    let mut fcfs_results: Vec<(u8, f64, Vec<FinishedPs>)> = Vec::new();
    let mut rr_results: Vec<(u8, f64, Vec<FinishedPs>)> = Vec::new();
    let mut hrrn_results: Vec<(u8, f64, Vec<FinishedPs>)> = Vec::new();
    let mut srt_results: Vec<(u8, f64, Vec<FinishedPs>)> = Vec::new();

    for _ in 0..30 {
        let random_ps = gen_random_processes();

        let mut fcfs_result = fcfs(random_ps);
        let mut rr_result = rr(random_ps);
        let mut hrrn_result = hrrn(random_ps);
        let mut srt_result = srt(random_ps);

        fcfs_result.2 = sort(&fcfs_result.2);
        rr_result.2 = sort(&rr_result.2);
        hrrn_result.2 = sort(&hrrn_result.2);
        srt_result.2 = sort(&srt_result.2);

        fcfs_results.push(fcfs_result);
        rr_results.push(rr_result);
        hrrn_results.push(hrrn_result);
        srt_results.push(srt_result);
    }
    
    for i in 0..30 {
        //write header for each run
        let random_processes = &fcfs_results[i].2;
        write_header(&mut file, i+1, &random_processes);
        
        //write the results of each run
        file.write(b"FIRST COME FIRST SERVE\n");
        write_result(&mut file, &fcfs_results[i]);
        file.write(b"\n");
        
        file.write(b"ROUND ROBIN\n");
        write_result(&mut file, &rr_results[i]);
        file.write(b"\n");
        
        file.write(b"HIGHEST RESPONSE RATIO NEXT\n");
        write_result(&mut file, &hrrn_results[i]);
        file.write(b"\n");
        
        file.write(b"SHORTEST REMAINING TIME\n");
        write_result(&mut file, &srt_results[i]);
        file.write(b"\n");
        
        
    }
    
    let (fcfs_avg_time, 
    fcfs_avg_throughput, 
    fcfs_avg_tat,
    fcfs_avg_response,
    fcfs_avg_ratio): (f64, f64, f64, f64, f64) = get_averages(&fcfs_results);

    let (rr_avg_time, 
    rr_avg_throughput, 
    rr_avg_tat,
    rr_avg_response,
    rr_avg_ratio): (f64, f64, f64, f64, f64) = get_averages(&rr_results);

    let (hrrn_avg_time, 
    hrrn_avg_throughput, 
    hrrn_avg_tat,
    hrrn_avg_response,
    hrrn_avg_ratio): (f64, f64, f64, f64, f64) = get_averages(&hrrn_results);

    let (srt_avg_time, 
    srt_avg_throughput, 
    srt_avg_tat,
    srt_avg_response,
    srt_avg_ratio): (f64, f64, f64, f64, f64) = get_averages(&srt_results);
    
    file.write(b"OVERALL RESULTS OF EACH ALGORITHM\n");
    file.write(b"--------------------------------------------------------------------------\n");
    file.write(b"\n");

    file.write(b"FIRST COME FIRST SERVE\n");
    write_overall_result(&mut file, fcfs_avg_time, fcfs_avg_throughput, fcfs_avg_tat, fcfs_avg_response, fcfs_avg_ratio);
    file.write(b"\n");

    file.write(b"ROUND ROBIN\n");
    write_overall_result(&mut file, rr_avg_time, rr_avg_throughput, rr_avg_tat, rr_avg_response, rr_avg_ratio);
    file.write(b"\n");

    file.write(b"HIGHEST RESPONSE RATIO NEXT\n");
    write_overall_result(&mut file, hrrn_avg_time, hrrn_avg_throughput, hrrn_avg_tat, hrrn_avg_response, hrrn_avg_ratio);
    file.write(b"\n");

    file.write(b"SHORTEST REMAINING TIME\n");
    write_overall_result(&mut file, srt_avg_time, srt_avg_throughput, srt_avg_tat, srt_avg_response, srt_avg_ratio);
    file.write(b"\n");

    let given_ps = gen_given_processes();
    
    println!("Given processes:");
    for ps in given_ps.iter() { println!("{ps:?}"); }
    
    println!();

    let fcfs1 = fcfs(given_ps);

    let rr1 = rr(given_ps);

    let hrrn1 = hrrn(given_ps);

    let srt1 = srt(given_ps);

    println!("RESULTS");
    println!("-----------\n");
    println!("FIRST COME FIRST SERVE RESULTS: \n");
    print_result(fcfs1);
    println!();
    println!("HIGHEST RESPONSE RATIO NEXT RESULTS: \n");
    print_result(hrrn1);
    println!();
    println!("ROUND ROBIN RESULTS: \n");
    print_result(rr1);
    println!();
    println!("SHORTEST REMAINING TIME RESULTS: \n");
    print_result(srt1);



}

fn print_result((t, throughput, finished): (u8, f64, Vec<FinishedPs>)) {
    println!("Time finished at t = {t}");
    println!("Throughput = {throughput}");
    for finished_ps in finished.iter() { 
        println!("Process details: {:?}", finished_ps.ps);
        println!("Process {:?} with response time = {}, turnaround time = {}, ratio of turnaround time with respect to service time = {}", 
        finished_ps.ps.id, 
        finished_ps.response_time, 
        finished_ps.turnaround_time,
        finished_ps.turnaround_time as f64 / finished_ps.ps.serv_time as f64); 
        println!();
    }
}

/*
    RESULTS FORMAT:

    RUN 1
    --------------------------------------------------------------------------
    PROCESS DETAILS
    --------------------------------------------------------------------------
    ALGORITHM X
    TIME_TAKEN: T
    THROUGHPUT: W
    PROCESS A WITH RESPONSE TIME = Y, TURNAROUND_TIME = TURN, TURNAROUND_TIME / SERVICE_TIME = Z
    PROCESS B WITH RESPONSE TIME...

    ALGORITHM Y
    ...
    PROCESS E WITH RESPONSE TIME = Y, TURNAROUND_TIME = TURN, TURNAROUND_TIME / SERVICE_TIME = Z

    ...

    RUN 2
    -------------
    SAME AS ABOVE
    */

fn write_header(file: &mut File, i: usize, finished_ps: &Vec<FinishedPs>){
    let str = format!("RUN {i}\n");
    file.write(str.as_bytes());
    file.write(b"--------------------------------------------------------------------------\n");

    for ps in finished_ps.iter(){
        file.write(ps.ps.format().as_bytes());
        file.write(b"\n");
    }

    file.write(b"--------------------------------------------------------------------------\n");

}

fn write_result(file: &mut File, (t, throughput, finished_ps): &(u8, f64, Vec<FinishedPs>)){
    let t_str = format!("TIME TAKEN: {t}s");
    file.write(t_str.as_bytes());
    file.write(b"\n");

    let throughput_str = format!("THROUGHPUT: {throughput}");
    file.write(throughput_str.as_bytes());
    file.write(b"\n");
    for ps in finished_ps.iter(){
        let str = format!("PROCESS {:?} WITH RESPONSE_TIME = {}, TURNAROUND_TIME = {}, TURNAROUND_TIME / SERVICE_TIME = {} ",
         ps.ps.id, 
         ps.response_time,
         ps.turnaround_time,
         ps.turnaround_time as f64 / ps.ps.serv_time as f64);
         file.write(str.as_bytes());
         file.write(b"\n");
    }
}
//overall avg time taken, overall avg throughput, overall avg response time, 
//overall average ratio of turnaround time with respect to service time
fn get_averages(times: &Vec<(u8, f64, Vec<FinishedPs>)>) -> (f64, f64, f64, f64, f64) {
    let (mut avg_time, mut avg_throughput, mut avg_response_time, mut avg_tat, mut avg_ratio): (f64, f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0, 0.0);
    let mut i = 0;
    for (t, throughput, results) in times {
        avg_time += *t as f64;
        avg_throughput += throughput;
        let mut tat: f64 = 0.0;
        let mut response_time: f64 = 0.0;
        let mut ratio: f64 = 0.0;
        for ps in results {
            tat += ps.turnaround_time as f64;
            response_time += ps.response_time as f64;
            ratio += ps.turnaround_time as f64 / ps.ps.serv_time as f64;
        }
        avg_tat += tat / 5.0;
        avg_response_time += response_time / 5.0;
        avg_ratio += ratio / 5.0;
        
        response_time = 0.0;
        tat = 0.0;
        ratio = 0.0;

        i += 1;
    }
    let count = i as f64;
    (avg_time / count, avg_throughput / count, avg_tat / count, avg_response_time / count, avg_ratio / count)
}

fn write_overall_result(file: &mut File, 
    overall_avg_time: f64,
    overall_avg_throughput: f64,
    overall_avg_tat: f64,
    overall_avg_response_time: f64,
    overall_avg_ratio: f64) {
    
    let mut write = format!("OVERALL AVERAGE TIME: {overall_avg_time}\n");
    file.write(write.as_bytes());

    let mut write = format!("OVERALL AVERAGE THROUGHPUT: {overall_avg_throughput}\n");
    file.write(write.as_bytes());

    let mut write = format!("OVERALL AVERAGE TURNAROUND TIME: {overall_avg_tat}\n");
    file.write(write.as_bytes());

    let mut write = format!("OVERALL AVERAGE RESPONSE TIME: {overall_avg_response_time}\n");
    file.write(write.as_bytes());

    let mut write = format!("OVERALL AVERAGE TURNAROUND TIME / SERVICE TIME: {overall_avg_ratio}\n");
    file.write(write.as_bytes());
    
}