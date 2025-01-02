use super::Val;

pub struct JObject(Vec<(String, Val)>);
pub struct JArray(Vec<Val>);

impl JObject {
    pub fn new() -> JObject {
        JObject(vec![])
    }

    pub fn get(&self, key: &str) -> Option<&Val> {
        self.0.iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    pub fn push(&mut self, (k, v): (String, Val)) {
        self.0.push((k, v));
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (String, Val)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, (String, Val)> {
        self.0.iter_mut()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<(String, Val)> {
        self.0.into_iter()
    }
}

impl JArray {
    pub fn new() -> JArray {
        JArray(vec![])
    }

    pub fn push(&mut self, v: Val) {
        self.0.push(v);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Val> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Val> {
        self.0.iter_mut()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<Val> {
        self.0.into_iter()
    }
}

impl IntoIterator for JArray {
    type Item = Val;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl IntoIterator for JObject {
    type Item = (String, Val);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }    
}

impl Default for JObject {
    fn default() -> Self {
        JObject::new()
    }
}

impl Default for JArray {
    fn default() -> Self {
        JArray::new()
    }
}