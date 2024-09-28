extern crate linfa;
extern crate linfa_linear;
extern crate ndarray;

use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::(Array2,Array1);
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> Result<(), Box <dyn std::error:Error>> {
    //loading data from .csv file (2 columns x, y
    let (features, targets) = load_csv_data("data.csv")?;

    //Convert the features and targets into ndarray structures
    let features= Array2::from_shape_vec((features.len(),10),features)?;
    let targets= Array1:: from(targets);

    //lin reg model creation

    let model= LinearRegression::default();

    //fit model to dataset
    let model = model.fit(&features, &targets).unwrap();

    //make predictions (can change test features as needed)
    let predictions = model.predict (&features);

    //do something with predictions here (output into a file, back into system, etc.

    Ok(())

}


fn load_csv_data filename: &str) -> Result <(vec<f64>, vec<f64>), Box <dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader= BufReader::new(file);

    let mut features = Vec::new();
    let mut targets= Vec ::new();

    // skip header of csv

    for (i, line) in reader.lines().enumerate(){
        let line= line?;
        if i==0 {continue;}
        
        let parts: vec<&str> = line.split(',').collect();
        
        for j in 0..10{
            features.push(parts[j].parse::<f64>()?);
        }
        
        targets.push(parts[10].parse::<f64>()?);
    }

    Ok((features, targets))

}
