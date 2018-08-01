pub mod iso_8601;
pub mod postgre_style;
pub mod sql;

fn safe_abs_u32(mut num: i32) -> u32 {
    let max = i32::max_value(); 
    let max_min = max * -1; 
    if num <= max_min {
        let result = max as u32; 
        num += max; 
        num = num * -1; 
        let result = result + num as u32;
        result 
    } else {
        num.abs() as u32
    }
}