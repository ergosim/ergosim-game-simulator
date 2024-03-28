use ::rand::prelude::*;

use rust_decimal::Decimal;
use crate::decimal::IntoDecimal;

pub trait SimulatorListener {
    fn next_input(&self) -> Result<SimulatorInput, SimulatorError>;
}

pub enum SimulatorInput {
    GrowthRate(Decimal),
}

#[derive(Debug)]
pub enum SimulatorError {
    UnableToChooseAGrowthRate,
    NoListenersRegistered,
}

pub struct Simulator {
    listeners: Vec<Box<dyn SimulatorListener>>,
    current_capital: Decimal,
    capital_evolution: Vec<Decimal>
}

impl Simulator {
    pub fn new(starting_capital: f64) -> Simulator {
        Simulator {
            listeners: Vec::new(),
            current_capital: starting_capital.into_decimal(),
            capital_evolution: vec![starting_capital.into_decimal()]
        }
    }

    pub fn add_listener(&mut self, listener: impl SimulatorListener + 'static) {
        self.listeners.push(Box::new(listener));
    }

    pub fn tick(&mut self) -> Result<Decimal, SimulatorError> {
        if self.listeners.is_empty() {
            return Err(SimulatorError::NoListenersRegistered);
        }

        for listener in &self.listeners {
            match listener.next_input()? {
                SimulatorInput::GrowthRate(growth_rate) => {
                    self.current_capital = self.current_capital * growth_rate;
                    self.capital_evolution.push(self.current_capital);
                }
            }
        }

        return Ok(self.current_capital);
    }
}

struct RandomGrowthRate {
    distribution: Vec<Decimal>
}

impl SimulatorListener for RandomGrowthRate {
    fn next_input(&self) -> Result<SimulatorInput, SimulatorError> {
        let mut rng = thread_rng();
        let growth_rate = self.distribution
            .choose(&mut rng)
            .ok_or(
                SimulatorError::UnableToChooseAGrowthRate
            )?
            .clone();

        Ok(SimulatorInput::GrowthRate(growth_rate))
    }
}

#[cfg(test)]
mod tests {
    use crate::decimal::IntoDecimalVec;
    use super::*;

    #[test]
    fn test_random_growth_rate() {
        let values = vec![0.1, 0.2, 0.3, 0.4].into_decimal_vec();
        let growth_rate = RandomGrowthRate {
            distribution: values.clone()
        };

        let SimulatorInput::GrowthRate(growth_rate) = growth_rate.next_input().unwrap();
        assert! (values.contains(&growth_rate))
    }

    #[test]
    fn test_simulator_has_a_starting_value(){
        let simulator = Simulator::new(100.0);
        assert_eq!(simulator.current_capital, 100.0.into_decimal());
    }

    #[test]
    fn test_simulator_ticks_once(){
        let possible_growth_rates: Vec<Decimal> = vec![0.9, 1.1].into_decimal_vec();
        let mut simulator = Simulator::new(100.0);
        simulator.add_listener(RandomGrowthRate {
            distribution: possible_growth_rates.clone()
        });
        simulator.tick().expect("Ticking should work");

        assert_eq!(simulator.capital_evolution.len(), 2);
        assert_eq!(&simulator.capital_evolution[0], &100.0.into_decimal());
        assert!(vec![Decimal::new(110, 0), 90.0.into_decimal()].contains(&simulator.capital_evolution[1]));
        assert!(vec![Decimal::new(110, 0), 90.0.into_decimal()].contains(&simulator.current_capital));
    }
}