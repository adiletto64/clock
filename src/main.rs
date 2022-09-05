#![allow(dead_code)]

use std::{thread, time::Duration};

mod geometry;


const RADIUS: u32 = 10;
const LENGTH: usize = (RADIUS * 2) as usize + 1;
const SPEED: u64 = 500;

// as char width is less than it's height we should compensate increasing width
const SPREED: usize = 3;


struct Clock {
    seconds: u32,
    minutes: u32,
    hours: u32
}


type Matrix<'a> = [[&'a str; LENGTH * SPREED]; LENGTH];


fn main() {

    let mut matrix: Matrix = [[" "; LENGTH * SPREED]; LENGTH];
    
    write_circle(&mut matrix);

    for row in matrix {
        for col in row {
            print!("{}", col);
        }

        println!();
    }

    
}



fn write_circle(matrix: &mut Matrix) {
    let radius = RADIUS as i32;

    for angle in 0..360 {

        let cosinus = (angle as f32).cos().abs();
        let sinus = (angle as f32).sin().abs();

        let x = sinus * radius as f32;
        let y = cosinus * radius as f32;

        for col in 0..LENGTH {
            for row in 0..LENGTH {

                let pos_y = col as f32;
                let pos_x = row as f32;

                if pos_y > y && pos_y < y + 1.0 && pos_x > x && pos_x < x + 1.0 {
                    matrix[col][row] = "*";
                }

            }
        }
    }




}



/// return max y, x and min x, y for specific part of circle by angle
fn calculate_part(angle: f32) -> ((i32, i32), (i32, i32)) {


    let dis_y = RADIUS as i32;
    let dis_x = (RADIUS as i32) * SPREED as i32;
   
    if angle <= 90.0 {
        return ((0, -dis_y), (dis_x, 0));

    } else if angle >= 90.0 && angle <= 180.0 {
        return ((0, 0), (dis_x, dis_y));

    } else if angle >= 180.0 && angle <= 270.0 {
        return ((-dis_x, 0), (0, dis_y));
    } else {
        return ((-dis_x, -dis_y), (0, 0));
    }
}


/// check if point in correct part of circle quarter
fn validate_angle_part(angle: f32, x: i32, y: i32) -> bool {
    let ((min_x, min_y), (max_x, max_y)) = calculate_part(angle);

    return x > min_x && x < max_x && y > min_y && y < max_y;

}
