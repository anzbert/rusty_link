cmake_minimum_required(VERSION 3.14)

# Build Link with the official C-Extension as a static library called 'lib_abl_link'
project(link-with-c-extension)

add_library(lib_abl_link STATIC ../link/extensions/abl_link/src/abl_link.cpp)

include(../link/AbletonLinkConfig.cmake)
include(../link/extensions/abl_link/abl_link.cmake)

target_link_libraries(lib_abl_link abl_link)

# CMAKE_INSTALL_PREFIX in CMake is referencing the same directory as $OUT_DIR in Cargo / Rust.
# This copies the compiled library to '$OUT_DIR/lib':
install(TARGETS lib_abl_link 
        DESTINATION "${CMAKE_INSTALL_PREFIX}/lib")