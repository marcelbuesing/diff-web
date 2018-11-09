///
/// Structure of Diff.json
///
struct Diff {
    real_size: String,
    size: u64,
    branch_name: String,
    target_branch_name: String,
    commit: String,
    merge_request_with: MergeRequestDiff,
    start_version: String,
    latest_diff: bool,
    latest_version_path: String,
    added_lines: u64,
    removed_lines: u64,
    render_overflow_warning: bool,
    email_patch_path: String,
    plain_diff_path: String,
    diff_files: Vec<DiffFile>,
    merge_request_diffs: Vec<MergeRequestDiff>,
}

struct Blob {
    id: String,
    path: String,
    name: String,
    mode: String,
    readable_text: bool,
    icon: String,
}

struct DiffRefs {
    base_sha: String,
    start_sha: String,
    head_sha: String,
}

struct DiffFile {
    submodule: bool,
    submodule_link: String,
    submodule_tree_url: String,
    blob: Blob
    can_modify_blob: bool,
    file_hash: String,
    file_path: String,
    too_large: bool,
    collapsed: bool,
    new_file: bool,
    deleted_file: bool,
    renamed_file: bool,
    old_path: String,
    new_path: String,
    mode_changed: bool,
    a_mode: String,
    b_mode: String,
    text: bool,
    added_lines: u64,
    removed_lines: u64,
    diff_refs	{…}
    content_sha: String,
    stored_externally: String,
    external_storage: String,
    load_collapsed_diff_url: String,
    old_path_html: Vec<String>,
    new_path_html: String,
    edit_path: String,
    view_path: String,
    replaced_view_path: String,
    context_lines_path: String,
    highlighted_diff_lines	[…]
    parallel_diff_lines	[…]
}

struct MergeRequestDiff {
    version_index: String,
    created_at: String,
    commits_count: u64,
    latest: bool,
    short_commit_sha: String,
    version_path: String,
    compare_path: String,
}