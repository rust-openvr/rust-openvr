
LIBOVR_INCLUDE_PATH=../OculusSDK/LibOVR/Include/
LIBOVR_LIB_PATH=../OculusSDK/LibOVR/Lib/Linux/Release/x86_64/libovr.a


all: test

libovr_wrapper.a: wrapper.o
	ar rcs libovr_wrapper.a wrapper.o

wrapper.o: wrapper.cpp
	c++ -fPIC -I $(LIBOVR_INCLUDE_PATH) -c -o wrapper.o wrapper.cpp

libovr-rs-44316370-0.1.so: libovr_wrapper.a lib.rs
	rustc lib.rs -L .

test: test.rs libovr-rs-44316370-0.1.so
	rustc test.rs -L . --link-args="-lovr -ludev -lstdc++ -lc -lX11 -lm -lpthread -lXinerama"
