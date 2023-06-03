pub enum EcdsaKeyEnvs {
    LocalDevelopment,
    Test,
    Production,
}

impl EcdsaKeyEnvs {
    pub fn to_key_name(&self) -> String {
        match self {
            EcdsaKeyEnvs::LocalDevelopment => "dfx_test_key",
            EcdsaKeyEnvs::Test => "test_key_1",
            EcdsaKeyEnvs::Production => "key_1",
        }
        .to_string()
    }
}
