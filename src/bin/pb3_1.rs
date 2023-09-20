use hw8::point::{Point, PolarPoint};
use std::{fs::File, error::Error, process, io::Write};

fn main() {

    let pt_list = match load_points("./pt_list.csv") {
        Ok(pts) => pts,
        Err(e) => {
            eprintln!("Error at loading csv contents : {}", e);
            process::exit(1)
        }
    };

    let polar = to_polar(&pt_list);


    match save_html(&polar, "polar_points") {
        Ok(_) => println!("Successfully save polar points HTML file"),
        Err(e) => {
            eprintln!("Error at saving polar HTML file : {}", e);
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

fn save_html(polar_pts : &[PolarPoint], filename : &str) -> Result<(), Box<dyn Error> > {

    let mut html = String::new();
    html.push_str("<h3> Polar Points </h3>");
    html.push_str("<table>");
    html.push_str("<style>table, td, th {border: 1px solid #000000;\
        border-collapse : collapse;\
        padding : 6px;\
        }</style>");
    html.push_str("<table>\
        <tr>\
            <th>r</th>\
            <th>t</th>\
        </tr>\
        ");

    for i in 0..(polar_pts).len() {

        let r = polar_pts[i].r.to_string();
        let t = polar_pts[i].t.to_string();
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