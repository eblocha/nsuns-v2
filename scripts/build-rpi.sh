(cd ./nsuns-client && yarn build) && rm -r ./dist/static && cp -r ./nsuns-client/dist ./dist/static

cargo build --release --target=aarch64-unknown-linux-gnu

cp target/aarch64-unknown-linux-gnu/release/nsuns-server ./dist
cp -r nsuns-server/config ./dist
cp -r nsuns-server/db ./dist
