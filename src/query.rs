use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
pub struct Query {
    data: HashMap<String, Vec<String>>,
    query_string: Option<String>,
}

#[allow(dead_code)]
impl Query {
    pub fn new() -> Query {
        Query {
            data: HashMap::<String, Vec<String>>::new(),
            query_string: None
        }
    }

    pub fn from_str(query_string: &str) -> Query {
        let mut data = HashMap::<String, Vec<String>>::new();

        if query_string.trim().len() > 0 {
            let re = Regex::new(r"([^=&]+)(=([^&]*))?").unwrap();
            for cap in re.captures_iter(query_string) {
                let key = cap.at(1).unwrap();
                // TODO: Decode query string (see this http://unixpapa.com/js/querystring.html)
                let val = cap.at(3).unwrap_or("");
                let mut query_vec = data.entry(key.to_owned()).or_insert(Vec::new());
                query_vec.push(val.to_owned());
            }
        }

        Query {
            data: data,
            query_string: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<Vec<String>> {
        match self.data.get(name) {
            Some(values) => {
                if values.is_empty() {
                    None
                }
                else {
                    Some(values.clone())
                }
            },
            None => None
        }
    }

    pub fn query_string(&self) -> Option<&str> {
        match self.query_string {
            Some(ref s) => Some(s),
            None => None,
        }
    }
}

fn format_query_param(k: &str, v: &Vec<String>) -> String {
    let mut result = String::new();

    let mut k = k.to_string();
    if v.len() > 1 {
        k.push_str("[]");
    }

    let mut iter = v.iter();
    match iter.next() {
        Some(i) => result.push_str(&format!("{}={}", k, i)),
        None => return result,
    };

    for i in iter {
        result.push_str(&format!("&{}={}", k, i));
    }

    result
}

impl ToString for Query {
    fn to_string(&self) -> String {
        let mut result = String::new();

        let mut iter = self.data.iter();
        match iter.next() {
            Some((k, v)) => result.push_str(&format_query_param(k, v)),
            None => return result,
        }

        for (k, v) in iter {
            result.push_str(&format!("&{}", format_query_param(k, v)));
        }

        result
    }
}
