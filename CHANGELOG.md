# Change Log

* Consolidated Named & FluentName update mod.rs.
* Adding validation of the FluentName String to alphanumeric or hyphens.
* Updating dependency version numbers to only use MAJOR and MINOR and not PATCH to avoid having to constantly update.
* Adding log warning to when FluentName defaults to blank bacause of bad name.
* I love the `thiserror` crate. 
* REVISION: Prefer doctests over unit tests for basic validation.
* REVISION: Updated Named::fluent_name_string to return &String
* Revision trying to move weighted vector into Named trait.
  * Got new_with_weight moved. 
  * Holy shit it worked. 