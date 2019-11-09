mod helpers;

use helpers::Playground;
use std::path::PathBuf;

#[test]
fn filesystem_change_from_current_directory_using_relative_path() {
    Playground::setup("cd_test_1", |dirs, _| {
        let actual = nu!(
            cwd: dirs.root(),
            r#"
                cd cd_test_1
                ls && pwd | echo $it
            "#
        );

        dbg!(&actual);

        assert_eq!(PathBuf::from(actual), *dirs.test());
    })
}