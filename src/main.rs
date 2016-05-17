extern crate tbd;
extern crate rand;
extern crate time;

mod menu;

use menu::*;
use tbd::tasklog::*;
use tbd::task::*;
use tbd::log::{Log, LogEntry, save_to_fs};
use rand::thread_rng;
use time::Tm;

fn new_active<T: TaskStatTrait>(task_stat: &mut T) -> bool {
    let title = read_string("Title: ");
    let factor: f32 = read_parse("Factor: ");
    let due_in: i16  = read_parse("Due in: ");
    task_stat.add_active_task(title, "-".to_string(), factor, due_in);
    false
}

fn new_pool<T: TaskStatTrait>(task_stat: &mut T) -> bool {
    let title = read_string("Title: ");
    let factor: f32 = read_parse("Factor: ");
    let propability: f32 = read_parse("Propabitity: ");
    let cool_down: i16 = read_parse("Cool down time: ");
    let due_days: i16 = read_parse("Due days: ");
    task_stat.add_pooled_task(title, "-".to_string(), factor,
                              propability, cool_down, due_days);
    false
}

trait Printable {
    fn print_single_line(&self);
}

fn tm_to_day_str(tm: &Tm) -> String {
    format!("{:0>#2}.{:0>#2}.{:0>#4}", tm.tm_mday, tm.tm_mon + 1, tm.tm_year + 1900)
}
fn tm_to_time_str(tm: &Tm) -> String {
    format!("{:0>#2}.{:0>#2}.{:0>#4} {:0>#2}:{:0>#2}:{:0>#2}",
            tm.tm_mday, tm.tm_mon + 1, tm.tm_year + 1900,
            tm.tm_hour, tm.tm_min, tm.tm_sec)
}

impl Printable for ActiveTask {
    fn print_single_line(&self) {
        println!("{}: {}", self.task.title, tm_to_day_str(&self.due));
    }
}

impl Printable for PooledTask {
    fn print_single_line(&self) {
        println!("{}: {} (cd: {}, dd: {})",
                 tm_to_day_str(&self.cooling_until),
                 self.task.title,
                 self.cool_down,
                 self.due_days);
    }
}

impl Printable for LogEntry<TaskAction> {
    fn print_single_line(&self) {
        print!("{}: ", tm_to_time_str(&self.dttm));
        match self.entry {
            TaskAction::ScheduleTask(ref a_task) => {
                print!("Added active task: ");
                a_task.print_single_line()
            },
            TaskAction::PoolTask(ref p_task) => {
                print!("Added pooled task: ");
                p_task.print_single_line()
            },
            TaskAction::CompleteTask(ref a_task) => {
                print!("Complete task: ");
                a_task.print_single_line()
            },
            TaskAction::ActivateTask(ref a_tasks) => {
                println!("Many tasks: ");
                for a_task in a_tasks {
                    print!(" - ");
                    a_task.print_single_line()
                }
            }
        }
    }
}

fn print_actives<T: TaskStatTrait>(task_stat: &mut T) -> bool {
    println!("Having {} tasks", task_stat.all_actives().len());
    for a_task in task_stat.all_actives() {
        a_task.print_single_line();
    }
    false
}

fn print_pool<T: TaskStatTrait>(task_stat: &mut T) -> bool {
    println!("Having {} tasks", task_stat.all_pooled().len());
    for p_task in task_stat.all_pooled() {
        p_task.print_single_line();
    }
    false
}


fn print_log(task_log: &mut TaskLog) -> bool {
    for log_entry in task_log.log.iter() {
        log_entry.print_single_line();
    }
    false
}

fn save_log(task_log: &mut TaskLog) -> bool {
    write_task_log_to_fs(&task_log, "./save/");
    false
}

fn load_log(task_log: &mut TaskLog) -> bool {
    read_task_log_to_fs(task_log, "./save/");
    false
}

fn mark_done(task_log: &mut TaskLog) -> bool {
    let mut counter: i16 = 1;
    let mut titles: Vec<String> = Vec::new();
    for (_, a_task) in task_log.task_stat.active.iter() {
        print!("{}: ", counter);
        a_task.print_single_line();
        titles.push(a_task.task.title.clone());
        counter += 1;
    }
    let choice: usize = read_parse("> ");
    if choice > 0 && choice <= titles.len() {
        let title = titles[choice - 1].clone();
        if task_log.mark_done(title.clone()) {
            println!("Done");
        } else {
            println!("Error");
        }
    }
    false
}

fn activate_tasks(task_log: &mut TaskLog) -> bool {
    task_log.activate(&mut thread_rng());
    true
}

fn main() {
    let main_menu: Menu<TaskLog> = mm("test", "Exit", vec![
        ("New active task", m_call(new_active)),
        ("New pooled task", m_call(new_pool)),
        ("Print active tasks", m_call(print_actives)),
        ("Print pooled tasks", m_call(print_pool)),
        ("Print log", m_call(print_log)),
        ("Save log", m_call(save_log)),
        ("Load log", m_call(load_log)),
        
        ("Pick tasks", sub_menu(m("Are you sure?", "No", vec![
            ("Yes", m_call(activate_tasks))
        ]))),
        ("Mark done", m_call(mark_done))
    ]);
    let mut task_log = TaskLog::new();
    main_menu.run(&mut task_log);
}
