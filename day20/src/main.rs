use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum SignalType {
    High,
    Low,
}

type Signal<'a> = (&'a str, &'a str, SignalType);

type Receivers<'a> = Vec<&'a str>;
type History<'a> = HashMap<&'a str, SignalType>;

#[derive(PartialEq, Clone)]
enum Module<'a> {
    FlipFlop(Receivers<'a>, bool),
    Conjunction(Receivers<'a>, History<'a>),
    Broadcaster(Receivers<'a>),
}
type Modules<'a> = HashMap<&'a str, Module<'a>>;

const RX_TRIGGER_MODULE_NAME: &str = "nc";

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let modules = parse(&file);

    let connected_modules = find_rx_connected_modules(&modules);

    let mut cycles: Vec<usize> = Vec::new();

    for connected_module in connected_modules {
        let cycle = find_cycle_and_high_pulse_for(connected_module, modules.clone());
        cycles.push(cycle)
    }

    let mut cycles_iterator = cycles.into_iter();

    let mut result = cycles_iterator.next().unwrap();

    for cycle in cycles_iterator {
        result = least_common_multiple(cycle, result)
    }

    println!("{}", result);
}

fn greatest_common_divisor(a: usize, b: usize) -> usize {
    if b == 0 { return a; }

    greatest_common_divisor(b, a % b)
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    a * b / greatest_common_divisor(a, b)
}

fn find_rx_connected_modules<'a>(modules: &'a Modules) -> Vec<&'a str> {
    let mut connected: Vec<&str> = Vec::new();

    for (name, module) in modules.iter() {
        let receivers = match module {
            Module::Broadcaster(receivers) => receivers,
            Module::FlipFlop(receivers, _) => receivers,
            Module::Conjunction(receivers, _) => receivers
        };

        if receivers.contains(&RX_TRIGGER_MODULE_NAME) { connected.push(*name) }
    }

    connected
}

fn find_cycle_and_high_pulse_for(module_name: &str, mut modules: Modules) -> usize {
    let mut i = 0;

    loop {
        i += 1;

        let recorded_high_pulse = activate(&mut modules, Some(module_name));
        if recorded_high_pulse { break i; }
    }
}

fn activate(modules: &mut Modules, record_high_pulse_from: Option<&str>) -> bool {
    let mut queue: Vec<Signal> = vec![("broadcaster", "button", SignalType::Low)];
    let mut next_queue: Vec<Signal> = Vec::new();
    let mut recorded_high_pulse = false;

    while !queue.is_empty() {
        let signal = queue.remove(0);

        if record_high_pulse_from.is_some() && signal.2 == SignalType::High && record_high_pulse_from.unwrap() == signal.1 {
            recorded_high_pulse = true;
        }

        if let Some(module) = modules.get_mut(signal.0) {
            match module {
                Module::Broadcaster(receivers) => {
                    add_signals_to_queue(receivers, signal.0, signal.2, &mut next_queue);
                }
                Module::FlipFlop(receivers, state) if signal.2 == SignalType::Low => {
                    *state = !(*state);
                    let signal_type = match state {
                        true => SignalType::High,
                        false => SignalType::Low,
                    };
                    add_signals_to_queue(receivers, signal.0, signal_type, &mut next_queue);
                }
                Module::Conjunction(receivers, history) => {
                    history.insert(signal.1, signal.2);

                    let signal_type = match history.values().all(|signal_type| *signal_type == SignalType::High) {
                        true => SignalType::Low,
                        false => SignalType::High,
                    };

                    add_signals_to_queue(receivers, signal.0, signal_type, &mut next_queue);
                }
                _ => {}
            };
        }

        if queue.is_empty() {
            queue = next_queue;
            next_queue = Vec::new();
        }
    }

    recorded_high_pulse
}

fn add_signals_to_queue<'a>(receivers: &Receivers<'a>, origin: &'a str, signal_type: SignalType, queue: &mut Vec<Signal<'a>>) {
    for receiver in receivers {
        queue.push((receiver, origin, signal_type));
    }
}

fn parse(file: &str) -> Modules {
    let mut modules_vec: Vec<(&str, Vec<&str>)> = Vec::new();

    for line in file.lines() {
        let (name_and_type, receivers) = line.split_once(" -> ").unwrap();

        let receivers: Vec<_> = receivers.split(", ").collect();

        modules_vec.push((name_and_type, receivers))
    }

    let mut modules: Modules = HashMap::new();

    for i in 0..modules_vec.len() {
        match &modules_vec[i].0[0..1] {
            "b" => modules.insert(modules_vec[i].0, Module::Broadcaster(modules_vec[i].1.clone())),
            "%" => modules.insert(&modules_vec[i].0[1..], Module::FlipFlop(modules_vec[i].1.clone(), false)),
            _ => {
                let mut history: History = HashMap::new();

                for (module_name, receivers) in modules_vec.iter() {
                    if receivers.contains(&&modules_vec[i].0[1..]) {
                        history.insert(if module_name.starts_with("b") { module_name } else { &module_name[1..] }, SignalType::Low);
                    }
                }

                modules.insert(&modules_vec[i].0[1..], Module::Conjunction(modules_vec[i].1.clone(), history))
            }
        };
    }

    modules
}
