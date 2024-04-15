use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    map: HashMap<String, u16>,
    rom_address: u16,
    variable_address: u16,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            rom_address: 0,
            variable_address: 0x10,
        }
    }
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
            st.insert_predefined(l.to_string(), *a);
        });
        st
    }

    pub fn increment_rom(&mut self) {
        self.rom_address += 1
    }

    pub fn insert_variable(&mut self, label: String) {
        if self.map.get(&label).is_some() {
            return;
        }
        self.map.insert(label, self.variable_address);
        self.variable_address += 1;
    }

    pub fn insert_label(&mut self, label: String) {
        self.map.insert(label, self.rom_address);
    }

    fn insert_predefined(
        &mut self,
        label: String,
        address: u16,
    ) {
        self.map.insert(label, address);
    }

    pub fn get(&self, label: String) -> Option<u16> {
        self.map.get(&label).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_rom_address() {
        let st = SymbolTable::new();
        assert_eq!(st.rom_address, 0);
    }

    #[test]
    fn default_var_address() {
        let st = SymbolTable::new();
        assert_eq!(st.variable_address, 16);
    }

    #[test]
    fn insert_label_duplicates_are_ignored() {
        let mut st = SymbolTable::new();
        st.insert_label("label".into());
        let before = st.get("label".into());
        st.insert_label("label".into());
        let after = st.get("label".into());
        assert_eq!(before, after);
    }

    #[test]
    fn insert_variable_duplicates_are_ignored() {
        let mut st = SymbolTable::new();

        st.insert_variable("variable".into());
        let before = st.get("variable".into());
        st.insert_variable("variable".into());
        let after = st.get("variable".into());
        assert_eq!(before, after);
    }
    #[test]
    fn retrieving_value() {
        let mut st = SymbolTable::new();
        st.insert_label("test".into());
        st.insert_label("test2".into());
        assert_eq!(st.get("test".into()), Some(0))
    }

    #[test]
    fn retrieving_predefined() {
        let st = SymbolTable::new();
        assert_eq!(st.get("SP".into()), Some(0));
        assert_eq!(st.get("R3".into()), Some(3));
        assert_eq!(st.get("SCREEN".into()), Some(0x4000));
        assert_eq!(st.get("KBD".into()), Some(0x6000));
    }

    #[test]
    fn retrieving_nonexistent_value() {
        let st = SymbolTable::new();
        assert_eq!(st.get("test".into()), None)
    }
}
