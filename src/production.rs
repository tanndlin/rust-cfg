use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Production {
    pub symbol: String,
    pub value: Vec<String>,
}

impl Production {
    pub fn is_null(&self) -> bool {
        self.value.len() == 1 && self.value[0] == "#"
    }

    pub fn remove_null_production(&self, nullable_name: &str) -> Vec<Production> {
        // Case where there is only 1 production
        let mut new_prods: HashSet<Production> = HashSet::new();

        // Add all productions with nullable prod removed
        permute_without(self.value.clone(), nullable_name)
            .iter()
            .filter(|vec| !vec.is_empty())
            .map(|vec| Production {
                symbol: self.symbol.clone(),
                value: vec.clone(),
            })
            .for_each(|p| {
                new_prods.insert(p);
            });

        new_prods.into_iter().collect()
    }
}

fn permute_without(strings: Vec<String>, to_remove: &str) -> Vec<Vec<String>> {
    let mut perms = HashSet::new();

    if !strings.iter().any(|c| c == to_remove) {
        return vec![strings];
    }

    strings.iter().enumerate().for_each(|(index, c)| {
        if c == to_remove {
            // Remvoe this char from the vec
            let mut new_chars = strings.clone();

            // Add the case where it is not remove
            perms.insert(new_chars.clone());

            // Add the case where it is removed
            new_chars.remove(index);
            perms.insert(new_chars.clone());

            // Recursively remove the rest
            let permuted = permute_without(new_chars, to_remove);

            // Add each case to the perms
            permuted.iter().for_each(|x| {
                perms.insert(x.clone());
            });
        }
    });

    perms.into_iter().collect()
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
    let mut output = permute_without(input, "A");
    let mut expected = vec![
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

    output.sort();
    expected.sort();
    assert_eq!(output, expected);
}
