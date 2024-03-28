use rust_decimal::Decimal;

pub trait IntoDecimal {
    fn into_decimal(self) -> Decimal;
}

impl IntoDecimal for f64 {
    fn into_decimal(self) -> Decimal {
        Decimal::new((self * 100f64).round() as i64, 2)
    }
}

pub trait IntoDecimalVec {
    fn into_decimal_vec(self) -> Vec<Decimal>;
}

impl IntoDecimalVec for Vec<f64> {
    fn into_decimal_vec(self) -> Vec<Decimal> {
        self.into_iter().map(|v| {
            Decimal::new((v * 100f64).round() as i64, 2)
        }
        ).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert(){
        let values: Vec<f64> = vec![10.0001, 20.0001, 30.0001, 40.0001];
        let decimal_values = values.into_decimal_vec();
        assert_eq!(decimal_values, vec![Decimal::new(100, 1), Decimal::new(200, 1), Decimal::new(300, 1), Decimal::new(400, 1)]);
    }
}