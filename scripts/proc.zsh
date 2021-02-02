#!/usr/bin/env zsh

# build binary for lambda environ
printf "building.."

{
	cargo build --release \
	--target x86_64-unknown-linux-musl &> /dev/null
} || {
	exit
}

printf "done\n"

# copy binary release to project root
printf "copying.."

{
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap . \
	&> /dev/null
} || {
	exit
}

printf "done\n"

# zip binary to prepare for lambda
printf "zipping.."

{
	zip lambda-rs.zip bootstrap \
	&> /dev/null
} || {
	exit
}
printf "done\n"

# upload to lambda function
printf "deploying.."

{
	aws lambda update-function-code \
	--function-name lambda-rs \
	--zip-file fileb://lambda-rs.zip &> /dev/null
} || {
	exit
}

wait

printf "done\n"

# clean up
printf "cleaning up.."

{
	rm bootstrap &>/dev/null
	rm lambda-rs.zip &> /dev/null
} || {
	exit
}

printf "done\n"
