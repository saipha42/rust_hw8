use hw8::point::{Point, PolarPoint};
use std::{fs::File, error::Error, process, io::Write};

fn main() {

    let polar_pt_list = match load_points("./polar_pts.csv") {
        Ok(pts) => pts,
        Err(e) => {
            eprintln!("Error at loading csv contents : {}", e);
            process::exit(1)
        }
    };

    let points = to_cartesian(&polar_pt_list);
   
    match save_html(&points, "cartesian_points") {
        Ok(_) => println!("Successfully save cartisian points HTML file"),
        Err(e) => {
            eprintln!("Error at saving cartesian HTML file : {}", e);
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

fn save_html(polar_pts : &[Point], filename : &str) -> Result<(), Box<dyn Error> > {

    let mut html = String::new();
    html.push_str("<h3> Cartesian Points </h3>");
    html.push_str("<table>");
    html.push_str("<style>table, td, th {border: 1px solid #000000;\
        border-collapse : collapse;\
        padding : 6px;\
        }</style>");
    html.push_str("<table>\
        <tr>\
            <th>x</th>\
            <th>y</th>\
        </tr>\
        ");

    for i in 0..(polar_pts).len() {

        let r = polar_pts[i].x.to_string();
        let t = polar_pts[i].y.to_string();
        let row = format!("<tr> <td>{}</td>  <td>{}</td>  </tr>", r, t);
        html.push_str(&row);


    }


    html.push_str("</table>");

    let mut filename = String::from(filename);
    if ! filename.contains(".html") {
        filename.push_str(".html");
    }

    let mut html_file = File::create(filename)?;

    html_file.write_all(html.as_bytes())?;
    
    Ok(())
}