use rand::seq::SliceRandom;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Production {
    pub symbol: String,
    value: Vec<String>,
}

impl Production {
    fn is_null(&self) -> bool {
        self.value.len() == 1 && self.value[0] == "#"
    }

    fn remove_null_production(&self, nullable_name: &str) -> Vec<Production> {
        // Case where there is only 1 production
        let mut new_prods: Vec<Production> = vec![];

        // Add all productions with nullable prod removed
        permute_without(self.value.clone(), &nullable_name)
            .iter()
            .filter(|vec| !vec.is_empty())
            .map(|vec| Production {
                symbol: self.symbol.clone(),
                value: vec.clone(),
            })
            .for_each(|p| {
                if !new_prods.contains(&p) {
                    new_prods.push(p);
                }
            });

        new_prods.into_iter().collect()
    }
}

fn permute_without(strings: Vec<String>, to_remove: &str) -> Vec<Vec<String>> {
    let mut perms = vec![];

    if !strings.iter().any(|c| c == to_remove) {
        return vec![strings];
    }

    strings.iter().enumerate().for_each(|(index, c)| {
        if c == to_remove {
            // Remvoe this char from the vec
            let mut new_chars = strings.clone();

            // Add the case where it is not remove
            if !perms.contains(&new_chars) {
                perms.push(new_chars.clone());
            }

            // Add the case where it is removed
            new_chars.remove(index);
            if !perms.contains(&new_chars) {
                perms.push(new_chars.clone());
            }

            // Recursively remove the rest
            let permuted = permute_without(new_chars, to_remove);

            // Add each case to the perms
            permuted.iter().for_each(|x| {
                if !perms.contains(x) {
                    perms.push(x.clone())
                }
            });
        }
    });

    perms
}

#[derive(Debug, Clone)]
pub struct CFG {
    starting_variable: String,
    productions: Vec<Production>,
}

impl CFG {
    pub fn new(input: &str) -> CFG {
        let mut cfg = read_cfg(input);
        cfg.to_cnf();

        cfg
    }

    fn find_index(&self, symbol: &String) -> usize {
        for (i, prod) in self.productions.iter().enumerate() {
            if prod.value.len() == 2 {
                if symbol == &prod.symbol {
                    return i;
                }
            }
        }

        for (i, prod) in self.productions.iter().enumerate() {
            if symbol == &prod.symbol {
                return i;
            }
        }

        panic!("Expected to find index for variable: {}", symbol);
    }

    fn is_variable(&self, name: &str) -> bool {
        self.productions
            .iter()
            .any(|p| p.symbol == name.to_string())
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("Starting Variable: {}", self.starting_variable);
        for prod in self.productions.iter() {
            print!("{} -> ", prod.symbol);
            println!("{}", prod.value.join(" ").as_str());
        }
    }

    // Tests if the string exists using the CYK algorithm
    pub fn test(&self, input: &str) -> bool {
        let n = input.len();
        if n == 0 {
            return false;
        }

        // The number of rules
        let r = self.productions.len();

        let mut table = vec![vec![vec![false; r]; n]; n];
        // let backpointing = vec![vec![vec![vec![]]; n]; n];

        for s in 0..n {
            let a = input.chars().nth(s).unwrap().to_string();
            // Find a R_v s.t. R_v -> a_s
            for v in 0..r {
                let prod = &self.productions[v];
                if prod.value.len() == 1 {
                    if prod.value[0] == a {
                        let index = self.find_index(&prod.symbol);
                        table[s][0][index] = true;
                    }
                }
            }
        }

        // Length of span
        for l in 1..n {
            // Start of span
            for s in 0..(n - l) {
                // Partition of span
                for p in 0..l {
                    for prod in &self.productions {
                        if prod.value.len() == 2 {
                            let a = self.find_index(&prod.symbol);
                            let b = self.find_index(&prod.value[0]);
                            let c = self.find_index(&prod.value[1]);

                            if table[s][p][b] && table[s + p + 1][l - p - 1][c] {
                                table[s][l][a] = true;
                            }
                        }
                    }
                }
            }
        }

        let start_symbol_index = self.find_index(&self.starting_variable);
        return table[0][n - 1][start_symbol_index];
    }

