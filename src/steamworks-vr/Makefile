
LIBSTEAMVR_INCLUDE_PATH=../thirdparty/Steamworks/public/steam/
LIBSTEAMVR_LIB_PATH=../thirdparty/Steamworks/redistributable_bin/linux64/

all: libsteamvr_wrapper.a

libsteamvr_wrapper.a: wrapper.o
	ar rcs libsteamvr_wrapper.a wrapper.o

wrapper.o: wrapper.cpp
	g++ -fPIC -I $(LIBSTEAMVR_INCLUDE_PATH) -DGNUC -DSTEAMWORKS -c -o wrapper.o wrapper.cpp