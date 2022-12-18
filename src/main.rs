use std::fs;

#[derive(Debug, Copy, Clone)]
struct Vertex {
    x: i32,
    y: i32,
    z: i32,
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

fn main() {
    let lava_coordinates = parse_input();

    let exposed_sides = calculate_exposed_sides(&lava_coordinates);

    let water = find_water_coordinates(&lava_coordinates);

    let water_neighbor_count = count_lava_with_water_as_neighbor(&water, &lava_coordinates);

    println!(
        "part_1: {}, part_2: {}",
        exposed_sides, water_neighbor_count
    );
}

fn is_connected(vec: &Vec<Vec<usize>>, i: usize, j: usize) -> bool {
    if let Some(vert) = vec.get(i) {
        return vert.contains(&j);
    }
    if let Some(vert) = vec.get(j) {
        return vert.contains(&i);
    }
    return false;
}

fn find_water_coordinates(lava: &Vec<Vertex>) -> Vec<Vertex> {
    let (min, max) = find_min_max_vert(lava);

    let mut q: Vec<Vertex> = Vec::new();
    let mut water: Vec<Vertex> = Vec::new();

    q.push(min);

    while q.len() > 0 {
        if let Some(vertex) = q.pop() {
            water.push(vertex);
            let mut neighbors = find_neighbors(&vertex, lava, &water, &min, &max, &q);
            q.append(&mut neighbors)
        };
    }

    return water;
}

fn find_neighbors(
    vertex: &Vertex,
    lava: &Vec<Vertex>,
    water: &Vec<Vertex>,
    min: &Vertex,
    max: &Vertex,
    q: &Vec<Vertex>,
) -> Vec<Vertex> {
    let mut neighbors: Vec<Vertex> = Vec::new();

    let left_of = Vertex {
        x: vertex.x - 1,
        y: vertex.y,
        z: vertex.z,
    };

    if left_of.x >= min.x
        && !water.contains(&left_of)
        && !lava.contains(&left_of)
        && !q.contains(&left_of)
    {
        neighbors.push(left_of)
    }

    let right_of = Vertex {
        x: vertex.x + 1,
        y: vertex.y,
        z: vertex.z,
    };

    if right_of.x <= max.x
        && !water.contains(&right_of)
        && !lava.contains(&right_of)
        && !q.contains(&right_of)
    {
        neighbors.push(right_of)
    }

    let in_front = Vertex {
        x: vertex.x,
        y: vertex.y - 1,
        z: vertex.z,
    };

    if in_front.y >= min.y
        && !water.contains(&in_front)
        && !lava.contains(&in_front)
        && !q.contains(&in_front)
    {
        neighbors.push(in_front)
    }

    let behind = Vertex {
        x: vertex.x,
        y: vertex.y + 1,
        z: vertex.z,
    };

    if behind.y <= max.y
        && !water.contains(&behind)
        && !lava.contains(&behind)
        && !q.contains(&behind)
    {
        neighbors.push(behind)
    }

    let above_of = Vertex {
        x: vertex.x,
        y: vertex.y,
        z: vertex.z + 1,
    };

    if above_of.z <= max.z
        && !water.contains(&above_of)
        && !lava.contains(&above_of)
        && !q.contains(&above_of)
    {
        neighbors.push(above_of)
    }

    let below_of = Vertex {
        x: vertex.x,
        y: vertex.y,
        z: vertex.z - 1,
    };

    if below_of.z >= min.z
        && !water.contains(&below_of)
        && !lava.contains(&below_of)
        && !q.contains(&below_of)
    {
        neighbors.push(below_of)
    }

    return neighbors;
}

fn find_min_max_vert(vertices: &Vec<Vertex>) -> (Vertex, Vertex) {
    let mut min: i32 = 0;
    let mut max: i32 = 0;

    for vertex in vertices {
        if vertex.x < min {
            min = vertex.x;
        }
        if vertex.y < min {
            min = vertex.y;
        }
        if vertex.z < min {
            min = vertex.z;
        }
        if vertex.x > max {
            max = vertex.x;
        }
        if vertex.y > max {
            max = vertex.y;
        }
        if vertex.z > max {
            max = vertex.z;
        }
    }

    return (
        Vertex {
            x: min - 1,
            y: min - 1,
            z: min - 1,
        },
        Vertex {
            x: max + 1,
            y: max + 1,
            z: max + 1,
        },
    );
}

fn parse_input() -> Vec<Vertex> {
    let content = fs::read_to_string("./input.txt").expect("failed to read file");

    let vertices = content.split("\n");
    let mut parsed_vertices: Vec<Vertex> = Vec::new();

    for i in vertices {
        let values: Vec<&str> = i.split(",").collect();
        let vertex = Vertex {
            x: str::parse(values[0]).expect("unable to parse vertex value"),
            y: str::parse(values[1]).expect("unable to parse vertex value"),
            z: str::parse(values[2]).expect("unable to parse vertex value"),
        };
        parsed_vertices.push(vertex)
    }

    return parsed_vertices;
}

fn calculate_exposed_sides(lava_coordinates: &Vec<Vertex>) -> i32 {
    let mut connections: Vec<Vec<usize>> = Vec::with_capacity(lava_coordinates.len());

    for _ in 0..lava_coordinates.len() {
        connections.push(Vec::new())
    }

    connections.fill(Vec::new());

    for (i, v1) in lava_coordinates.iter().enumerate() {
        for (j, v2) in lava_coordinates.iter().enumerate() {
            if i == j {
                continue;
            }

            if is_connected(&connections, i, j) {
                continue;
            }

            if v1.x == v2.x && v1.y == v2.y && (v1.z - v2.z).abs() <= 1 {
                connections[i].push(j);
                connections[j].push(i);
            }

            if v1.x == v2.x && v1.z == v2.z && (v1.y - v2.y).abs() <= 1 {
                connections[i].push(j);
                connections[j].push(i);
            }

            if v1.y == v2.y && v1.z == v2.z && (v1.x - v2.x).abs() <= 1 {
                connections[i].push(j);
                connections[j].push(i);
            }
        }
    }

    let mut exposed_sides = 0;

    for c in connections {
        if c.len() < 6 {
            exposed_sides += 6 - c.len();
        }
    }

    return exposed_sides as i32;
}

fn count_lava_with_water_as_neighbor(water: &Vec<Vertex>, lava_coordinates: &Vec<Vertex>) -> i32 {
    let mut water_neighbor_count = 0;

    for lava in lava_coordinates {
        if water.contains(&Vertex {
            x: lava.x + 1,
            y: lava.y,
            z: lava.z,
        }) {
            water_neighbor_count += 1;
        }

        if water.contains(&Vertex {
            x: lava.x - 1,
            y: lava.y,
            z: lava.z,
        }) {
            water_neighbor_count += 1;
        }

        if water.contains(&Vertex {
            x: lava.x,
            y: lava.y + 1,
            z: lava.z,
        }) {
            water_neighbor_count += 1;
        }

        if water.contains(&Vertex {
            x: lava.x,
            y: lava.y - 1,
            z: lava.z,
        }) {
            water_neighbor_count += 1;
        }

        if water.contains(&Vertex {
            x: lava.x,
            y: lava.y,
            z: lava.z + 1,
        }) {
            water_neighbor_count += 1;
        }

        if water.contains(&Vertex {
            x: lava.x,
            y: lava.y,
            z: lava.z - 1,
        }) {
            water_neighbor_count += 1;
        }
    }

    return water_neighbor_count;
}
