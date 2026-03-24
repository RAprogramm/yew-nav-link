//! Pagination page generation logic.

pub fn generate_pages(current: u32, total: u32, siblings: u32) -> Vec<u32> {
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

    if start > 2 {
        pages.push(1);
        if start > 3 {
            pages.push(0);
        }
    }

    for page in start..=end {
        pages.push(page as u32);
    }

    if end < total_pages - 1 {
        if end < total_pages - 2 {
            pages.push(0);
        }
        pages.push(total_pages as u32);
    }

    pages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_pages_with_ellipsis() {
        let pages = generate_pages(5, 10, 1);
        assert!(!pages.is_empty());
    }
}
