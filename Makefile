
LIBOVR_INCLUDE_PATH=OculusSDK/LibOVR/Include/
LIBOVR_LIB_PATH=OculusSDK/LibOVR/Lib/Linux/Release/x86_64/libovr.a

all: libovr_wrapper.a

libovr_wrapper.a: wrapper.o
	ar rcs libovr_wrapper.a wrapper.o

wrapper.o: wrapper.cpp
	c++ -fPIC -I $(LIBOVR_INCLUDE_PATH) -c -o wrapper.o wrapper.cpp
