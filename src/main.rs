fn main() {}

#[cfg(test)]
mod tests {
    use std::char;

    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    fn fisher_yates_shuffle<T: Clone>(slice: &[T], seed: u64) -> Vec<T> {
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        let mut shuffled = slice.to_vec();

        for i in (1..shuffled.len()).rev() {
            let j = rng.next_u32() as usize % (i + 1);
            shuffled.swap(i, j);
        }

        shuffled
    }

    // This requires the entire array
    fn is_selected_with_full_list<T: Clone + PartialEq>(
        full_list: &[T],
        cutoff: usize,
        original_index: usize,
        seed: u64,
    ) -> bool {
        let shuffled = fisher_yates_shuffle(full_list, seed);
        let new_position = shuffled
            .iter()
            .position(|x| *x == full_list[original_index])
            .unwrap();

        new_position < cutoff
    }

    // This simply requires length, cutoff, original_index and seed
    fn is_selected_optimized(
        len: usize,
        cutoff: usize,
        original_index: usize,
        seed: u64,
    ) -> bool {
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        let mut current_index = original_index;

        for i in (1..len).rev() {
            let j = rng.next_u32() as usize % (i + 1);
            if i == current_index {
                current_index = j;
            } else if j == current_index {
                current_index = i;
            }
        }

        current_index < cutoff
    }

    #[test]
    fn test_shuffle() {
        let full_list: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];

        let seed = 12345; // random seed
        let shuffled = fisher_yates_shuffle(&full_list, seed);

        assert_eq!(
            shuffled,
            vec!['D', 'B', 'G', 'F', 'H', 'C', 'J', 'I', 'E', 'A']
        );

        let cutoff = 5;

        // 'A' -> 9
        assert_eq!(
            is_selected_with_full_list(&full_list, cutoff, 0, seed),
            false,
        );

        assert_eq!(
            is_selected_optimized(10, cutoff, 0, seed),
            false,
        );

        // 'H' -> 4
        assert_eq!(
            is_selected_with_full_list(&full_list, cutoff, 7, seed),
            true
        );

        assert_eq!(
            is_selected_optimized(10, cutoff, 7, seed),
            true,
        );
    }
}
