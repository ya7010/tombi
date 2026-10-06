use tombi_diagnostic::Level;
use tombi_linter::test_lint;
use tombi_test_lib::pyproject_schema_path;

test_lint! {
    #[test]
    fn readme_relative_path(
        r#"[project]
name = "example"
version = "1.0"
readme = "docs/README.md"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn readme_absolute_path(
        r#"[project]
name = "example"
version = "1.0"
readme = "/README.md"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "not-schema-match", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn readme_file_with_content_type(
        r#"[project]
name = "example"
version = "1.0"
readme = { file = "README.txt", content-type = "text/plain" }
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn readme_absolute_file(
        r#"[project]
name = "example"
version = "1.0"
readme = { file = "/README.md", content-type = "text/markdown" }
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "not-schema-match", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn readme_file_missing_content_type(
        r#"[project]
name = "example"
version = "1.0"
readme = { file = "README.md" }
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "table-key-required", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn readme_text_with_content_type(
        r#"[project]
name = "example"
version = "1.0"
readme = { text = "Example", content-type = "text/plain" }
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn readme_text_missing_content_type(
        r#"[project]
name = "example"
version = "1.0"
readme = { text = "Example" }
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "table-key-required", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn readme_content_type_only(
        r#"[project]
name = "example"
version = "1.0"
readme = { content-type = "text/plain" }
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn readme_posix_relative_drive_filename(
        r#"[project]
name = "example"
version = "1.0"
readme = 'C:\README.md'
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn authors_and_maintainers_names(
        r#"[project]
name = "example"
version = "1.0"
authors = [{name = "Example Person"}]
maintainers = [{name = "Example Organization"}]
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn author_name_with_comma(
        r#"[project]
name = "example"
version = "1.0"
authors = [{name = "Person, Example"}]
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "string-pattern", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn maintainer_name_with_comma(
        r#"[project]
name = "example"
version = "1.0"
maintainers = [{name = "Person, Example"}]
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "string-pattern", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn urls_label_32(
        r#"[project]
name = "example"
version = "1.0"
[project.urls]
"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" = "https://example.com"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn urls_label_33(
        r#"[project]
name = "example"
version = "1.0"
[project.urls]
"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" = "https://example.com"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "string-max-length", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn urls_supplementary_unicode_32(
        r#"[project]
name = "example"
version = "1.0"
[project.urls]
"😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀" = "https://example.com"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn urls_combining_unicode_33(
        r#"[project]
name = "example"
version = "1.0"
[project.urls]
"ááááááááááááááááa" = "https://example.com"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "string-max-length", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn scripts_non_recommended_name(
        r#"[project]
name = "example"
version = "1.0"
[project.scripts]
"!tool with space" = "example:main [legacy-extra]"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn scripts_name_equals(
        r#"[project]
name = "example"
version = "1.0"
[project.scripts]
"tool=bad" = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "not-schema-match", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn scripts_name_bracket(
        r#"[project]
name = "example"
version = "1.0"
[project.scripts]
"[tool" = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "not-schema-match", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn scripts_name_leading_space(
        r#"[project]
name = "example"
version = "1.0"
[project.scripts]
" tool" = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "not-schema-match", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn scripts_name_trailing_space(
        r#"[project]
name = "example"
version = "1.0"
[project.scripts]
"tool " = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "not-schema-match", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn gui_scripts_non_recommended_name(
        r#"[project]
name = "example"
version = "1.0"
[project.gui-scripts]
"!tool with space" = "example:main [legacy-extra]"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn gui_scripts_name_equals(
        r#"[project]
name = "example"
version = "1.0"
[project.gui-scripts]
"tool=bad" = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "not-schema-match", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn gui_scripts_name_bracket(
        r#"[project]
name = "example"
version = "1.0"
[project.gui-scripts]
"[tool" = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "not-schema-match", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn gui_scripts_name_leading_space(
        r#"[project]
name = "example"
version = "1.0"
[project.gui-scripts]
" tool" = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "not-schema-match", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn gui_scripts_name_trailing_space(
        r#"[project]
name = "example"
version = "1.0"
[project.gui-scripts]
"tool " = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "not-schema-match", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn entry_points_example_plugins_non_recommended_name(
        r#"[project]
name = "example"
version = "1.0"
[project.entry-points."example.plugins"]
"!tool with space" = "example:main [legacy-extra]"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn entry_points_example_plugins_name_equals(
        r#"[project]
name = "example"
version = "1.0"
[project.entry-points."example.plugins"]
"tool=bad" = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "not-schema-match", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn entry_points_example_plugins_name_bracket(
        r#"[project]
name = "example"
version = "1.0"
[project.entry-points."example.plugins"]
"[tool" = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "not-schema-match", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn entry_points_example_plugins_name_leading_space(
        r#"[project]
name = "example"
version = "1.0"
[project.entry-points."example.plugins"]
" tool" = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "not-schema-match", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn entry_points_example_plugins_name_trailing_space(
        r#"[project]
name = "example"
version = "1.0"
[project.entry-points."example.plugins"]
"tool " = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "not-schema-match", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn import_namespaces_nonempty(
        r#"[project]
name = "example"
version = "1.0"
import-namespaces = ["example.namespace"]
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn import_namespaces_empty(
        r#"[project]
name = "example"
version = "1.0"
import-namespaces = []
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "array-min-values", level: Level::ERROR }])
}

test_lint! {
    #[test]
    fn import_names_empty(
        r#"[project]
name = "example"
version = "1.0"
import-names = []
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn import_names_missing_parents(
        r#"[project]
name = "example"
version = "1.0"
import-names = ["example.module"]
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn scripts_empty_name_has_only_existing_warning(
        r#"[project]
name = "example"
version = "1.0"
[project.scripts]
"" = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "key-empty", level: Level::WARNING }])
}

test_lint! {
    #[test]
    fn gui_scripts_empty_name_has_only_existing_warning(
        r#"[project]
name = "example"
version = "1.0"
[project.gui-scripts]
"" = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "key-empty", level: Level::WARNING }])
}

test_lint! {
    #[test]
    fn entry_points_example_plugins_empty_name_has_only_existing_warning(
        r#"[project]
name = "example"
version = "1.0"
[project.entry-points."example.plugins"]
"" = "example:main"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Diagnostics([{ code: "key-empty", level: Level::WARNING }])
}

test_lint! {
    #[test]
    fn readme_relative_parent_path(
        r#"[project]
name = "example"
version = "1.0"
readme = "../README.md"
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn readme_relative_parent_file(
        r#"[project]
name = "example"
version = "1.0"
readme = { file = "../README.md", content-type = "text/markdown" }
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}

test_lint! {
    #[test]
    fn omitted_project_table(
        r#"[tool.example]
enabled = true
"#,
        SchemaPath(pyproject_schema_path()),
    ) -> Ok(_)
}
