
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PsId {
    A, B, C, D, E, None = -1
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ps {
    pub id: PsId,
    pub arr_time: u8,
    pub serv_time: u8,
    pub disk_io_time: u8,
    pub disk_io_act: u8,
}

pub trait New {
    fn new(id: PsId, arr_time: u8, serv_time: u8, disk_io_time: u8, disk_io_act: u8) -> Ps;
}

impl New for Ps {
    fn new(id: PsId, arr_time: u8, serv_time: u8, disk_io_time: u8, disk_io_act: u8) -> Ps {
        return Ps {
            id: id,
            arr_time: arr_time, 
            serv_time: serv_time, 
            disk_io_time: disk_io_time, 
            disk_io_act: disk_io_act
        }
    }
}

pub trait Format {
    fn format(&self) -> String;
}

impl Format for Ps {
    fn format(&self) -> String {
        //PROCESS A: ARR_TIME: arr_time, SERVICE_TIME: serv_time, DISK_IO_TIME: disk_io_time, DISK_IO_ACTIVITIES: disk_io_act
        format!("PROCESS {:?}: ARR_TIME: {}, SERVICE_TIME: {}, DISK_IO_TIME: {}, DISK_IO_ACTIVITIES: {}",
        self.id,
        self.arr_time,
        self.serv_time,
        self.disk_io_time,
        self.disk_io_act)

    }
}

pub fn gen_given_processes() -> [Ps; 5] {

    let a = Ps::new(PsId::A, 0, 6, 1, 1);
    let b = Ps::new(PsId::B, 2, 12, 2, 2);
    let c = Ps::new(PsId::C, 4, 8, 1, 1);
    let d = Ps::new(PsId::D, 6, 10, 1, 1);
    let e = Ps::new(PsId::E, 8, 4, 2, 2);

    [a,b,c,d,e]
}

pub fn gen_random_processes() -> [Ps; 5] {

    //randomize arrival times
    let arr_times: [u8; 5] = gen_random_arr_times();
    //randomize service times
    let serv_times: [u8; 5] = gen_random_serv_times();

    //randomize disk io times (must be <= service time)
    let disk_io_times: [u8; 5] = gen_random_disk_io_times(serv_times);

    //randomize disk activity times (must divide disk io time evenly)
    let disk_io_activities: [u8; 5] = gen_random_disk_io_activity_times(disk_io_times);

    let a = Ps::new(PsId::A, arr_times[0], serv_times[0], disk_io_times[0], disk_io_activities[0]);
    let b = Ps::new(PsId::B, arr_times[1], serv_times[1], disk_io_times[1], disk_io_activities[1]);
    let c = Ps::new(PsId::C, arr_times[2], serv_times[2], disk_io_times[2], disk_io_activities[2]);
    let d = Ps::new(PsId::D, arr_times[3], serv_times[3], disk_io_times[3], disk_io_activities[3]);
    let e = Ps::new(PsId::E, arr_times[4], serv_times[4], disk_io_times[4], disk_io_activities[4]);

    [a,b,c,d,e]
}

fn gen_random_arr_times() -> [u8; 5] {
    let mut rng: ThreadRng = rand::thread_rng();
    let range = 0..13;
    let between = Uniform::from(range); //limit process arrival times to anywhere between 0 to 20 seconds
    let mut arr_times: Vec<u8> = Vec::new();    
    for _ in 0..5 {
        arr_times.push(between.sample(&mut rng));
    }

    [arr_times[0], arr_times[1], arr_times[2], arr_times[3], arr_times[4]]
}

fn gen_random_serv_times() -> [u8; 5] {
    let mut rng: ThreadRng = rand::thread_rng();
    let range = 4..17; //minimum 4 and maximum 16
    let between = Uniform::from(range); //service time is minimum 4 seconds and maximum 16
    let mut serv_times: Vec<u8> = Vec::new();
    for _ in 0..5 {
        serv_times.push(between.sample(&mut rng));
    }

    [serv_times[0], serv_times[1], serv_times[2], serv_times[3], serv_times[4]]
}

fn gen_random_disk_io_times(serv_times: [u8; 5]) -> [u8; 5] {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut disk_io_times: Vec<u8> = Vec::new();
    for i in 0..5 {
        let between = Uniform::from(0..(serv_times[i]+1)/2);
        let mut random_number = between.sample(&mut rng);
        //using even numbers here makes calculating disk io activity time much easier since it is at equal intervals during service time
        random_number = (random_number/2) * 2;
        disk_io_times.push(random_number);
    }
    [disk_io_times[0],disk_io_times[1],disk_io_times[2],disk_io_times[3],disk_io_times[4]]
}

fn gen_random_disk_io_activity_times(disk_io_times: [u8; 5]) -> [u8; 5] {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut disk_io_activities: Vec<u8> = Vec::new();
    let between = Uniform::from(1..5); //divide the number between 0..4
    for i in 0..5 {
        let disk_io_time = disk_io_times[i];
        if disk_io_time == 0 { 
            disk_io_activities.push(0);
            continue;
        }
        let mut disk_io_activity: u8;
        loop {
            disk_io_activity = between.sample(&mut rng);
            if disk_io_time % disk_io_activity == 0 { break; }
        }
        disk_io_activities.push(disk_io_time / disk_io_activity);
    }

    [disk_io_activities[0], disk_io_activities[1], disk_io_activities[2], disk_io_activities[3], disk_io_activities[4]]
}