//
// Standard dialog test program for the Fast Light Tool Kit (FLTK).
//
// This also demonstrates how to trap attempts by the user to
// close the last window by overriding Fl::exit
//
// Copyright 1998-2017 by Bill Spitzak and others.
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

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <FL/Fl.H>
#include <FL/Fl_Double_Window.H>
#include <FL/Fl_Input.H>
#include <FL/Fl_Button.H>
#include <FL/Fl_Return_Button.H>
#include <FL/Fl_Box.H>

#include <FL/fl_ask.H>

void update_input_text(Fl_Widget* o, const char *input) {
  if (input) {
    o->copy_label(input);
    o->redraw();
  }
}

void rename_me(Fl_Widget*o) {
  const char *input = fl_input("Input:", o->label());
  update_input_text(o, input);
}

void rename_me_pwd(Fl_Widget*o) {
  const char *input = fl_password("Input PWD:", o->label());
  update_input_text(o, input);
}

void window_callback(Fl_Widget *win, void*) {
  int hotspot = fl_message_hotspot();
  fl_message_hotspot(0);
  fl_message_title("note: no hotspot set for this dialog");
  int rep = fl_choice("Are you sure you want to quit?",
                      "Cancel", "Quit", "Dunno");
  fl_message_hotspot(hotspot);
  if (rep==1)
    exit(0);
  else if (rep==2) { // (Dunno)
    fl_message_position(win);
    fl_message_title("This dialog must be centered over the main window");
    fl_message("Well, maybe you should know before we quit.");
  }
}

/*
  This timer callback shows a message dialog (fl_choice) window
  every 5 seconds to test "recursive" common dialogs.

  The timer can be stopped by clicking the button "Stop these funny popups"
  or pressing the Enter key. As it is currently implemented, clicking the
  "Close" button will reactivate the popups (only possible if "recursive"
  dialogs are enabled, see below).

  Note 1: This dialog box is blocked in FLTK 1.3.x if another common dialog
  is already open because the window used is a static (i.e. permanently
  allocated) Fl_Window instance. This should be fixed in FLTK 1.4.0.
  See STR #334 (sic !) and also STR #2751 ("Limit input field characters").
*/
void timer_cb(void *) {

  static int stop = 0;
  static const double delta = 5.0;

  Fl_Box *message_icon = (Fl_Box *)fl_message_icon();

  Fl::repeat_timeout(delta, timer_cb);

  if (stop == 1) {
    message_icon->color(FL_WHITE);
    return;
  }

  // Change the icon box color:
  Fl_Color c = message_icon->color();
  c = (c+1) % 32;
  if (c == message_icon->labelcolor()) c++;
  message_icon->color((Fl_Color)c);

  // pop up a message:
  stop = fl_choice("Timeout. Click the 'Close' button.\n"
                   "Note: this message was blocked in FLTK 1.3\n"
                   "if another message window is open.\n"
                   "This *should* be fixed in FLTK 1.4.0!\n"
                   "This message should pop up every 5 seconds.",
                   "Close", "Stop these funny popups", NULL);
}

int main(int argc, char **argv) {
  char buffer[128] = "Test text";
  char buffer2[128] = "MyPassword";

  // This is a test to make sure automatic destructors work. Pop up
  // the question dialog several times and make sure it doesn't crash.

  Fl_Double_Window window(200, 105);
  Fl_Return_Button b(20, 10, 160, 35, buffer);
  b.callback(rename_me);
  Fl_Button b2(20, 50, 160, 35, buffer2);
  b2.callback(rename_me_pwd);
  window.end();
  window.resizable(&b);
  window.show(argc, argv);

  // Also we test to see if the exit callback works:
  window.callback(window_callback);

  // Test: set default message window title:
  // fl_message_title_default("Default Window Title");

  // Test: multiple (nested, aka "recursive") popups
  Fl::add_timeout(5.0, timer_cb);

  return Fl::run();
}
