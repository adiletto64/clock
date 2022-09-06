#![allow(dead_code)]

use std::{thread, time::Duration};

mod intersection;


const RADIUS: u32 = 20;
const LENGTH: usize = (RADIUS * 2) as usize + 1;
const SPEED: u64 = 1000;

// as char width is less than it's height we should compensate increasing width
const SPREED: usize = 2;


struct Clock {
    seconds: u32,
    minutes: u32,
    hours: u32
}


type Matrix<'a> = [[&'a str; LENGTH * SPREED]; LENGTH];


fn main() {

    let mut matrix: Matrix = [[" "; LENGTH * SPREED]; LENGTH];
    

    let mut current_angle = 180.0;
    write_circle(&mut matrix);

    print!("\x1B[2J\x1B[1;1H");

    loop {

        let mut previus_cooridates: Vec<(usize, usize)> = Vec::new();

        write_circle(&mut matrix);
        write_arrow(&mut matrix, current_angle, &mut previus_cooridates);


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
        current_angle -= 6.0;




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


fn write_arrow(matrix: &mut Matrix, angle: f32, previous_coordinates: &mut Vec<(usize, usize)>) {
    let radius = RADIUS as i32;

    let cosinus = (angle as f32).to_radians().cos();
    let sinus = (angle as f32).to_radians().sin();

    let x = sinus * radius as f32 * SPREED as f32;
    let y = cosinus * radius as f32;

    const THICKNESS: f32 = 1.5;

    for col in 0..LENGTH {
        for row in 0..LENGTH * SPREED as usize {

            let pos_y = col as i32 - radius;
            let pos_x = row as i32 - radius * SPREED as i32;

            let point = intersection::Point::new(pos_x as f32, pos_y as f32);
            let point_xplus_yplus = intersection::Point::new(pos_x as f32 + THICKNESS, pos_y as f32 + THICKNESS);

            let point_xplus = intersection::Point::new(pos_x as f32 + THICKNESS, pos_y as f32);
            let point_yplus = intersection::Point::new(pos_x as f32, pos_y as f32 + THICKNESS);

            let start_point = intersection::Point::new(0.0, 0.0);
            let end_point = intersection::Point::new(x, y);


            if intersection::determine_intersection_exists(&point, &point_xplus_yplus, &start_point,& end_point) ||
               intersection::determine_intersection_exists(&point_xplus,& point_yplus,& start_point, &end_point)  {
                matrix[col][row] = "*";
                previous_coordinates.push((col, row));
            }
        }
    }

}


fn clean_previus_arrow(matrix: &mut Matrix) {
    let radius = RADIUS as i32;

    for col in 0..LENGTH {
        for row in 0..LENGTH * SPREED as usize {

            let pos_y = col as i32 - radius;
            let pos_x = row as i32 - radius * SPREED as i32;

            let hypotenuse_len = (((pos_x / SPREED as i32).pow(2) + pos_y.pow(2)) as f32).sqrt();
            
            if hypotenuse_len + 1.0 < radius as f32 {
                if matrix[col][row] == "*" {
                    matrix[col][row] = " ";
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
