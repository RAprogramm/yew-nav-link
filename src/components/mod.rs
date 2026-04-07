//! Navigation UI components.
//!
//! Provides a collection of reusable components for building navigation
//! interfaces, including badges, dropdowns, tabs, pagination, and more.

mod badge;
mod dropdown;
mod header;
mod icon;
mod page_item;
mod page_link;
mod pagination;
mod pagination_page;
mod tab_item;
mod tab_panel;
mod tabs;
mod text;

pub use badge::{NavBadge, NavBadgeProps};
pub use dropdown::{
    NavDropdown, NavDropdownDivider, NavDropdownDividerProps, NavDropdownItem,
    NavDropdownItemProps, NavDropdownProps
};
pub use header::{NavHeader, NavHeaderProps};
pub use icon::{NavIcon, NavIconProps, NavIconSize, NavLinkWithIcon, NavLinkWithIconProps};
pub use page_item::{PageItem, PageItemProps};
pub use page_link::{PageLink, PageLinkProps};
pub use pagination::{Pagination, PaginationProps};
pub use tab_item::{NavTab, NavTabProps};
pub use tab_panel::{NavTabPanel, NavTabPanelProps};
pub use tabs::{NavTabs, NavTabsProps};
pub use text::{NavText, NavTextProps};
