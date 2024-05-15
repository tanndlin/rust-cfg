use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Variable {
    pub name: String,
    productions: Vec<Vec<String>>,
}
impl Variable {
    fn remove_null_production(&mut self, nullable_name: &str) {
        // Case where there is only 1 production
        let mut new_prods: Vec<Vec<String>> = vec![];

        // For each current production
        self.productions.iter().for_each(|x| {
            // Add all productions with nullable prod removed
            permute_without(x.clone(), &nullable_name)
                .iter()
                .for_each(|p| {
                    if !new_prods.contains(p) {
                        new_prods.push(p.clone())
                    }
                });
        });

        // Remove the production with just the null symbol
        self.productions = new_prods
            .iter()
            .filter(|x| x.len() > 1 || (x.len() == 1 && x[0] != "#"))
            .map(|x| x.clone())
            .collect();
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

#[derive(Debug)]
pub struct CFG {
    starting_variable: String,
    variables: HashMap<String, Variable>,
}

impl CFG {
    pub fn new() -> CFG {
        let (starting_variable, variables) = read_cfg();
        let mut cfg = CFG {
            starting_variable,
            variables,
        };

        cfg.to_cnf();

        cfg
    }

    fn get_variable(&self, name: &str) -> &Variable {
        self.variables.get(name).unwrap()
    }

    fn is_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    // pub fn test(&self, string: &str) -> bool {}
    /*
    3 Rules for CNF:
    1. All variables have at most 2 variables
    2. If there is one production, it must be a terminal
    3. All children are either variables or terminals
    */
    fn is_cnf(&self) -> bool {
        for (_, variable) in &self.variables {
            for production in &variable.productions {
                if production.len() == 1 {
                    continue;
                }

                if production.len() > 2
                    && production.iter().filter(|x| self.is_variable(x)).count() > 2
                {
                    return false;
                }

                // THIS ASSUMES THAT ALL VARIABLES ARE UPPERCASE
                let all_uppercase = production.iter().all(|x| self.is_variable(x));
                let all_lowercase = production.iter().all(|x| !self.is_variable(x));

                if !(all_uppercase || all_lowercase) {
                    return false;
                }
            }
        }

        true
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

        // Step 4: Remove variables with more than 2 variables
    }

    fn remove_start_symbol(&mut self) {
        let contains_start = self.variables.iter().any(|(_, variable)| {
            variable
                .productions
                .iter()
                .any(|x| x.contains(&self.starting_variable))
        });

        // TODO We need to smartly generate variables later
        if contains_start {
            let old_starting_var = self.starting_variable.clone();

            self.starting_variable = "Z".to_string();
            self.variables.insert(
                self.starting_variable.clone(),
                Variable {
                    name: self.starting_variable.clone(),
                    productions: vec![vec![old_starting_var]],
                },
            );
        }
    }

    fn remove_null_productions(&mut self) {
        let nullable_names: Vec<_> = self
            .variables
            .iter()
            .filter_map(|(nullable_name, nullable_var)| {
                let contains_null = nullable_var
                    .productions
                    .iter()
                    .any(|x| x.contains(&"#".to_string()));
                if contains_null {
                    Some(nullable_name.clone())
                } else {
                    None
                }
            })
            .collect();

        for nullable_name in nullable_names {
            self.variables
                .iter_mut()
                .filter(|(_, containing_var)| {
                    containing_var
                        .productions
                        .iter()
                        .any(|x| x.contains(&nullable_name))
                })
                .for_each(|(_, containing_var)| {
                    containing_var.remove_null_production(nullable_name.clone().as_str())
                });
        }
    }

    fn remove_unit_productions(&mut self) {
        let mut unit_production_pairs: Vec<(String, String)> = vec![];
        self.variables.iter().for_each(|(name, var)| {
            var.productions.iter().for_each(|production| {
                if production.len() == 1 && self.is_variable(production[0].as_str()) {
                    unit_production_pairs.push((name.clone(), production[0].clone()));
                }
            });
        });

        // Add all the productions of the result to the parent and remove the unit production
        for (unit_name, unit_prod) in unit_production_pairs {
            let result_prods = self.get_variable(unit_prod.as_str()).productions.clone();
            let parent_prod = &mut self.variables.get_mut(&unit_name).unwrap().productions;
            for prod in result_prods {
                if !parent_prod.contains(&prod) {
                    parent_prod.push(prod);
                }
            }

            // Remove the unit production
            parent_prod.retain(|x| x.len() != 1 || x[0] != unit_prod);
        }
    }

    fn remove_useless_productions(&mut self) {
        let mut reachable_vars: Vec<String> = vec![self.starting_variable.clone()];

        loop {
            let mut new_reachable_vars = vec![];

            for var in &reachable_vars {
                let var_prods = self.get_variable(var.as_str()).productions.clone();
                for prod in var_prods {
                    for symbol in prod {
                        if self.is_variable(symbol.as_str())
                            && !reachable_vars.contains(&symbol)
                            && !new_reachable_vars.contains(&symbol)
                        {
                            new_reachable_vars.push(symbol);
                        }
                    }
                }
            }

            if new_reachable_vars.len() == 0 {
                break;
            }

            new_reachable_vars.iter().for_each(|x| {
                if !reachable_vars.contains(x) {
                    reachable_vars.push(x.clone());
                }
            });
        }

        // Remove all variables that are not reachable
        self.variables
            .retain(|name, _| reachable_vars.contains(name));
    }
}

fn create_var_refs(lines: Vec<&str>) -> (String, HashMap<String, Variable>) {
    let mut variables: HashMap<String, Variable> = HashMap::new();
    let starting_variable = lines[0].split(" ").next().unwrap().to_string();

    for line in lines {
        // First char is the variable name
        let name = line.split(" ").next().unwrap().to_string();
        let mut children: Vec<Vec<String>> = Vec::new();

        let split = line.split(" -> ");
        let children_str = split.last().unwrap();

        let children_sep = children_str.split(" | ").map(|x| x.trim().split(" "));

        // Store just the name of the children for now
        for child in children_sep {
            let child_vec: Vec<String> = child.map(|x| x.to_string()).collect();
            children.push(child_vec);
        }

        let variable = Variable {
            name: name.clone(),
            productions: children,
        };

        // Store the variable in the map
        variables.insert(name, variable);
    }

    (starting_variable, variables)
}

fn read_cfg() -> (String, HashMap<String, Variable>) {
    let file_data = std::fs::read_to_string("cfg.txt").unwrap();
    create_var_refs(file_data.lines().collect())
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
