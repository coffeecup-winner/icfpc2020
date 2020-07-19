//
// Filename list routines for the Fast Light Tool Kit (FLTK).
//
// Copyright 1998-2018 by Bill Spitzak and others.
//
// This library is free software. Distribution and use rights are outlined in
// the file "COPYING" which should have been included with this file.  If this
// file is missing or damaged, see the license at:
//
//     https://www.fltk.org/COPYING.php
//
// Please see the following page on how to report bugs and issues:
//
//     https://www.fltk.org/bugs.php
//

// Wrapper for scandir with const-correct function prototypes.

#include <FL/filename.H>
#include <FL/Fl.H>
#include "Fl_System_Driver.H"
#include <FL/fl_utf8.h>
#include "flstring.h"
#include <stdlib.h>


int fl_alphasort(struct dirent **a, struct dirent **b) {
  return strcmp((*a)->d_name, (*b)->d_name);
}

int fl_casealphasort(struct dirent **a, struct dirent **b) {
  return strcasecmp((*a)->d_name, (*b)->d_name);
}


/**
  Portable and const-correct wrapper for the scandir() function.

  For each file in that directory a "dirent" structure is created.
  The only portable thing about a dirent is that dirent.d_name is the
  nul-terminated file name. A pointers array to these dirent's is created
  and a pointer to the array is returned in *list.
  The number of entries is given as a return value.
  If there is an error reading the directory a number less than zero is
  returned, and errno has the reason; errno does not work under Windows.

  \b Include:
  \code
  #include <FL/filename.H>
  \endcode

  \param[in] d the name of the directory to list.  It does not matter if it has a trailing slash.
  \param[out] list table containing the resulting directory listing
  \param[in] sort sorting functor:
  - fl_alphasort: The files are sorted in ascending alphabetical order;
      upper and lowercase letters are compared according to their ASCII ordering  uppercase before lowercase.
  - fl_casealphasort: The files are sorted in ascending alphabetical order;
      upper and lowercase letters are compared equally case is not significant.
  - fl_casenumericsort: The files are sorted in ascending "alphanumeric" order, where an attempt is made
      to put unpadded numbers in consecutive order; upper and lowercase letters
      are compared equally case is not significant.
  - fl_numericsort: The files are sorted in ascending "alphanumeric" order, where an attempt is made
      to put unpadded numbers in consecutive order; upper and lowercase letters are compared
      according to their ASCII ordering - uppercase before lowercase.
  \return the number of entries if no error, a negative value otherwise.
*/
int fl_filename_list(const char *d, dirent ***list, Fl_File_Sort_F *sort) {
  return Fl::system_driver()->filename_list(d, list, sort);
}

/**
 \brief Free the list of filenames that is generated by fl_filename_list().

 Free everything that was allocated by a previous call to fl_filename_list().
 Use the return values as parameters for this function.

 \param[in,out] list table containing the resulting directory listing
 \param[in] n number of entries in the list
 */
void fl_filename_free_list(struct dirent ***list, int n)
{
  if (n<0) return;

  int i;
  for (i = 0; i < n; i ++) {
    if ((*list)[i])
      free((*list)[i]);
  }
  free(*list);
  *list = 0;
}