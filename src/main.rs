use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap, time::SystemTime};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut valves: HashMap<String, Valve> = HashMap::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut l = line.split(";");
            let mut l1 = l.next().unwrap().split_whitespace();
            let l2: Vec<String> = l.next().unwrap()[24..].split(',').map(|a| a.trim().to_owned()).collect();
            
            let name = l1.nth(1).unwrap();
            let flow_rate = l1.nth(2).unwrap().split("=").nth(1).unwrap().parse::<usize>().unwrap();

            let valve = Valve {
                flow_rate,
                tunnels_to: l2
            };

            valves.insert(name.to_owned(), valve);
        }
    }

    println!("{:?}", valves);

    let mut cache = HashMap::new();

    let start = SystemTime::now();
    let res = calculate(&valves, &"AA".to_owned(), 30, &vec![], &mut cache);
    let elapsed = start.elapsed().unwrap().as_millis();

    println!("{} took {}ms", res, elapsed)
}

fn calculate_pressure_released(valves: &HashMap<String, Valve>, opened_valves: &Vec<String>) -> usize {
    let mut res = 0;

    for valve in opened_valves {
        let valve = valves.get(valve).expect("How");
        res += valve.flow_rate;
    }

    res
}

fn calculate(valves: &HashMap<String, Valve>, current_valve: &String, time_left: usize, opened_valves: &Vec<String>, cache: &mut HashMap<State, usize>) -> usize {
    let mut result = 0;

    let mut state = State {
        current_valve: current_valve.to_owned(),
        opened_valves: opened_valves.clone(),
        time_left
    };

    if cache.contains_key(&state) {
        return *cache.get(&state).unwrap();
    }
    
    if time_left > 0 {
        // println!("{}", current_valve);
        let valve = valves.get(current_valve).expect("How");
        if valve.flow_rate != 0 && !opened_valves.contains(current_valve) {
            result += calculate_pressure_released(valves, &state.opened_valves);
            state.opened_valves.push(current_valve.to_owned());

            if time_left > 1 {
                result += calculate_pressure_released(valves, &state.opened_valves);
                result += valves.get(current_valve).unwrap().tunnels_to.iter().map(|a| calculate(valves, a, time_left - 2, &state.opened_valves, cache)).max().unwrap_or(0);
            }
        }
        else {
            result += calculate_pressure_released(valves, &state.opened_valves);
            result += valves.get(current_valve).unwrap().tunnels_to.iter().map(|a| calculate(valves, a, time_left - 1, &state.opened_valves, cache)).max().unwrap_or(0);
        }
        
        // result += valves.get(current_valve).unwrap().tunnels_to.iter().map(|a| calculate(valves, a, time_left - 1, cache)).max().unwrap_or(0);
    }

    cache.entry(state).or_insert(result);

    result
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct State {
    current_valve: String,
    opened_valves: Vec<String>,
    time_left: usize
}

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    tunnels_to: Vec<String>
}

impl Valve {
    pub fn pressure_released(&self, mins_left: usize) -> usize {
        self.flow_rate * mins_left
    }
}