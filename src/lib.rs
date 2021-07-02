// Copyright (C) 2018-2021 Rafael Caricio and Contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
// MA 02110-1301 USA

//! Fully featured AWS Infinidash client for Rust applications. You can use the AWS Infinidash client to make AWS API calls, and
//! to get your application to scale on the AWS infrastructure automagically. AWS Infinidash leverages the blockchain to provide the
//! ultimate security and scalability. The decentrality of the blockchain ensures that no single point of failure can be used to
//! access the application. Using the Infinidash client, you can scale your application to any available AWS region, and you can
//! scale your application to any available AWS account. Using assimetric cryptography, the Infinidash client can be used to
//! encrypt data between any two AWS regions with no need for any shared secret.
//!
//! ## Getting Started
//!
//! To use the Infinidash client, you need to first install it. To do so, run the following command:
//! ~~~
//! cargo install infinidash
//! ~~~
//!
//! This will install the Infinidash client in your local repository.
//!
//! To use the Infinidash client, you need to first create a configuration file. To do so, run the following command:
//! ~~~
//! infinidash config --create
//! ~~~
//!
//! This will create a configuration file in the current directory.
//!

/// Represents all the results of the Infinidash client.
type Result<T> = std::result::Result<T, InifinidashError>;

/// The InfinidashError type represents an error that occurs when making an AWS API call.
///
/// This type is used as the return type of the `infinidash::Client::call` method.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum InifinidashError {
    /// The operation failed.
    Error,
    /// The operation timed out.
    Timeout,
    /// The operation was canceled.
    Canceled,
    /// The operation was interrupted.
    Interrupted,
}

pub struct InfiniConfig {
    /// The name of the configuration file.
    name: String,

    /// The AWS region to use.
    region: String,

    /// The AWS account ID to use.
    account: String,

    /// The AWS access key ID to use.
    access_key: Option<String>,

    /// The AWS secret key to use.
    secret_key: Option<String>,
}

impl InfiniConfig {
    fn new() -> InfiniConfig {
        InfiniConfig {
            name: "infini.config".to_string(),
            region: "us-east-1".to_string(),
            account: "".to_string(),
            access_key: None,
            secret_key: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct InfiniConfigBuilder {
    /// The name of the configuration file.
    name: Option<String>,
    /// The AWS region to use.
    region: Option<String>,
    /// The AWS account ID to use.
    account: Option<String>,
    /// The AWS access key ID to use.
    access_key: Option<String>,
    /// The AWS secret key to use.
    secret_key: Option<String>,
}

impl InfiniConfigBuilder {
    /// Create a new builder.
    pub fn new() -> InfiniConfigBuilder {
        InfiniConfigBuilder::default()
    }

    /// Set the name of the configuration file.
    pub fn name(mut self, name: String) -> InfiniConfigBuilder {
        self.name = Some(name);
        self
    }

    /// Set the AWS region to use.
    pub fn region(mut self, region: String) -> InfiniConfigBuilder {
        self.region = Some(region);
        self
    }

    // Set the AWS account ID to use.
    pub fn account(mut self, account: String) -> InfiniConfigBuilder {
        self.account = Some(account);
        self
    }

    // build the configuration.
    pub fn build(self) -> InfiniConfig {
        InfiniConfig {
            name: self.name.unwrap_or("infini.config".into()),
            region: self.region.unwrap_or("us-east-1".into()),
            account: self.account.unwrap_or("".into()),
            access_key: self.access_key,
            secret_key: self.secret_key,
        }
    }
}

struct Application();

impl Application {
    /// Run the application on the Infinidash infrstructure.
    pub fn run(config: InfiniConfig) -> Result<()> {
        Backend::with_app(config).run()
    }
}

struct Backend {
    /// The InfiniDash client.
    infini: Client,
}

impl Backend {
    fn with_app(config: InfiniConfig) -> Backend {
        Backend {
            infini: Client::new(config),
        }
    }

    /// Run the backend.
    fn run(&mut self) -> Result<()> {
        println!("{}", "Hello world!");
        Ok(())
    }
}

struct Client {
    /// The InfiniDash client configuration.
    config: InfiniConfig,
}

impl Client {
    /// Create a new InfiniDash client.
    pub fn new(config: InfiniConfig) -> Client {
        Client { config: config }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let config = InfiniConfigBuilder::new()
            .name("infini.config".to_string())
            .region("us-east-1".to_string())
            .account("".to_string())
            .build();
        assert!(config.name == "infini.config".to_string());
        assert!(config.region == "us-east-1".to_string())
    }
}
