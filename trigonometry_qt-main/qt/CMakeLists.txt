cmake_minimum_required(VERSION 3.5)

project(TrigonometryQT)

set(CMAKE_CXX_STANDARD 11)

# ✅ Aktifkan fitur auto .ui, .moc dan .qrc
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

# ✅ Ganti path ini sesuai Qt kamu (contoh default bawaan Qt Installer Linux)
set(CMAKE_PREFIX_PATH "/opt/Qt/5.15.2/gcc_64/lib/cmake")

# ✅ Temukan Qt5 Widgets
find_package(Qt5 REQUIRED COMPONENTS Widgets)

# ✅ Include dir
include_directories(${Qt5Widgets_INCLUDE_DIRS})
include_directories(lib)

# ✅ Set path library Rust
set(RUST_LIBRARY_PATH "${CMAKE_SOURCE_DIR}/../target/release/libtrigonometry.so")

qt5_add_resources(RESOURCES resources.qrc)

# ✅ Tambahkan executable + file .ui
add_executable(TrigonometryQT
    main.cpp
    mainwindow.cpp
    mainwindow.h
    mainwindow.ui
    ${RESOURCES}
)

# ✅ Copy library Rust ke build directory
add_custom_command(TARGET TrigonometryQT POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
    "${RUST_LIBRARY_PATH}"
    $<TARGET_FILE_DIR:TrigonometryQT>
)

# ✅ Link ke Qt5
target_link_libraries(TrigonometryQT
    Qt5::Widgets
)

# ✅ Set RPATH agar aplikasi bisa menemukan library di folder yang sama
set(CMAKE_INSTALL_RPATH "$ORIGIN")