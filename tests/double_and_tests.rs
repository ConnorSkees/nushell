mod helpers;

use helpers::Playground;
use std::path::PathBuf;

#[test]
fn cd_in_and_out_and_in() {
    Playground::setup("double_and_test_1", |dirs, _| {
        let actual = nu!(
            cwd: dirs.root(),
            r#"
                cd double_and_test_1 && cd .. && cd double_and_test_1
                pwd | echo $it
            "#
        );
        assert_eq!(PathBuf::from(actual), *dirs.test());
    })
}

#[test]
fn ls_and_pwd_pipe() {
    Playground::setup("double_and_test_1", |dirs, _| {
        let actual = nu!(
            cwd: dirs.root(),
            r#"
                cd double_and_test_2
                ls && pwd | echo $it
            "#
        );
        assert_eq!(PathBuf::from(actual), *dirs.test());
    })
}