IMG_BUILT=$(docker ps -a | grep 'rusty_dbg_suleif' | wc -l)
if [ $IMG_BUILT -eq 0 ]; then
	docker build . -t rusty_dbg_suleif
fi	
docker run -v .:/app rusty_dbg_suleif cargo build
cp ./target/debug/simple-dbg ./simple-dbg
rm -rf ./Cargo.lock ./target
