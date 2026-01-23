use crate::GenSerialData;

pub struct ExpireDate {
    year: u32,
    month: u32,
    day: u32,
    name: String,
}

impl ExpireDate {
    pub fn new() -> Self {
        ExpireDate {
            name: "ExpireDate".to_owned(),
            year: 0,
            month: 0,
            day: 0,
        }
    }
}

impl GenSerialData for ExpireDate {
    fn verify(&self, data: &str) -> bool {
        let year = data[0..4].parse().unwrap();
        let month = data[4..6].parse().unwrap();
        let day = data[6..8].parse().unwrap();

        self.year == year && self.month == month && self.day == day
    }

    fn get_length(&self) -> usize {
        8
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
        false
    }
}
