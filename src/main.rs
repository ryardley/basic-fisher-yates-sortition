fn main() {}

#[cfg(test)]
mod tests {
    use std::{char, collections::HashSet, usize};

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
    fn is_selected_optimized(len: usize, cutoff: usize, original_index: usize, seed: u64) -> bool {
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

    fn is_selected_with_skip_list(
        list_length: usize,
        cutoff: usize,
        original_index: usize,
        skip_list: &[usize],
        seed: u64,
    ) -> bool {
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        let skip_set: HashSet<usize> = skip_list.iter().cloned().collect();

        // If the original index is in the skip list, it can't be selected
        if skip_set.contains(&original_index) {
            return false;
        }

        // Create a vector of indices and shuffle it
        let mut indices: Vec<usize> = (0..list_length).collect();
        for i in (1..list_length).rev() {
            let j = rng.next_u32() as usize % (i + 1);
            indices.swap(i, j);
        }

        // Find the new position of our item of interest
        let new_position = indices.iter().position(|&x| x == original_index).unwrap();

        // Count non-skipped items before our item
        let items_before = indices[0..new_position]
            .iter()
            .filter(|&&x| !skip_set.contains(&x))
            .count();

        // Our item is selected if the number of non-skipped items before it is less than the cutoff
        items_before < cutoff
    }

    #[test]
    fn test_shuffle() {
        let full_list: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
        let skip_list: Vec<usize> = vec![1]; // B has been removed;

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
            is_selected_with_skip_list(full_list.len(), cutoff, 0, &skip_list, seed),
            false,
        );

        // 'H' -> 4
        assert_eq!(
            is_selected_with_full_list(&full_list, cutoff, 7, seed),
            true
        );

        assert_eq!(
            is_selected_with_skip_list(full_list.len(), cutoff, 7, &skip_list, seed),
            true,
        );

        // 'C' -> 5
        assert_eq!(
            is_selected_with_full_list(&full_list, cutoff, 2, seed),
            false
        );

        assert_eq!(
            is_selected_with_skip_list(full_list.len(), cutoff, 2, &skip_list, seed),
            true, // NOTE this is included because B was skipped
        );

        // 'B' -> 1
        assert_eq!(
            is_selected_with_full_list(&full_list, cutoff, 1, seed),
            true
        );


        assert_eq!(
            is_selected_with_skip_list(full_list.len(), cutoff, 1, &skip_list, seed),
            false, // Note 'B' is Skipped because it is in the skip list
        );
    }
}
