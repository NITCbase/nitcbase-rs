# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.20

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
CMAKE_SOURCE_DIR = /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/cmake-build-debug

# Include any dependencies generated for this target.
include CMakeFiles/XFS-Interface.dir/depend.make
# Include the progress variables for this target.
include CMakeFiles/XFS-Interface.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/XFS-Interface.dir/flags.make

CMakeFiles/XFS-Interface.dir/interface.cpp.o: CMakeFiles/XFS-Interface.dir/flags.make
CMakeFiles/XFS-Interface.dir/interface.cpp.o: ../interface.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/cmake-build-debug/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/XFS-Interface.dir/interface.cpp.o"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/XFS-Interface.dir/interface.cpp.o -c /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/interface.cpp

CMakeFiles/XFS-Interface.dir/interface.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/XFS-Interface.dir/interface.cpp.i"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/interface.cpp > CMakeFiles/XFS-Interface.dir/interface.cpp.i

CMakeFiles/XFS-Interface.dir/interface.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/XFS-Interface.dir/interface.cpp.s"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/interface.cpp -o CMakeFiles/XFS-Interface.dir/interface.cpp.s

CMakeFiles/XFS-Interface.dir/Disk.cpp.o: CMakeFiles/XFS-Interface.dir/flags.make
CMakeFiles/XFS-Interface.dir/Disk.cpp.o: ../Disk.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/cmake-build-debug/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/XFS-Interface.dir/Disk.cpp.o"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/XFS-Interface.dir/Disk.cpp.o -c /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/Disk.cpp

CMakeFiles/XFS-Interface.dir/Disk.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/XFS-Interface.dir/Disk.cpp.i"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/Disk.cpp > CMakeFiles/XFS-Interface.dir/Disk.cpp.i

CMakeFiles/XFS-Interface.dir/Disk.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/XFS-Interface.dir/Disk.cpp.s"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/Disk.cpp -o CMakeFiles/XFS-Interface.dir/Disk.cpp.s

CMakeFiles/XFS-Interface.dir/schema.cpp.o: CMakeFiles/XFS-Interface.dir/flags.make
CMakeFiles/XFS-Interface.dir/schema.cpp.o: ../schema.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/cmake-build-debug/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Building CXX object CMakeFiles/XFS-Interface.dir/schema.cpp.o"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/XFS-Interface.dir/schema.cpp.o -c /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/schema.cpp

CMakeFiles/XFS-Interface.dir/schema.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/XFS-Interface.dir/schema.cpp.i"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/schema.cpp > CMakeFiles/XFS-Interface.dir/schema.cpp.i

CMakeFiles/XFS-Interface.dir/schema.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/XFS-Interface.dir/schema.cpp.s"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/schema.cpp -o CMakeFiles/XFS-Interface.dir/schema.cpp.s

CMakeFiles/XFS-Interface.dir/block_access.cpp.o: CMakeFiles/XFS-Interface.dir/flags.make
CMakeFiles/XFS-Interface.dir/block_access.cpp.o: ../block_access.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/cmake-build-debug/CMakeFiles --progress-num=$(CMAKE_PROGRESS_4) "Building CXX object CMakeFiles/XFS-Interface.dir/block_access.cpp.o"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/XFS-Interface.dir/block_access.cpp.o -c /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/block_access.cpp

CMakeFiles/XFS-Interface.dir/block_access.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/XFS-Interface.dir/block_access.cpp.i"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/block_access.cpp > CMakeFiles/XFS-Interface.dir/block_access.cpp.i

CMakeFiles/XFS-Interface.dir/block_access.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/XFS-Interface.dir/block_access.cpp.s"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/block_access.cpp -o CMakeFiles/XFS-Interface.dir/block_access.cpp.s

CMakeFiles/XFS-Interface.dir/external_fs_commands.cpp.o: CMakeFiles/XFS-Interface.dir/flags.make
CMakeFiles/XFS-Interface.dir/external_fs_commands.cpp.o: ../external_fs_commands.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/cmake-build-debug/CMakeFiles --progress-num=$(CMAKE_PROGRESS_5) "Building CXX object CMakeFiles/XFS-Interface.dir/external_fs_commands.cpp.o"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/XFS-Interface.dir/external_fs_commands.cpp.o -c /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/external_fs_commands.cpp

CMakeFiles/XFS-Interface.dir/external_fs_commands.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/XFS-Interface.dir/external_fs_commands.cpp.i"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/external_fs_commands.cpp > CMakeFiles/XFS-Interface.dir/external_fs_commands.cpp.i

CMakeFiles/XFS-Interface.dir/external_fs_commands.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/XFS-Interface.dir/external_fs_commands.cpp.s"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/external_fs_commands.cpp -o CMakeFiles/XFS-Interface.dir/external_fs_commands.cpp.s

CMakeFiles/XFS-Interface.dir/OpenRelTable.cpp.o: CMakeFiles/XFS-Interface.dir/flags.make
CMakeFiles/XFS-Interface.dir/OpenRelTable.cpp.o: ../OpenRelTable.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/cmake-build-debug/CMakeFiles --progress-num=$(CMAKE_PROGRESS_6) "Building CXX object CMakeFiles/XFS-Interface.dir/OpenRelTable.cpp.o"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/XFS-Interface.dir/OpenRelTable.cpp.o -c /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/OpenRelTable.cpp

