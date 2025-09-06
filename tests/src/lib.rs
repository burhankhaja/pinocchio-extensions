#[cfg(test)]
pub mod registry;

pub mod helpers {
    pub mod extensions {
        pub mod registry;
    }

    pub mod suite {
        pub mod core;
        pub mod solana_kite;
        pub mod types;
    }
}
