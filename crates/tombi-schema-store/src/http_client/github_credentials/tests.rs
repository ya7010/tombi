use rstest::rstest;

use super::environment_token;

#[rstest]
#[case(Some("gh-token"), Some("github-token"), Some("gh-token"))]
#[case(None, Some("github-token"), Some("github-token"))]
#[case(None, None, None)]
fn selects_environment_token_in_priority_order(
    #[case] gh_token: Option<&str>,
    #[case] github_token: Option<&str>,
    #[case] expected: Option<&str>,
) {
    let selected = environment_token(|name| match name {
        "GH_TOKEN" => gh_token.map(str::to_owned),
        "GITHUB_TOKEN" => github_token.map(str::to_owned),
        _ => panic!("unexpected credential source"),
    });
    assert_eq!(selected.as_deref(), expected);
}
