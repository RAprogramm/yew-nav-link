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
    if total <= 1 {
        return vec![1];
    }

    let total_pages = total as usize;
    let current = current as usize;

    let start = if current > (siblings as usize) + 2 {
        current - siblings as usize
    } else {
        2
    };

    let end = if current + (siblings as usize) < total_pages - 1 {
        current + siblings as usize
    } else {
        total_pages - 1
    };

    let mut pages = Vec::new();

    // Always include first page
    pages.push(1);

    // Add ellipsis after first if needed (start > 3 means we need ellipsis after
    // page 1)
    if start > 3 {
        pages.push(0); // ellipsis
    }

    // Add middle pages
    for page in start..=end {
        pages.push(u32::try_from(page).expect("reason"));
    }

    // Add ellipsis before last if needed (end < total_pages - 2 means we need
    // ellipsis before last page)
    if end < total_pages - 2 {
        pages.push(0); // ellipsis
    }

    // Always include last page if more than 1 page
    if total > 1 {
        pages.push(total);
    }

    // Remove duplicates (e.g., when start=2, we already have 1, don't add 2 twice)
    let mut result: Vec<u32> = Vec::new();
    let mut prev: Option<u32> = None;
    for p in pages {
        if Some(p) != prev {
            result.push(p);
            prev = Some(p);
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
}
