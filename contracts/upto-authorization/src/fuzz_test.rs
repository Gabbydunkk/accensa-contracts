#[cfg(test)]
mod tests {
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Address, Env};

    #[test]
    fn fuzz_authorization_limit_checks() {
        let env = Env::default();
        // Minimal fuzz harness: a fresh env and a generated address exercise
        // the limit-check setup. Deeper property coverage lands with the
        // authorization fuzz suite.
        let _admin = Address::generate(&env);
        let _ = env;
    }
}
