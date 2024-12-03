use solutions::{
    day1::hystorian_hysterica::HystorianHysteria,
    day2::red_nosed_reports::RedNosedReports,
    day3::mull_it_over::MullItOver,
    solution::Solution,
};

pub mod solutions;
pub mod utils;

fn main() {
    let hystorian_hysterica = HystorianHysteria;
    let red_nosed_reports = RedNosedReports;
    let mull_it_over = MullItOver;
    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(hystorian_hysterica),
        Box::new(red_nosed_reports),
        Box::new(mull_it_over),
    ];

    for (index, solution) in (&solutions).iter().enumerate() {
        let day = index + 1;
        println!("\n------------ SOLUTION OF DAY {} ------------", day);
        println!("\n~~~~~~~~~~~~~~~~~~TASK 1:~~~~~~~~~~~~~~~~~~ \n");
        solution.solve_first_task();
        println!("\n~~~~~~~~~~~~~~~~~~TASK 2:~~~~~~~~~~~~~~~~~~ \n");
        solution.solve_second_task();
        println!("\n")
    }
}
