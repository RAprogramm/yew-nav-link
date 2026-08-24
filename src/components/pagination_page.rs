// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! # `Pagination Page Generation`
//!
//! Internal logic for computing which page numbers to display in a
//! [`Pagination`](super::Pagination) component. Returns a `Vec<u32>` where
//! `0` represents an ellipsis gap.
//!
//! # Example
//!
//! ```ignore
//! // Pages 1..10 with 1 sibling around page 5: [1, 0, 4, 5, 6, 0, 10]
//! let pages = generate_pages(5, 10, 1);
//! assert_eq!(pages, vec![1, 0, 4, 5, 6, 0, 10]);
//! ```

/// Generates a list of page numbers to display, with `0` representing ellipsis
/// gaps.
///
/// `current` is clamped into `[1, total]` and `siblings` to `total - 1`, so
/// out-of-range inputs cannot drift the window math. All arithmetic uses
/// saturating helpers, so adversarial inputs (e.g. `siblings = u32::MAX` on
/// wasm32, where `usize` is 32 bits) cannot overflow. Consecutive duplicates
/// are collapsed at the end (e.g. a window start of 2 already covered by the
/// leading `1`, or `start == end == total` when `current` is the last page).
///
/// # Arguments
///
/// * `current` - The currently active page (1-indexed).
/// * `total` - The total number of pages.
/// * `siblings` - How many pages to show on each side of the current page.
///
/// # Returns
///
/// A `Vec<u32>` of page numbers. `0` entries represent ellipsis (`…`).
pub fn generate_pages(current: u32, total: u32, siblings: u32) -> Vec<u32> {
    if total == 0 {
        return Vec::new();
    }
    if total == 1 {
        return vec![1];
    }

    let current = current.clamp(1, total);
    let siblings = siblings.min(total.saturating_sub(1));

    let start = if current > siblings.saturating_add(2) {
        current.saturating_sub(siblings)
    } else {
        2
    };

    let end = if current.saturating_add(siblings) < total.saturating_sub(1) {
        current.saturating_add(siblings)
    } else {
        total.saturating_sub(1)
    };

    let mut pages: Vec<u32> = Vec::new();
    pages.push(1);

    if start > 3 {
        pages.push(0);
    }

    if end >= start {
        for page in start..=end {
            pages.push(page);
        }
    }

    if end < total.saturating_sub(2) {
        pages.push(0);
    }

    pages.push(total);

    let mut result: Vec<u32> = Vec::with_capacity(pages.len());
    for p in pages {
        if result.last().copied() != Some(p) {
            result.push(p);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_pages_single_page() {
        let pages = generate_pages(1, 1, 1);
        assert_eq!(pages, vec![1]);
    }

    #[test]
    fn generate_pages_two_pages() {
        let pages = generate_pages(1, 2, 1);
        assert_eq!(pages, vec![1, 2]);
    }

    #[test]
    fn generate_pages_first_page() {
        let pages = generate_pages(1, 10, 1);
        assert_eq!(pages, vec![1, 2, 0, 10]);
    }

    #[test]
    fn generate_pages_last_page() {
        let pages = generate_pages(10, 10, 1);
        assert_eq!(pages, vec![1, 0, 9, 10]);
    }

    #[test]
    fn generate_pages_middle_page() {
        let pages = generate_pages(5, 10, 1);
        assert_ne!(pages, Vec::<u32>::new());
        assert!(pages.contains(&5));
        assert_eq!(pages, vec![1, 0, 4, 5, 6, 0, 10]);
    }

    #[test]
    fn generate_pages_with_ellipsis() {
        let pages = generate_pages(5, 10, 1);
        assert_ne!(pages, Vec::<u32>::new());
        assert!(pages.contains(&0));
    }

    #[test]
    fn generate_pages_many_siblings() {
        let pages = generate_pages(5, 20, 3);
        assert_ne!(pages, Vec::<u32>::new());
        assert_eq!(pages, vec![1, 2, 3, 4, 5, 6, 7, 8, 0, 20]);
    }

    #[test]
    fn generate_pages_edge_case_first() {
        let pages = generate_pages(2, 10, 1);
        assert!(pages.contains(&1));
        assert!(pages.contains(&2));
        assert_eq!(pages, vec![1, 2, 3, 0, 10]);
    }

    #[test]
    fn generate_pages_edge_case_last() {
        let pages = generate_pages(9, 10, 1);
        assert!(pages.contains(&9));
        assert!(pages.contains(&10));
        assert_eq!(pages, vec![1, 0, 8, 9, 10]);
    }

    #[test]
    fn generate_pages_all_visible() {
        let pages = generate_pages(2, 3, 1);
        assert_eq!(pages, vec![1, 2, 3]);
    }

    #[test]
    fn generate_pages_current_equals_total() {
        let pages = generate_pages(10, 10, 2);
        assert!(pages.contains(&10));
        assert_eq!(pages, vec![1, 0, 8, 9, 10]);
    }

    #[test]
    fn generate_pages_siblings_zero() {
        let pages = generate_pages(5, 10, 0);
        assert_eq!(pages, vec![1, 0, 5, 0, 10]);
    }

    #[test]
    fn generate_pages_no_ellipsis_after_first() {
        let pages = generate_pages(2, 10, 1);
        assert_eq!(pages, vec![1, 2, 3, 0, 10]);
    }

    #[test]
    fn generate_pages_no_ellipsis_before_last() {
        let pages = generate_pages(9, 10, 1);
        assert_eq!(pages, vec![1, 0, 8, 9, 10]);
    }

    #[test]
    fn generate_pages_both_ellipsis() {
        let pages = generate_pages(4, 10, 1);
        assert_eq!(pages, vec![1, 3, 4, 5, 0, 10]);
    }

    #[test]
    fn generate_pages_no_ellipsis_when_total_small() {
        let pages = generate_pages(2, 4, 1);
        assert_eq!(pages, vec![1, 2, 3, 4]);
    }

    #[test]
    fn generate_pages_show_all_when_siblings_large() {
        let pages = generate_pages(3, 5, 10);
        assert_eq!(pages, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn generate_pages_total_zero_returns_empty() {
        assert_eq!(generate_pages(1, 0, 0), Vec::<u32>::new());
        assert_eq!(generate_pages(5, 0, 3), Vec::<u32>::new());
    }

    #[test]
    fn generate_pages_current_above_total_is_clamped() {
        let pages = generate_pages(99, 10, 1);
        assert_eq!(pages, vec![1, 0, 9, 10]);
    }

    #[test]
    fn generate_pages_current_zero_is_clamped() {
        let pages = generate_pages(0, 10, 1);
        assert_eq!(pages, vec![1, 2, 0, 10]);
    }

    #[test]
    fn generate_pages_siblings_max_does_not_overflow() {
        let pages = generate_pages(5, 10, u32::MAX);
        assert_eq!(pages.first(), Some(&1));
        assert_eq!(pages.last(), Some(&10));
        assert_eq!(pages, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
