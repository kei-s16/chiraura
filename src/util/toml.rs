use super::super::types::Profile;

pub fn parse_toml_to_config(text: String) -> Result<Profile, String> {
    // TODO: エラーハンドリングちゃんとする
    let profile: Profile = toml::from_str(&text).expect("something went wrong");

    Ok(profile)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_return_profile_when_input_valid_format_toml() {
        let test_string = r#"
        version = "0.0.1"
        author = "kei-s16"
        domain = "k16em.net"
        description = "neovimとかarchlinuxとか使って遊んでます"

        [[website]]
        title = "blog"
        url = "https://blog.k16em.net"
        description = "比較的ちゃんと書いてるブログ"

        [[software]]
        title = "techblog-linter-settings"
        url = "https://github.com/kei-s16/techblog-linter-settings"
        description =  "技術ブログとかやるときに日本語lintする設定"

        [[software]]
        title = "dbot-dice"
        url = "https://github.com/kei-s16/dbot-dice"
        description =  "Discordのdicebotくん"
        "#;

        let _result = parse_toml_to_config(test_string.to_string()).unwrap();

        assert_eq!(_result.version, "0.0.1");
        assert_eq!(_result.website.len(), 1);
        assert_eq!(_result.software.len(), 2);
        assert_eq!(_result.website[0].title, "blog");
        assert_eq!(_result.website[0].url, "https://blog.k16em.net");
        assert_eq!(_result.software[0].title, "techblog-linter-settings");
        assert_eq!(_result.software[0].url, "https://github.com/kei-s16/techblog-linter-settings");
        assert_eq!(_result.software[0].description, "技術ブログとかやるときに日本語lintする設定");
    }

    // TODO: エラーハンドリングちゃんとする
    #[test]
    #[should_panic(expected = "something went wrong")]
    fn fail_panic_when_input_invalid_format_toml() {
        let test_string = r#"
        ip = '127.0.0.1'

        [keys]
        github = 'xxxxxxxxxxxxxxxxx'
        travis = 'yyyyyyyyyyyyyyyyy'
        "#;

        let _result = parse_toml_to_config(test_string.to_string()).unwrap();

        assert!(false, "something went wrong");
    }

    // TODO: エラーハンドリングちゃんとする
    #[test]
    #[should_panic(expected = "something went wrong")]
    fn fail_panic_when_input_table_but_expected_array_of_tables() {
        let test_string = r#"
        version = "0.0.1"

        [website]
        title = "blog"
        url = "https://blog.k16em.net"

        [[software]]
        title = "techblog-linter-settings"
        url = "https://github.com/kei-s16/techblog-linter-settings"
        description =  "技術ブログとかやるときに日本語lintする設定"
        "#;

        let _result = parse_toml_to_config(test_string.to_string()).unwrap();

        assert!(false, "something went wrong");
    }

    // TODO: エラーハンドリングちゃんとする
    #[test]
    #[should_panic(expected = "something went wrong")]
    fn fail_panic_when_missing_requirement_table() {
        let test_string = r#"
        version = "0.0.1"

        [[software]]
        title = "techblog-linter-settings"
        url = "https://github.com/kei-s16/techblog-linter-settings"
        description =  "技術ブログとかやるときに日本語lintする設定"
        "#;

        let _result = parse_toml_to_config(test_string.to_string()).unwrap();

        assert!(false, "something went wrong");
    }
}
