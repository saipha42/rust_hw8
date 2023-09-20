use hw8::point::{Point, PolarPoint};
use std::{fs::File, error::Error, process};

fn main() {

    let pt_list = match load_points("./polar_pts.csv") {
        Ok(pts) => pts,
        Err(e) => {
            eprintln!("Error at loading csv contents : {}", e);
            process::exit(1)
        }
    };

    let polar = to_cartesian(&pt_list);

   
    match save_polar_points(&polar) {
        Ok(_) => println!("Successfully save cartisian points"),
        Err(e) => {
            eprintln!("Error at saving csv contents : {}", e);
            process::exit(1)
        }
    };

}

fn load_points(file_path : &str) -> Result<Vec<PolarPoint>, Box<dyn Error>> {
    
    let mut points: Vec<PolarPoint> = Vec::new();
    let pt_list = File::open(file_path)?;
    let mut pt_list = csv::ReaderBuilder::new().has_headers(false).from_reader(pt_list);

    for pt in pt_list.records() {
        if let Ok(rec) = pt {
            points.push(PolarPoint { r: rec[0].parse::<f32>().unwrap(), t: rec[1].parse::<f32>().unwrap()})
        }

    }
    Ok(points)
}

fn to_cartesian(pt_list: &[PolarPoint])-> Vec<Point> {

    let mut polar_list = Vec::new();

    for i in 0..pt_list.len() {
        let PolarPoint {r, t}= pt_list[i];
        let x = (r* t.cos()).round();
        let y = (r* t.sin()).round();

        polar_list.push( Point { x, y } );
    }


    polar_list
}

fn save_polar_points(polar_pts : &[Point]) -> Result<(), Box<dyn Error>> {

    let save_file_name = "saved_catisian_points.csv";
    let mut writer = csv::Writer::from_path(save_file_name)?;

    for i in 0..(polar_pts).len() {

        let x = polar_pts[i].x.to_string();
        let y = polar_pts[i].y.to_string();
        writer.write_record(vec![ x, y])?;

    }

    Ok(())
}