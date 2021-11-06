Run1:
	cargo build
	./target/debug/ray_tracingin_one_weekend >> ./image.ppm
	open  ./image.ppm
