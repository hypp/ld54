
release:
	trunk build --public-url ld54

clean:
	trunk clean

cleanall:
	cargo clean

.PHONY: release clean cleanall
