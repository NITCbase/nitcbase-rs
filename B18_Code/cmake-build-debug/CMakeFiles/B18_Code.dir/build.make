# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.19

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Disable VCS-based implicit rules.
% : %,v


# Disable VCS-based implicit rules.
% : RCS/%


# Disable VCS-based implicit rules.
% : RCS/%,v


# Disable VCS-based implicit rules.
% : SCCS/s.%


# Disable VCS-based implicit rules.
% : s.%


.SUFFIXES: .hpux_make_needs_suffix_list


# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /Applications/CLion.app/Contents/bin/cmake/mac/bin/cmake

# The command to remove a file.
RM = /Applications/CLion.app/Contents/bin/cmake/mac/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /Users/gokulsreekumar/NITCBase/nitcbase/B18_Code

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /Users/gokulsreekumar/NITCBase/nitcbase/B18_Code/cmake-build-debug

# Include any dependencies generated for this target.
include CMakeFiles/B18_Code.dir/depend.make

# Include the progress variables for this target.
include CMakeFiles/B18_Code.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/B18_Code.dir/flags.make

CMakeFiles/B18_Code.dir/main.cpp.o: CMakeFiles/B18_Code.dir/flags.make
CMakeFiles/B18_Code.dir/main.cpp.o: ../main.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/gokulsreekumar/NITCBase/nitcbase/B18_Code/cmake-build-debug/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/B18_Code.dir/main.cpp.o"
	/usr/bin/g++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/B18_Code.dir/main.cpp.o -c /Users/gokulsreekumar/NITCBase/nitcbase/B18_Code/main.cpp

CMakeFiles/B18_Code.dir/main.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/B18_Code.dir/main.cpp.i"
	/usr/bin/g++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/gokulsreekumar/NITCBase/nitcbase/B18_Code/main.cpp > CMakeFiles/B18_Code.dir/main.cpp.i

CMakeFiles/B18_Code.dir/main.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/B18_Code.dir/main.cpp.s"
	/usr/bin/g++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/gokulsreekumar/NITCBase/nitcbase/B18_Code/main.cpp -o CMakeFiles/B18_Code.dir/main.cpp.s

CMakeFiles/B18_Code.dir/Disk.cpp.o: CMakeFiles/B18_Code.dir/flags.make
CMakeFiles/B18_Code.dir/Disk.cpp.o: ../Disk.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/gokulsreekumar/NITCBase/nitcbase/B18_Code/cmake-build-debug/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/B18_Code.dir/Disk.cpp.o"
	/usr/bin/g++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/B18_Code.dir/Disk.cpp.o -c /Users/gokulsreekumar/NITCBase/nitcbase/B18_Code/Disk.cpp

CMakeFiles/B18_Code.dir/Disk.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/B18_Code.dir/Disk.cpp.i"
	/usr/bin/g++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/gokulsreekumar/NITCBase/nitcbase/B18_Code/Disk.cpp > CMakeFiles/B18_Code.dir/Disk.cpp.i

CMakeFiles/B18_Code.dir/Disk.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/B18_Code.dir/Disk.cpp.s"
	/usr/bin/g++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/gokulsreekumar/NITCBase/nitcbase/B18_Code/Disk.cpp -o CMakeFiles/B18_Code.dir/Disk.cpp.s

# Object files for target B18_Code
B18_Code_OBJECTS = \
"CMakeFiles/B18_Code.dir/main.cpp.o" \
"CMakeFiles/B18_Code.dir/Disk.cpp.o"

# External object files for target B18_Code
B18_Code_EXTERNAL_OBJECTS =

B18_Code: CMakeFiles/B18_Code.dir/main.cpp.o
B18_Code: CMakeFiles/B18_Code.dir/Disk.cpp.o
B18_Code: CMakeFiles/B18_Code.dir/build.make
B18_Code: CMakeFiles/B18_Code.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/Users/gokulsreekumar/NITCBase/nitcbase/B18_Code/cmake-build-debug/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Linking CXX executable B18_Code"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/B18_Code.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/B18_Code.dir/build: B18_Code

.PHONY : CMakeFiles/B18_Code.dir/build

CMakeFiles/B18_Code.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/B18_Code.dir/cmake_clean.cmake
.PHONY : CMakeFiles/B18_Code.dir/clean

CMakeFiles/B18_Code.dir/depend:
	cd /Users/gokulsreekumar/NITCBase/nitcbase/B18_Code/cmake-build-debug && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /Users/gokulsreekumar/NITCBase/nitcbase/B18_Code /Users/gokulsreekumar/NITCBase/nitcbase/B18_Code /Users/gokulsreekumar/NITCBase/nitcbase/B18_Code/cmake-build-debug /Users/gokulsreekumar/NITCBase/nitcbase/B18_Code/cmake-build-debug /Users/gokulsreekumar/NITCBase/nitcbase/B18_Code/cmake-build-debug/CMakeFiles/B18_Code.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/B18_Code.dir/depend

