use crate::GenSerialData;

pub struct ProductID {
    id: Option<String>,
    digit: usize,
    name: String,
}

impl ProductID {
    pub fn new(digit: usize) -> Self {
        ProductID {
            name: "ProductID".to_owned(),
            digit,
            id: None,
        }
    }
}

impl GenSerialData for ProductID {
    fn get_length(&self) -> usize {
        self.digit
    }

    fn get_rawdata(&self) -> Option<String> {
        if self.id.is_some() {
            self.id.clone()
        } else {
            None
        }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn put_rawdata(&mut self, data: &str) {
        self.id = Some(data.to_owned());
    }

    fn get_arg_name(&self) -> &str {
        "productid"
    }

    fn get_help(&self) -> String {
        format!("Product ID with {}-digit", self.digit)
    }

    fn get_mandatory(&self) -> bool {
        true
    }
}
