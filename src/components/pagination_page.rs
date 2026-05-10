// SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
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
/// # Arguments
///
/// * `current` - The currently active page (1-indexed).
/// * `total` - The total number of pages.
/// * `siblings` - How many pages to show on each side of the current page.
///
/// # Returns
///
/// A `Vec<u32>` of page numbers. `0` entries represent ellipsis (`...`).
pub fn generate_pages(current: u32, total: u32, siblings: u32) -> Vec<u32> {
    if total == 0 {
        return Vec::new();
    }
    if total == 1 {
        return vec![1];
    }

    // Clamp current into [1, total] so out-of-range inputs don't drift the
    // window math; clamp siblings to (total - 1) so it can never push the
    // window past the available page count.
    let current = current.clamp(1, total);
    let siblings = siblings.min(total.saturating_sub(1));

    // All arithmetic from here on operates on u32 with saturating helpers so
    // that adversarial inputs (e.g. siblings = u32::MAX on wasm32 where
    // usize is 32 bits) cannot overflow.
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

    // Ellipsis between first page and the window's start.
    if start > 3 {
        pages.push(0);
    }

    // Middle window. `end >= start` is invariant by the clamps above except
    // when `end < 2`, which means there is no middle window at all.
    if end >= start {
        for page in start..=end {
            pages.push(page);
        }
    }

    // Ellipsis between the window's end and the last page.
    if end < total.saturating_sub(2) {
        pages.push(0);
    }

    pages.push(total);

    // Collapse consecutive duplicates (e.g. start = 2 already covered by the
    // pushed `1`, or start == end == total when current is the last page).
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
        assert!(!pages.is_empty());
        assert!(pages.contains(&5));
        // Check the exact expected value for middle page with 1 sibling
        assert_eq!(pages, vec![1, 0, 4, 5, 6, 0, 10]);
    }

    #[test]
    fn generate_pages_with_ellipsis() {
        let pages = generate_pages(5, 10, 1);
        assert!(!pages.is_empty());
        assert!(pages.contains(&0));
    }

    #[test]
    fn generate_pages_many_siblings() {
        let pages = generate_pages(5, 20, 3);
        assert!(!pages.is_empty());
        // With 3 siblings around page 5, we expect: [1, 0, 2, 3, 4, 5, 6, 7, 8, 0, 20]
        // Let's compute: start = 5-3=2, end=5+3=8 -> then we push 1, then since
        // start=2>3? false -> no ellipsis after first? Wait, the condition for
        // ellipsis after first is: if start > 3 -> then push ellipsis. 2>3 is false ->
        // no ellipsis after first. Then we push pages from 2 to 8 ->
        // 2,3,4,5,6,7,8. Then we check for ellipsis before last: if end <
        // total_pages-2 -> 8 < 18 -> true -> push ellipsis. Then push last page
        // 20. So we get: [1, 2,3,4,5,6,7,8,0,20] -> but note we always push 1
        // at the beginning and then we push from start to end, so we have 1, then 2..8.
        // Then we push ellipsis and 20.
        // So the expected is [1,2,3,4,5,6,7,8,0,20]
        assert_eq!(pages, vec![1, 2, 3, 4, 5, 6, 7, 8, 0, 20]);
    }

    #[test]
    fn generate_pages_edge_case_first() {
        let pages = generate_pages(2, 10, 1);
        assert!(pages.contains(&1));
        assert!(pages.contains(&2));
        // Expected: [1,2,3,0,10]
        assert_eq!(pages, vec![1, 2, 3, 0, 10]);
    }

    #[test]
    fn generate_pages_edge_case_last() {
        let pages = generate_pages(9, 10, 1);
        assert!(pages.contains(&9));
        assert!(pages.contains(&10));
        // Expected: [1,0,8,9,10]
        assert_eq!(pages, vec![1, 0, 8, 9, 10]);
    }

    #[test]
    fn generate_pages_all_visible() {
        // When total pages small, no ellipsis needed
        let pages = generate_pages(2, 3, 1);
        assert_eq!(pages, vec![1, 2, 3]);
    }

    #[test]
    fn generate_pages_current_equals_total() {
        let pages = generate_pages(10, 10, 2);
        assert!(pages.contains(&10));
        // Expected: [1,0,8,9,10]? Let's compute:
        // total=10, current=10, siblings=2
        // start: current > (siblings+2) -> 10 > 4 -> true -> start = 10-2=8.
        // end: current+siblings < total_pages-1 -> 10+2=12 < 9? false -> end=9.
        // pages: 1, then start=8>3 -> true -> push ellipsis, then 8,9,10 (from start to
        // end: 8,9,10), then end=9<8? false -> no ellipsis, then push total=10 ->
        // [1,0,8,9,10,10] -> remove duplicates: [1,0,8,9,10]
        assert_eq!(pages, vec![1, 0, 8, 9, 10]);
    }

    #[test]
    fn generate_pages_siblings_zero() {
        let pages = generate_pages(5, 10, 0);
        // With 0 siblings, we expect: [1,0,5,0,10]
        assert_eq!(pages, vec![1, 0, 5, 0, 10]);
    }

    #[test]
    fn generate_pages_no_ellipsis_after_first() {
        // When start <= 3, we should not add ellipsis after first
        let pages = generate_pages(2, 10, 1);
        assert_eq!(pages, vec![1, 2, 3, 0, 10]);
    }

    #[test]
    fn generate_pages_no_ellipsis_before_last() {
        // When end >= total_pages-2, we should not add ellipsis before last
        let pages = generate_pages(9, 10, 1);
        assert_eq!(pages, vec![1, 0, 8, 9, 10]);
    }

    #[test]
    fn generate_pages_both_ellipsis() {
        // When we need ellipsis both after first and before last
        let pages = generate_pages(4, 10, 1);
        // Expected: [1,3,4,5,0,10]
        assert_eq!(pages, vec![1, 3, 4, 5, 0, 10]);
    }

    #[test]
    fn generate_pages_no_ellipsis_when_total_small() {
        // When total pages are small enough that we show all pages without ellipsis
        let pages = generate_pages(2, 4, 1);
        // Expected: [1,2,3,4]
        assert_eq!(pages, vec![1, 2, 3, 4]);
    }

    #[test]
    fn generate_pages_show_all_when_siblings_large() {
        // When siblings are large enough to show all pages
        let pages = generate_pages(3, 5, 10);
        // Expected: [1,2,3,4,5]
        assert_eq!(pages, vec![1, 2, 3, 4, 5]);
    }

    // ── Boundary cases ─────────────────────────────────────────────────────

    #[test]
    fn generate_pages_total_zero_returns_empty() {
        // Zero pages → empty list, no synthesised "1".
        assert!(generate_pages(1, 0, 0).is_empty());
        assert!(generate_pages(5, 0, 3).is_empty());
    }

    #[test]
    fn generate_pages_current_above_total_is_clamped() {
        // current > total should not push the window past the last page.
        let pages = generate_pages(99, 10, 1);
        assert_eq!(pages, vec![1, 0, 9, 10]);
    }

    #[test]
    fn generate_pages_current_zero_is_clamped() {
        // current = 0 is treated as page 1 (1-indexed contract).
        let pages = generate_pages(0, 10, 1);
        assert_eq!(pages, vec![1, 2, 0, 10]);
    }

    #[test]
    fn generate_pages_siblings_max_does_not_overflow() {
        // siblings = u32::MAX would overflow `current as usize + siblings as
        // usize` on wasm32 (32-bit usize) without saturating arithmetic.
        // We only assert the call does not panic and the output is sane.
        let pages = generate_pages(5, 10, u32::MAX);
        assert!(pages.first() == Some(&1));
        assert!(pages.last() == Some(&10));
        // When siblings >= total - 1, every page from 2..=total-1 is in
        // the window — no ellipses anywhere.
        assert_eq!(pages, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
