use hw8::point::{Point, PolarPoint};
use std::{fs::File, error::Error, process};

fn main() {

    let pt_list = match load_points("./pt_list.csv") {
        Ok(pts) => pts,
        Err(e) => {
            eprintln!("Error at loading csv contents : {}", e);
            process::exit(1)
        }
    };

    let polar = to_polar(&pt_list);

   
    match save_polar_points(&polar) {
        Ok(_) => println!("Successfully save polar points"),
        Err(e) => {
            eprintln!("Error at loading csv contents : {}", e);
            process::exit(1)
        }
    };

}

fn load_points(file_path : &str) -> Result<Vec<Point>, Box<dyn Error>> {
    
    let mut points: Vec<Point> = Vec::new();
    let pt_list = File::open(file_path)?;
    let mut pt_list = csv::ReaderBuilder::new().has_headers(false).from_reader(pt_list);

    for pt in pt_list.records() {
        if let Ok(rec) = pt {
            points.push(Point { x: rec[0].parse::<f32>().unwrap(), y: rec[1].parse::<f32>().unwrap()})
        }

    }
    Ok(points)
}

fn to_polar(pt_list: &[Point])-> Vec<PolarPoint> {

    let mut polar_list = Vec::new();

    for i in 0..pt_list.len() {
        let Point {x, y}= pt_list[i];
        let r = ((x* x + y* y).sqrt()*100.0).round()/ 100.0;
        let t = (y.atan2(x)*100.0).round()/100.0;

        polar_list.push( PolarPoint { r, t } );
    }
    polar_list
}

fn save_polar_points(polar_pts : &[PolarPoint]) -> Result<(), Box<dyn Error>> {

    let save_file_name = "saved_polar_points.csv";
    let mut writer = csv::Writer::from_path(save_file_name)?;

    for i in 0..(polar_pts).len() {

        let r = polar_pts[i].r.to_string();
        let t = polar_pts[i].t.to_string();
        writer.write_record(vec![ r, t])?;

    }

    Ok(())
}