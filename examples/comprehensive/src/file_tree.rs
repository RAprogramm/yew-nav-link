use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum FileTreeNode {
    Dir {
        name:     String,
        children: Vec<FileTreeNode>
    },
    File {
        name: String
    }
}

#[derive(Properties, PartialEq)]
pub struct FileTreeProps {
    pub nodes: Vec<FileTreeNode>
}

#[function_component]
pub fn FileTree(props: &FileTreeProps) -> Html {
    html! {
        <div class="file-tree">
            { for props.nodes.iter().map(|node| render_node(node, 0)) }
        </div>
    }
}

fn render_node(node: &FileTreeNode, depth: usize) -> Html {
    match node {
        FileTreeNode::Dir {
            name,
            children
        } => {
            html! {
                <DirNode depth={depth} name={name.clone()} children={children.clone()} />
            }
        }
        FileTreeNode::File {
            name
        } => {
            html! {
                <FileNode depth={depth} name={name.clone()} />
            }
        }
    }
}

#[derive(Properties, PartialEq)]
struct DirNodeProps {
    depth:    usize,
    name:     String,
    children: Vec<FileTreeNode>
}

#[function_component]
fn DirNode(props: &DirNodeProps) -> Html {
    let is_open = use_state(|| true);
    let is_open_for_toggle = is_open.clone();
    let indent = (props.depth as f64) * 1.25;

    let toggle = Callback::from(move |_: MouseEvent| is_open_for_toggle.set(!*is_open_for_toggle));

    html! {
        <div>
            <div
                class="file-tree-item file-tree-dir"
                style={format!("padding-left: {}rem", indent)}
                onclick={toggle}
            >
                if *is_open {
                    <svg class="file-tree-icon" viewBox="0 0 16 16" width="16" height="16" fill="currentColor">
                        <path d="M.513 1.513A1.75 1.75 0 0 1 1.75 1h3.5c.55 0 1.07.26 1.4.7l.9 1.2a.25.25 0 0 0 .2.1H13a1 1 0 0 1 1 1v.5H2.75a.75.75 0 0 0 0 1.5h11.978a1 1 0 0 1 .994 1.117L15 13.25A1.75 1.75 0 0 1 13.25 15H1.75A1.75 1.75 0 0 1 0 13.25V2.75c0-.464.184-.91.513-1.237Z"/>
                    </svg>
                } else {
                    <svg class="file-tree-icon" viewBox="0 0 16 16" width="16" height="16" fill="currentColor">
                        <path d="M0 2.75C0 1.784.784 1 1.75 1H5c.55 0 1.07.26 1.4.7l.9 1.2a.25.25 0 0 0 .2.1h6.75c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0 1 14.25 15H1.75A1.75 1.75 0 0 1 0 13.25Zm1.75-.25a.25.25 0 0 0-.25.25v10.5c0 .138.112.25.25.25h12.5a.25.25 0 0 0 .25-.25v-8.5a.25.25 0 0 0-.25-.25H7.5c-.55 0-1.07-.26-1.4-.7l-.9-1.2a.25.25 0 0 0-.2-.1Z"/>
                    </svg>
                }
                <span class="file-tree-name">{ &props.name }</span>
            </div>
            if *is_open {
                <div class="file-tree-children">
                    { for props.children.iter().map(|n| render_node(n, props.depth + 1)) }
                </div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct FileNodeProps {
    depth: usize,
    name:  String
}

fn file_icon(name: &str) -> Html {
    let ext = name.rsplit('.').next().unwrap_or("");
    match ext {
        "rs" => html! {
            <svg class="file-tree-icon" viewBox="0 0 16 16" width="16" height="16" fill="currentColor">
                <path d="M4 1.75C4 .784 4.784 0 5.75 0h5.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v8.586A1.75 1.75 0 0 1 14.25 15h-9a.75.75 0 0 1 0-1.5h9a.25.25 0 0 0 .25-.25V6h-2.75A1.75 1.75 0 0 1 10 4.25V1.5H5.75a.25.25 0 0 0-.25.25v2.5a.75.75 0 0 1-1.5 0Zm1.72 4.97a.75.75 0 0 1 1.06 0l2 2a.75.75 0 0 1 0 1.06l-2 2a.749.749 0 0 1-1.275-.326.749.749 0 0 1 .215-.734l1.47-1.47-1.47-1.47a.75.75 0 0 1 0-1.06ZM3.28 7.78 1.81 9.25l1.47 1.47a.751.751 0 0 1-.018 1.042.751.751 0 0 1-1.042.018l-2-2a.75.75 0 0 1 0-1.06l2-2a.751.751 0 0 1 1.042.018.751.751 0 0 1 .018 1.042Zm8.22-6.218V4.25c0 .138.112.25.25.25h2.688l-.011-.013-2.914-2.914-.013-.011Z"/>
            </svg>
        },
        "toml" => html! {
            <svg class="file-tree-icon" viewBox="0 0 16 16" width="16" height="16" fill="currentColor">
                <path d="M0 2.75C0 1.784.784 1 1.75 1H5c.55 0 1.07.26 1.4.7l.9 1.2a.25.25 0 0 0 .2.1h6.75c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0 1 14.25 15H1.75A1.75 1.75 0 0 1 0 13.25Zm9.42 9.36 2.883-2.677a.25.25 0 0 0 0-.366L9.42 6.39a.249.249 0 0 0-.42.183V8.5H4.75a.75.75 0 0 0 0 1.5H9v1.927c0 .218.26.331.42.183Z"/>
            </svg>
        },
        "html" => html! {
            <svg class="file-tree-icon" viewBox="0 0 16 16" width="16" height="16" fill="currentColor">
                <path d="M0 2.75C0 1.784.784 1 1.75 1h12.5c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0 1 14.25 15H1.75A1.75 1.75 0 0 1 0 13.25ZM14.5 6h-13v7.25c0 .138.112.25.25.25h12.5a.25.25 0 0 0 .25-.25Zm-6-3.5v2h6V2.75a.25.25 0 0 0-.25-.25ZM5 2.5v2h2v-2Zm-3.25 0a.25.25 0 0 0-.25.25V4.5h2v-2Z"/>
            </svg>
        },
        "css" | "scss" => html! {
            <svg class="file-tree-icon" viewBox="0 0 16 16" width="16" height="16" fill="currentColor">
                <path d="M11.134 1.535c.7-.509 1.416-.942 2.076-1.155.649-.21 1.463-.267 2.069.34.603.601.568 1.411.368 2.07-.202.668-.624 1.39-1.125 2.096-1.011 1.424-2.496 2.987-3.775 4.249-1.098 1.084-2.132 1.839-3.04 2.3a3.744 3.744 0 0 1-1.055 3.217c-.431.431-1.065.691-1.657.861-.614.177-1.294.287-1.914.357A21.151 21.151 0 0 1 .797 16H.743l.007-.75H.749L.742 16a.75.75 0 0 1-.743-.742l.743-.008-.742.007v-.054a21.25 21.25 0 0 1 .13-2.284c.067-.647.187-1.287.358-1.914.17-.591.43-1.226.86-1.657a3.746 3.746 0 0 1 3.227-1.054c.466-.893 1.225-1.907 2.314-2.982 1.271-1.255 2.833-2.75 4.245-3.777ZM1.62 13.089c-.051.464-.086.929-.104 1.395.466-.018.932-.053 1.396-.104a10.511 10.511 0 0 0 1.668-.309c.526-.151.856-.325 1.011-.48a2.25 2.25 0 1 0-3.182-3.182c-.155.155-.329.485-.48 1.01a10.515 10.515 0 0 0-.309 1.67Zm10.396-10.34c-1.224.89-2.605 2.189-3.822 3.384l1.718 1.718c1.21-1.205 2.51-2.597 3.387-3.833.47-.662.78-1.227.912-1.662.134-.444.032-.551.009-.575h-.001V1.78c-.014-.014-.113-.113-.548.027-.432.14-.995.462-1.655.942Zm-4.832 7.266-.001.001a9.859 9.859 0 0 0 1.63-1.142L7.155 7.216a9.7 9.7 0 0 0-1.161 1.607c.482.302.889.71 1.19 1.192Z"/>
            </svg>
        },
        "md" => html! {
            <svg class="file-tree-icon" viewBox="0 0 16 16" width="16" height="16" fill="currentColor">
                <path d="M1.75 2h12.5c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0 1 14.25 14H1.75A1.75 1.75 0 0 1 0 12.25v-8.5C0 2.784.784 2 1.75 2ZM1.5 12.251c0 .138.112.25.25.25h12.5a.25.25 0 0 0 .25-.25V3.75a.25.25 0 0 0-.25-.25H1.75a.25.25 0 0 0-.25.25v8.501ZM3 5.25a.75.75 0 0 1 .75-.75h1.5a.75.75 0 0 1 .75.75v5.5a.75.75 0 0 1-.75.75h-1.5a.75.75 0 0 1-.75-.75v-5.5Zm4 0a.75.75 0 0 1 .75-.75h1.5a.75.75 0 0 1 .75.75v2.5a.75.75 0 0 1-.75.75h-1.5a.75.75 0 0 1-.75-.75v-2.5Z"/>
            </svg>
        },
        _ => html! {
            <svg class="file-tree-icon" viewBox="0 0 16 16" width="16" height="16" fill="currentColor">
                <path d="M3.75 1.5a.25.25 0 0 0-.25.25v12.5c0 .138.112.25.25.25h8.5a.25.25 0 0 0 .25-.25V4.664a.25.25 0 0 0-.073-.177l-4.5-4.5a.25.25 0 0 0-.177-.073H3.75ZM1 1.75C1 .784 1.784 0 2.75 0h5.5c.331 0 .649.132.884.367l4.5 4.5c.235.235.367.553.367.884V14.25c0 .966-.784 1.75-1.75 1.75h-8.5A1.75 1.75 0 0 1 1 14.25Z"/>
            </svg>
        }
    }
}

#[function_component]
fn FileNode(props: &FileNodeProps) -> Html {
    let indent = (props.depth as f64) * 1.25;
    html! {
        <div
            class="file-tree-item file-tree-file"
            style={format!("padding-left: {}rem", indent)}
        >
            { file_icon(&props.name) }
            <span class="file-tree-name">{ &props.name }</span>
        </div>
    }
}
