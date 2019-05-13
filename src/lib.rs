#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseErrorKind {
    EntryOutsideOfSection,
    MissingClosingBracket,
    MissingEquals,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseError {
    line: usize,
    kind: ParseErrorKind,
}

impl ParseError {
    fn new(line: usize, kind: ParseErrorKind) -> ParseError {
        ParseError { line, kind }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry {
    pub key: String,
    pub value: String,
}

impl Entry {
    pub fn new(&self, key: &str, value: &str) -> Entry {
        Entry {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} = {}", self.key, self.value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Section {
    pub name: String,
    pub entries: Vec<Entry>,
}

impl Section {
    pub fn to_string(&self) -> String {
        let mut result = format!("[{}]", self.name);
        for entry in self.entries.iter() {
            result += "\n";
            result += &entry.to_string();
        }
        result += "\n";
        result
    }
}

impl Section {
    pub fn new(name: String) -> Section {
        Section {
            name,
            entries: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Conf {
    pub sections: Vec<Section>,
}

impl Conf {
    pub fn new() -> Conf {
        Conf {
            sections: Vec::new(),
        }
    }

    pub fn parse_str(s: &str) -> Result<Conf, ParseError> {
        let mut conf = Conf::new();
        let mut line_no = 0;
        for line in s.lines() {
            line_no += 1;
            let line = line.trim();
            if line.starts_with('[') {
                if line.ends_with(']') {
                    let name = &line[1..line.len() - 1];
                    conf.sections.push(Section::new(name.to_string()));
                } else {
                    return Err(ParseError::new(
                        line_no,
                        ParseErrorKind::MissingClosingBracket,
                    ));
                }
            } else if line.len() != 0 {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    if let Some(section) = conf.sections.last_mut() {
                        section.entries.push(Entry {
                            key: parts[0].trim().to_string(),
                            value: parts[1].trim().to_string(),
                        });
                    } else {
                        return Err(ParseError::new(
                            line_no,
                            ParseErrorKind::EntryOutsideOfSection,
                        ));
                    }
                } else {
                    return Err(ParseError::new(line_no, ParseErrorKind::MissingEquals));
                }
            }
        }
        Ok(conf)
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        let mut is_first = true;
        for section in self.sections.iter() {
            if is_first {
                is_first = false;
            } else {
                output += "\n";
            }
            output += &section.to_string();
        }
        output
    }

    pub fn add_section(&mut self, name: String, entries: Vec<Entry>) {
        self.sections.push(Section { name, entries });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section() {
        let mut expected = Conf::new();
        expected.add_section("mySection".to_string(), Vec::new());
        assert_eq!(Conf::parse_str("[mySection]"), Ok(expected));
    }

    #[test]
    fn test_entry() {
        let mut expected = Conf::new();
        expected.add_section("mySection".to_string(), vec![
            Entry {
                key: "a".to_string(),
                value: "b".to_string(),
            }
        ]);
        assert_eq!(Conf::parse_str("[mySection]\na = b"), Ok(expected));
    }

    #[test]
    fn test_to_string() {
        let mut conf = Conf::new();
        conf.add_section("sec1".to_string(), vec![
            Entry {
                key: "a".to_string(),
                value: "b".to_string(),
            }
        ]);
        conf.add_section("sec2".to_string(), vec![
            Entry {
                key: "c".to_string(),
                value: "d".to_string(),
            }
        ]);
        assert_eq!(conf.to_string(), "[sec1]\na = b\n\n[sec2]\nc = d\n");
    }

    #[test]
    fn test_missing_closing_bracket() {
        assert_eq!(
            Conf::parse_str("[mySection"),
            Err(ParseError::new(1, ParseErrorKind::MissingClosingBracket))
        );
    }

    #[test]
    fn test_missing_equals() {
        assert_eq!(
            Conf::parse_str("[mySection]\nmyKey"),
            Err(ParseError::new(2, ParseErrorKind::MissingEquals))
        );
    }

    #[test]
    fn test_entry_outside_of_section() {
        assert_eq!(
            Conf::parse_str("a = b"),
            Err(ParseError::new(1, ParseErrorKind::EntryOutsideOfSection))
        );
    }
}
