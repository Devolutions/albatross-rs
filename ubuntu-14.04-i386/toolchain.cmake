set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR i686)

set(SYSROOT_NAME "ubuntu-14.04-i386")

execute_process(COMMAND llvm-config --prefix
	OUTPUT_VARIABLE ALBATROSS_LLVM_PREFIX
	OUTPUT_STRIP_TRAILING_WHITESPACE)

set(ALBATROSS_HOME "$ENV{ALBATROSS_HOME}" CACHE STRING "" FORCE)
set(CROSS_LLVM_PREFIX "${ALBATROSS_LLVM_PREFIX}" CACHE STRING "" FORCE)
set(CMAKE_SYSROOT "${ALBATROSS_HOME}/sysroot/${SYSROOT_NAME}" CACHE STRING "" FORCE)

set(CMAKE_CROSS_COMPILING TRUE)
set(CROSS_TARGET "${CMAKE_SYSTEM_PROCESSOR}-linux-gnu")

set(SYSROOT_LIBGCC_BASE_DIR "${CMAKE_SYSROOT}/usr/lib/gcc/${CROSS_TARGET}")
file(GLOB CROSS_GCC_VERSIONS RELATIVE "${SYSROOT_LIBGCC_BASE_DIR}" "${SYSROOT_LIBGCC_BASE_DIR}/*")
list(GET CROSS_GCC_VERSIONS 0 CROSS_GCC_VERSION)
set(CROSS_GCC_BASEVER "${CROSS_GCC_VERSION}")

set(CMAKE_C_COMPILER clang)
set(CMAKE_C_COMPILER_TARGET ${CROSS_TARGET})
set(CMAKE_CXX_COMPILER clang++)
set(CMAKE_CXX_COMPILER_TARGET ${CROSS_TARGET})
set(CMAKE_AR ar CACHE FILEPATH "" FORCE)

set(CROSS_LIBSTDCPP_INC_DIR "/usr/include/c++/${CROSS_GCC_BASEVER}")
set(CROSS_LIBSTDCPPBITS_INC_DIR "${CROSS_LIBSTDCPP_INC_DIR}/${CROSS_TARGET}")
set(CROSS_LIBGCC_DIR "/usr/lib/gcc/${CROSS_TARGET}/${CROSS_GCC_BASEVER}")
set(CROSS_COMPILER_FLAGS "")
set(CROSS_LINKER_FLAGS "-fuse-ld=lld -stdlib=libstdc++ -L \"${CROSS_LIBGCC_DIR}\"")
set(CMAKE_AR "${CROSS_LLVM_PREFIX}/bin/llvm-ar" CACHE FILEPATH "" FORCE)
set(CMAKE_NM "${CROSS_LLVM_PREFIX}/bin/llvm-nm" CACHE FILEPATH "" FORCE)
set(CMAKE_RANLIB "${CROSS_LLVM_PREFIX}/bin/llvm-ranlib" CACHE FILEPATH "" FORCE)
set(CMAKE_C_COMPILER ${CROSS_LLVM_PREFIX}/bin/clang)
set(CMAKE_C_COMPILER_TARGET ${CROSS_TARGET})
set(CMAKE_C_FLAGS "${CROSS_COMPILER_FLAGS} ${CROSS_MACHINE_FLAGS}" CACHE STRING "" FORCE)
set(CMAKE_C_ARCHIVE_CREATE "<CMAKE_AR> qc <TARGET> <OBJECTS>")
set(CMAKE_CXX_COMPILER ${CROSS_LLVM_PREFIX}/bin/clang++)
set(CMAKE_CXX_COMPILER_TARGET ${CROSS_TARGET})
set(CMAKE_CXX_FLAGS "${CROSS_COMPILER_FLAGS} ${CROSS_MACHINE_FLAGS}" CACHE STRING "" FORCE)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -iwithsysroot \"${CROSS_LIBSTDCPP_INC_DIR}\"")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -iwithsysroot \"${CROSS_LIBSTDCPPBITS_INC_DIR}\"")
set(CMAKE_CXX_ARCHIVE_CREATE "<CMAKE_AR> qc <TARGET> <OBJECTS>")
set(CMAKE_EXE_LINKER_FLAGS ${CROSS_LINKER_FLAGS} CACHE STRING "" FORCE)
set(CMAKE_MODULE_LINKER_FLAGS ${CROSS_LINKER_FLAGS} CACHE STRING "" FORCE)
set(CMAKE_SHARED_LINKER_FLAGS ${CROSS_LINKER_FLAGS} CACHE STRING "" FORCE)
set(CMAKE_STATIC_LINKER_FLAGS ${CROSS_LINKER_FLAGS} CACHE STRING "" FORCE)
set(CMAKE_FIND_ROOT_PATH ${CROSS_LLVM_PREFIX})
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)

