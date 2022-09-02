use std::env::{args, Args};
fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let f_num = first.parse::<f32>().unwrap();
    let s_num = second.parse::<f32>().unwrap();
    let result = operate(operator, f_num, s_num);

    println!("{:?}", output(f_num, operator, s_num, result));

    fn operate(operator: char, f_num: f32, s_num: f32) -> f32 {
        match operator {
            '+' => f_num + s_num,
            '-' => f_num - s_num,
            '/' => f_num / s_num,
            'x' => f_num * s_num,
            _ => panic!("Invalid operator: {} used.", operator),
        }
    }

    fn output(f_num: f32, operator: char, s_num: f32, result: f32) -> String {
        format!("{} {} {} = {}", f_num, operator, s_num, result)
    }
}
