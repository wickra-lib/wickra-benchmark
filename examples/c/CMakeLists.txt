# Build and run the C and C++ examples against the wickra-benchmark C ABI.
#
#   cargo build --release -p wickra-benchmark-c
#   cmake -S examples/c -B examples/c/build
#   cmake --build examples/c/build --config Release
#   ctest --test-dir examples/c/build -C Release --output-on-failure
#
# WICKRA_BENCHMARK_LIB_DIR defaults to the workspace target/release directory and
# can be overridden for out-of-tree builds.

cmake_minimum_required(VERSION 3.15)
project(wickra_benchmark_examples C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(WICKRA_BENCHMARK_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../target/release"
    CACHE PATH "Directory containing the built wickra-benchmark C ABI library")

set(WICKRA_BENCHMARK_INCLUDE "${CMAKE_CURRENT_SOURCE_DIR}/../../bindings/c/include")
set(WICKRA_BENCHMARK_DATA "${CMAKE_CURRENT_SOURCE_DIR}/../data")

find_library(WICKRA_BENCHMARK_LIB
    NAMES wickra_benchmark libwickra_benchmark
    HINTS "${WICKRA_BENCHMARK_LIB_DIR}"
    REQUIRED)

add_executable(run run.c)
target_include_directories(run PRIVATE "${WICKRA_BENCHMARK_INCLUDE}")
target_link_libraries(run PRIVATE "${WICKRA_BENCHMARK_LIB}")
target_compile_definitions(run PRIVATE DATA_DIR="${WICKRA_BENCHMARK_DATA}")

add_executable(runpp run.cpp)
target_include_directories(runpp PRIVATE "${WICKRA_BENCHMARK_INCLUDE}")
target_link_libraries(runpp PRIVATE "${WICKRA_BENCHMARK_LIB}")
target_compile_definitions(runpp PRIVATE DATA_DIR="${WICKRA_BENCHMARK_DATA}")

# On Windows the runtime DLL must sit next to the executable (there is no rpath);
# without it the loader pops a modal "DLL not found" dialog that hangs ctest. Copy
# it next to each example after build. (Unix resolves the .so/.dylib via the
# linker's embedded path, so this is Windows-only.)
if(WIN32)
    foreach(example run runpp)
        add_custom_command(TARGET ${example} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "${WICKRA_BENCHMARK_LIB_DIR}/wickra_benchmark.dll"
                "$<TARGET_FILE_DIR:${example}>"
            VERBATIM)
    endforeach()
endif()

enable_testing()
add_test(NAME run COMMAND run)
add_test(NAME runpp COMMAND runpp)
# A missing runtime dependency must fail fast, never block the runner for hours.
set_tests_properties(run runpp PROPERTIES TIMEOUT 60)
