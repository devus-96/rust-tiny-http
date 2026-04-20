use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Headers {
    data: HashMap<String, Vec<String>>,
}

#[allow(dead_code)]
impl Headers {
    pub fn new() -> Self {
        Headers {
            data: HashMap::<String, Vec<String>>::new(),
        }
    }

    pub fn with_data(data: HashMap<String, Vec<String>>) -> Self {
        Headers {
            data: data,
        }
    }

    pub fn parse(&mut self, header: &str) -> &Self {
        let header: Vec<_> = header.split(": ").collect();
        let name = header[0];

        for value in header[1].split(',') {
            let mut vec = self.data.entry(name.trim().to_owned()).or_insert(Vec::<String>::new());
            vec.push(value.trim().to_owned());
        }

        self
    }

    pub fn insert(&mut self, name: &str, value: &str) {
        let mut vec = self.data.entry(name.to_owned()).or_insert(Vec::<String>::new());
        vec.push(value.to_owned());
    }

    pub fn find(&self, key: &str) -> Option<Vec<&str>> {
        match self.data.get(key) {
            Some(vec) => {
                if vec.is_empty() {
                    None
                } else {
                    let vec: Vec<&str> = vec.iter().map(|x| x.as_ref()).collect();
                    Some(vec)
                }
            }
            _ => None
        }
    }

    pub fn has(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn all(&self) -> Vec<(&str, Vec<&str>)> {
        let vec = self.data.iter().map(|(key, values)| {
            let header_vec: Vec<&str> = values.iter().map(|x| x.as_ref()).collect();
            (key.as_ref(), header_vec)
        }).collect();

        vec
    }
}

impl ToString for Headers {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for (key, vec) in &self.data {
            let mut iter = vec.iter();
            match iter.next() {
                Some(i) => result.push_str(&format!("{}: {}", key, i)),
                None => return result,
            }

            for i in iter {
                result.push_str(&format!(", {}", i));
            }

            result.push_str("\r\n");
        }

        result
    }
}
