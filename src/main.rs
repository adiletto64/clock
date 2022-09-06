#![allow(dead_code)]

use std::{thread, time::Duration};
use chrono::prelude::*;


mod intersection;


const RADIUS: u32 = 22;
const LENGTH: usize = (RADIUS * 2) as usize + 1;
const SPEED: u64 = 1000;

// as char width is less than it's height we should compensate increasing width
const STRAIN: usize = 2;


struct Arrow {
    angle_per_second: f64,
    thickness: f64,
    angle: f64,
    len: f64
}


type Matrix<'a> = [[&'a str; LENGTH * STRAIN]; LENGTH];


fn main() {

    let mut matrix: Matrix = [[" "; LENGTH * STRAIN]; LENGTH];
    let dt = Local::now();

    let mut seconds = Arrow {
        angle_per_second: 6.0,
        thickness: 0.6,
        angle: dt.second() as f64 * 6.0,
        len: RADIUS as f64,
    };

    let mut minutes = Arrow {
        angle_per_second: 0.1,
        thickness: 0.8,
        angle: dt.minute() as f64 * 6.0,
        len: (RADIUS - RADIUS / 4) as f64
    };

    let mut hours = Arrow {
        angle_per_second: 0.01666666666,
        thickness: 1.0,
        angle: dt.hour() as f64 * 30.0,
        len: (RADIUS - RADIUS / 2) as f64
    };

    write_circle(&mut matrix);

    loop {

        // as clock arrow moves we should clear previous, so we need to keep 
        // printed cells for futhermore cleaning
        let mut previus_cooridates: Vec<(usize, usize)> = Vec::new();

       
        write_arrow(
            &mut matrix, 
            vec![&seconds, &minutes, &hours], 
            &mut previus_cooridates
        );

        // print to consle
        for row in matrix {

            for col in row {
                print!("{}", col);
            }
            println!();
        }


        clean_previous(&mut matrix, &mut previus_cooridates);
        previus_cooridates.clear();

        thread::sleep(Duration::from_millis(SPEED));

        seconds.angle += seconds.angle_per_second;
        minutes.angle += minutes.angle_per_second;
        hours.angle += hours.angle_per_second;

        // go to next empty line at row 0 col 0
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);


    }

}



fn write_circle(matrix: &mut Matrix) {
    let radius = RADIUS as i32;

    for angle in 0..360 {

        let cosinus = (angle as f64).to_radians().cos();
        let sinus = (angle as f64).to_radians().sin(); 

        let x = sinus * radius as f64 * STRAIN as f64;
        let y = cosinus * radius as f64;

        for col in 0..LENGTH {
            for row in 0..LENGTH * STRAIN as usize {

                let pos_y = (col as i32 - radius) as f64 + 1.0;
                let pos_x = (row as i32 - radius * STRAIN as i32) as f64 + 1.0;

                if pos_y > y && pos_y < y + 1.0 && pos_x > x && pos_x < x + 1.0 {
                    matrix[col][row] = "*";
                }
            }
        }
    }
}


fn write_arrow(matrix: &mut Matrix, arrows: Vec<&Arrow>, previous_coordinates: &mut Vec<(usize, usize)>) {
    let radius = RADIUS as i32;


    for col in 0..LENGTH {
        for row in 0..LENGTH * STRAIN as usize {

            let pos_y = col as i32 - radius;
            let pos_x = row as i32 - radius * STRAIN as i32;          

            // write arrows
            for arrow in &arrows {


                let cosinus = (arrow.angle as f64 + 270.0).to_radians().cos();
                let sinus = (arrow.angle as f64 + 270.0).to_radians().sin();            

                let x = cosinus * radius as f64 * STRAIN as f64;
                let y = sinus * radius as f64;           


                let start_point = intersection::Point::new(0.0, 0.0);
                let end_point = intersection::Point::new(x, y);  


                let point = intersection::Point::new(
                    pos_x as f64, 
                    pos_y as f64
                );

                let is_crosses = check_double_cross_intersection(
                    &start_point, 
                    &end_point, 
                    &point, 
                    &arrow.thickness
                );

                if is_crosses  {

                    let hypotenuse_len = ((pos_x * pos_x / 4.0 as i32 + pos_y * pos_y) as f64).sqrt();

                    if hypotenuse_len < arrow.len {
                        matrix[col][row] = "*";
                        previous_coordinates.push((col, row));                            
                    }                
                }                
            }

            // write every time mark from 1 to 12 for 30 decree
            for n in 1..13 {
                let cosinus = (n as f64 * 30.0 + 270.0).to_radians().cos();
                let sinus = (n as f64 * 30.0 + 270.0).to_radians().sin();            

                let x = cosinus * radius as f64 * STRAIN as f64;
                let y = sinus * radius as f64;           


                let start_point = intersection::Point::new(0.0, 0.0);
                let end_point = intersection::Point::new(x, y);  


                let point = intersection::Point::new(
                    pos_x as f64, 
                    pos_y as f64
                );

                let is_crosses = check_double_cross_intersection(
                    &start_point, 
                    &end_point, 
                    &point, 
                    &1.0,
                );

                if is_crosses  {

                    let hypotenuse_len = ((pos_x * pos_x / 4.0 as i32 + pos_y * pos_y) as f64).sqrt();

                    if hypotenuse_len > (RADIUS as f64 / 1.3) {
                        matrix[col][row] = "*";
                        previous_coordinates.push((col, row));                            
                    }                
                }  
            }



        }
    }

}


/// check if line segment created by start_point and end_point
/// goes on cell. To check we create cross lines from cell
/// and check if line crosses at least one of them
fn check_double_cross_intersection(
    start_point: &intersection::Point, 
    end_point: &intersection::Point, 
    cell: &intersection::Point,
    thickness: &f64
) -> bool {
    let point_xplus_yplus = intersection::Point::new(
        cell.x as f64 + thickness, 
        cell.y as f64 + thickness
    );

    let point_xplus = intersection::Point::new(
        cell.x as f64 + thickness, 
        cell.y as f64
    );

    let point_yplus = intersection::Point::new(
        cell.x as f64, 
        cell.y as f64 + thickness
    );

    return intersection::determine_intersection_exists(&cell, &point_xplus_yplus, &start_point, &end_point) ||
           intersection::determine_intersection_exists(&point_xplus,&point_yplus,  &start_point, &end_point);

}


fn clean_previous(matrix: &mut Matrix, previous_coordinates: &mut Vec<(usize, usize)>) {
    for col in 0..LENGTH {
        for row in 0..LENGTH * STRAIN as usize {
            for point in previous_coordinates.iter_mut() {
                if point.0 == col && point.1 == row {
                    matrix[col][row] = " ";
                }
            }
        }
    }
}
