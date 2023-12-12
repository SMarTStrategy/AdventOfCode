use std::convert::TryInto;


fn main() {
//     let input = "Time:      7  15  30
// Distance:  9  40  200";
    let input = "Time:        44707080
Distance:   283113411341491";

    let time_and_distance: [[i64; 1]; 2]  = input.split("\n").collect::<Vec<&str>>()
        .into_iter().map(|line| {
        return line.split(":").collect::<Vec<&str>>()[1].trim().split("  ").collect::<Vec<&str>>().into_iter().map(|x| {
            println!("{}", x);
            x.parse().unwrap()
        }).collect::<Vec<i64>>().try_into().unwrap();
    }).collect::<Vec<[i64; 1]>>().try_into().unwrap();
    let mut result = 1;
    for (i, time) in time_and_distance[0].iter().enumerate() {
        let mut min = get_min_press_short(time.clone(), time_and_distance[1][i as usize]).unwrap();
        // let mut max = get_min_press_long(time.clone(), time_and_distance[1][i as usize]).unwrap();
        let mut max = time - min;
        println!("{} {}", min, max );
        result *= (max - min) + 1;
    }

    println!("{}", result);

}

fn get_min_press_short(time: i64, distance_to_beat: i64) -> Option<i64> {
    for i  in 0..time {
        if (i * (time - i)) > distance_to_beat {
            return Some(i.into());
        }
    }
    return None;
}
