pub fn get_power_consumption(diagnosis: &[u8]) -> usize {
    get_gamma_rate(diagnosis) * get_epsilon_rate(diagnosis)
}

pub fn get_epsilon_rate(diagnosis: &[u8]) -> usize {
    get_rate(diagnosis, emmit_bit_least_frequent) as usize
}
pub fn get_gamma_rate(diagnosis: &[u8]) -> usize {
    get_rate(diagnosis, emmit_bit_most_frequent) as usize
}
fn get_rate(diagnosis: &[u8], f: fn(u8, &u8) -> u8) -> u8 {
    let reading_size: usize = 5;
    let total_count_position: usize = reading_size;

    let state = vec![0; reading_size + 1];
    let bit_count = diagnosis.iter().scan(state, |state, b| {
        for position in 0..reading_size {
            let mask = 0b00000001u8;
            let is_present: u8 = (b & (mask << position)) >> position;
            state[position] += is_present;
        }
        state[total_count_position] += 1;
        Some(state.clone())
    }).last().unwrap();
    let average = bit_count[total_count_position] / 2;

    bit_count.iter().take(reading_size).map(|bit_count|
        f(average, bit_count)
    ).enumerate().scan(0, |state, (idx, average)| {
        let bit = average << idx;
        *state = *state | bit;
        Some(*state)
    }).last().unwrap()
}

fn emmit_bit_most_frequent(average: u8, bit_count: &u8) -> u8 {
    (*bit_count > average) as u8
}
fn emmit_bit_least_frequent(average: u8, bit_count: &u8) -> u8 {
    (*bit_count < average) as u8
}

#[cfg(test)]
mod test {
    use crate::day_3_binary_diagnostic::{get_gamma_rate, get_epsilon_rate};

    #[test]
    fn get_gamma_rate_test() {
        let diagnosis = vec![0b00100u8,
                             0b11110u8,
                             0b10110u8,
                             0b10111u8,
                             0b10101u8,
                             0b01111u8,
                             0b00111u8,
                             0b11100u8,
                             0b10000u8,
                             0b11001u8,
                             0b00010u8,
                             0b01010u8];
        let gamma_rate = get_gamma_rate(&diagnosis);
        assert_eq!(22, gamma_rate)
    }
    #[test]
    fn get_epsilon_rate_test() {
        let diagnosis = vec![0b00100u8,
                             0b11110u8,
                             0b10110u8,
                             0b10111u8,
                             0b10101u8,
                             0b01111u8,
                             0b00111u8,
                             0b11100u8,
                             0b10000u8,
                             0b11001u8,
                             0b00010u8,
                             0b01010u8];
        let epsilon_rate = get_epsilon_rate(&diagnosis);
        assert_eq!(9, epsilon_rate)
    }
}