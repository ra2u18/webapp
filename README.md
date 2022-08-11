We are using the bunyan cli to prettify the outputted logs
`cargo install bunyan`
When you want to see all logs coming out of a certain test case to debug it and run
`TEST_LOG=true cargo test health_check_works | bunyan`