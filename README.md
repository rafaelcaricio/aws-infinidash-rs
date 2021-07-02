# AWS Infinidash client

Fully featured AWS Infinidash client for Rust applications. You can use the AWS Infinidash client to make AWS API calls, and
to get your application to scale on the AWS infrastructure automagically. AWS Infinidash leverages the blockchain to provide the
ultimate security and scalability. The decentrality of the blockchain ensures that no single point of failure can be used to
access the application. Using the Infinidash client, you can scale your application to any available AWS region, and you can
scale your application to any available AWS account. Using assimetric cryptography, the Infinidash client can be used to
encrypt data between any two AWS regions with no need for any shared secret.


# Usage

AWS Infinidash client expects a JSON configuration file
to be present in the working directory. The configuration file should be named `aws-infinidash.json` and it should be in the
same directory as the executable.

Make sure to check out the [example configuration file](https://github.com/infinidash/aws-infinidash/blob/master/aws-infinidash.json).

Credentials are read from environment variables.

# Contributing

Open a pull request with your changes. Make sure to add tests for your changes. Only PRs with passing tests are accepted.
