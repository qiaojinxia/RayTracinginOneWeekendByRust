Run1:
	cargo build
	@echo 多线程渲染图像开始...
	./target/debug/ray_tracingin_one_weekend
	chmod u+x *.sh
	./build.sh
