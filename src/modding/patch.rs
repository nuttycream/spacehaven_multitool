use std::{collections::HashMap, error::Error};

use amxml::dom::NodePtr;

use super::{database::Mod, merge::CoreLibrary};
use crate::utils::get_attribute_value_node;

struct PatchArgs<'a> {
    value: &'a str,
    attribute: Option<String>,
    core_lib_elems: &'a [NodePtr],
}

fn attribute_set(patch_args: PatchArgs) {
    // Your implementation here
}

fn attribute_add(patch_args: PatchArgs) {
    // Your implementation here
}

fn attribute_remove(patch_args: PatchArgs) {
    // Your implementation here
}

fn attribute_math(patch_args: PatchArgs) {
    // Your implementation here
}

fn node_add(patch_args: PatchArgs) {
    // Your implementation here
}

fn node_insert(patch_args: PatchArgs) {
    // Your implementation here
}

fn node_remove(patch_args: PatchArgs) {
    // Your implementation here
}

fn node_replace(patch_args: PatchArgs) {
    // Your implementation here
}

fn patch_dispatch(p_type: &str) -> fn(PatchArgs) {
    match p_type {
        "AttributeSet" => attribute_set,
        "AttributeAdd" => attribute_add,
        "AttributeRemove" => attribute_remove,
        "AttributeMath" => attribute_math,
        "Add" => node_add,
        "Insert" => node_insert,
        "Remove" => node_remove,
        "Replace" => node_replace,
        _ => panic!("BAD PATCH OPERATION"),
    }
}

pub fn do_patches(
    core_lib: &mut CoreLibrary,
    mod_lib: &HashMap<String, Vec<NodePtr>>,
    active_mod: &Mod
) -> Result<(), Box<dyn Error>> {
    for (location, patch_list) in mod_lib {
        log::info!("Patching {} for {}", location, active_mod.name);
        for patch_root in patch_list {
            let patch_operations = patch_root.get_nodeset("//Patch/Operation")?;
            for patch_operation in patch_operations {
                let p_type: String = get_attribute_value_node(&patch_operation, "Class")?;
                let xpath = patch_operation
                    .get_first_node("//xpath/text()")
                    .ok_or(format!("Could not find xpath for {}", location))?
                    .value();

                //log::info!("Current xpath op: {}", xpath);
                let current_core_lib_elems =
                    core_lib.node_dictionary[location].get_nodeset(&xpath)?;

                // Get a mutable reference to the patch value
                let mut value = patch_operation
                    .get_first_node("//value/text()")
                    .ok_or(format!("Could not find value for {}", location))?
                    .value();

                // Check if there are any config variables for this mod
                if let Some(config_variables) = &active_mod.config_variables {
                    for (var_name, var_value) in config_variables {
                        value = value.replace(var_name, &var_value.value); // assuming var_value.value is of type String
                    }
                }


                let patch_args = PatchArgs {
                    value: &value,
                    attribute: patch_operation
                        .get_first_node("//attribute/text()")
                        .map(|node| node.value()),

                    core_lib_elems: &current_core_lib_elems,
                };

                patch_dispatch(&p_type)(patch_args);
            }
        }
    }
    Ok(())
}
