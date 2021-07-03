![AWS Infinidash Rust Client](https://github.com/rafaelcaricio/aws-infinidash-rs/raw/main/aws-infinidash-logo.jpeg)
# AWS Infinidash - Fully featured Rust client

Fully featured AWS Infinidash client for Rust applications. You can use the AWS Infinidash client to make AWS API calls, and
to get your application to scale on the AWS infrastructure automagically. AWS Infinidash leverages the blockchain to provide the
ultimate security and scalability. The decentrality of the blockchain ensures that no single point of failure can be used to
access the application. Using the Infinidash client, you can scale your application to any available AWS region, and you can
scale your application to any available AWS account. Using asymmetric cryptography, the Infinidash client can be used to
encrypt data between any two AWS regions with no need for any shared secret.

The client was fully developed with assistance of [GitHub Copilot](https://copilot.github.com/) and is currently in beta. Production usage
is not recommended for now, but the client is still in development. If you find any bugs, please report them on the
[issue tracker](https://github.com/rafaelcaricio/aws-infinidash-rs/issues).

# Usage

AWS Infinidash client expects a JSON configuration file
to be present in the working directory. The configuration file should be named `aws-infinidash.json` and it should be in the
same directory as the executable.

```rust

use infinidash::{Infinidash, InfinidashClient};
use web3::{
    contract::{Contract, Options},
    types::U256,
};

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
     let client = InfinidashClient::new();
     let algo = infinidash::Algorithm::AES_256_GCM_IV12_TAG16_HKDF_SHA256;
     let key = b"12345678901234567890123456789012";
     client.encrypt("Hello World".to_owned(), algo, key).unwrap();

     client.publish().expect("Failed to publish");
     let application: infinidash::Application = client.application().expect("Failed to get application");
     let app_id = application.get_application_id().unwrap();
     application.scale_to(1_345).expect("Failed to scale application");

     let actor_infra = application.get_actor_mode().expect("Failed to get actor mode");
     let available_actors = actor_infra.acquire::<Vec<aws_infinidash::Actor>>().unwrap();
     for actor in &available_actors {
         println!("{}", actor.get_id().unwrap());
     }

     let contract = Contract::deploy(web3.eth(), include_bytes!("../src/infinidash_app_desc.json"))?
        .confirmations(0)
        .options(Options::with(|opt| {
            opt.value = Some(5.into());
            opt.gas_price = Some(5.into());
            opt.gas = Some(3_000_000.into());
        }))
        .execute(
            bytecode,
            (U256::from(1_000_000_u64), "My Token".to_owned(), 3u64, "MT".to_owned()),
            my_account,
        )
        .await?;

    let contract_address = contract.address();
    println!("Contract address: {}", contract_address);

    Ok(());
}

```

Make sure to check out the [example configuration file](https://github.com/infinidash/aws-infinidash/blob/master/aws-infinidash.json).

Credentials are read from environment variables.

# Contributing

Open a pull request with your changes. Make sure to add tests for your changes. Only PRs with passing tests are accepted.
