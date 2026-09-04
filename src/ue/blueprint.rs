
use brickworks::{br_print, set_module_name};
use super::coreuobject::*;
use super::FFrame;
set_module_name!(b"blueprints\0");

pub struct BlueprintFunction
{
    pub class: Option<&'static str>,
    pub function_name: &'static str,
    pub function: unsafe fn(obj: *mut UObject, stack: &mut FFrame, result: *mut ()),
}

inventory::collect!(BlueprintFunction);

///
/// Creates function overrides in blueprints
///
/// ```
/// bp_function(my_function_name |obj, stack, result| 
/// {
///     struct InputParams {
///         /* input parameters you specify in blueprint*/
///         pub str: FString
///     }
///     let params = stack.get_input_params()  as *mut InputParams;
/// });
/// ```
///
#[macro_export]
macro_rules! bp_function {
    ($name:ident |$obj:ident, $stack:ident, $result:ident| $body:block) => {
        use $crate::ue::fframe::*;
        use $crate::ue::coreuobject::*;
        use $crate::ue::blueprint::*;
        use inventory::*;

        #[allow(non_snake_case)]
        unsafe fn $name($obj: *mut UObject, $stack: &mut FFrame, $result: *mut ()) $body
            inventory::submit! {
                BlueprintFunction
                {
                    class: None,
                    function_name: stringify!($name),
                    function: $name,
                }
        }
    };
    ($name:expr, |$obj:ident, $stack:ident, $result:ident| $body:block) => {
        use $crate::ue::fframe::*;
        use $crate::ue::coreuobject::*;
        use $crate::ue::blueprint::*;
        use inventory::*;

        #[allow(non_snake_case)]
        unsafe fn exec($obj: *mut UObject, $stack: &mut FFrame, $result: *mut ()) $body
            inventory::submit! {
                BlueprintFunction
                {
                    class: None,
                    function_name: $name,
                    function: exec,
                }
        }
    };
    ($class: expr, $name:expr, |$obj:ident, $stack:ident, $result:ident| $body:block) => {
        use $crate::ue::fframe::*;
        use $crate::ue::coreuobject::*;
        use $crate::ue::blueprint::*;
        use inventory::*;

        #[allow(non_snake_case)]
        unsafe fn exec($obj: *mut UObject, $stack: &mut FFrame, $result: *mut ()) $body
        inventory::submit! {
            BlueprintFunction
            {
                class: Some($class),
                function_name: $name,
                function: exec,
            }
        }
    };
}
pub(crate) unsafe fn init()
{
    for f in inventory::iter::<BlueprintFunction>
    {
        match f.class
        {
            None => {
                br_print!("Registered {}", f.function_name);
            }
            Some(c) =>
            {
                br_print!("Registered {}::{}", c, f.function_name);
            }
        }
    }
}
