use hw8::point::{Point, PolarPoint};


fn main() {

    let pt_list = [
        Point {x: 10.0, y: 10.0},
        Point {x: -10.0, y: 0.0} 
    ];

    let polar = to_polar(&pt_list);

    println!("{:?}", polar);

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