struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn show_temp(temp: &Temperature) {
        println!("{:?} degrees F", temp.degrees_f)
    }

    fn show_temp_self(&self) {
        println!("{:?} degrees F", self.degrees_f)
    }

    fn freezing(d_f: f64) -> Temperature {
        Temperature { degrees_f: d_f }
    }
}

fn main() {
    let hot = Temperature { degrees_f: 99.9 };
    Temperature::show_temp(&hot);
    hot.show_temp_self();
    let cold  = Temperature::freezing(10.9);
    cold.show_temp_self();
}
