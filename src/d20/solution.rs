use std::process::exit;
use std::sync::Mutex;
use std::{fs, collections::HashMap};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use lazy_static::lazy_static;

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d20/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

fn transform_vec_to_map(modules: Vec<Module>) -> HashMap<String, Box<Module>> {
    let map: HashMap<String, Box<Module>> = modules
        .into_iter()
        .map(|module| (module.name.clone(), Box::new(module)))
        .collect();
    map
}


lazy_static! {
    static ref HIGH_PULSE_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref LOW_PULSE_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref BUTTON_PRESSES: AtomicUsize = AtomicUsize::new(0);
    static ref INPUT_STATES_OF_RS: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

pub fn p1() {
    let puzzle_input: String = read_puzzle_input();
    let modules: Vec<Module> = puzzle_input.split("\n").map(|line| {
        parse_module(line.to_string())
    }).collect::<Vec<Module>>();
    let mut modules: HashMap<String, Box<Module>> = transform_vec_to_map(modules);
    modules.clone().values().for_each(|module| {
        module.propagate_to.iter().for_each(|name| {
            let module_ref = modules.get_mut(name);
            match module_ref {
                Some(module_ref) => {
                    module_ref.inputs.push((false, module.name.clone()));
                },
                None => {},
            } 
        })
    });
    modules.values().for_each(|module| println!("{module:?}"));
    (1..=1000).into_iter().for_each(|_x| push_button(&mut modules));
    println!("\nresult:\nlow pulse counter: {}\nhigh pulse counter: {}", LOW_PULSE_COUNTER.load(Ordering::SeqCst), HIGH_PULSE_COUNTER.load(Ordering::SeqCst));
    println!("#low x #high = {}", LOW_PULSE_COUNTER.load(Ordering::SeqCst) * HIGH_PULSE_COUNTER.load(Ordering::SeqCst))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn lcm_of_list(numbers: &[usize]) -> usize {
    numbers.iter().fold(1, |a, &b| lcm(a, b))
}

fn push_button(modules: &mut HashMap<String, Box<Module>>) {
    //println!("\n----\npush button\n----\n");
    let mut signal_propagation_queue: VecDeque<(String, String, (bool, Vec<String>))> = VecDeque::new();
    let next_signal = ("button".to_string(), "broadcaster".to_string(), handle_module(false, "".to_string(), &mut modules.get_mut("broadcaster").unwrap()));
    let (from, to, (new_pulse, destinations)) = next_signal.clone();
    //println!("sending low from {from} to {to} resulting in {} to {destinations:?}", if new_pulse {"high"} else {"low"});
    signal_propagation_queue.push_back(next_signal);
    while let Some((_from, to, (pulse, propagate_to))) = signal_propagation_queue.pop_front() {
        for dest in propagate_to {
            if dest == "rx" {
                let rs_module = modules.get(&to).unwrap();
                if rs_module.inputs.iter().any(|inp| inp.0) {
                    let cur_presses = BUTTON_PRESSES.load(Ordering::SeqCst);
                    println!("button press: {cur_presses} - current state of source modules inputs: {:?}", rs_module.inputs);
                    let mut map = INPUT_STATES_OF_RS.lock().unwrap();
                    let inp_with_high_pulse = rs_module.inputs.iter().find(|elem| elem.0).unwrap();
                    let first_appearance_of_cur_high_pulse = map.get(&inp_with_high_pulse.1);
                    match first_appearance_of_cur_high_pulse {
                        Some(_) => {},
                        None => {
                            map.insert(inp_with_high_pulse.clone().1, cur_presses);
                        },
                    }
                    let all_found = rs_module.inputs.iter().all(|inp| map.get(&inp.1).is_some());
                    if all_found {
                        println!("{map:?}");
                        let lcm = lcm_of_list(&map.values().map(|x| *x+1).collect::<Vec<usize>>());
                        println!("result = {lcm}");
                        exit(0);
                    }
                    // TODO if all values exists in map -> exit;    
                }
                //println!("sending {} signal from {} to {}", if pulse {"high"} else {"low"}, to, dest);
            }
            let dest_module = &mut modules.get_mut(&dest);
            match dest_module {
                Some(dest_module) => {
                    let next_signal = (to.clone(), dest.clone(), handle_module(pulse, to.clone(), dest_module));
                    let (from, to, (new_pulse, destinations))  = next_signal.clone();
                    //println!("sending {} from {from} to {to} resulting in {} to {destinations:?}", if pulse {"high"} else {"low"}, if new_pulse {"high"} else {"low"});
                    signal_propagation_queue.push_back(next_signal)
                },
                None => {
                    if pulse {
                        HIGH_PULSE_COUNTER.fetch_add(1, Ordering::SeqCst);
                    }
                    else {
                        LOW_PULSE_COUNTER.fetch_add(1, Ordering::SeqCst);
                    }
                },
            }
        }
    }
}

fn handle_module(pulse: bool, from: String, module: &mut Module) -> (bool, Vec<String>) {
    if pulse {
        HIGH_PULSE_COUNTER.fetch_add(1, Ordering::SeqCst);
    }
    else {
        LOW_PULSE_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    match module.type_ {
        '%' => {handle_flip_flop(pulse, from, module)},
        '&' => {handle_conjunction(pulse, from, module)},
        ' ' => {handle_broadcast(pulse, from, module)},
        _ => {panic!("unexpected char denoting type")},
    }
}

fn handle_flip_flop(pulse: bool, _from: String, module: &mut Module) -> (bool, Vec<String>) {
    if pulse == true { return (true, vec![]) }
    module.state = !module.state;
    (module.state, module.propagate_to.clone())
}

fn handle_conjunction(pulse: bool, from: String, module: &mut Module) -> (bool, Vec<String>) {
    let index = module.inputs.iter().position(|(_state, inp)| *inp == from).expect(&format!("did not find input module with name '{from}' in module {module:?}"));
    module.inputs.get_mut(index).unwrap().0 = pulse;
    let all_inputs_true = module.inputs.iter().all(|(state, inp)| *state);
    let propagation_pulse = if all_inputs_true { false } else { true };
    (propagation_pulse, module.propagate_to.clone())
}

fn handle_broadcast(pulse: bool, _from: String, module: &mut Module) -> (bool, Vec<String>) {
    (pulse, module.propagate_to.clone())
}

#[derive(Debug, Clone)]
struct Module {
    type_: char,
    name: String,
    propagate_to: Vec<String>,
    inputs: Vec<(bool, String)>,
    state: bool,
}

fn parse_module(line: String) -> Module {
    let parts = line.split(" -> ").collect::<Vec<_>>();
    let first_symbol = parts.get(0).unwrap().chars().next().unwrap();
    let type_ = if first_symbol == 'b' {' '} else {first_symbol};
    let name = parts.get(0).unwrap().replace(type_, "").to_string();
    let propagate_to = parts.get(1).unwrap().split(", ").map(|x| x.to_string()).collect::<Vec<String>>(); 
    Module {
        type_, name, propagate_to, inputs: vec![], state: false
    }
}

pub fn p2() {
    let puzzle_input: String = read_puzzle_input();
    let modules: Vec<Module> = puzzle_input.split("\n").map(|line| {
        parse_module(line.to_string())
    }).collect::<Vec<Module>>();
    let mut modules: HashMap<String, Box<Module>> = transform_vec_to_map(modules);
    modules.clone().values().for_each(|module| {
        module.propagate_to.iter().for_each(|name| {
            let module_ref = modules.get_mut(name);
            match module_ref {
                Some(module_ref) => {
                    module_ref.inputs.push((false, module.name.clone()));
                },
                None => {},
            } 
        })
    });
    modules.values().for_each(|module| println!("{module:?}"));
    loop {
        push_button(&mut modules);
        BUTTON_PRESSES.fetch_add(1, Ordering::SeqCst);
    };
    // 225386464601017
}