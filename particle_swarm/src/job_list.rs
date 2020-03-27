use std::usize::MAX;
use std::cmp::max;

pub struct Jobs {
    n_jobs : usize,
    n_machines : usize,
    processing_times : Vec<usize>, // Processing time of task j from job i : [i * machines + j]
    machine_numbers : Vec<usize>, // Same as above but machine number
}

impl Jobs {
    pub fn init(machines_and_times : Vec<(usize, usize)>, n_machines : usize, n_jobs : usize) -> Jobs {
        let mut processing_times = Vec::new();
        let mut machine_numbers = Vec::new();
        let mut tasks_per_machines = vec![0; n_machines*n_jobs];
        for (machine, duration) in machines_and_times {
            processing_times.push(duration);
            machine_numbers.push(machine);
        }
        for (i, &machine) in machine_numbers.iter().enumerate() {
            let task_number = i % n_machines;
            let job_number = i / n_machines;
            tasks_per_machines[machine * n_jobs + job_number] = task_number;
        }
        return Jobs {n_jobs, n_machines, processing_times, machine_numbers}
    }

    pub fn n_jobs(&self) -> usize {
        return self.n_jobs
    }

    pub fn n_machines(&self) -> usize {
        return self.n_machines
    }

    pub fn processing_time(&self, idx : usize) -> usize {
        return self.processing_times[idx];
    }

    pub fn machine_for_task(&self, job : usize, task : usize) -> usize {
        return self.machine_numbers[job * self.n_machines + task];
    }
}

pub struct Ordering<'a> {
    tasks_order : Vec<(usize, usize)>, // A task is identified by a job number and a task number. Organized by : machine number, then order
    n_machines : usize,
    n_jobs : usize,
    jobs : &'a Jobs,
    end_time : usize,
}

impl Ordering<'_> {
    pub fn new(tasks_order : Vec<(usize, usize)>, jobs : &Jobs) -> Ordering {
        let mut tmp_ord = Ordering {tasks_order, n_machines : jobs.n_machines, n_jobs : jobs.n_jobs, jobs, end_time : 0};
        tmp_ord.end_time = tmp_ord.eval();
        return tmp_ord
    }

    pub fn end_time(&self) -> usize {
        return self.end_time;
    }

    pub fn eval(&self) -> usize {
        let mut max_time = 0;
        let all_times = self.generate_times();
        for i in 0..self.n_jobs {
            max_time = max(max_time, all_times[(i + 1) * self.n_machines - 1].1);
        }
        return max_time
    }

    pub fn output(&self) -> String {
        let mut machine_strings : Vec<String> = vec![String::from(""); self.n_machines];
        let all_times = self.generate_times();
        for &(job, task) in &self.tasks_order {
            let machine_num = self.jobs.machine_for_task(job, task);
            let (start, end) = all_times[job * self.n_machines + task];
            machine_strings[machine_num].push_str(&format!("{} {} {} {} ", job, task, start, end));
        }
        let mut final_result = String::from("");
        for s in machine_strings {
            final_result.push_str(&s);
            final_result.push_str("\n");
        }
        return final_result
    }

    fn generate_times(&self) -> Vec<(usize, usize)> {
        let mut times : Vec<(usize, usize)> = vec![(MAX, MAX); self.tasks_order.len()];
        let ord = self.order_per_machines();
        for &(job, task) in &self.tasks_order {
            self.get_time(job, task, &mut times, &ord);
        }
        return times
    }

    fn get_time(&self, job : usize, task : usize, times : &mut Vec<(usize, usize)>, ord : &Vec<(usize, usize)>) {
        let idx = job * self.n_machines + task;
        if times[idx] != (MAX, MAX) {return;}
        if self.jobs.processing_time(idx) == 0 {times[idx] = (0, 0);}
        let proc_time = self.jobs.processing_time(idx);

        let first_of_job = task == 0;
        let first_of_machine = self.is_first_of_machine(job, task, ord);
        if first_of_job && first_of_machine {
            times[idx] = (0, proc_time);
        }
        else if first_of_job {
            let (prev_job, prev_task) = self.get_previous_task_machine(job, task, ord);
            self.get_time(prev_job, prev_task, times, ord);
            let prev_end_time = times[prev_job * self.n_machines + prev_task].1;
            times[idx] = (prev_end_time, prev_end_time + proc_time);
        }
        else if first_of_machine {
            let (prev_job, prev_task) = self.get_previous_task_job(job, task);
            self.get_time(prev_job, prev_task, times, ord);
            let prev_end_time = times[prev_job * self.n_machines + prev_task].1;
            times[idx] = (prev_end_time, prev_end_time + proc_time);
        }
        else {
            let (prev_job1, prev_task1) = self.get_previous_task_job(job, task);
            let (prev_job2, prev_task2) = self.get_previous_task_machine(job, task, ord);
            self.get_time(prev_job1, prev_task1, times, ord);
            self.get_time(prev_job2, prev_task2, times, ord);
            let prev_end_time = max(times[prev_job1 * self.n_machines + prev_task1].1, times[prev_job2 * self.n_machines + prev_task2].1);
            times[idx] = (prev_end_time, prev_end_time + proc_time);
        }
    }

    fn get_previous_task_job(&self, job : usize, task : usize) -> (usize, usize) {
        return (job, task - 1)
    }

    fn get_previous_task_machine(&self, job : usize, task : usize, ord : &Vec<(usize, usize)>) -> (usize, usize) {
        let machine_num = self.jobs.machine_for_task(job, task);
        let skip_num = machine_num * self.n_jobs;
        let index = ord.iter().skip(skip_num).take(self.n_jobs).position(|&pair| pair == (job, task)).unwrap() + skip_num - 1;
        return ord[index]
    }

    fn is_first_of_machine(&self, job : usize, task : usize, ord : &Vec<(usize, usize)>) -> bool {
        let machine_num = self.jobs.machine_for_task(job, task);
        return ord[machine_num * self.n_jobs] == (job, task)
    }

    pub fn order_per_machines(&self) -> Vec<(usize, usize)> {
        let mut machine_list : Vec<Vec<(usize, usize)>> = vec![Vec::new(); self.jobs.n_machines()];
        for &(job, task) in &self.tasks_order {
            machine_list[self.jobs.machine_for_task(job, task)].push((job, task));
        }
        return machine_list.into_iter().flatten().collect()
    }
}