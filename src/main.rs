use solutions::{
    day1::hystorian_hysterica::HystorianHysteria, day2::red_nosed_reports::RedNosedReports, day3::mull_it_over::MullItOver, day4::ceres_search::CeresSearch, day5::print_queue::PrintQueue, day6::guard_gallivant::GuardGallivant, solution::Solution
};

pub mod solutions;
pub mod utils;

fn main() {
    let hystorian_hysterica = HystorianHysteria;
    let red_nosed_reports = RedNosedReports;
    let mull_it_over = MullItOver;
    let ceres_search = CeresSearch;
    let print_queue = PrintQueue;
    let guard_gallivant = GuardGallivant;

    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(hystorian_hysterica),
        Box::new(red_nosed_reports),
        Box::new(mull_it_over),
        Box::new(ceres_search),
        Box::new(print_queue),
        Box::new(guard_gallivant)
    ];

    for (index, solution) in (&solutions).iter().enumerate() {
        let day = index + 1;
        println!("\n------------ SOLUTION OF DAY {} ------------", day);
        println!("\n~~~~~~~~~~~~~~~~~~TASK 1:~~~~~~~~~~~~~~~~~~ \n");
        solution.solve_first_task();
        println!("\n~~~~~~~~~~~~~~~~~~TASK 2:~~~~~~~~~~~~~~~~~~ \n");
        solution.solve_second_task();
        println!("\n");
    }
}
