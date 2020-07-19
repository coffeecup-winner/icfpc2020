bindgen fltk-sys/cfltk/cfl_box.h -o fltk-sys/src/frame.rs
bindgen fltk-sys/cfltk/cfl_button.h -o fltk-sys/src/button.rs
bindgen fltk-sys/cfltk/cfl_group.h -o fltk-sys/src/group.rs 
bindgen fltk-sys/cfltk/cfl_text.h -o fltk-sys/src/text.rs 
bindgen fltk-sys/cfltk/cfl_input.h -o fltk-sys/src/input.rs
bindgen fltk-sys/cfltk/cfl_output.h -o fltk-sys/src/output.rs
bindgen fltk-sys/cfltk/cfl_widget.h -o fltk-sys/src/widget.rs
bindgen fltk-sys/cfltk/cfl_window.h -o fltk-sys/src/window.rs -- -DCFLTK_USE_GL
bindgen fltk-sys/cfltk/cfl_menu.h -o fltk-sys/src/menu.rs
bindgen fltk-sys/cfltk/cfl_dialog.h -o fltk-sys/src/dialog.rs
bindgen fltk-sys/cfltk/cfl_valuator.h -o fltk-sys/src/valuator.rs
bindgen fltk-sys/cfltk/cfl_browser.h -o fltk-sys/src/browser.rs
bindgen fltk-sys/cfltk/cfl_image.h -o fltk-sys/src/image.rs
bindgen fltk-sys/cfltk/cfl_misc.h -o fltk-sys/src/misc.rs
bindgen fltk-sys/cfltk/cfl_draw.h -o fltk-sys/src/draw.rs
bindgen fltk-sys/cfltk/cfl_table.h -o fltk-sys/src/table.rs
bindgen fltk-sys/cfltk/cfl_tree.h -o fltk-sys/src/tree.rs
bindgen fltk-sys/cfltk/cfl.h -o fltk-sys/src/fl.rs
# bindgen fltk-sys/cfltk/cfl_gl.h -o fltk-sys/src/glu.rs