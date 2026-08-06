
use brickworks::{br_print, set_module_name};
use inventory::*;
use super::coreuobject::fnProcessInternal;
set_module_name!(b"blueprints\0");

pub struct BlueprintFunction
{
    pub class: Option<&'static str>,
    pub function_name: &'static str,
    pub function: fnProcessInternal,
}

inventory::collect!(BlueprintFunction);

#[macro_export]
macro_rules! bp_function {
    ($name:ident |$obj:ident, $stack:ident, $result:ident| $body:block) => {
        use $crate::ue::fframe::*;
        use $crate::ue::coreuobject::*;
        use $crate::ue::blueprint::*;
        use inventory::*;

        #[allow(non_snake_case)]
        unsafe extern "C" fn $name($obj: *mut UObject, $stack: *mut FFrame, $result: *mut ()) $body
            inventory::submit! {
                BlueprintFunction
                {
                    class: None,
                    function_name: stringify!($name),
                    function: $name,
                }
        }
    };
    ($class: ident $name:ident |$obj:ident, $stack:ident, $result:ident| $body:block) => {
        use $crate::ue::fframe::*;
        use $crate::ue::coreuobject::*;
        use $crate::ue::blueprint::*;
        use inventory::*;

        #[allow(non_snake_case)]
        unsafe extern "C" fn $name($obj: *mut UObject, $stack: *mut FFrame, $result: *mut ()) $body
        inventory::submit! {
            BlueprintFunction
            {
                class: Some(stringify!($class)),
                function_name: stringify!($name),
                function: $name,
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
