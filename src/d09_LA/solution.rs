use std::{fs, process::exit, f64::NEG_INFINITY};
use regex::Regex;

extern crate nalgebra as na;
use na::{DMatrix, DVector};

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d09/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

fn pair_vecs<T: Copy, U: Copy>(vec1: &Vec<T>, vec2: &Vec<U>) -> Vec<(T, U)> {
    vec1.iter().zip(vec2.iter()).map(|(&a, &b)| (a, b)).collect()
}

fn poly_of_x(poly_as: &Vec<f64>, x: i128) -> f64 {
    poly_as.iter().enumerate().map(|(idx, coef)| *coef * x.pow(idx as u32) as f64).fold(0.0, |acc, iter| acc + iter)
}

fn calculate_abs_difference(vec1: &Vec<i128>, vec2: &Vec<i128>) -> i128 {
    if vec1.len() != vec2.len() {panic!("vecs not of same size")};

    let res = vec1.into_iter()
        .zip(vec2.into_iter())
        .map(|(&a, &b)| {
            (a - b).abs()  // compute absolute difference of each pair
        })
        .sum::<i128>();  // sum up all the absolute differences
    res
}

fn create_vec(n: usize, xs: &Vec<i128>) -> Vec<i128> {
    let mut vec: Vec<i128> = Vec::new();
    for i in 0..n+1 {
        for j in i..(i+n+1) {
            vec.push(xs.iter().map(|x| x.pow(j as u32)).sum());
        }
    }
    vec
}

fn regression(degree: usize, xs: &Vec<i128>, ys: &Vec<i128>) -> DVector<f64> {
    let x: DMatrix<i128> = DMatrix::from_row_slice(
        degree+1, degree+1,
        &create_vec(degree, &xs)
    );

    let b: DVector<i128> = DVector::from_column_slice(
        &(0..(degree+1)).map(|i| {
            pair_vecs::<i128,i128>(&xs.iter().map(|x| x.pow(i as u32)).collect::<Vec<i128>>(), &ys)
                .iter().fold(0, |acc, iter| acc + iter.0 * iter.1)
            }).collect::<Vec<i128>>()
    );

    let x_inv = x.map(|x| x as f64).try_inverse();

    let a:  DVector<f64> = x_inv.unwrap() * b.map(|x| x as f64);
    a
}

pub fn p1() {
    let input_lines = read_puzzle_input().split("\n").filter_map(|val| if val != "" {Some(val.to_owned())} else {None}).collect::<Vec<String>>();
    let whitespaces = Regex::new(r"\s+").unwrap();
    let data_points_of_functions = input_lines.iter()
        .map(
            |line| {
                let line = whitespaces.replace_all(line, ",");
                line.split(",").map(|x| {
                    x.parse::<i128>().unwrap()
                }
                ).collect::<Vec<i128>>()   
            }
        )
        .collect::<Vec<Vec<i128>>>();
    let sum = data_points_of_functions.iter().map(|ys| get_next_value(ys)).fold(0, |acc,iter| acc + iter.round() as usize);
    println!("sum: {}{}", sum, if sum <= 1796116935 {" which is too small"} else {""});
}

fn get_next_value(ys: &Vec<i128>) -> f64 {
    let n = ys.len() as i128;
    let xs = (1..(n+1)).into_iter().collect::<Vec<i128>>();
    let func = get_regression_with_minimal_viable_degree(&xs, &ys);

    let result = poly_of_x(&func, n+1);

    result
}

fn get_regression_with_minimal_viable_degree(xs: &Vec<i128>, ys: &Vec<i128>) -> Vec<f64> {
    let mut degree = 0;
    let mut errors: Vec<_>     = vec![];
    let mut funcs: Vec<Vec<f64>> = vec![];
    loop {
        println!("\ntry regression for {}", degree);
        let func = regression(degree, &xs, &ys).iter().map(|x| x.to_owned()).collect::<Vec<f64>>();
        let y_hat = xs.iter().map(|x| poly_of_x(&func, *x).round() as i128).collect::<Vec<i128>>();
        let error = calculate_abs_difference(ys, &y_hat);
        println!("y : {:?}", ys);
        println!("y^: {:?}", y_hat);
        println!("\nerror: {}", error);
        println!("func: {func:?}");
        if error > *errors.last().or_else(|| Some(&i128::MAX)).unwrap() {
            println!("\ntry with less numbers at degree {} (existing elements: {})", degree, xs.len());
            let xs = &xs[(xs.len()-degree)..].to_vec();
            let ys = &ys[(ys.len()-degree)..].to_vec();
            println!("{xs:?}");
            let new_func = regression(degree+1, xs, &ys).iter().map(|x| x.to_owned()).collect::<Vec<f64>>();
            let y_hat = xs.iter().map(|x| poly_of_x(&new_func, *x).round() as i128).collect::<Vec<i128>>();
            println!("ys: {ys:?}");
            println!("y^: {y_hat:?}");
            let error = calculate_abs_difference(&ys, &y_hat);
            println!("\nerror: {}", error);
            println!("func: {func:?}");
            
            exit(1)
        }
        if error < 1 {
            println!("line required a regression of degree {} to archive r-squarred error of {}", degree, error);
            println!("{errors:?}");
            return func;
        }
        errors.push(error);
        funcs.push(func);
        degree+=1;
    }
}

pub fn p2() {
    let input_lines = read_puzzle_input().split("\n").map(|val| val.to_owned()).collect::<Vec<String>>();
    println!("{input_lines:#?}");
}
