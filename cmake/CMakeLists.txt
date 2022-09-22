cmake_minimum_required(VERSION 3.0)

# Build Link with the official C-Extension as a static library called 'lib_abl_link_c'
project(link-with-c-extension)

add_library(lib_abl_link_c STATIC ../link/extensions/abl_link/src/abl_link.cpp)

include(../link/AbletonLinkConfig.cmake)
include(../link/extensions/abl_link/abl_link.cmake)

target_link_libraries(lib_abl_link_c abl_link)