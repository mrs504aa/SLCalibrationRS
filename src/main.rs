#![allow(non_snake_case)]

use core::f64::consts::PI;

fn BandEnergy(Dc: f64, Om: f64, k: f64) -> f64 {
    return 0.5 * (Dc + (8.0 * (1.0 + (2.0 * k).cos()) * Om.powi(2) + Dc.powi(2)).sqrt());
}

fn GenerateDatabase() -> (Vec<f64>, Vec<f64>) {
    let mut RatioList: Vec<f64> = vec![0.0; 20001];
    let mut BCList: Vec<f64> = vec![0.0; 20001];
    let (mut k, mut Sum): (f64, f64);

    for i in 0..20001 {
        RatioList[i] = 2.0 * (i as f64) / 20000.0
    }

    for i in 0..20001 {
        Sum = 0.0;
        for j in 0..1000 {
            k = PI * (j as f64) / (1000.0);
            Sum += BandEnergy(1.0, RatioList[i], k);
        }
        BCList[i] = Sum / 1000.0;
    }

    return (RatioList, BCList);
}

fn Interpolate(xData: &Vec<f64>, yData: &Vec<f64>, x: f64) -> f64 {
    let DataSize: usize = xData.len();
    let mut i: usize = 0;

    if x >= xData[DataSize - 2] {
        i = DataSize - 2
    } else {
        while x > xData[i + 1] {
            i += 1;
        }
    }

    let xL: f64 = xData[i];
    let yL: f64 = yData[i];
    let xR: f64 = xData[i + 1];
    let yR: f64 = yData[i + 1];
    let dydx: f64 = (yR - yL) / (xR - xL);

    return yL + dydx * (x - xL);
}
fn main() {
    let mut InputLine: String;
    let (mut BC, mut DC): (f64, f64);
    let (RatioList, BCList): (Vec<f64>, Vec<f64>) = GenerateDatabase();

    loop {
        InputLine = String::new();
        println!("-------------------------------------------");
        println!("Input \"exit\" to exit.");
        println!("Input Peak or Dip Position, unit Gamma or MHz:");
        std::io::stdin().read_line(&mut InputLine).unwrap();
        InputLine = InputLine.trim_end().to_string();
        if InputLine == "exit" {
            break;
        }
        match InputLine.parse::<f64>() {
            Ok(val) => BC = val,
            Err(_err) => {
                println!("not a number");
                continue;
            }
        };

        InputLine = String::new();
        println!("Input \"exit\" to exit.");
        println!("Input Laser Detuning, unit Gamma or MHz:");
        std::io::stdin().read_line(&mut InputLine).unwrap();
        InputLine = InputLine.trim_end().to_string();
        if InputLine == "exit" {
            break;
        }
        match InputLine.parse::<f64>() {
            Ok(val) => DC = val,
            Err(_err) => {
                println!("not a number");
                continue;
            }
        };

        let Max: f64 = BCList.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let Min: f64 = BCList.iter().cloned().fold(f64::INFINITY, f64::min);

        if BC / DC <= Min {
            println!("Too small number !! Plz Remake database!");
            continue;
        }
        if BC / DC >= Max {
            println!("Too big number !! Plz Remake database!");
            continue;
        }

        let Result: f64 = Interpolate(&BCList, &RatioList, BC / DC);
        println!("The Rabi Frequency is {:.3} Gamma or MHz.", Result * DC);
    }

    return ();
}
