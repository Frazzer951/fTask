cmake_minimum_required(VERSION 3.11)
project(TaskManager C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

if(MSVC)
  add_compile_options(-W4)
  string(REGEX REPLACE "/W3" "/W4" CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}")
else()
  add_compile_options(-Wall -Wextra -Werror -Wdouble-promotion -Wconversion -fno-common -Wno-sign-compare)
endif()

list(APPEND CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/cmake)
include(ClangFormat)
enable_testing()

file(GLOB Sources
     "src/*.cpp"
)

file(GLOB Headers
     "include/*.hpp"
)

include_directories(include)

add_subdirectory(tests)

add_executable(${PROJECT_NAME} ${Sources} ${Headers})

add_library(${PROJECT_NAME}_lib ${Sources} ${Headers} )
target_include_directories(${PROJECT_NAME}_lib PUBLIC include)

target_clangformat_setup(${PROJECT_NAME})