use std::{fs, collections::HashMap};

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d19/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Symbol { GT, LT }

#[derive(Clone, Debug, PartialEq, Eq)]
struct Condition {
    category: char,
    symbol: Symbol,
    value: usize
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rule {
    condition: Option<Condition>,
    workflow_name: String
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>
}

pub fn p1() {
    let puzzle_input: String = read_puzzle_input();
    let mut parse_iter = puzzle_input.split("\n\n");
    let workflows = parse_iter.next().unwrap();
    let parts = parse_iter.next().unwrap();
    let workflows = workflows.split('\n').map(|line| parse_workflow(line)).collect::<Vec<Workflow>>();
    let workflows: HashMap<String, Workflow> = workflows.into_iter()
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect();
    let parts = parts.split('\n').map(|line| parse_part(line)).collect::<Vec<Part>>();
    let accepted_parts = parts.iter().filter(|part| check_part(&workflows, part));
    let result = accepted_parts.clone().fold(0, |acc, part| acc + part.x + part.m + part.a + part.s);
    println!("found {} accepted paths", accepted_parts.collect::<Vec<_>>().len());
    //19114 too low
    println!("sum of xmas for all accepted parts: {result}");
}

fn check_part(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    let mut next_workflow_name = "in".to_string();
    while next_workflow_name != "R" && next_workflow_name != "A" {
        //println!("{next_workflow_name}");
        next_workflow_name = find_next_workflow(workflows, part, next_workflow_name);
    }
    return next_workflow_name == "A";
}

fn find_next_workflow(workflows: &HashMap<String, Workflow>, part: &Part, workflow_name: String) -> String {
    let workflow = workflows.get(&workflow_name).unwrap();
    for rule in workflow.rules.iter() {
        match rule.condition.clone() {
            Some(condition) => {
                let part_value = match condition.category {
                    'x' => {part.x},
                    'm' => {part.m},
                    'a' => {part.a},
                    's' => {part.s},
                    _ => {panic!("invalid condition")},
                };
                match condition.symbol {
                    Symbol::GT => {if part_value > condition.value {return rule.workflow_name.clone()}},
                    Symbol::LT => {if part_value < condition.value {return rule.workflow_name.clone()}},
                }
            },
            None => {return rule.workflow_name.clone() },
        }
    }
    panic!("did not find any next workflow for current rule");
}

fn parse_part(line: &str) -> Part {
    let parts: Vec<&str> = line.trim_matches(|c| c == '{' || c == '}').split(',').collect();
    let mut x = 0;
    let mut m = 0;
    let mut a = 0;
    let mut s = 0;

    for part in parts {
        let key_value: Vec<&str> = part.split('=').collect();
        match key_value[0] {
            "x" => x = key_value[1].parse::<usize>().unwrap(),
            "m" => m = key_value[1].parse::<usize>().unwrap(),
            "a" => a = key_value[1].parse::<usize>().unwrap(),
            "s" => s = key_value[1].parse::<usize>().unwrap(),
            _ => {},
        }
    }

    Part { x, m, a, s }
}

fn parse_workflow(line: &str) -> Workflow {
    let name_end = line.find('{').unwrap();
    let name = line[..name_end].to_string();
    let rules_chunk = &line[name_end+1..line.len()-1];
    let mut rules = Vec::new();
    let mut rule_strings: Vec<&str> = rules_chunk.split(',').collect();

    let fallback_workflow_name = rule_strings.pop().unwrap();
    for rule_string in rule_strings {
        let parts: Vec<&str> = rule_string.split(':').collect();
        let workflow_name = parts.get(1).unwrap().to_string();
        
        let condition_string = parts.get(0).unwrap();
        let condition_string = condition_string.chars().collect::<Vec<char>>(); 
        
        let category = *condition_string.get(0).unwrap();
        let symbol = match condition_string.get(1).unwrap() {
            '>' => {Symbol::GT},
            '<' => {Symbol::LT},
            _ => {panic!("tried to parse char '{}' to gt or lt", condition_string.get(1).unwrap())},
        };
        
        let value: usize = condition_string[2..].iter().collect::<String>().parse().unwrap();

        let condition = Condition {
            category,
            symbol,
            value
        };
        rules.push(Rule {
            condition: Some(condition),
            workflow_name
        });
    }

    rules.push(Rule {
        condition: None,
        workflow_name: fallback_workflow_name.to_string()
    });

    Workflow {
        name,
        rules
    }
}

// fn split_following_workflows(workflows: &HashMap<String, Workflow>, conditions: Vec<Option<Condition>>, workflow_name: String) -> Vec<Option<Condition>> {
//     let workflow = workflows.get(&workflow_name).unwrap();
//     for rule in workflow.rules.iter() {
//         match rule.condition.clone() {
//             Some(condition) => {
//                 match condition.symbol {
//                     Symbol::GT => {
//                         // TODO check if the current conditions in this subtree would allow for this condition to result in any valid parts
//                         if part_value > condition.value {return rule.workflow_name.clone()}
//                     },
//                     Symbol::LT => {
//                         // TODO check if the current conditions in this subtree would allow for this condition to result in any valid parts
//                         if part_value < condition.value {return rule.workflow_name.clone()}
//                     },
//                 }
//             },
//             None => {
//                 return rule.workflow_name.clone()
//             },
//         }
//     }
//     panic!("did not find any next workflow for current rule");
// }

pub fn p2() {
    let puzzle_input: String = read_puzzle_input();
    let mut parse_iter = puzzle_input.split("\n\n");
    let workflows = parse_iter.next().unwrap();
    let workflows = workflows.split('\n').map(|line| parse_workflow(line)).collect::<Vec<Workflow>>();
    let workflows: HashMap<String, Workflow> = workflows.into_iter()
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect();

    
    // println!("{parts:?}");
    // let parts = 
    // apply_workflows_to_parts(workflows, parts)

    // println!("found {} accepted paths", accepted_parts.collect::<Vec<_>>().len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_workflow() {
        let line = "px{a<2006:qkq,m>2090:A,rfg}";
        let result = parse_workflow(line);
        
        let expected = Workflow {
            name: "px".to_string(),
            rules: vec![
                Rule { 
                    condition: Some(Condition {
                        category: 'a',
                        symbol: Symbol::LT,
                        value: 2006,
                    }),
                    workflow_name: "qkq".to_string(),
                },
                Rule { 
                    condition: Some(Condition {
                        category: 'm',
                        symbol: Symbol::GT,
                        value: 2090,
                    }),
                    workflow_name: "A".to_string(),
                },
                Rule {
                    condition: None,
                    workflow_name: "rfg".to_string(),
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_part() {
        let input = "{x=787,m=2655,a=1222,s=2876}";
        let expected = Part {
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876,
        };
        assert_eq!(parse_part(input), expected);
    }
}