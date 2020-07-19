use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_valuator_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());

    let set_bounds = Ident::new(
        format!("{}_{}", name_str, "set_bounds").as_str(),
        name.span(),
    );
    let minimum = Ident::new(format!("{}_{}", name_str, "minimum").as_str(), name.span());
    let set_minimum = Ident::new(
        format!("{}_{}", name_str, "set_minimum").as_str(),
        name.span(),
    );
    let maximum = Ident::new(format!("{}_{}", name_str, "maximum").as_str(), name.span());
    let set_maximum = Ident::new(
        format!("{}_{}", name_str, "set_maximum").as_str(),
        name.span(),
    );
    let set_range = Ident::new(
        format!("{}_{}", name_str, "set_range").as_str(),
        name.span(),
    );
    let step = Ident::new(format!("{}_{}", name_str, "step").as_str(), name.span());
    let set_step = Ident::new(format!("{}_{}", name_str, "set_step").as_str(), name.span());
    let set_precision = Ident::new(
        format!("{}_{}", name_str, "set_precision").as_str(),
        name.span(),
    );
    let value = Ident::new(format!("{}_{}", name_str, "value").as_str(), name.span());
    let set_value = Ident::new(
        format!("{}_{}", name_str, "set_value").as_str(),
        name.span(),
    );
    let format = Ident::new(format!("{}_{}", name_str, "format").as_str(), name.span());
    let round = Ident::new(format!("{}_{}", name_str, "round").as_str(), name.span());
    let clamp = Ident::new(format!("{}_{}", name_str, "clamp").as_str(), name.span());
    let increment = Ident::new(
        format!("{}_{}", name_str, "increment").as_str(),
        name.span(),
    );

    let gen = quote! {
        unsafe impl ValuatorExt for #name {
            fn set_bounds(&mut self, a: f64, b: f64) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_bounds(self._inner, a, b)
                }
            }

            fn minimum(&self) -> f64 {
                unsafe {
                    assert!(!self.was_deleted());
                    #minimum(self._inner)
                }
            }

            fn set_minimum(&mut self, a: f64) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_minimum(self._inner, a)
                }
            }

            fn maximum(&self) -> f64 {
                unsafe {
                    assert!(!self.was_deleted());
                    #maximum(self._inner)
                }
            }

            fn set_maximum(&mut self, a: f64) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_maximum(self._inner, a)
                }
            }

            fn set_range(&mut self, a: f64, b: f64) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_range(self._inner, a, b)
                }
            }

            fn set_step(&mut self, a: f64, b: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_step(self._inner, a, b)
                }
            }

            fn step(&self) -> f64 {
                unsafe {
                    assert!(!self.was_deleted());
                    #step(self._inner)
                }
            }

            fn set_precision(&mut self, digits: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_precision(self._inner, digits)
                }
            }

            fn value(&self) -> f64 {
                unsafe {
                    assert!(!self.was_deleted());
                    #value(self._inner)
                }
            }


            fn set_value(&mut self, arg2: f64) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_value(self._inner, arg2);
                }
            }


            fn format(&mut self, arg2: &str) -> Result<(), FltkError> {
                unsafe {
                    let arg2 = CString::new(arg2).unwrap();
                    assert!(!self.was_deleted());
                    let x = #format(self._inner, arg2.as_ptr() as *mut raw::c_char);
                    if x < 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn round(&self, arg2: f64) -> f64 {
                unsafe {
                    assert!(!self.was_deleted());
                    #round(self._inner, arg2)
                }
            }


            fn clamp(&self, arg2: f64) -> f64 {
                unsafe {
                    assert!(!self.was_deleted());
                    #clamp(self._inner, arg2)
                }
            }

            fn increment(&mut self, arg2: f64, arg3: i32) -> f64 {
                unsafe {
                    assert!(!self.was_deleted());
                    #increment(self._inner, arg2, arg3)
                }
            }
        }
    };
    gen.into()
}
