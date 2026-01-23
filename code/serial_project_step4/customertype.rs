use crate::GenSerialData;

#[derive(Clone)]
enum CustomerKind {
    Business,
    Individual,
    Company,
}

impl From<CustomerKind> for usize {
    fn from(item: CustomerKind) -> usize {
        match item {
            CustomerKind::Business => 1,
            CustomerKind::Individual => 2,
            CustomerKind::Company => 3,
        }
    }
}

pub struct CustomerType {
    customer_type: Option<CustomerKind>,
    digit: usize,
    name: String,
}

impl CustomerType {
    pub fn new() -> Self {
        CustomerType {
            name: "CustomerType".to_owned(),
            digit: 1,
            customer_type: None,
        }
    }
}

impl GenSerialData for CustomerType {
    fn get_length(&self) -> usize {
        self.digit
    }

    fn get_rawdata(&self) -> Option<String> {
        if let Some(kind) = &self.customer_type {
            Some(format!("{}", usize::from((*kind).clone())))
        } else {
            None
        }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn put_rawdata(&mut self, data: &str) {
        let kind = match data {
            "1" => CustomerKind::Business,
            "2" => CustomerKind::Individual,
            "3" => CustomerKind::Company,
            _ => CustomerKind::Business,
        };
        self.customer_type = Some(kind);
    }

    fn get_arg_name(&self) -> &str {
        "customer_type"
    }

    fn get_help(&self) -> String {
        format!("Customer type (1-Business, 2-Individual, 3-Company): ")
    }

    fn get_mandatory(&self) -> bool {
        false
    }
}
