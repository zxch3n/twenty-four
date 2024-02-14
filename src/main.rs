use twenty_four::solve_list;

pub fn main() {
    // read from terminal args
    let args: Vec<String> = std::env::args().collect();
    let ans = solve_list(
        24,
        &args
            .iter()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect::<Vec<isize>>(),
    );
    if let Some(mut ans) = ans {
        println!("{}", ans.show());
    } else {
        println!("No solution");
    }
}
