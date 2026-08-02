use std::collections::HashMap;

struct Hook
{

}

static mut HOOKS: Option<HashMap<*const (), Hook>> = None;


