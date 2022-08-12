We are using the bunyan cli to prettify the outputted logs
`cargo install bunyan`
When you want to see all logs coming out of a certain test case to debug it and run
`TEST_LOG=true cargo test health_check_works | bunyan`

## Configuration
Let's take a look at a more refined approach:
*) A base config file, for values that are shared across our local and production env (eg. database name)
*) A collection of env-specific config files, specifying values for fields that require customisation on a per-env basis
*) An env variable, APP_ENVIRONMENT to determine the running env (production or local)