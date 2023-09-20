use hw8::point::{Point, PolarPoint};


fn main() {

    let pt_list = [
        PolarPoint { r: 14.14, t: 0.79 }, 
        PolarPoint { r: 10.0, t: 3.14 }
    ];

    let point = to_cartesian(&pt_list);

    println!("{:?}", point);

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