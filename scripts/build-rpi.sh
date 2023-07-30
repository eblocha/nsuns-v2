(cd ./nsuns-client && yarn build)

gzip -k dist/assets/*.js dist/assets/*.css dist/assets/*.ico

cargo build --release --target=aarch64-unknown-linux-gnu

cp target/aarch64-unknown-linux-gnu/release/nsuns-server ./dist
cp -r nsuns-server/config ./dist
cp -r nsuns-server/db ./dist