CMakeFiles/XFS-Interface.dir/OpenRelTable.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/XFS-Interface.dir/OpenRelTable.cpp.i"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/OpenRelTable.cpp > CMakeFiles/XFS-Interface.dir/OpenRelTable.cpp.i

CMakeFiles/XFS-Interface.dir/OpenRelTable.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/XFS-Interface.dir/OpenRelTable.cpp.s"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/OpenRelTable.cpp -o CMakeFiles/XFS-Interface.dir/OpenRelTable.cpp.s

CMakeFiles/XFS-Interface.dir/algebra.cpp.o: CMakeFiles/XFS-Interface.dir/flags.make
CMakeFiles/XFS-Interface.dir/algebra.cpp.o: ../algebra.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/cmake-build-debug/CMakeFiles --progress-num=$(CMAKE_PROGRESS_7) "Building CXX object CMakeFiles/XFS-Interface.dir/algebra.cpp.o"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/XFS-Interface.dir/algebra.cpp.o -c /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/algebra.cpp

CMakeFiles/XFS-Interface.dir/algebra.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/XFS-Interface.dir/algebra.cpp.i"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/algebra.cpp > CMakeFiles/XFS-Interface.dir/algebra.cpp.i

CMakeFiles/XFS-Interface.dir/algebra.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/XFS-Interface.dir/algebra.cpp.s"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/algebra.cpp -o CMakeFiles/XFS-Interface.dir/algebra.cpp.s

CMakeFiles/XFS-Interface.dir/BPlusTree.cpp.o: CMakeFiles/XFS-Interface.dir/flags.make
CMakeFiles/XFS-Interface.dir/BPlusTree.cpp.o: ../BPlusTree.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/cmake-build-debug/CMakeFiles --progress-num=$(CMAKE_PROGRESS_8) "Building CXX object CMakeFiles/XFS-Interface.dir/BPlusTree.cpp.o"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/XFS-Interface.dir/BPlusTree.cpp.o -c /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/BPlusTree.cpp

CMakeFiles/XFS-Interface.dir/BPlusTree.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/XFS-Interface.dir/BPlusTree.cpp.i"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/BPlusTree.cpp > CMakeFiles/XFS-Interface.dir/BPlusTree.cpp.i

CMakeFiles/XFS-Interface.dir/BPlusTree.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/XFS-Interface.dir/BPlusTree.cpp.s"
	/Library/Developer/CommandLineTools/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/BPlusTree.cpp -o CMakeFiles/XFS-Interface.dir/BPlusTree.cpp.s

# Object files for target XFS-Interface
XFS__Interface_OBJECTS = \
"CMakeFiles/XFS-Interface.dir/interface.cpp.o" \
"CMakeFiles/XFS-Interface.dir/Disk.cpp.o" \
"CMakeFiles/XFS-Interface.dir/schema.cpp.o" \
"CMakeFiles/XFS-Interface.dir/block_access.cpp.o" \
"CMakeFiles/XFS-Interface.dir/external_fs_commands.cpp.o" \
"CMakeFiles/XFS-Interface.dir/OpenRelTable.cpp.o" \
"CMakeFiles/XFS-Interface.dir/algebra.cpp.o" \
"CMakeFiles/XFS-Interface.dir/BPlusTree.cpp.o"

# External object files for target XFS-Interface
XFS__Interface_EXTERNAL_OBJECTS =

XFS-Interface: CMakeFiles/XFS-Interface.dir/interface.cpp.o
XFS-Interface: CMakeFiles/XFS-Interface.dir/Disk.cpp.o
XFS-Interface: CMakeFiles/XFS-Interface.dir/schema.cpp.o
XFS-Interface: CMakeFiles/XFS-Interface.dir/block_access.cpp.o
XFS-Interface: CMakeFiles/XFS-Interface.dir/external_fs_commands.cpp.o
XFS-Interface: CMakeFiles/XFS-Interface.dir/OpenRelTable.cpp.o
XFS-Interface: CMakeFiles/XFS-Interface.dir/algebra.cpp.o
XFS-Interface: CMakeFiles/XFS-Interface.dir/BPlusTree.cpp.o
XFS-Interface: CMakeFiles/XFS-Interface.dir/build.make
XFS-Interface: CMakeFiles/XFS-Interface.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/cmake-build-debug/CMakeFiles --progress-num=$(CMAKE_PROGRESS_9) "Linking CXX executable XFS-Interface"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/XFS-Interface.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/XFS-Interface.dir/build: XFS-Interface
.PHONY : CMakeFiles/XFS-Interface.dir/build

CMakeFiles/XFS-Interface.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/XFS-Interface.dir/cmake_clean.cmake
.PHONY : CMakeFiles/XFS-Interface.dir/clean

CMakeFiles/XFS-Interface.dir/depend:
	cd /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/cmake-build-debug && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/cmake-build-debug /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/cmake-build-debug /Users/gokul/Developer/nitcbase/B18_Code/XFS_Interface/cmake-build-debug/CMakeFiles/XFS-Interface.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/XFS-Interface.dir/depend

