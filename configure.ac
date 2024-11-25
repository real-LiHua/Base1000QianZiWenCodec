#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.72])
AC_INIT([Base1000],[0.0.1],[https://github.com/real-LiHua/Base1000QianZiWenCodec/issues],[base1000],[https://github.com/real-LiHua/Base1000QianZiWenCodec])
AM_INIT_AUTOMAKE
AC_CONFIG_HEADERS([config.hpp])

# Checks for programs.
AC_PROG_CXX
AC_PROG_CC
AC_PROG_MAKE_SET

# Checks for libraries.
PKG_CHECK_MODULES([CLI11], [CLI11])
PKG_CHECK_MODULES([ICU], [icu-uc])

# Checks for header files.
AC_CHECK_FILE([encode.hpp])
AC_CHECK_FILE([decode.hpp])

# Checks for typedefs, structures, and compiler characteristics.
AC_CHECK_HEADER_STDBOOL
AC_DEFUN([READ_FILE], [
  FILE_CONTENT=$(cat "$srcdir/$2" | sed ':a;N;$!ba;s/\n//g;s/ //g')
  AC_SUBST([$1], [$FILE_CONTENT])
])

READ_FILE([QZW1], [千字文/1.txt])

# Checks for library functions.

AC_CONFIG_FILES([Makefile])
AC_OUTPUT
