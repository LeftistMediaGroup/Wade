
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 80,
                        patch: 1,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "x86_64-unknown-linux-gnu".to_owned(),
                    short_version_string: "rustc 1.80.1 (3f5fd8dd4 2024-08-06)".to_owned(),
                    commit_hash: Some("3f5fd8dd41153bc5fdca9427e9e05be2c767ba23".to_owned()),
                    commit_date: Some("2024-08-06".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            