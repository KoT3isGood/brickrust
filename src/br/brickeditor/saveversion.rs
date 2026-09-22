pub type FBrickRigsSaveVersion = u8;
// Current save version, the legacy UBrickpub statics version ended at 6
pub const BR_SAVE_VERSION: FBrickRigsSaveVersion = 17;
// Version where the brick units and brick size types were removed
pub const BR_SAVE_BRICK_UNITS_REMOVED_VERSION: FBrickRigsSaveVersion = 15;
// Version where the driver seat stopped being serialized
pub const BR_SAVE_DRIVER_SEAT_BY_INDEX_VERSION: FBrickRigsSaveVersion = 15;
// Version where color was saved as RGB again instead of HSV
pub const BR_SAVE_COLOR_RGB_VERSION: FBrickRigsSaveVersion = 15;
// Version where brick units started to be saved as a float instead of uint16
pub const BR_SAVE_BRICK_UNITS_FLOAT_VERSION: FBrickRigsSaveVersion = 14;
// Version where wheel meshes were moved so their connectors would line up with the origin
pub const BR_SAVE_ALIGNED_WHEEL_ORIGIN_VERSION: FBrickRigsSaveVersion = 14;
// Prior to this version GetNameStringByIndex was used instead of GetNameStringByIndex in FEnumBrickPropertyBase::SerializeProperty
pub const BR_SAVE_FIXED_ENUM_SERIALIZATION_VERSION: FBrickRigsSaveVersion = 13;
// Version where input channel indices where replaced by brick references
pub const BR_SAVE_INPUT_CHANNEL_REFERENCE_VERSION: FBrickRigsSaveVersion = 13;
// Version where item mass and price started to be saved to the meta data
pub const BR_SAVE_MASS_AND_PRICE: FBrickRigsSaveVersion = 12;
// Version where min and max sensor input values were replaced by universal input channel parameters
pub const BR_SAVE_INPUT_CHANNEL_CURVE_VERSION: FBrickRigsSaveVersion = 11;
// Version where the min and max actuation limit properties have been inverted
pub const BR_SAVE_ACTUATOR_LIMITS_VERSION: FBrickRigsSaveVersion = 10;
// Version where suspension damping was made player controllable
pub const BR_SAVE_SUSPENSION_DAMPING_VERSION: FBrickRigsSaveVersion = 9;
// Version where the tags were added to the meta data
pub const BR_SAVE_TAGS_VERSION: FBrickRigsSaveVersion = 8;
// Version where we transition from the UBrickStatics save functions to the UBrickEditorSaveInterface
pub const BR_SAVE_INTERFACE_VERSION: FBrickRigsSaveVersion = 7;
// Version before the save interface was implemented
pub const BR_SAVE_LAST_LEGACY_VERSION: FBrickRigsSaveVersion = 6;
// Version where input channels were introduced and the input axes enum has been changed
pub const BR_SAVE_INPUT_CHANNEL_VERSION: FBrickRigsSaveVersion = 6;
// Legacy version where the brick location accuracy was increased
pub const BR_SAVE_SMALLER_STEPS_VERSION: FBrickRigsSaveVersion = 4;
// Version where the saved element size has been fixed
pub const BR_SAVE_FIXED_ELEMENT_SIZE_VERSION: FBrickRigsSaveVersion = 3;
