use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn split_dimension(dm: String) -> Vec<i32> {
    return dm
        .split(|sl| sl == 'x')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn calc_raw_area(dm: Vec<i32>) -> Vec<i32> {
    return vec![dm[0] * dm[1], dm[1] * dm[2], dm[2] * dm[0]];
}

fn calc_area(dm: &Vec<i32>) -> Vec<i32> {
    return dm.clone().into_iter().map(|x| x * 2).collect::<Vec<i32>>();
}

fn add_vec(dm: &Vec<i32>) -> i32 {
    return dm.clone().into_iter().reduce(|a, b| a + b).unwrap();
}

fn multiply_vec(dm: &Vec<i32>) -> i32 {
    return dm.clone().into_iter().reduce(|a, b| a * b).unwrap();
}

fn find_smallest(acc: i32, cur: i32) -> i32 {
    if cur < acc {
        return cur;
    } else {
        return acc;
    }
}

fn find_largest(acc: i32, cur: i32) -> i32 {
    if cur > acc {
        return cur;
    } else {
        return acc;
    }
}

fn find_slack(dm: &Vec<i32>) -> i32 {
    return dm.clone().into_iter().fold(dm[0], find_smallest);
}

fn find_smallest_distance(dm: &Vec<i32>) -> i32 {
    if (dm.clone().into_iter().min() == dm.clone().into_iter().max()) {
        // All sides the same
        return dm[0] * 4;
    } else if (dm
        .clone()
        .iter()
        .filter(|&n| *n == dm.clone().into_iter().max().unwrap())
        .count()
        > 1)
    {
        // 2 sides the same
        let first_position_found = dm
            .clone()
            .into_iter()
            .position(|x| x != dm.clone().into_iter().max().unwrap())
            .unwrap();
        let mut remove = dm.clone();
        remove.remove(first_position_found);
        let multiplied_value = remove.into_iter().fold(0, |a, b| a + (b * 2));
        // println!("Multiplied Value: {:?}", multiplied_value);
        return multiplied_value;
    } else {
        // println!("Original Value : {:?}", dm);
        let largest_value = dm.clone().into_iter().fold(dm[0], find_largest);
        // println!("Largest Value: {:?}", largest_value);
        let filtered_value = dm
            .clone()
            .into_iter()
            .filter(|&x| x != largest_value)
            .collect::<Vec<i32>>();
        // println!("Removed Largest Value: {:?}", filtered_value);
        let multiplied_value = filtered_value.into_iter().fold(0, |a, b| a + (b * 2));
        // println!("Multiplied Value: {:?}", multiplied_value);
        return multiplied_value;
    }
}

fn main() {
    let file = File::open("../boxes.txt").unwrap();
    let reader = BufReader::new(file);
    let dimension = reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    // let dimension = vec!("2x3x4".to_string(), "1x1x10".to_string());
    let full_size = dimension
        .clone()
        .into_iter()
        .map(|dm| {
            let raw_dm: Vec<i32> = calc_raw_area(split_dimension(dm));
            let full_dm: i32 = add_vec(&calc_area(&raw_dm)) + find_slack(&raw_dm);
            return full_dm;
        })
        .reduce(|acc, dm| acc + dm)
        .unwrap();
    println!("full size: {}", full_size);
    let total_ribbon = dimension
        .clone()
        .into_iter()
        .map(|dm| {
            let raw_dm: Vec<i32> = split_dimension(dm.clone());
            let full_dm: u64 = (multiply_vec(&split_dimension(dm))
                + find_smallest_distance(&raw_dm))
            .try_into()
            .unwrap();
            // println!("{:?}", full_dm);
            return full_dm;
        })
        .reduce(|acc, dm| acc + dm)
        .unwrap();
    println!("Total Ribbon: {}", total_ribbon);
}
