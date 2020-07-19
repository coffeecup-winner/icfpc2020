#include "cfl_dialog.h"
#include <FL/Fl.H>
#include <FL/Fl_Help_Dialog.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Native_File_Chooser.H>
#include <FL/fl_ask.H>
#include <new>
#include <string.h>

#ifndef LOCK
#define LOCK(x)                                                                                    \
    Fl::lock();                                                                                    \
    x;                                                                                             \
    Fl::unlock();                                                                                  \
    Fl::awake();
#endif

Fl_Native_File_Chooser *Fl_Native_File_Chooser_new(int val) {
    return new (std::nothrow) Fl_Native_File_Chooser(val);
}

void Fl_Native_File_Chooser_delete(Fl_Native_File_Chooser *self) {
    delete self;
}

const char *Fl_Native_File_Chooser_filename(Fl_Native_File_Chooser *self) {
    const char *x = self->filename();
    if (!strcmp(x, ""))
        return NULL;
    else
        return x;
}

const char *Fl_Native_File_Chooser_filenames(Fl_Native_File_Chooser *self, int cnt) {
    const char *x = self->filename(cnt);
    if (!strcmp(x, ""))
        return NULL;
    else
        return x;
}

int Fl_Native_File_Chooser_count(Fl_Native_File_Chooser *self) {
    return self->count();
}

void Fl_Native_File_Chooser_set_directory(Fl_Native_File_Chooser *self, const char *val) {
    LOCK(self->directory(val);)
}

const char *Fl_Native_File_Chooser_directory(Fl_Native_File_Chooser *self) {
    return self->directory();
}

int Fl_Native_File_Chooser_show(Fl_Native_File_Chooser *self) {
    int ret = 0;
    LOCK(ret = self->show());
    return ret;
}

void Fl_Native_File_Chooser_set_option(Fl_Native_File_Chooser *self, int opt) {
    LOCK(self->options(opt);)
}

void Fl_Native_File_Chooser_set_type(Fl_Native_File_Chooser *self, int typ) {
    LOCK(self->type(typ);)
}

void Fl_Native_File_Chooser_set_title(Fl_Native_File_Chooser *self, const char *title) {
    LOCK(self->title(title);)
}

void Fl_Native_File_Chooser_set_filter(Fl_Native_File_Chooser *self, const char *f) {
    LOCK(self->filter(f);)
}

void Fl_Native_File_Chooser_set_preset_file(Fl_Native_File_Chooser *self, const char *f) {
    LOCK(self->preset_file(f);)
}

const char *Fl_Native_File_Chooser_errmsg(Fl_Native_File_Chooser *self) {
    return self->errmsg();
}

void cfl_message(int x, int y, const char *txt) {
    fl_message_position(x, y, 0);
    fl_message("%s", txt);
}

void cfl_alert(int x, int y, const char *txt) {
    fl_message_position(x, y, 0);
    fl_alert("%s", txt);
}

int cfl_choice(int x, int y, const char *txt, const char *b0, const char *b1, const char *b2) {
    fl_message_position(x, y, 0);
    if (strlen(b2) == 0)
        b2 = NULL;
    return fl_choice("%s", b0, b1, b2, txt);
}

const char *cfl_input(int x, int y, const char *txt, const char *deflt) {
    fl_message_position(x, y, 0);
    return fl_input("%s", deflt, txt);
}

const char *cfl_password(int x, int y, const char *txt, const char *deflt) {
    fl_message_position(x, y, 0);
    return fl_password("%s", deflt, txt);
}

Fl_Help_Dialog *Fl_Help_Dialog_new(void) {
    return new (std::nothrow) Fl_Help_Dialog();
}

void Fl_Help_Dialog_delete(Fl_Help_Dialog *self) {
    delete self;
}

int Fl_Help_Dialog_h(Fl_Help_Dialog *self) {
    return self->h();
}

void Fl_Help_Dialog_hide(Fl_Help_Dialog *self) {
    return self->hide();
}

int Fl_Help_Dialog_load(Fl_Help_Dialog *self, const char *f) {
    int ret = 0;
    LOCK(ret = self->load(f));
    return ret;
}

void Fl_Help_Dialog_position(Fl_Help_Dialog *self, int xx, int yy) {
    LOCK(self->position(xx, yy);)
}

void Fl_Help_Dialog_resize(Fl_Help_Dialog *self, int xx, int yy, int ww, int hh) {
    LOCK(self->resize(xx, yy, ww, hh);)
}

void Fl_Help_Dialog_show(Fl_Help_Dialog *self) {
    LOCK(self->show();)
}

void Fl_Help_Dialog_set_text_size(Fl_Help_Dialog *self, int s) {
    LOCK(self->textsize(s);)
}

int Fl_Help_Dialog_text_size(Fl_Help_Dialog *self) {
    return self->textsize();
}

void Fl_Help_Dialog_set_value(Fl_Help_Dialog *self, const char *f) {
    LOCK(self->value(f);)
}

const char *Fl_Help_Dialog_value(const Fl_Help_Dialog *self) {
    return self->value();
}

int Fl_Help_Dialog_visible(Fl_Help_Dialog *self) {
    return self->visible();
}

int Fl_Help_Dialog_w(Fl_Help_Dialog *self) {
    return self->w();
}

int Fl_Help_Dialog_x(Fl_Help_Dialog *self) {
    return self->x();
}

int Fl_Help_Dialog_y(Fl_Help_Dialog *self) {
    return self->y();
}

void Fl_beep(int type) {
    fl_beep(type);
}

#undef LOCK