    fn to_cnf(&mut self) {
        // Step 1: Remove the start symbol from the RHS
        self.remove_start_symbol();

        // Step 2: Remove null, unit, and useless productions
        // Step 2a remove null productions
        self.remove_null_productions();

        // Step 2b Remove unit productions
        self.remove_unit_productions();

        // Step 2c Remove useless productions
        self.remove_useless_productions();

        // Step 3: Remove terminals from RHS if it exists with a variable
        self.isolate_terminals();

        // Step 4: Remove variables with more than 2 variables
        self.remove_long_productions();
    }

    fn remove_start_symbol(&mut self) {
        let contains_start = self
            .productions
            .iter()
            .any(|prod| prod.value.contains(&self.starting_variable));

        // TODO We need to smartly generate variables later
        if contains_start {
            let old_starting_var = self.starting_variable.clone();

            self.starting_variable = "S`".to_string();
            self.productions.push(Production {
                symbol: self.starting_variable.clone(),
                value: vec![old_starting_var],
            });
        }
    }

    fn remove_null_productions(&mut self) {
        let nullable_names: Vec<_> = self
            .productions
            .iter()
            .filter(|p| p.is_null())
            .map(|p| p.symbol.clone())
            .collect();

        for nullable_name in nullable_names {
            let new_prods: Vec<Production> = self
                .productions
                .iter()
                .filter(|prod| prod.value.contains(&nullable_name))
                .flat_map(|prod| prod.remove_null_production(&nullable_name))
                .collect();

            new_prods.iter().for_each(|p| {
                if !self.productions.contains(p) {
                    self.productions.push(p.clone())
                }
            });
        }

        self.productions.retain(|p| !p.is_null());
    }

    fn remove_unit_productions(&mut self) {
        let mut unit_production_pairs: Vec<(String, String)> = vec![];
        let mut productions_to_add: Vec<Production> = vec![];

        self.productions.iter().for_each(|prod| {
            if prod.value.len() == 1 && self.is_variable(prod.value[0].as_str()) {
                unit_production_pairs.push((prod.symbol.clone(), prod.value[0].clone()))
            }
        });

        // Add all the productions of the result to the parent and remove the unit production
        for (unit_name, unit_prod) in unit_production_pairs.clone() {
            let productions_from_unit = self.productions.iter().filter(|p| p.symbol == unit_prod);
            for prod in productions_from_unit {
                productions_to_add.push(Production {
                    symbol: unit_name.clone(),
                    value: prod.value.clone(),
                })
            }
        }

        let should_remove = |p: &Production| -> bool {
            if p.value.len() > 1 {
                return false;
            }

            // Check if name and value are in unit_production_pairs
            return unit_production_pairs
                .iter()
                .any(|(name, value)| &p.symbol == name && &p.value[0] == value);
        };

        // Remove the unit production
        self.productions.retain(|p| !should_remove(p));

        productions_to_add
            .iter()
            .for_each(|p| self.productions.push(p.clone()));
    }

    fn remove_useless_productions(&mut self) {
        let mut reachable_vars: Vec<String> = vec![self.starting_variable.clone()];

        loop {
            let mut new_reachable_vars: Vec<String> = vec![];

            for symbol in reachable_vars.clone() {
                let prods = self.productions.iter().filter(|p| p.symbol == symbol);
                for prod in prods {
                    for s in &prod.value {
                        if self.is_variable(s.as_str())
                            && !reachable_vars.contains(s)
                            && !new_reachable_vars.contains(s)
                        {
                            new_reachable_vars.push(s.clone());
                        }
                    }
                }
            }

            // If there are no new reachable vars, break
            if new_reachable_vars.is_empty() {
                break;
            }

            reachable_vars.extend(new_reachable_vars);
        }

        // Remove all variables that are not reachable
        self.productions
            .retain(|p| reachable_vars.contains(&p.symbol));
    }

