cmake_minimum_required(VERSION 3.16)

project(magisk_native C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS ON)

option(RAFAELIA_ENABLE_LZ4 "Enable LZ4 ramdisk support" ON)
option(RAFAELIA_ENABLE_LZMA "Enable XZ/LZMA ramdisk support" ON)

add_library(bootimg STATIC
    bootimg/bootimg_parser.c
)

target_include_directories(bootimg PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/bootimg
)

add_library(cpio STATIC
    cpio/cpio_parser.c
)

target_include_directories(cpio PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/cpio
)

add_library(ramdisk STATIC
    ramdisk/ramdisk.c
)

target_include_directories(ramdisk PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/ramdisk
)

add_library(repack STATIC
    repack/repack.c
)

target_include_directories(repack PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/repack
)

add_library(raf STATIC
    raf/raf_utils.c
)

target_include_directories(raf PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/raf
)

add_executable(magiskboot
    magiskboot/magiskboot.c
    magiskboot/unpack.c
)

target_include_directories(magiskboot PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/magiskboot
    ${CMAKE_CURRENT_SOURCE_DIR}/bootimg
    ${CMAKE_CURRENT_SOURCE_DIR}/cpio
    ${CMAKE_CURRENT_SOURCE_DIR}/ramdisk
    ${CMAKE_CURRENT_SOURCE_DIR}/repack
    ${CMAKE_CURRENT_SOURCE_DIR}/raf
)

target_link_libraries(magiskboot PRIVATE
    bootimg
    cpio
    ramdisk
    repack
    raf
)

# ---------------- RAFAELIA BEGIN ----------------
find_package(OpenSSL REQUIRED)
find_package(ZLIB REQUIRED)

if(RAFAELIA_ENABLE_LZMA)
    find_library(LZMA_LIB NAMES lzma)
endif()

if(RAFAELIA_ENABLE_LZ4)
    find_library(LZ4_LIB NAMES lz4 lz4frame)
endif()

if(ANDROID)
    if(NOT DEFINED ANDROID_ABI OR ANDROID_ABI STREQUAL "")
        message(WARNING "ANDROID build without ANDROID_ABI; skipping prebuilt fallback detection")
    else()
        set(PREBUILT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/prebuilt/${ANDROID_ABI}")

        if(RAFAELIA_ENABLE_LZMA AND NOT LZMA_LIB)
            set(PREBUILT_LZMA "${PREBUILT_DIR}/liblzma.a")
            if(EXISTS "${PREBUILT_LZMA}")
                set(LZMA_LIB "${PREBUILT_LZMA}")
                message(STATUS "Using prebuilt liblzma: ${PREBUILT_LZMA}")
            else()
                message(FATAL_ERROR "liblzma missing for ABI ${ANDROID_ABI}. Provide ${PREBUILT_LZMA} or disable RAFAELIA_ENABLE_LZMA.")
            endif()
        endif()

        if(RAFAELIA_ENABLE_LZ4 AND NOT LZ4_LIB)
            set(PREBUILT_LZ4 "${PREBUILT_DIR}/liblz4.a")
            if(EXISTS "${PREBUILT_LZ4}")
                set(LZ4_LIB "${PREBUILT_LZ4}")
                message(STATUS "Using prebuilt liblz4: ${PREBUILT_LZ4}")
            else()
                message(FATAL_ERROR "liblz4 missing for ABI ${ANDROID_ABI}. Provide ${PREBUILT_LZ4} or disable RAFAELIA_ENABLE_LZ4.")
            endif()
        endif()
    endif()
endif()

if(RAFAELIA_ENABLE_LZMA AND NOT LZMA_LIB)
    message(WARNING "liblzma not found; continuing without LZMA linkage")
endif()

if(RAFAELIA_ENABLE_LZ4 AND NOT LZ4_LIB)
    message(WARNING "liblz4 not found; continuing without LZ4 linkage")
endif()

if(TARGET magiskboot)
    target_link_libraries(magiskboot PRIVATE ZLIB::ZLIB OpenSSL::Crypto)

    if(RAFAELIA_ENABLE_LZMA AND LZMA_LIB)
        target_link_libraries(magiskboot PRIVATE ${LZMA_LIB})
    endif()

    if(RAFAELIA_ENABLE_LZ4 AND LZ4_LIB)
        target_link_libraries(magiskboot PRIVATE ${LZ4_LIB})
    endif()
endif()
# ---------------- RAFAELIA END ----------------
