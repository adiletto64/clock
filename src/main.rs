#![allow(dead_code)]

use std::{thread, time::Duration};
use chrono::prelude::*;

mod intersection;


const RADIUS: u32 = 20;
const LENGTH: usize = (RADIUS * 2) as usize + 1;
const SPEED: u64 = 1000;

// as char width is less than it's height we should compensate increasing width
const SPREED: usize = 2;


struct Arrow {
    angle_per_second: f32,
    thickness: f32,
    angle: f32,
    len: f32
}


type Matrix<'a> = [[&'a str; LENGTH * SPREED]; LENGTH];


fn main() {

    let mut matrix: Matrix = [[" "; LENGTH * SPREED]; LENGTH];
    let dt = Utc::now();

    let mut seconds = Arrow {
        angle_per_second: 6.0,
        thickness: 0.5,
        angle:  dt.second() as f32 * 6.0 + 90.0,
        len: RADIUS as f32,
    };

    let mut minutes = Arrow {
        angle_per_second: 0.1,
        thickness: 0.5,
        angle: dt.minute() as f32 * 6.0 + 90.0,
        len: RADIUS as f32 - 5.0
    };

    let mut hours = Arrow {
        angle_per_second: 0.01666666666666666666666666666666666,
        thickness: 0.5,
        angle: dt.hour() as f32 * 30.0 + 90.0,
        len: RADIUS as f32 - 10.0,
    };

    loop {

        let mut previus_cooridates: Vec<(usize, usize)> = Vec::new();

        write_circle(&mut matrix);
        write_arrow(&mut matrix, vec![&seconds, &minutes, &hours], &mut previus_cooridates);


        for row in matrix {

            for col in row {
                print!("{}", col);
            }

            println!();
        }
        //clean_previus_arrow(&mut matrix);  
        clean_previous(&mut matrix, &mut previus_cooridates);
        previus_cooridates.clear();

        thread::sleep(Duration::from_millis(SPEED));

        seconds.angle -= seconds.angle_per_second;
        minutes.angle -= minutes.angle_per_second;
        hours.angle -= hours.angle_per_second;

        print!("\x1B[2J\x1B[1;1H");


    }

}



fn write_circle(matrix: &mut Matrix) {
    let radius = RADIUS as i32;

    for angle in 0..360 {

        let cosinus = (angle as f32).cos();
        let sinus = (angle as f32).sin();

        let x = sinus * radius as f32 * SPREED as f32;
        let y = cosinus * radius as f32;

        for col in 0..LENGTH {
            for row in 0..LENGTH * SPREED as usize {

                let pos_y = (col as i32 - radius) as f32 + 1.0;
                let pos_x = (row as i32 - radius * SPREED as i32) as f32 + 1.0;

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
        for row in 0..LENGTH * SPREED as usize {

          

            for arrow in &arrows {

                let pos_y = col as i32 - radius;
                let pos_x = row as i32 - radius * SPREED as i32;

                let cosinus = (arrow.angle as f32).to_radians().cos();
                let sinus = (arrow.angle as f32).to_radians().sin();            

                let x = sinus * radius as f32 * SPREED as f32;
                let y = cosinus * radius as f32;           


                let start_point = intersection::Point::new(0.0, 0.0);
                let end_point = intersection::Point::new(x, y);  


                let point = intersection::Point::new(pos_x as f32, pos_y as f32);
                let point_xplus_yplus = intersection::Point::new(pos_x as f32 + arrow.thickness, pos_y as f32 + arrow.thickness);

                let point_xplus = intersection::Point::new(pos_x as f32 + arrow.thickness, pos_y as f32);
                let point_yplus = intersection::Point::new(pos_x as f32, pos_y as f32 + arrow.thickness);

                if intersection::determine_intersection_exists(&point, &point_xplus_yplus, &start_point,& end_point) ||
                intersection::determine_intersection_exists(&point_xplus,& point_yplus,& start_point, &end_point)  {

                    let hypotenuse_len = ((pos_x * pos_x / 4.0 as i32 + pos_y * pos_y) as f32).sqrt();

                    if hypotenuse_len < arrow.len {
                        matrix[col][row] = "*";
                        previous_coordinates.push((col, row));                            
                    }                

                }                
            }


        }
    }

}


fn clean_previous(matrix: &mut Matrix, previous_coordinates: &mut Vec<(usize, usize)>) {
    for col in 0..LENGTH {
        for row in 0..LENGTH * SPREED as usize {
            for point in previous_coordinates.iter_mut() {
                if point.0 == col && point.1 == row {
                    matrix[col][row] = " ";
                }
            }
        }
    }
}
