/// Create enum 'Unit' holding the temperature units.
#[derive(Debug, PartialEq)]
pub enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

/// Temperature struct holding the value of the input and the unit which has a type from the 'Unit' enum.
#[derive(Debug)]
pub struct Temperature {
    value: f64,
    unit: Unit,
}

/// An implementation of the temperature struct.
impl Temperature {
    pub fn new(value:f64, unit: Unit) -> Self {
        Self {
            value,
            unit,
        }
    }

    /// To get the value from a result variable holding the converted unit, use this method!
    pub fn get_value(&self) -> f64 {
        self.value
    }

    /// A function to convert either celsius, fahrenheit or kelvin to celsius using a floating point input.
    ///
    /// Convert a temp to Celsius from Fahrenheit.
    /// ```
    ///
    /// use t_convert::{Temperature, Unit};
    ///
    /// let temp = Temperature::new(0.0, Unit::Fahrenheit);
    /// // Store the result in a new variable to ensure we don't get 'temp', which is now an f64
    ///
    /// let result = temp.to_celsius();
    ///
    /// println!("Temperature in Celsius: {}, Result {}", temp.get_value(), result);
    /// ```
    pub fn to_celsius(&self) -> f64 {
        match self.unit {
            Unit::Celsius => self.value,

            Unit::Fahrenheit => (self.value - 32.0) * 5.0 / 9.0,

            Unit::Kelvin => self.value - 273.15,
        }
    }

    /// A function to convert either fahrenheit, celsius or kelvin to fahrenheit using a floating point input.
    ///
    /// Convert a temp to Fahrenheit from celsius.
    /// ```
    ///
    /// use t_convert::{Temperature, Unit};
    ///
    /// let temp = Temperature::new(0.0, Unit::Celsius);
    /// let temp = temp.to_fahrenheit();
    ///
    /// println!("Temperature in Fahrenheit: {}", temp);
    /// ```
    pub fn to_fahrenheit(&self) -> f64 {
        match self.unit {
            Unit::Fahrenheit => self.value,

            Unit::Celsius => (self.value * 9.0 / 5.0) + 32.0,

            Unit::Kelvin => (self.value - 273.15) * 9.0 / 5.0 + 32.0,
        }
    }

    /// A function to convert either kelvin, celsius or fahrenheit to kelvin using a floating point input.
    ///
    /// Convert a temp to Kelvin from celsius.
    /// ```
    ///
    /// use t_convert::{Temperature, Unit};
    ///
    /// let temp = Temperature::new(0.0, Unit::Celsius);
    /// let temp = temp.to_kelvin();
    ///
    /// println!("Temperature in kelvin: {}", temp);
    /// ```
    pub fn to_kelvin(&self) -> f64 {
        match self.unit {
            Unit::Kelvin => self.value,

            Unit::Celsius => self.value + 273.15,

            Unit::Fahrenheit => (self.value - 32.0) * 5.0 / 9.0 + 273.15,
        }
    }
}