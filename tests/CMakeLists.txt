cmake_minimum_required(VERSION 3.11)

project(Tests)

set(CMAKE_CXX_STANDARD 17)

include(ClangFormat)
include(FetchContent)
include(GoogleTest)

FetchContent_Declare(
  googletest
  GIT_REPOSITORY https://github.com/google/googletest.git
  GIT_TAG        release-1.10.0
)

FetchContent_GetProperties(googletest)
if(NOT googletest_POPULATED)
  set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
  FetchContent_Populate(googletest)
  add_subdirectory(${googletest_SOURCE_DIR} ${googletest_BINARY_DIR})
endif()

set( Sources
    TaskManagerTests.cpp
)

add_executable(${PROJECT_NAME} ${Sources})
target_link_libraries(${PROJECT_NAME} gtest_main TaskManager_lib)

gtest_add_tests(TARGET ${PROJECT_NAME})

target_clangformat_setup(${PROJECT_NAME})