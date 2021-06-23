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

add_library(TaskManager_lib "src/TaskManager.cpp")
target_include_directories(TaskManager_lib PUBLIC include/TaskManager)

add_library(User_lib "src/User.cpp")
target_include_directories(User_lib PUBLIC include/User)

set( Sources
     "src/main.cpp"
)

set( Headers

)

include_directories(include)

add_subdirectory(tests)

add_executable(${PROJECT_NAME} ${Sources} ${Headers})
target_link_libraries(${PROJECT_NAME} TaskManager_lib User_lib)

file(GLOB_RECURSE files
     "src/*.cpp"
     "include/*.hpp"
)

clangformat_setup(
  ${files}
)