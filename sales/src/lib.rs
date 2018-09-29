use std::vec::Vec;

pub mod salesperson {
    pub struct Salesperson {
        name: String,
        revenue: Vec<f32>,
        expenses: Vec<f32>,
    }

    impl Salesperson {
        pub fn new(name: String) -> Salesperson {
            Salesperson {
                name,
                revenue: Vec::new(),
                expenses: Vec::new()
            }
        }

        pub fn add_expense(&mut self, new_expense: f32) {
            self.expenses.push(new_expense)
        }

        pub fn add_revenue(&mut self, new_revenue: f32) {
            self.revenue.push(new_revenue)
        }

        pub fn calculate_commission(&self, rate: f32) -> f32 {
            let it = self.revenue.iter().zip(self.expenses.iter());

            let mut profit: f32 = 0.0;

            for (i, (rev, exp)) in it.enumerate() {
                let p = rev - exp;

                if p > 0.0 {
                    profit += (rev - exp)
                }
            }

            return profit * rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use salesperson::*;
    use std::string;

    #[test]
    fn test_challenge() {
        let salespeople = [
            ("Johnver", [190, 325, 682, 829], [120, 300, 50, 67], 92),
            ("Vanston", [140, 19, 14, 140], [65, 10, 299, 254], 5),
            ("Danbree", [1926, 293, 852, 609], [890, 23, 1290, 89], 113),
            ("Vansey", [14, 1491, 56, 120], [54, 802, 12, 129], 45),
            ("Mundyke", [143, 162, 659, 87], [430, 235, 145, 76], 32),
        ];

        let rate = 6.2 / 100.0;

        for person in salespeople.iter() {
            let mut salesperson = Salesperson::new(String::from(person.0));

            for rev in person.1.iter() {
                salesperson.add_revenue(*rev as f32);
            }

            for exp in person.2.iter() {
                salesperson.add_expense(*exp as f32);
            }

            assert_eq!(salesperson.calculate_commission(rate).floor(), person.3 as f32);
        }
    }
}
