use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use rayon::prelude::*;
use dashmap::DashMap;

type Point = (f64,f64,f64);
type Cell  = (i16,i16,i16);


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let points: Vec<Point> = parse_file(file_path).unwrap();

    let total_count: usize;
    let maximum_distance: f64 = 0.05;
    let maximum_distance_sq: f64 = maximum_distance.powf(2.0);
    
    let grid_map: DashMap<Cell,Vec<Point>> = 
        DashMap::with_capacity_and_shard_amount(100_000,512); //roughly hand adjusted for positions_large.xyz
    
    points.into_par_iter().for_each(|point| { 
        let cell = sort_into_cell(point, maximum_distance); 
        grid_map.entry(cell).or_insert_with(Vec::new).push(point);
    });
    
    total_count = grid_map.par_iter().map(|entry| {
        count_pairs(entry.key(), &grid_map, maximum_distance_sq)
        }).sum();   
    

    println!("number of pairs: {}", total_count); //expected result: 1436965 for positions_large.xyz
    Ok(())
}


fn parse_file(file_path: String) -> Result<Vec<Point>, io::Error> {
    let mut points: Vec<Point> = Vec::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let nums: Vec<f64> = line   
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        points.push((nums[0],nums[1],nums[2]));
    }
    Ok(points)
}


fn sort_into_cell(point:Point, maximum_distance: f64) -> Cell {
    (
    (point.0/maximum_distance).floor() as i16, 
    (point.1/maximum_distance).floor() as i16, 
    (point.2/maximum_distance).floor() as i16)
}


fn count_pairs(
    cell: &Cell, 
    grid_map: &DashMap<Cell,Vec<Point>>,  
    maximum_distance_sq: f64) -> usize {

    let mut pair_count: usize = 0;

    let domestic_points: &Vec<Point> = &grid_map.get(cell).unwrap();    

    let shard_key = ((cell.0 & 1) | ((cell.1 & 1) << 1) | ((cell.2 & 1) << 2)) as u8;

    if shard_key != 7 {
        let surrounding_cells: Vec<Cell> = get_surrounding_cells(cell, shard_key);

        for sur_cell in surrounding_cells {
            if let Some(sur_points) = grid_map.get(&sur_cell)  {
                for &sur_p in sur_points.iter() {
                    for &dom_p in domestic_points.iter() {
                        if close_enough(sur_p, dom_p, maximum_distance_sq) {
                            pair_count += 1;
                        }
                    }
                }
            }
        }
    }
    
    let amount_dom_p = domestic_points.len();
    
    for i in 0..amount_dom_p-1 {
        for j in i+1..amount_dom_p {
            if close_enough(domestic_points[i], domestic_points[j], maximum_distance_sq) {
                pair_count += 1;
            }
        }
    }
    
    pair_count
}


fn close_enough(p1: Point, p2: Point, maximum_distance_sq: f64) -> bool {    
    (p1.0-p2.0).powf(2.0)+
    (p1.1-p2.1).powf(2.0)+
    (p1.2-p2.2).powf(2.0) < maximum_distance_sq
}


fn get_surrounding_cells(center: &Cell, shard_key: u8) -> Vec<Cell> {
    let (x,y,z) = *center;
    match shard_key { //shard_key can only equal 0,1,2,3,4,5,6
        0 => vec![
            (x-1,y+1,z+1),(x  ,y+1,z+1),(x+1,y+1,z+1),
            (x-1,y  ,z+1),(x  ,y  ,z+1),(x+1,y  ,z+1),
            (x-1,y-1,z+1),(x  ,y-1,z+1),(x+1,y-1,z+1),
    
            (x-1,y+1,z  ),(x  ,y+1,z  ),(x+1,y+1,z  ),
            (x-1,y  ,z  )              ,(x+1,y  ,z  ),
            (x-1,y-1,z  ),(x  ,y-1,z  ),(x+1,y-1,z  ),
            
            (x-1,y+1,z-1),(x  ,y+1,z-1),(x+1,y+1,z-1),
            (x-1,y  ,z-1),(x  ,y  ,z-1),(x+1,y  ,z-1),
            (x-1,y-1,z-1),(x  ,y-1,z-1),(x+1,y-1,z-1),],

        1 => vec![
            (x-1,y+1,z+1),(x  ,y+1,z+1),(x+1,y+1,z+1),
            (x-1,y  ,z+1),(x  ,y  ,z+1),(x+1,y  ,z+1),
            (x-1,y-1,z+1),(x  ,y-1,z+1),(x+1,y-1,z+1),
    
            (x-1,y+1,z  ),(x  ,y+1,z  ),(x+1,y+1,z  ),
                     
            (x-1,y-1,z  ),(x  ,y-1  ,z),(x+1,y-1,z  ),
            
            (x-1,y+1,z-1),(x  ,y+1,z-1),(x+1,y+1,z-1),
            (x-1,y  ,z-1),(x  ,y  ,z-1),(x+1,y  ,z-1),
            (x-1,y-1,z-1),(x  ,y-1,z-1),(x+1,y-1,z-1),],

        2 =>  vec![
            (x-1,y+1,z+1),(x  ,y+1,z+1),(x+1,y+1,z+1),
            (x-1,y  ,z+1),(x  ,y  ,z+1),(x+1,y  ,z+1),
            (x-1,y-1,z+1),(x  ,y-1,z+1),(x+1,y-1,z+1),
    
            
            (x-1,y  ,z  )              ,(x+1,y  ,z  ),
            
            
            (x-1,y+1,z-1),(x  ,y+1,z-1),(x+1,y+1,z-1),
            (x-1,y  ,z-1),(x  ,y  ,z-1),(x+1,y  ,z-1),
            (x-1,y-1,z-1),(x  ,y-1,z-1),(x+1,y-1,z-1),],

        3 => vec![
            (x-1,y+1,z+1),(x  ,y+1,z+1),(x+1,y+1,z+1),
            (x-1,y  ,z+1),(x  ,y  ,z+1),(x+1,y  ,z+1),
            (x-1,y-1,z+1),(x  ,y-1,z+1),(x+1,y-1,z+1),
    
            
            
            
            
            (x-1,y+1,z-1),(x  ,y+1,z-1),(x+1,y+1,z-1),
            (x-1,y  ,z-1),(x  ,y  ,z-1),(x+1,y  ,z-1),
            (x-1,y-1,z-1),(x  ,y-1,z-1),(x+1,y-1,z-1),],

        4 => vec![

            (x-1,y+1,z  ),(x  ,y+1,z  ),(x+1,y+1,z  ),
            (x-1,y  ,z  )              ,(x+1,y  ,z  ),
            (x-1,y-1,z  ),(x  ,y-1,z  ),(x+1,y-1,z  ),
        
        ],
        5 => vec![

            (x-1,y+1,z  ),(x  ,y+1,z  ),(x+1,y+1,z  ),
                    
            (x-1,y-1,z  ),(x  ,y-1,z  ),(x+1,y-1,z  ),

        ],
        _ => vec![ 

            (x-1,y  ,z  )             ,(x+1,y  ,z  )

        ]
    }    
}
