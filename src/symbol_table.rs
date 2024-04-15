use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct SymbolTable {
    map: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut st = SymbolTable::default();

        let mut predefined_symbols: Vec<(String, u16)> = vec![
            ("SP".into(), 0),
            ("LCL".into(), 1),
            ("ARG".into(), 2),
            ("THIS".into(), 3),
            ("THAT".into(), 4),
            ("SCREEN".into(), 0x4000),
            ("KBD".into(), 0x6000),
        ];

        for r in 0..16u16 {
            predefined_symbols.push((format!("R{}", r), r));
        }

        predefined_symbols.iter().for_each(|(l, a)| {
            st.insert(l.to_string(), *a);
        });
        st
    }

    pub fn insert(&mut self, label: String, address: u16) {
        if self.map.get(&label).is_some() {
            return;
        }
        self.map.insert(label, address);
    }

    pub fn has(&self, label: &String) -> bool {
        self.map.get(label).is_some()
    }

    pub fn get(&self, label: &String) -> Option<u16> {
        self.map.get(label).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retrieving_value() {
        let mut st = SymbolTable::new();
        st.insert("test".into(), 0);
        st.insert("test2".into(), 1);
        assert_eq!(st.get(&"test".into()), Some(0));
        assert_eq!(st.get(&"test2".into()), Some(1));
    }

    #[test]
    fn retrieving_predefined() {
        let st = SymbolTable::new();
        assert_eq!(st.get(&"SP".into()), Some(0));
        assert_eq!(st.get(&"R3".into()), Some(3));
        assert_eq!(st.get(&"SCREEN".into()), Some(0x4000));
        assert_eq!(st.get(&"KBD".into()), Some(0x6000));
    }

    #[test]
    fn retrieving_nonexistent_value() {
        let st = SymbolTable::new();
        assert_eq!(st.get(&"test".into()), None)
    }
}
