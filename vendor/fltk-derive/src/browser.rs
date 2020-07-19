use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_browser_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());

    let remove = Ident::new(format!("{}_{}", name_str, "remove").as_str(), name.span());
    let add = Ident::new(format!("{}_{}", name_str, "add").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let move_item = Ident::new(format!("{}_{}", name_str, "move").as_str(), name.span());
    let swap = Ident::new(format!("{}_{}", name_str, "swap").as_str(), name.span());
    let clear = Ident::new(format!("{}_{}", name_str, "clear").as_str(), name.span());
    let size = Ident::new(format!("{}_{}", name_str, "size").as_str(), name.span());
    let set_size = Ident::new(format!("{}_{}", name_str, "set_size").as_str(), name.span());
    let select = Ident::new(format!("{}_{}", name_str, "select").as_str(), name.span());
    let selected = Ident::new(format!("{}_{}", name_str, "selected").as_str(), name.span());
    let text = Ident::new(format!("{}_{}", name_str, "text").as_str(), name.span());
    let set_text = Ident::new(format!("{}_{}", name_str, "set_text").as_str(), name.span());
    let load_file = Ident::new(
        format!("{}_{}", name_str, "load_file").as_str(),
        name.span(),
    );
    let text_size = Ident::new(
        format!("{}_{}", name_str, "text_size").as_str(),
        name.span(),
    );
    let set_text_size = Ident::new(
        format!("{}_{}", name_str, "set_text_size").as_str(),
        name.span(),
    );
    let set_icon = Ident::new(format!("{}_{}", name_str, "set_icon").as_str(), name.span());
    let icon = Ident::new(format!("{}_{}", name_str, "icon").as_str(), name.span());
    let remove_icon = Ident::new(
        format!("{}_{}", name_str, "remove_icon").as_str(),
        name.span(),
    );
    let topline = Ident::new(format!("{}_{}", name_str, "topline").as_str(), name.span());
    let middleline = Ident::new(
        format!("{}_{}", name_str, "middleline").as_str(),
        name.span(),
    );
    let bottomline = Ident::new(
        format!("{}_{}", name_str, "bottomline").as_str(),
        name.span(),
    );
    let format_char = Ident::new(
        format!("{}_{}", name_str, "format_char").as_str(),
        name.span(),
    );
    let set_format_char = Ident::new(
        format!("{}_{}", name_str, "set_format_char").as_str(),
        name.span(),
    );
    let column_char = Ident::new(
        format!("{}_{}", name_str, "column_char").as_str(),
        name.span(),
    );
    let set_column_char = Ident::new(
        format!("{}_{}", name_str, "set_column_char").as_str(),
        name.span(),
    );
    let column_widths = Ident::new(
        format!("{}_{}", name_str, "column_widths").as_str(),
        name.span(),
    );
    let set_column_widths = Ident::new(
        format!("{}_{}", name_str, "set_column_widths").as_str(),
        name.span(),
    );
    let displayed = Ident::new(
        format!("{}_{}", name_str, "displayed").as_str(),
        name.span(),
    );
    let make_visible = Ident::new(
        format!("{}_{}", name_str, "make_visible").as_str(),
        name.span(),
    );
    let position = Ident::new(format!("{}_{}", name_str, "position").as_str(), name.span());
    let set_position = Ident::new(
        format!("{}_{}", name_str, "set_position").as_str(),
        name.span(),
    );
    let hposition = Ident::new(
        format!("{}_{}", name_str, "hposition").as_str(),
        name.span(),
    );
    let set_hposition = Ident::new(
        format!("{}_{}", name_str, "set_hposition").as_str(),
        name.span(),
    );
    let has_scrollbar = Ident::new(
        format!("{}_{}", name_str, "has_scrollbar").as_str(),
        name.span(),
    );
    let set_has_scrollbar = Ident::new(
        format!("{}_{}", name_str, "set_has_scrollbar").as_str(),
        name.span(),
    );
    let scrollbar_size = Ident::new(
        format!("{}_{}", name_str, "scrollbar_size").as_str(),
        name.span(),
    );
    let set_scrollbar_size = Ident::new(
        format!("{}_{}", name_str, "set_scrollbar_size").as_str(),
        name.span(),
    );
    let scrollbar_width = Ident::new(
        format!("{}_{}", name_str, "scrollbar_width").as_str(),
        name.span(),
    );
    let set_scrollbar_width = Ident::new(
        format!("{}_{}", name_str, "set_scrollbar_width").as_str(),
        name.span(),
    );
    let sort = Ident::new(format!("{}_{}", name_str, "sort").as_str(), name.span());
    let scrollbar = Ident::new(format!("{}_{}", name_str, "scrollbar").as_str(), name.span());
    let hscrollbar = Ident::new(format!("{}_{}", name_str, "hscrollbar").as_str(), name.span());

    let gen = quote! {
        unsafe impl BrowserExt for #name {
            fn remove(&mut self, line: u32) {
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #remove(self._inner, line as i32)
                }
            }

            fn add(&mut self, item: &str) {
                let item = CString::new(item).unwrap();
                unsafe {
                    assert!(!self.was_deleted());
                    #add(self._inner, item.as_ptr())
                }
            }

            fn insert(&mut self, line: u32, item: &str) {
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                let item = CString::new(item).unwrap();
                unsafe {
                    assert!(!self.was_deleted());
                    #insert(self._inner, line as i32, item.as_ptr())
                }
            }

            fn move_item(&mut self, to: u32, from: u32) {
                debug_assert!(to <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                debug_assert!(from <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #move_item(self._inner, to as i32, from as i32)
                }
            }

            fn swap(&mut self, a: u32, b: u32) {
                debug_assert!(a <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                debug_assert!(b <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #swap(self._inner, a as i32, b as i32)
                }
            }

            fn clear(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    #clear(self._inner)
                }
            }

            fn size(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #size(self._inner) as u32
                }
            }

            fn set_size(&mut self, w: i32, h: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_size(self._inner, w, h)
                }
            }

            fn select(&mut self, line: u32) {
                assert!(line > 0);
                if line < self.size() {
                    unsafe {
                        assert!(!self.was_deleted());
                        #select(self._inner, line as i32);
                    }
                }
            }

            fn selected(&self, line: u32) -> bool {
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    match #selected(self._inner, line as i32) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn text(&self, line: u32) -> Option<String> {
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    let text = #text(self._inner, line as i32);
                    if text.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(text as *mut raw::c_char).to_string_lossy().to_string())
                    }
                }
            }

            fn set_text(&mut self, line: u32, txt: &str) {
                assert!(line > 0);
                assert!(line <= self.size());
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                let txt = CString::new(txt).unwrap();
                unsafe {
                    assert!(!self.was_deleted());
                    #set_text(self._inner, line as i32, txt.as_ptr())
                }
            }

            fn load(&mut self, path: &std::path::Path) -> Result<(), FltkError> {
                if !path.exists() {
                    return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
                }
                let path = path.to_str().unwrap();
                let path = CString::new(path)?;
                unsafe {
                    assert!(!self.was_deleted());
                    #load_file(self._inner, path.as_ptr());
                    Ok(())
                }
            }

            fn text_size(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #text_size(self._inner) as u32
                }
            }

            fn set_text_size(&mut self, c: u32) {
                debug_assert!(c <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #set_text_size(self._inner, c as i32)
                }
            }

            fn set_icon<Img: ImageExt>(&mut self, line: u32, image: Option<Img>) {
                assert!(!self.was_deleted());
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                if let Some(image) = image {
                    assert!(!image.was_deleted());
                    unsafe { #set_icon(self._inner, line as i32, image.as_ptr()) }
                } else {
                    unsafe { #set_icon(self._inner, line as i32, std::ptr::null_mut() as *mut raw::c_void) }
                }
            }

            fn icon(&self, line: u32) -> Option<Image> {
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    let icon_ptr = #icon(self._inner, line as i32);
                    if icon_ptr.is_null() {
                        None
                    } else {
                        Some(Image::from_raw(icon_ptr as *mut fltk_sys::image::Fl_Image))
                    }
                }
            }

            fn remove_icon(&mut self, line: u32) {
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #remove_icon(self._inner, line as i32)
                }
            }

            fn topline(&mut self, line: u32) {
                assert!(!self.was_deleted());
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #topline(self._inner, line as i32)
                }
            }

            fn bottomline(&mut self, line: u32) {
                assert!(!self.was_deleted());
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #bottomline(self._inner, line as i32)
                }
            }

            fn middleline(&mut self, line: u32) {
                assert!(!self.was_deleted());
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #middleline(self._inner, line as i32)
                }
            }

            fn format_char(&self) -> char {
                assert!(!self.was_deleted());
                unsafe {
                    #format_char(self._inner) as u8 as char
                }
            }

            fn set_format_char(&mut self, c: char) {
                assert!(!self.was_deleted());
                assert!(c.is_ascii());
                unsafe {
                    #set_format_char(self._inner, c as raw::c_char)
                }
            }

            fn column_char(&self) -> char {
                assert!(!self.was_deleted());
                unsafe {
                    #column_char(self._inner) as u8 as char
                }
            }

            fn set_column_char(&mut self, c: char) {
                assert!(!self.was_deleted());
                assert!(c.is_ascii());
                unsafe {
                    #set_column_char(self._inner, c as raw::c_char)
                }
            }

            fn column_widths(&self) -> Vec<i32> {
                assert!(!self.was_deleted());
                unsafe {
                    let widths = #column_widths(self._inner);
                    assert!(!widths.is_null());
                    let mut v: Vec<i32> = vec![];
                    let mut i = 0;
                    while (*widths.offset(i) != 0) {
                        v.push(*widths.offset(i));
                        i += 1;
                    }
                    v
                }
            }

            fn set_column_widths(&mut self, arr: &[i32]) {
                assert!(!self.was_deleted());
                unsafe {
                    let mut v = arr.to_vec();
                    v.push(0);
                    let v = mem::ManuallyDrop::new(v);
                    #set_column_widths(self._inner, v.as_ptr());
                }
            }

            fn displayed(&self, line: u32,) -> bool {
                assert!(!self.was_deleted());
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #displayed(self._inner, line as i32,) != 0
                }
            }

            fn make_visible(&mut self, line: u32) {
                assert!(!self.was_deleted());
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #make_visible(self._inner, line as i32)
                }
            }

            fn position(&self) -> u32 {
                assert!(!self.was_deleted());
                unsafe {
                    #position(self._inner) as u32
                }
            }

            fn set_position(&mut self, pos: u32) {
                assert!(!self.was_deleted());
                debug_assert!(pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #set_position(self._inner, pos as i32)
                }
            }

            fn hposition(&self) -> u32 {
                assert!(!self.was_deleted());
                unsafe {
                    #hposition(self._inner) as u32
                }
            }

            fn set_hposition(&mut self, pos: u32) {
                assert!(!self.was_deleted());
                debug_assert!(pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #set_hposition(self._inner, pos as i32)
                }
            }

            fn has_scrollbar(&self) -> BrowserScrollbar {
                assert!(!self.was_deleted());
                unsafe {
                    mem::transmute(#has_scrollbar(self._inner))
                }
            }

            fn set_has_scrollbar(&mut self, mode: BrowserScrollbar) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_has_scrollbar(self._inner, mode as raw::c_uchar)
                }
            }

            fn scrollbar_size(&self) -> u32 {
                assert!(!self.was_deleted());
                unsafe {
                    #scrollbar_size(self._inner) as u32
                }
            }

            fn set_scrollbar_size(&mut self, new_size: u32) {
                assert!(!self.was_deleted());
                debug_assert!(new_size <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #set_scrollbar_size(self._inner, new_size as i32)
                }
            }

            fn scrollbar_width(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe {
                    #scrollbar_width(self._inner)
                }
            }

            fn set_scrollbar_width(&mut self, width: i32) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_scrollbar_width(self._inner, width)
                }
            }

            fn sort(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    #sort(self._inner)
                }
            }

            fn scrollbar(&self) -> crate::valuator::Scrollbar {
                assert!(!self.was_deleted());
                unsafe {
                    let ptr = #scrollbar(self._inner);
                    assert!(!ptr.is_null());
                    crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)
                }
            }
        
            fn hscrollbar(&self) -> crate::valuator::Scrollbar {
                assert!(!self.was_deleted());
                unsafe {
                    let ptr = #hscrollbar(self._inner);
                    assert!(!ptr.is_null());
                    crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)
                }
            }
        }
    };
    gen.into()
}
