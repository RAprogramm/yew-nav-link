//! # Breadcrumbs
//!
//! Auto-generates a breadcrumb trail from the current URL. The
//! [`use_breadcrumbs`] hook splits the path into segments and returns
//! one [`BreadcrumbItem`] per level.
//!
//! For a URL like `/docs/api/reference`, you get:
//!
//! - `/` — is_active: false
//! - `/docs` — is_active: false
//! - `/docs/api` — is_active: false
//! - `/docs/api/reference` — is_active: true
//!
//! The trail updates automatically when the user navigates — no manual
//! subscriptions or state management needed.
//!
//! # How It Works
//!
//! 1. The hook calls `use_route::<R>()` to get the current route.
//! 2. It calls `route.to_path()` (e.g. `"/docs/api"`).
//! 3. It splits by `/` and builds cumulative paths: `["/", "/docs", "/docs/api"]`.
//! 4. Each segment becomes a `BreadcrumbItem { route, label, is_active }`.
//! 5. The last item always has `is_active: true`.
//!
//! # BreadcrumbItem Fields
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `route` | `R` | The current route (same for all items) |
//! | `label` | `String` | Cumulative path: `"/"`, `"/docs"`, `"/docs/api"` |
//! | `is_active` | `bool` | `true` only for the last item |
//!
//! # Memory & Performance
//!
//! The hook allocates a `Vec<BreadcrumbItem<R>>` on every render where the
//! route changes. Each item holds a cloned `R` and a `String` label. For a
//! typical 3-level path this is ~4 small allocations — negligible for web use.