    fn isolate_terminals(&mut self) {
        let variable_names: HashSet<_> =
            self.productions.iter().map(|p| p.symbol.clone()).collect();
        let mut to_insert = Vec::new();
        let self_clone = self.clone();

        for prod in self.productions.iter_mut() {
            let mut new_value: Vec<String>;

            // Make sure terminals appear on their own
            if !self_clone.is_isolated(&prod.value) {
                continue;
            }

            // Replace the terminal with a new variable
            new_value = prod.value.clone();
            prod.value.iter().enumerate().for_each(|(i, s)| {
                if !variable_names.contains(s) {
                    // Create the new var to hold this terminal
                    let new_name = format!("{}{}", s, "`");
                    to_insert.push(Production {
                        symbol: new_name.clone(),
                        value: vec![s.clone()],
                    });

                    // Remove the prod and replace with the new var
                    new_value.remove(i);
                    new_value.insert(i, new_name);
                }
            });

            prod.value = new_value;
        }

        for prod in to_insert {
            if !self.productions.contains(&prod) {
                self.productions.push(prod);
            }
        }
    }

    // Make sure that if there is more than one item, its not a terminal
    fn is_isolated(&self, strings: &Vec<String>) -> bool {
        if strings.len() == 1 {
            return false;
        }

        for s in strings {
            if !self.is_variable(s.as_str()) {
                return true;
            }
        }

        return false;
    }

    // This will follow the similar pattern as isolate_terminals
    fn remove_long_productions(&mut self) {
        let mut to_insert = Vec::new();
        for prod in self.productions.iter_mut() {
            // If the length is longer than 2
            while prod.value.len() > 2 {
                // Replace 2 of the variables with a new variable
                let last2 = prod.value.split_off(prod.value.len() - 2);
                let new_name = last2.join("");
                to_insert.push(Production {
                    symbol: new_name.clone(),
                    value: last2,
                });

                // Replace with the new var
                prod.value.push(new_name);
            }
        }

        for prod in to_insert {
            self.productions.push(prod);
        }
    }

    pub fn generate_sample_langauge(&self, n: usize) -> Vec<String> {
        let mut sample_strings = Vec::new();
        for _ in 0..n {
            let sample = self.generate_sample_string(vec![self.starting_variable.clone()]);
            sample_strings.push(sample);
        }

        sample_strings
    }

    fn generate_sample_string(&self, s: Vec<String>) -> String {
        for (i, c) in s.iter().enumerate() {
            if self.is_variable(c) {
                let prods: Vec<_> = self.productions.iter().filter(|p| &p.symbol == c).collect();

                // Choose a random production
                let prod = prods.choose(&mut rand::thread_rng()).unwrap();

                // Replace the variable with the production
                // Insert the vector from prod.value into the new_s
                let mut new_s = s.clone();
                new_s.splice(i..i + 1, prod.value.clone());

                return self.generate_sample_string(new_s);
            }
        }

        return s.join("");
    }
}

fn read_cfg(input: &str) -> CFG {
    let lines: Vec<&str> = input.lines().collect();

    let mut prods: Vec<Production> = Vec::new();
    let starting_variable = lines[0].split(" ").next().unwrap().to_string();

    for line in lines {
        // First char is the variable name
        let name = line.split(" ").next().unwrap().to_string();

        let split = line.split(" -> ");
        let children_str = split.last().unwrap();

        let children_sep = children_str.split(" | ").map(|x| x.trim().split(" "));

        // Store just the name of the children for now
        for child in children_sep {
            let value_vec: Vec<String> = child.map(|x| x.to_string()).collect();
            let prod = Production {
                symbol: name.clone(),
                value: value_vec,
            };

            prods.push(prod);
        }
    }

    CFG {
        starting_variable,
        productions: prods,
    }
}

#[cfg(test)]
#[test]
fn test_permute_without() {
    let input = vec![
        'A'.to_string(),
        'B'.to_string(),
        'A'.to_string(),
        'C'.to_string(),
    ];
    let output = permute_without(input, "A");
    let expected = vec![
        vec![
            'A'.to_string(),
            'B'.to_string(),
            'A'.to_string(),
            'C'.to_string(),
        ],
        vec!['B'.to_string(), 'A'.to_string(), 'C'.to_string()],
        vec!['B'.to_string(), 'C'.to_string()],
        vec!['A'.to_string(), 'B'.to_string(), 'C'.to_string()],
    ];

    assert_eq!(output, expected);
}
