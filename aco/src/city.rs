use std::fs;

#[derive(Debug, Copy, Clone)]
pub struct City {
    pub name: i32,
    pub x: i32,
    pub y: i32,
}

impl PartialEq for City {
    // Used to compare 2 cities
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}


fn parse_coordinates(v: Vec<&str>) -> Vec<(i32, i32)> {
    // From ChatGPT
    v.into_iter().map(|s| {
        let mut coordinates = s.split(',');
        let x = coordinates.next().unwrap().trim().parse::<i32>().unwrap();
        let y = coordinates.next().unwrap().trim().parse::<i32>().unwrap();
        (x, y)
    }).collect()
}


pub fn cities_from_coordinates(file_path:&str) -> Vec<City> {
    // Takes a file path to coordinates.txt and returns a vec of Cities
    let coords_str:String = fs::read_to_string(file_path).expect("Cannot read file");
    let lines: Vec<_> = coords_str.split("\r\n").collect();
    let int_coords: Vec<(i32, i32)> = parse_coordinates(lines);
    let mut city_vec:Vec<City> = Vec::with_capacity(int_coords.len());
    for (i, (c1,c2)) in (1..=int_coords.len()).zip(int_coords) {
        city_vec.push(City{name:(i as i32), x:c1, y:c2});
    }
    return city_vec;
}


pub fn get_shortest_path(file_path:&str, city_vec:&Vec<City>) -> Vec<City> {
    // Takes a file with city numbers and returns a vector of Cities
    let short_path_nums :String = fs::read_to_string(file_path).expect("Cannot read file");    
    let lines: Vec<&str> = short_path_nums.split(" ").collect();
    let vec2: Vec<i32> = lines.into_iter().map(|s| s.trim().parse::<i32>().unwrap()).collect();
    let mut short_path: Vec<City> = Vec::with_capacity(vec2.len());
    for city_num in vec2{
        'inner_loop: for city in city_vec.iter().clone(){
            if city_num == city.name {
                short_path.push(City{name:city.name, x:city.x, y:city.y});
                break 'inner_loop
            }
        }
    }
    return short_path;
}  