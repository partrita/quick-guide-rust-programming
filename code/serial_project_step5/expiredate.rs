use crate::GenSerialData;

pub struct ExpireDate {
    year: usize,
    month: usize,
    day: usize,
    name: String,
    digit: usize,
    mandatory: bool,
}

impl ExpireDate {
    pub fn new(digit: usize, mandatory: bool) -> Self {
        ExpireDate {
            name: "ExpireDate".to_owned(),
            year: 0,
            month: 0,
            day: 0,
            digit,
            mandatory,
        }
    }
}

impl GenSerialData for ExpireDate {
    fn verify(&self, data: &str) -> bool {
        let year: usize = data[0..4].parse().unwrap();
        let month: usize = data[4..6].parse().unwrap();
        let day: usize = data[6..8].parse().unwrap();

        self.year == year && self.month == month && self.day == day
    }

    fn get_length(&self) -> usize {
        self.digit
    }

    fn get_rawdata(&self) -> Option<String> {
        if self.year != 0 && self.month != 0 && self.day != 0 {
            Some(format!("{:04}{:02}{:02}", self.year, self.month, self.day))
        } else {
            None
        }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn put_rawdata(&mut self, data: &str) {
        let year = data[0..4].parse().unwrap();
        let month = data[4..6].parse().unwrap();
        let day = data[6..8].parse().unwrap();
        self.year = year;
        self.month = month;
        self.day = day;
    }

    fn get_arg_name(&self) -> &str {
        "expiredate"
    }

    fn get_help(&self) -> String {
        format!("Expire date with YYYYMMDD format")
    }

    fn get_mandatory(&self) -> bool {
        self.mandatory
    }
}
