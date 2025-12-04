use crate::*;
unsafe extern "C" {
    /// **Stop Repeat**<br/>
    /// Stops a Repeat sequence and
    /// continues to the next code block.
    ///
    /// ## Works With
    /// - `Repeat`
    ///
    pub unsafe fn DF_ACTION__control__StopRepeat(...);
    /// **Return From Function**<br/>
    /// Skips the rest of a Function
    /// sequence and returns to the
    /// block it was called from.
    ///
    /// ## Additional Info
    /// - This occurs automatically
    ///   at the end of a Function.
    ///
    pub unsafe fn DF_ACTION__control__Return(...) -> !;
    /// **Send Debug Message**<br/>
    /// Sends a formatted message to
    /// the specified plot staff group
    /// regardless of which mode
    /// they're currently in.
    /// Clicking on the message will
    /// teleport you to this block.
    ///
    /// ## Tags
    /// - Permission:
    ///   - `Owner`
    ///   - `Developer` (Default)
    ///   - `Builder`
    ///   - `Developer or builder`
    ///   - `Whitelisted`
    ///   - `All`
    /// - Text Value Merging:
    ///   - `Add Spaces` (Default)
    ///   - `No Spaces`
    /// - Highlighting:
    ///   - `None` (Default)
    ///   - `Error`
    ///   - `Warning`
    ///   - `Other`
    /// - Sound:
    ///   - `None`
    ///   - `Default` (Default)
    ///   - `Success`
    ///   - `Error`
    ///   - `Warning`
    ///   - `LagSlayer`
    /// - Message Style:
    ///   - `Custom`
    ///   - `Debug` (Default)
    ///   - `Error`
    ///   - `Warning`
    ///   - `Info`
    ///   - `LagSlayer`
    ///
    /// ## Arguments
    /// - `df_opaque` `[]?` (Any):
    ///   Message to format
    ///
    /// ## Additional Info
    /// - Players must be on the plot
    ///   to receive debug messages.
    /// - Multiple values (of any type)
    ///   will be merged together.
    ///
    pub unsafe fn DF_ACTION__control__PrintDebug(Permission : df_string, TextSPECIALSpace_ValueSPECIALSpace_Merging : df_string, Highlighting : df_string, Sound : df_string, MessageSPECIALSpace_Style : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__control__ReturnNTimes(...) -> !;
    /// **Skip Iteration**<br/>
    /// Skips the rest of this repeat
    /// statement's code and continues
    /// to the next repetition.
    ///
    /// ## Works With
    /// - `Repeat`
    ///
    pub unsafe fn DF_ACTION__control__Skip(...);
    /// **End Thread**<br/>
    /// Stops the current event
    /// thread. Any code after this
    /// block will not be executed.
    ///
    /// ## Additional Info
    /// - This occurs automatically
    ///   when a thread has no valid
    ///   event targets or selection.
    ///   A player target is invalid
    ///   if they have left the game.
    ///
    pub unsafe fn DF_ACTION__control__End(...) -> !;
    /// **Wait**<br/>
    /// Pauses the current code
    /// sequence for a duration of
    /// ticks, seconds, or minutes.
    ///
    /// ## Tags
    /// - Time Unit:
    ///   - `Ticks` (Default)
    ///   - `Seconds`
    ///   - `Minutes`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Wait duration
    ///   - Default = 1
    ///
    /// ## Additional Info
    /// - It is not possible to wait
    ///   fractions of a tick.
    ///
    pub unsafe fn DF_ACTION__control__Wait(TimeSPECIALSpace_Unit : df_string, ...);
    /// **Set to String**<br/>
    /// Sets a variable to a string, or combines
    /// multiple values into one string.
    ///
    /// ## Tags
    /// - Text Value Merging:
    ///   - `Add spaces`
    ///   - `No spaces` (Default)
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_opaque` `[]?` (Any):
    ///   String to set to
    ///
    /// ## Additional Info
    /// - All values will be converted to string.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__String(TextSPECIALSpace_ValueSPECIALSpace_Merging : df_string, ...);
    /// **Get All Regex Groups**<br/>
    /// Gets a list of all capture groups
    /// in a Regex match.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` (String):
    ///   Input string
    /// - `df_string` (String):
    ///   Regex pattern
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   List of all capture groups
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__AllRegexGroups(...);
    /// **Set Bundle Contents**<br/>
    /// Sets the contents of a bundle.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Bundle
    /// - `df_item` `[]?` (Item):
    ///   Contents
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetBundleItems(...);
    /// **Set Particle Effect Type**<br/>
    /// Sets a particle effect's type.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` `?` (Particle):
    ///   Effect to
    ///   change
    /// - `df_string` (String):
    ///   Type
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetParticleType(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemEnchants(...);
    /// **Clear Item Custom Tags**<br/>
    /// Removes all item custom tags.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ClearItemTag(...);
    /// **Purge Matching Variables**<br/>
    /// Clears all variables with names
    /// that match the given text.
    ///
    /// ## Tags
    /// - Match Requirement:
    ///   - `Entire name`
    ///   - `Full word(s) in name` (Default)
    ///   - `Any part of name`
    /// - Ignore Case:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   Name to match
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__PurgeVars(MatchSPECIALSpace_Requirement : df_string, IgnoreSPECIALSpace_Case : df_string, ...);
    /// **Shift Location on All Axes**<br/>
    /// Shifts a location's coordinates
    /// on the X, Y, and Z axes.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` `?` (Location):
    ///   Location to change
    /// - `df_number` `?` (Number):
    ///   X Change
    /// - `df_number` `?` (Number):
    ///   Y Change
    /// - `df_number` `?` (Number):
    ///   Z Change
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ShiftAllAxes(...);
    /// **Get Particle Effect Material**<br/>
    /// Gets a particle effect's particle
    /// display material.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` (Particle):
    ///   Effect to get
    ///   material of
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Material ID
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetParticleMat(...);
    /// **Set Particle Effect Spread**<br/>
    /// Sets a particle effect's horizontal
    /// and vertical spread.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` `?` (Particle):
    ///   Effect to change
    /// - `df_number` (Number):
    ///   Horizontal spread
    /// - `df_number` (Number):
    ///   Vertical spread
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetParticleSprd(...);
    /// **Set to Absolute Value**<br/>
    /// Makes a number positive
    /// if it is negative.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `?` (Number):
    ///   Number input
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__AbsoluteValue(...);
    /// **Remove Item Attributes**<br/>
    /// Removes all attributes from an
    /// item with the given attribute type and slot.
    ///
    /// ## Tags
    /// - Slot:
    ///   - `All` (Default)
    ///   - `Main hand`
    ///   - `Off hand`
    ///   - `Head`
    ///   - `Body`
    ///   - `Legs`
    ///   - `Feet`
    /// - Attribute:
    ///   - `All` (Default)
    ///   - `Armor`
    ///   - `Armor toughness`
    ///   - `Attack damage`
    ///   - `Attack knockback`
    ///   - `Attack speed`
    ///   - `Burning time`
    ///   - `Explosion knockback resistance`
    ///   - `Fall damage multiplier`
    ///   - `Flying speed`
    ///   - `Follow range`
    ///   - `Gravity`
    ///   - `Jump strength`
    ///   - `Knockback resistance`
    ///   - `Luck`
    ///   - `Maximum absorption health`
    ///   - `Maximum health`
    ///   - `Movement efficiency`
    ///   - `Walking speed`
    ///   - `Oxygen bonus`
    ///   - `Safe fall distance`
    ///   - `Scale`
    ///   - `Step height`
    ///   - `Water movement efficiency`
    ///   - `Tempt range`
    ///   - `Block break speed`
    ///   - `Block interaction range`
    ///   - `Entity interaction range`
    ///   - `Mining efficiency`
    ///   - `Sneaking speed`
    ///   - `Submerged mining speed`
    ///   - `Sweeping damage ratio`
    ///   - `Zombie spawn reinforcements`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RemoveItemAttrs(Slot : df_string, Attribute : df_string, ...);
    /// **Append Value to List**<br/>
    /// Adds a value to the end of a list
    /// variable.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   List to append to
    /// - `df_opaque` `[]` (Any):
    ///   Value(s) to append
    ///
    /// ## Additional Info
    /// - Appending a list will result in
    ///   the list itself being appended
    ///   to the list (a nested list).
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__AppendValue(...);
    /// **Set to Remainder (%)**<br/>
    /// Sets a variable to the remainder
    /// after dividing two numbers with
    /// a whole quotient.
    ///
    /// ## Tags
    /// - Remainder Mode:
    ///   - `Remainder` (Default)
    ///   - `Modulo`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` (Number):
    ///   Dividend
    /// - `df_number` (Number):
    ///   Divisor
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALPercent_(RemainderSPECIALSpace_Mode : df_string, ...);
    /// **Shift Location on Vector**<br/>
    /// Shifts a location along a
    /// vector.
    ///
    /// ## Tags
    /// - Add Location Rotation:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` `?` (Location):
    ///   Location to shift
    /// - `df_vector` (Vector):
    ///   Shift vector
    /// - 
    /// - `df_number` (Number):
    ///   Shift distance
    /// - OR
    /// - None:
    ///   Adds full vector
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ShiftOnVector(AddSPECIALSpace_LocationSPECIALSpace_Rotation : df_string, ...);
    /// **Get Item Attribute**<br/>
    /// Get an attribute's
    /// multiplier for a specific slot.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Armor` (Default)
    ///   - `Armor toughness`
    ///   - `Attack damage`
    ///   - `Attack knockback`
    ///   - `Attack speed`
    ///   - `Burning time`
    ///   - `Explosion knockback resistance`
    ///   - `Fall damage multiplier`
    ///   - `Flying speed`
    ///   - `Follow range`
    ///   - `Gravity`
    ///   - `Jump strength`
    ///   - `Knockback resistance`
    ///   - `Luck`
    ///   - `Maximum absorption health`
    ///   - `Maximum health`
    ///   - `Movement efficiency`
    ///   - `Walking speed`
    ///   - `Oxygen bonus`
    ///   - `Safe fall distance`
    ///   - `Scale`
    ///   - `Step height`
    ///   - `Water movement efficiency`
    ///   - `Tempt range`
    ///   - `Block break speed`
    ///   - `Block interaction range`
    ///   - `Entity interaction range`
    ///   - `Mining efficiency`
    ///   - `Sneaking speed`
    ///   - `Submerged mining speed`
    ///   - `Sweeping damage ratio`
    ///   - `Zombie spawn reinforcements`
    /// - Active Equipment Slot:
    ///   - `Any` (Default)
    ///   - `Main hand`
    ///   - `Off hand`
    ///   - `Head`
    ///   - `Body`
    ///   - `Legs`
    ///   - `Feet`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Multiplier of the
    ///   specified item attribute
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemAttribute(Attribute : df_string, ActiveSPECIALSpace_EquipmentSPECIALSpace_Slot : df_string, ...);
    /// **Clear Dictionary**<br/>
    /// Removes all entries from
    /// a dictionary.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Dictionary to clear
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ClearDict(...);
    /// **Add Numbers (+)**<br/>
    /// Sets a variable to the sum of
    /// the given numbers.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `[]` (Number):
    ///   Numbers to add
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALPlus_(...);
    /// **Shift Location Rotation**<br/>
    /// Rotates a location by shifting its pitch
    /// (up/down) or yaw (left/right) value.
    ///
    /// ## Tags
    /// - Rotation Axis:
    ///   - `Pitch` (Default)
    ///   - `Yaw`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` `?` (Location):
    ///   Location to shift
    /// - `df_number` (Number):
    ///   Rotation amount
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ShiftRotation(RotationSPECIALSpace_Axis : df_string, ...);
    /// **Subtract Numbers (-)**<br/>
    /// Sets a variable to the difference
    /// between the given numbers.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `[]` (Number):
    ///   Numbers to subtract
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALHyphen_(...);
    /// **Get Item Name**<br/>
    /// Gets an item's name.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get name of
    ///
    /// ## Returns
    /// - `df_text` (Styled Text):
    ///   Item name
    ///
    /// ## Additional Info
    /// - If the item is unnamed, a \<translate\>
    ///   component is used.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALSpace_GetItemNameSPECIALSpace_(...);
    /// **Get Item Rarity**<br/>
    /// Gets an item's rarity.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   One of the following:
    ///   ■ "common"
    ///   ■ "uncommon"
    ///   ■ "rare"
    ///   ■ "epic"
    ///
    /// ## Additional Info
    /// - Item rarity determines the
    ///   item's default name color.
    /// - An item's rarity is determined
    ///   by its enchantments and type.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemRarity(...);
    /// **Multiply Vector**<br/>
    /// Multiplies a vector's length
    /// by a number.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` `?` (Vector):
    ///   Vector to multiply
    /// - `df_number` (Number):
    ///   Multiplier
    ///
    /// ## Returns
    /// - `df_vector` (Vector):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__MultiplyVector(...);
    /// **Divide Numbers (÷)**<br/>
    /// Sets a variable to the quotient
    /// of the given numbers.
    ///
    /// ## Tags
    /// - Division Mode:
    ///   - `Default` (Default)
    ///   - `Floor result`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `[]` (Number):
    ///   Numbers to divide
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALSlash_(DivisionSPECIALSpace_Mode : df_string, ...);
    ///
    /// ## Tags
    /// - Sign Line:
    ///   - `1` (Default)
    ///   - `2`
    ///   - `3`
    ///   - `4`
    ///   - `All lines`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetSignText(SignSPECIALSpace_Line : df_string, ...);
    /// **Set to Bitwise Operation**<br/>
    /// Sets a variable to the result
    /// of a bitwise operation.
    ///
    /// ## Tags
    /// - Operator:
    ///   - `|` (Default)
    ///   - `&`
    ///   - `~`
    ///   - `^`
    ///   - `<<`
    ///   - `>>`
    ///   - `>>>`
    /// - Bit Precision:
    ///   - `Default` (Default)
    ///   - `64-bit`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` (Number):
    ///   Operand 1
    /// - `df_number` `?` (Number):
    ///   Operand 2
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    /// ## Additional Info
    /// - Input values are treated
    ///   as whole numbers. Decimal
    ///   value is ignored.
    /// - Numbers are stored as
    ///   64-bit signed integers.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Bitwise(Operator : df_string, BitSPECIALSpace_Precision : df_string, ...);
    /// **Get Lectern Page**<br/>
    /// Gets the displayed page
    /// number of a Lectern.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Lectern location
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Page number
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetLecternPage(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ParseX(...);
    /// **Shift Location on Axis**<br/>
    /// Shifts the X, Y, or Z coordinate
    /// of a location on its axis.
    ///
    /// ## Tags
    /// - Coordinate:
    ///   - `X` (Default)
    ///   - `Y`
    ///   - `Z`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` `?` (Location):
    ///   Location to shift
    /// - `df_number` (Number):
    ///   Shift distance
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ShiftOnAxis(Coordinate : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ParseY(...);
    /// **Set to Vector Between Locations**<br/>
    /// Sets a variable to the vector
    /// between two locations.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Start location
    /// - `df_location` (Location):
    ///   End location
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__VectorBetween(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ParseZ(...);
    /// **Set Item Break Sound**<br/>
    /// Sets which sound an item
    /// makes when it breaks.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - 
    /// - `df_sound` (Sound):
    ///   Break sound
    /// - OR
    /// - None:
    ///   Sets to default break sound
    ///
    /// ## Additional Info
    /// - Pitch and volume are ignored.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetBreakSound(...);
    /// **Get Vector Component**<br/>
    /// Gets a vector's X, Y, or Z
    /// component.
    ///
    /// ## Tags
    /// - Component:
    ///   - `X` (Default)
    ///   - `Y`
    ///   - `Z`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` (Vector):
    ///   Vector to get
    ///   component of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Value of the
    ///   specified component
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetVectorComp(Component : df_string, ...);
    /// **Set to Value (=)**<br/>
    /// Sets a variable to a value.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_opaque` (Any):
    ///   Value
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALEqual_(...);
    ///
    /// ## Tags
    /// - Regular Expressions:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RmText(RegularSPECIALSpace_Expressions : df_string, ...);
    /// **Add Item Attribute**<br/>
    /// Adds an attribute modifier to an
    /// item, which is active in a certain
    /// equipment slot.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Armor` (Default)
    ///   - `Armor toughness`
    ///   - `Attack damage`
    ///   - `Attack knockback`
    ///   - `Attack speed`
    ///   - `Burning time`
    ///   - `Explosion knockback resistance`
    ///   - `Fall damage multiplier`
    ///   - `Flying speed`
    ///   - `Follow range`
    ///   - `Gravity`
    ///   - `Jump strength`
    ///   - `Knockback resistance`
    ///   - `Luck`
    ///   - `Maximum absorption health`
    ///   - `Maximum health`
    ///   - `Movement efficiency`
    ///   - `Walking speed`
    ///   - `Oxygen bonus`
    ///   - `Safe fall distance`
    ///   - `Scale`
    ///   - `Step height`
    ///   - `Water movement efficiency`
    ///   - `Tempt range`
    ///   - `Block break speed`
    ///   - `Block interaction range`
    ///   - `Entity interaction range`
    ///   - `Mining efficiency`
    ///   - `Sneaking speed`
    ///   - `Submerged mining speed`
    ///   - `Sweeping damage ratio`
    ///   - `Zombie spawn reinforcements`
    /// - Operation:
    ///   - `Add number` (Default)
    ///   - `Add percentage to base`
    ///   - `Multiply modifier by percentage`
    /// - Active Equipment Slot:
    ///   - `Any`
    ///   - `Main hand` (Default)
    ///   - `Off hand`
    ///   - `Head`
    ///   - `Body`
    ///   - `Legs`
    ///   - `Feet`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item
    /// - `df_number` (Number):
    ///   Modifier Amount
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__AddItemAttribute(Attribute : df_string, Operation : df_string, ActiveSPECIALSpace_EquipmentSPECIALSpace_Slot : df_string, ...);
    /// **Set to Center Location**<br/>
    /// Finds an average position (center)
    /// of the given locations.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` `[]` (Location):
    ///   Locations to center
    ///
    /// ## Returns
    /// - `df_location` (Location):
    ///   Center location
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetCenterLoc(...);
    /// **Decode from Base64**<br/>
    /// Decodes Base64-encoded
    /// bytes as a list of numbers.
    ///
    /// ## Tags
    /// - Signed:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `[]` (Byte):
    ///   Base64 byte(s) to decode
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Bytes as a list of numbers
    ///
    /// ## Additional Info
    /// - Returns 0 if the result is too large.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Base64Decode(Signed : df_string, ...);
    /// **Align Location**<br/>
    /// Aligns a location to the center
    /// or corner of the block it is in.
    ///
    /// ## Tags
    /// - Alignment Mode:
    ///   - `Block center` (Default)
    ///   - `Lower block corner`
    /// - Coordinates:
    ///   - `All coordinates` (Default)
    ///   - `X and Z`
    ///   - `Only Y`
    /// - Rotation:
    ///   - `Keep rotation` (Default)
    ///   - `Remove rotation`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` `?` (Location):
    ///   Location to align
    ///
    /// ## Returns
    /// - `df_location` (Location):
    ///   Aligned location
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__AlignLoc(AlignmentSPECIALSpace_Mode : df_string, Coordinates : df_string, Rotation : df_string, ...);
    /// **Get Sound Volume**<br/>
    /// Gets a sound's volume.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_sound` (Sound):
    ///   Sound to get volume of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Sound volume
    ///
    /// ## Additional Info
    /// - Sound volume decreases based
    ///   on distance to the sound's origin.
    /// - The maximum distance to hear a
    ///   sound is equal to 16.0 × volume.
    /// - At volumes below 1.0, the sound's
    ///   loudness at its origin decreases.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetSoundVolume(...);
    /// **Set to Random Number**<br/>
    /// Sets a variable to a random
    /// number between two other
    /// numbers.
    ///
    /// ## Tags
    /// - Rounding Mode:
    ///   - `Whole number` (Default)
    ///   - `Decimal number`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` (Number):
    ///   Minimum number
    /// - `df_number` (Number):
    ///   Maximum number
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Random number
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RandomNumber(RoundingSPECIALSpace_Mode : df_string, ...);
    /// **Get Crossbow Projectiles**<br/>
    /// Gets a crossbow's
    /// loaded projectile list.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Crossbow
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Contains one Item for
    ///   each loaded projectile
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetCrossbowProj(...);
    /// **Decompress from Gzip**<br/>
    /// Decompresses Gzip-compressed
    /// bytes as a list of numbers.
    ///
    /// ## Tags
    /// - Signed:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `[]` (Byte):
    ///   Byte(s) to decompress
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Bytes as a list of numbers
    ///
    /// ## Additional Info
    /// - Returns 0 if the result is too large.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GzipDecompress(Signed : df_string, ...);
    /// **Get Container Name**<br/>
    /// Gets a container's name at
    /// a location.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Container location
    ///
    /// ## Returns
    /// - `df_text` (Styled Text):
    ///   Container name
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ContainerName(...);
    /// **Raycast from Location**<br/>
    /// Raycasts from a location
    /// to the first intersection.
    ///
    /// ## Tags
    /// - Entity Collision:
    ///   - `True`
    ///   - `False` (Default)
    /// - Block Collision:
    ///   - `All blocks` (Default)
    ///   - `Non-fluid blocks`
    ///   - `Solid blocks`
    ///   - `None`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Ray origin
    /// - `df_number` (Number):
    ///   Ray distance
    ///
    /// ## Returns
    /// - `df_location` (Location):
    ///   End location
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Raycast(EntitySPECIALSpace_Collision : df_string, BlockSPECIALSpace_Collision : df_string, ...);
    /// **Rotate Vector Around Vector**<br/>
    /// Rotates a vector around
    /// another vector by an angle.
    ///
    /// ## Tags
    /// - Angle Units:
    ///   - `Degrees` (Default)
    ///   - `Radians`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` `?` (Vector):
    ///   Vector to rotate
    /// - `df_vector` (Vector):
    ///   Axis vector
    /// - `df_number` (Number):
    ///   Angle
    ///
    /// ## Returns
    /// - `df_vector` (Vector):
    ///   Rotated vector
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RotateAroundVec(AngleSPECIALSpace_Units : df_string, ...);
    ///
    /// ## Tags
    /// - Can Always Eat:
    ///   - `True`
    ///   - `False` (Default)
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemFood(CanSPECIALSpace_AlwaysSPECIALSpace_Eat : df_string, ...);
    /// **Get Particle Effect Motion**<br/>
    /// Gets a particle effect's particle
    /// motion.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` (Particle):
    ///   Effect to get
    ///   motion of
    ///
    /// ## Returns
    /// - `df_vector` (Vector):
    ///   Particle motion
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetParticleMotion(...);
    /// **Set Item Tooltip Style**<br/>
    /// Sets an item's custom tooltip.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_string` `?` (String):
    ///   Tooltip key
    ///   - A namespaced key ("minecraft:" by default)
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetTooltipStyle(...);
    /// **Set Consumable Properties**<br/>
    /// Adds consumable properties to an item.
    ///
    /// ## Tags
    /// - Can Always Eat:
    ///   - `True`
    ///   - `False` (Default)
    /// - Show Particles:
    ///   - `True` (Default)
    ///   - `False`
    /// - Consuming Animation:
    ///   - `None`
    ///   - `Eat` (Default)
    ///   - `Drink`
    ///   - `Block`
    ///   - `Bow`
    ///   - `Spear`
    ///   - `Crossbow`
    ///   - `Spyglass`
    ///   - `Toot horn`
    ///   - `Brush`
    ///   - `Bundle`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_sound` `?` (Sound):
    ///   Consuming sound
    /// - `df_number` `?` (Number):
    ///   Nutrition
    /// - `df_number` `?` (Number):
    ///   Saturation
    /// - `df_number` `?` (Number):
    ///   Use duration
    ///   (seconds)
    ///   - Default = 1.6
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetConsumable(CanSPECIALSpace_AlwaysSPECIALSpace_Eat : df_string, ShowSPECIALSpace_Particles : df_string, ConsumingSPECIALSpace_Animation : df_string, ...);
    /// **Set Particle Effect Motion**<br/>
    /// Sets a particle effect's particle
    /// motion and motion variation.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` `?` (Particle):
    ///   Effect to
    ///   change
    /// - `df_vector` `?` (Vector):
    ///   Particle motion
    /// - `df_number` `?` (Number):
    ///   Motion variation (%)
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetParticleMotion(...);
    /// **Set to Average Number**<br/>
    /// Sets a variable to the average
    /// of the given numbers.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `[]` (Number):
    ///   Numbers to average
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Average(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__WrapNumber(...);
    /// **Clamp Location**<br/>
    /// Clamps a location to a region
    /// defined by 2 corners.
    ///
    /// ## Tags
    /// - Coordinates:
    ///   - `All coordinates` (Default)
    ///   - `X and Z`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` `?` (Location):
    ///   Location to clamp
    /// - `df_location` (Location):
    ///   Corner 1
    /// - `df_location` (Location):
    ///   Corner 2
    ///
    /// ## Returns
    /// - `df_location` (Location):
    ///   Clamped location
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ClampLoc(Coordinates : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetY(...);
    /// **Set Map Texture**<br/>
    /// Sets a map item's texture to the
    /// image at the given URL.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_string` (String):
    ///   Image URL
    ///
    /// ## Additional Info
    /// - Cannot be used more than 10
    ///   times without delay. An additional
    ///   usage is given every 5 seconds.
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetMapTexture(...);
    /// **Get Block Data**<br/>
    /// Gets a block state tag
    /// value at a location.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Block location
    /// - `df_string` (String):
    ///   Tag name
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Tag value
    ///
    /// ## Additional Info
    /// - Sets the variable to "0" if the
    ///   tag is not present on the block.
    /// - Always returns a string value.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetBlockData(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetX(...);
    /// **Sort Dictionary**<br/>
    /// Sorts a dictionary
    /// by its keys or values.
    ///
    /// ## Tags
    /// - Sorting Type:
    ///   - `Sort by Key` (Default)
    ///   - `Sort by Value`
    /// - Sorting Order:
    ///   - `Ascending` (Default)
    ///   - `Descending`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_dict` `?` (Dict):
    ///   Dictionary to sort
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SortDict(SortingSPECIALSpace_Type : df_string, SortingSPECIALSpace_Order : df_string, ...);
    /// **Get Lectern Book**<br/>
    /// Gets the book on the
    /// Lectern at a location.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Lectern location
    ///
    /// ## Returns
    /// - `df_item` (Item):
    ///   Lectern book
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetLecternBook(...);
    /// **Get Custom Sound Key**<br/>
    /// Gets the key of a custom sound.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_sound` (Sound):
    ///   Sound to get key of
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Sound key
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetCustomSound(...);
    /// **Set to Cross Product**<br/>
    /// Sets a variable to the cross
    /// product of two vectors.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` (Vector):
    ///   Vector 1
    /// - `df_vector` (Vector):
    ///   Vector 2
    ///
    /// ## Returns
    /// - `df_vector` (Vector):
    ///   Cross product
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__CrossProduct(...);
    /// **Multiply Numbers (×)**<br/>
    /// Sets a variable to the product
    /// of the given numbers.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `[]` (Number):
    ///   Numbers to multiply
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__x(...);
    /// **Get Particle Effect Roll**<br/>
    /// Gets a particle effect's roll.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` (Particle):
    ///   Effect to get
    ///   roll of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Particle roll
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetParticleRoll(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ParseYaw(...);
    /// **Set to Dot Product**<br/>
    /// Sets a variable to the dot
    /// product of two vectors.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` (Vector):
    ///   Vector 1
    /// - `df_vector` (Vector):
    ///   Vector 2
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Dot product
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__DotProduct(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetZ(...);
    /// **Set Armor Trim**<br/>
    /// Sets the trim of an
    /// armor item.
    ///
    /// ## Tags
    /// - Trim Pattern:
    ///   - `None` (Default)
    ///   - `Bolt`
    ///   - `Coast`
    ///   - `Dune`
    ///   - `Eye`
    ///   - `Flow`
    ///   - `Rib`
    ///   - `Sentry`
    ///   - `Snout`
    ///   - `Spire`
    ///   - `Tide`
    ///   - `Vex`
    ///   - `Ward`
    ///   - `Wayfinder`
    ///   - `Shaper`
    ///   - `Silence`
    ///   - `Raiser`
    ///   - `Host`
    ///   - `Wild`
    /// - Trim Material:
    ///   - `Amethyst` (Default)
    ///   - `Copper`
    ///   - `Diamond`
    ///   - `Emerald`
    ///   - `Gold`
    ///   - `Iron`
    ///   - `Lapis Lazuli`
    ///   - `Netherite`
    ///   - `Quartz`
    ///   - `Resin`
    ///   - `Redstone`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetArmorTrim(TrimSPECIALSpace_Pattern : df_string, TrimSPECIALSpace_Material : df_string, ...);
    /// **Pop List Value**<br/>
    /// Gets a list variable's value at
    /// an index and removes it.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_list` (List):
    ///   List to get value of
    /// - `df_number` `?` (Number):
    ///   Index
    ///   - Default = Last index
    ///
    /// ## Returns
    /// - `df_opaque` (Any):
    ///   List value
    ///
    /// ## Additional Info
    /// - List indices start at 1.
    /// - If the index is outside the
    ///   given list, 0 is returned.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__PopListValue(...);
    /// **Set Particle Effect Opacity**<br/>
    /// Sets a particle effect's opacity.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` `?` (Particle):
    ///   Effect to
    ///   change
    /// - `df_number` (Number):
    ///   Particle opacity
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetParticleOpac(...);
    /// **Get Noise**<br/>
    /// Gets a seeded noise value.
    ///
    /// ## Tags
    /// - Return Type:
    ///   - `Simplex` (Default)
    ///   - `Perlin`
    ///   - `Voronoi`
    ///   - `Worley`
    ///   - `Value`
    /// - Dimensions:
    ///   - `3D` (Default)
    ///   - `2D`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Noise location
    /// - `df_number` `?` (Number):
    ///   Frequency
    ///   - Modifies the scale that the
    ///     noise generates at. Default = 1
    /// - `df_number` `?` (Number):
    ///   Octaves
    ///   - Adds an additional layer of noise
    ///     to the output. 1-16, Default = 1
    /// - `df_number` `?` (Number):
    ///   Lacunarity
    ///   - Every time a new octave is added,
    ///     the frequency is multiplied
    ///     by this value. Default = 1.5
    /// - `df_number` `?` (Number):
    ///   Gain
    ///   - Every time a new octave is added,
    ///     the weight of it is multiplied
    ///     by this value. Default = 0.5
    /// - `df_number` `?` (Number):
    ///   Generation seed
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   A decimal value between 0 and 1
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Noise(ReturnSPECIALSpace_Type : df_string, Dimensions : df_string, ...);
    /// **Set to Minimum Number**<br/>
    /// Sets a variable to the lowest
    /// number in a set.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `[]` (Number):
    ///   Number set
    ///   to compare
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__MinNumber(...);
    /// **Get Potion Effect Type**<br/>
    /// Gets a potion effect's type.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_potion` (Potion):
    ///   Potion to get
    ///   type of
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Effect type
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetPotionType(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemName(...);
    /// **Get List Length**<br/>
    /// Gets the number of values
    /// a list has.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_list` (List):
    ///   List to measure
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   List length
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ListLength(...);
    /// **Set to Sine**<br/>
    /// Sets a variable to the trigonometric
    /// sine function of a number.
    ///
    /// ## Tags
    /// - Sine Variant:
    ///   - `Sine` (Default)
    ///   - `Inverse sine (arcsine)`
    ///   - `Hyperbolic sine`
    /// - Input:
    ///   - `Degrees` (Default)
    ///   - `Radians`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` (Number):
    ///   Number input
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    /// ## Additional Info
    /// - Use the variant tag to swap between
    ///   sin, asin and sinh functions.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Sine(SineSPECIALSpace_Variant : df_string, Input : df_string, ...);
    /// **Set to Direction Name**<br/>
    /// Sets a variable to the name
    /// of the direction of a vector.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` (Vector):
    ///   Direction
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   One of the following:
    ///   "north", "east", "south", "west", 
    ///   "up", "down"
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__DirectionName(...);
    /// **Repeat String**<br/>
    /// Repeats a string the given number
    /// of times.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` (String):
    ///   String to repeat
    /// - `df_number` (Number):
    ///   Times to repeat
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Repeated string
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RepeatString(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemLore(...);
    /// **Join String**<br/>
    /// Combines a list of strings.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_list` (List):
    ///   Strings to join
    /// - `df_string` `?` (String):
    ///   Joining string
    /// - `df_string` `?` (String):
    ///   Final joining string
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Joined string
    ///
    /// ## Additional Info
    /// - The joining string will be added
    ///   between each of the joined strings.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__JoinString(...);
    /// **Reverse List**<br/>
    /// Reverses the order of a
    /// list variable's values.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_list` `?` (List):
    ///   List to reverse
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Reversed list
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ReverseList(...);
    /// **Remove Duplicate List Elements**<br/>
    /// Removes list elements that appear
    /// more than once.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_list` `?` (List):
    ///   List to de-duplicate
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Modified list
    ///
    /// ## Additional Info
    /// - Only the first appearance of
    ///   a value is kept.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__DedupList(...);
    /// **Create Dictionary**<br/>
    /// Creates a dictionary with the
    /// given keys and values.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_list` `?` (List):
    ///   Key list
    /// - `df_list` `?` (List):
    ///   Value list
    ///
    /// ## Returns
    /// - `df_dict` (Dict):
    ///   A dictionary of
    ///   specified keys and values
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__CreateDict(...);
    /// **Get Item Use Leftover**<br/>
    /// Gets an item's leftover item that
    /// gets replaced on use, e.g. drinking
    /// stew gives a bowl.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get remainder of
    ///
    /// ## Returns
    /// - `df_item` (Item):
    ///   Remainder item
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemLeftover(...);
    /// **Get Blocks by Minecraft Tag**<br/>
    /// Gets a list of blocks that
    /// belongs to the specified
    /// block tag.
    ///
    /// ## Tags
    /// - Return Value Type:
    ///   - `Material ID (golden_apple)` (Default)
    ///   - `Material Name (Golden Apple)`
    ///   - `Item`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` (String):
    ///   Block tag ID
    ///   - Example: "#mineable/pickaxe"
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Materials
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetBlockByMCTag(ReturnSPECIALSpace_ValueSPECIALSpace_Type : df_string, ...);
    ///
    /// ## Tags
    /// - Round Mode:
    ///   - `Floor`
    ///   - `Nearest` (Default)
    ///   - `Ceiling`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RoundNumber(RoundSPECIALSpace_Mode : df_string, ...);
    /// **Face Location**<br/>
    /// Sets a location's rotation to
    /// face another location.
    ///
    /// ## Tags
    /// - Face Direction:
    ///   - `Toward location` (Default)
    ///   - `Away from location`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` `?` (Location):
    ///   Location to change
    /// - `df_location` (Location):
    ///   Target location
    ///
    /// ## Returns
    /// - `df_location` (Location):
    ///   Facing location
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__FaceLocation(FaceSPECIALSpace_Direction : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemLoreLine(...);
    /// **Set Vector Length**<br/>
    /// Sets a vector's length. This
    /// affects all components.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` `?` (Vector):
    ///   Vector to change
    /// - `df_number` `?` (Number):
    ///   Length
    ///   - Default = 1
    ///
    /// ## Additional Info
    /// - Vectors with an X, Y, and Z
    ///   of zero will not be affected.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetVectorLength(...);
    /// **Set Potion Effect Duration**<br/>
    /// Sets a potion effect's duration.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_potion` `?` (Potion):
    ///   Potion to change
    /// - `df_number` (Number):
    ///   Duration (ticks)
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetPotionDur(...);
    /// **Get Block Blast Resistance**<br/>
    /// Gets a block's blast resistance.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Block to check
    /// - OR
    /// - `df_location` (Location):
    ///   Location to check
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Blast resistance
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__BlockResistance(...);
    /// **Split String**<br/>
    /// Splits a string into a list of strings.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` (String):
    ///   String to split
    /// - `df_string` `?` (String):
    ///   Splitter string
    ///   - Default = " "
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Contains one String for
    ///   each split part
    ///
    /// ## Additional Info
    /// - Leading and trailing spaces
    ///   are removed from the result.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SplitString(...);
    /// **Set to Normally Distributed Random Number**<br/>
    /// Sets a variable to a normally distributed
    /// random number. Values closer to μ are
    /// more likely to be chosen.
    ///
    /// ## Tags
    /// - Distribution:
    ///   - `Normal` (Default)
    ///   - `Folded normal`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` (Number):
    ///   μ (Mean midpoint)
    /// - `df_number` (Number):
    ///   σ (Standard deviation)
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Random number
    ///
    /// ## Additional Info
    /// - Also known as a Gaussian distribution.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__NormalRandom(Distribution : df_string, ...);
    /// **Set Potion Effect Type**<br/>
    /// Sets a potion effect's type.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_potion` `?` (Potion):
    ///   Potion to change
    /// - `df_string` (String):
    ///   Type
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetPotionType(...);
    /// **Get Regex Capture Group**<br/>
    /// Gets the value of a specific Regex Group
    /// in a Regex match.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` (String):
    ///   Input string
    /// - `df_string` (String):
    ///   Regex pattern
    /// - 
    /// - `df_string` (String):
    ///   Group name
    /// - OR
    /// - `df_number` (Number):
    ///   Group index
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Group content
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetRegexGroup(...);
    /// **Align Vector**<br/>
    /// Aligns a vector to the
    /// nearest axis.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` `?` (Vector):
    ///   Vector to align
    ///
    /// ## Returns
    /// - `df_vector` (Vector):
    ///   Aligned vector
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__AlignVector(...);
    /// **Set Item Durability**<br/>
    /// Sets an item's durability.
    ///
    /// ## Tags
    /// - Durability Type:
    ///   - `Set Damage` (Default)
    ///   - `Set Damage Percentage`
    ///   - `Set Remaining`
    ///   - `Set Remaining Percentage`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item
    /// - `df_number` (Number):
    ///   Item durability
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemDura(DurabilitySPECIALSpace_Type : df_string, ...);
    /// **Set Item Breakability**<br/>
    /// Sets whether an item is
    /// unbreakable.
    ///
    /// ## Tags
    /// - Breakability:
    ///   - `Breakable`
    ///   - `Unbreakable` (Default)
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetBreakability(Breakability : df_string, ...);
    /// **Set Item Maximum Stack Size**<br/>
    /// Set an item's maximum stack size.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_number` `?` (Number):
    ///   Maximum stack size
    ///   (1-99)
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetMaxAmount(...);
    /// **Get Sign Text**<br/>
    /// Gets a sign's text at a location.
    ///
    /// ## Tags
    /// - Sign Line:
    ///   - `1` (Default)
    ///   - `2`
    ///   - `3`
    ///   - `4`
    ///   - `All lines`
    /// - Sign Side:
    ///   - `Front` (Default)
    ///   - `Back`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Location
    ///
    /// ## Returns
    /// - `df_text` (Styled Text):
    ///   Sign text
    /// - OR
    /// - `df_list` (List):
    ///   Contains one Styled Text
    ///   for each line of the sign
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALSpace_GetSignTextSPECIALSpace_(SignSPECIALSpace_Line : df_string, SignSPECIALSpace_Side : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RaycastEntity(...);
    /// **Set Dictionary Value**<br/>
    /// Sets the given key to the value.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Dictionary to add to
    /// - `df_string` (String):
    ///   Key
    /// - `df_opaque` (Any):
    ///   Value
    ///
    /// ## Additional Info
    /// - Setting a value to an existing
    ///   key, will overwrite the old value.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetDictValue(...);
    /// **Set All Location Coordinates**<br/>
    /// Sets a location's coordinates or
    /// creates a new location.
    ///
    /// ## Tags
    /// - Coordinate Type:
    ///   - `Plot coordinate` (Default)
    ///   - `World coordinate`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` `?` (Location):
    ///   Location to change
    /// - `df_number` `?` (Number):
    ///   New X
    /// - `df_number` `?` (Number):
    ///   New Y
    /// - `df_number` `?` (Number):
    ///   New Z
    /// - `df_number` `?` (Number):
    ///   New Pitch
    /// - `df_number` `?` (Number):
    ///   New Yaw
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetAllCoords(CoordinateSPECIALSpace_Type : df_string, ...);
    /// **Set to RGB Color**<br/>
    /// Creates a color hex based on red,
    /// green, and blue channels.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - 
    /// - `df_number` (Number):
    ///   Red (0-255)
    /// - `df_number` (Number):
    ///   Green (0-255)
    /// - `df_number` (Number):
    ///   Blue (0-255)
    /// - OR
    /// - `df_list` (List):
    ///   R, G, B values
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Hex code of the color
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RGBColor(...);
    /// **Set Breakable Blocks**<br/>
    /// Sets which blocks an item
    /// can break in Adventure Mode.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_item` `[]` (Block):
    ///   Breakable blocks
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetCanDestroy(...);
    /// **Set to HSL Color**<br/>
    /// Creates a color based on hue,
    /// saturation, and lightness.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - 
    /// - `df_number` (Number):
    ///   Hue (Color circle, 0-360)
    /// - `df_number` `?` (Number):
    ///   Saturation (0-100)
    ///   - Default = 100
    /// - `df_number` `?` (Number):
    ///   Lightness (0-100)
    ///   - Default = 50
    /// - OR
    /// - `df_list` (List):
    ///   H, S, L values
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Hex code of the color
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__HSLColor(...);
    ///
    /// ## Tags
    /// - Return Type:
    ///   - `Text (3D)` (Default)
    ///   - `Text (2D)`
    ///   - `Vector`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALSpace_GetDirectionSPECIALSpace_(ReturnSPECIALSpace_Type : df_string, ...);
    /// **Get Item Lore**<br/>
    /// Gets an item's lore list.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get lore from
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Contains one Styled Text
    ///   for each line in the lore
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALSpace_GetItemLoreSPECIALSpace_(...);
    /// **Remove List Value at Index**<br/>
    /// Removes a list variable's value
    /// at an index and shifts all values
    /// after it to the left.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   List to change
    /// - `df_number` `[]` (Number):
    ///   Index to remove
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RemoveListIndex(...);
    /// **Get Cellular Noise**<br/>
    /// Gets cellular noise: A type of noise
    /// based on distance from cell origins.
    ///
    /// ## Tags
    /// - Return Type:
    ///   - `Voronoi` (Default)
    ///   - `Worley`
    ///   - `Secondary`
    ///   - `Additive`
    ///   - `Subtractive`
    ///   - `Divisive`
    ///   - `Multiplicative`
    ///   - `Origin`
    /// - Distance Calculation:
    ///   - `Euclidean` (Default)
    ///   - `Manhattan`
    ///   - `Natural`
    /// - Domain Fractal:
    ///   - `Progressive` (Default)
    ///   - `Independent`
    /// - Dimensions:
    ///   - `3D` (Default)
    ///   - `2D`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Noise location
    /// - `df_number` `?` (Number):
    ///   Frequency
    ///   - Modifies the scale that the
    ///     noise generates at. Default = 1
    /// - `df_number` `?` (Number):
    ///   Marbling
    ///   - Every 0.5 adds an additional ring
    ///     to the noise output. Default = 0.5
    /// - `df_number` `?` (Number):
    ///   Octaves
    ///   - Adds an additional layer of cellular noise
    ///     to the output. 1-16, Default = 1
    /// - `df_number` `?` (Number):
    ///   Lacunarity
    ///   - Every time a new octave is added,
    ///     the frequency is multiplied
    ///     by this value. Default = 1.5
    /// - `df_number` `?` (Number):
    ///   Gain
    ///   - Every time a new octave is added,
    ///     the weight of it is multiplied
    ///     by this value. Default = 0.5
    /// - `df_number` `?` (Number):
    ///   Warping
    ///   - Distorts the noise output. Default = 0
    /// - `df_number` `?` (Number):
    ///   Resonance
    ///   - Modifies the frequency of warping. Default = 1
    /// - `df_number` `?` (Number):
    ///   Domains
    ///   - Adds an additional layer of warping
    ///     to the output. Default = 1
    /// - `df_number` `?` (Number):
    ///   Domain lacunarity
    ///   - Every time a new domain is added,
    ///     the resonance is multiplied
    ///     by this value. Default = 1.5
    /// - `df_number` `?` (Number):
    ///   Domain gain
    ///   - Every time a new domain is added,
    ///     the weight of it is multiplied
    ///     by this value. Default = 0.5
    /// - `df_number` `?` (Number):
    ///   Generation seed
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   A decimal value between 0 and 1
    /// - OR
    /// - `df_vector` (Vector):
    ///   A vector pointing towards the cell origin.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__CellularNoise(ReturnSPECIALSpace_Type : df_string, DistanceSPECIALSpace_Calculation : df_string, DomainSPECIALSpace_Fractal : df_string, Dimensions : df_string, ...);
    /// **Encode to Base64**<br/>
    /// Encodes bytes as a list
    /// of numbers to Base64.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `[]` (Byte):
    ///   Byte(s) to encode
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Bytes as a list of numbers
    ///
    /// ## Additional Info
    /// - Returns 0 if the result is too large.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Base64Encode(...);
    /// **Set to Logarithm**<br/>
    /// Finds the logarithm of a number.
    /// A logarithm is the power the
    /// base must be raised to to get
    /// the given input.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `?` (Number):
    ///   Number input
    /// - `df_number` (Number):
    ///   Base
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    /// ## Additional Info
    /// - If the result of the logarithm
    ///   is undefined, 0 will be returned.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Logarithm(...);
    /// **Set Item Custom Tag**<br/>
    /// Sets the value of or creates
    /// a custom stored tag value.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_string` (String):
    ///   Tag name
    /// - 
    /// - `df_number` (Number):
    ///   Tag value
    /// - OR
    /// - `df_string` (String):
    ///   Tag value
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemTag(...);
    /// **Trim String**<br/>
    /// Trims a string, starting and ending
    /// at the given positions.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` `?` (String):
    ///   String to trim
    /// - `df_number` (Number):
    ///   Start character position
    /// - `df_number` `?` (Number):
    ///   End character position
    ///   - Default = String length
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   String trimmed from
    ///   the original string
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__TrimString(...);
    ///
    /// ## Tags
    /// - Allowed Tags:
    ///   - `Style Only` (Default)
    ///   - `Dynamic`
    ///   - `Full`
    /// - Parse Legacy Color Codes:
    ///   - `True`
    ///   - `False` (Default)
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ParseMiniMessageExpr(AllowedSPECIALSpace_Tags : df_string, ParseSPECIALSpace_LegacySPECIALSpace_ColorSPECIALSpace_Codes : df_string, ...);
    /// **Get Item Stack Size**<br/>
    /// Gets an item's stack size.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get stack
    ///   size of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Stack size
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemAmount(...);
    /// **Set Potion Effect Amplifier**<br/>
    /// Sets a potion effect's amplifier.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_potion` `?` (Potion):
    ///   Potion to change
    /// - `df_number` (Number):
    ///   Amplifier
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetPotionAmp(...);
    /// **Get Breakable Blocks**<br/>
    /// Gets which blocks an item
    /// can break in Adventure Mode.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Contains one Item for
    ///   each breakable block
    ///
    /// ## Additional Info
    /// - Output is in arbitrary order.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetCanDestroy(...);
    /// **Rotate Vector Around Axis**<br/>
    /// Rotates a vector around an
    /// axis by an angle.
    ///
    /// ## Tags
    /// - Axis:
    ///   - `X` (Default)
    ///   - `Y`
    ///   - `Z`
    /// - Angle Units:
    ///   - `Degrees` (Default)
    ///   - `Radians`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` `?` (Vector):
    ///   Vector to rotate
    /// - `df_number` (Number):
    ///   Angle
    ///
    /// ## Returns
    /// - `df_vector` (Vector):
    ///   Rotated vector
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RotateAroundAxis(Axis : df_string, AngleSPECIALSpace_Units : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemName(...);
    /// **Get Item Durability**<br/>
    /// Gets an item's current or
    /// maximum durability.
    ///
    /// ## Tags
    /// - Durability Type:
    ///   - `Get Damage` (Default)
    ///   - `Get Damage Percentage`
    ///   - `Get Remaining`
    ///   - `Get Remaining Percentage`
    ///   - `Get Maximum`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Item durability
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemDura(DurabilitySPECIALSpace_Type : df_string, ...);
    /// **Shift Location in Direction**<br/>
    /// Shifts a location forward, upward,
    /// or sideways based on its rotation.
    ///
    /// ## Tags
    /// - Direction:
    ///   - `Forward` (Default)
    ///   - `Upward`
    ///   - `Sideways`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` `?` (Location):
    ///   Location to shift
    /// - `df_number` `?` (Number):
    ///   Shift distance
    ///   - Default = 1
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ShiftInDirection(Direction : df_string, ...);
    /// **Set to Vector from Rotation**<br/>
    /// Sets a variable to a vector
    /// facing a given rotation.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` (Number):
    ///   Pitch
    /// - `df_number` (Number):
    ///   Yaw
    /// - `df_number` `?` (Number):
    ///   Length
    ///   - Default = 1
    ///
    /// ## Returns
    /// - `df_vector` (Vector):
    ///   Vector facing the
    ///   specified rotation.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RotationVector(...);
    /// **Convert JSON to Value**<br/>
    /// Converts JSON to a value.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` (String):
    ///   JSON to convert
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Converted value
    /// - OR
    /// - `df_string` (String):
    ///   Converted value
    /// - OR
    /// - `df_list` (List):
    ///   Converted value
    /// - OR
    /// - `df_dict` (Dict):
    ///   Converted value
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__JsonToValue(...);
    /// **Wrap Number**<br/>
    /// Checks if a number is inside
    /// the bounds and if not, wraps
    /// it around the farthest bound.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `?` (Number):
    ///   Number to wrap
    /// - `df_number` (Number):
    ///   Lower bound (inclusive)
    /// - `df_number` (Number):
    ///   Upper bound (exclusive)
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    /// ## Examples
    /// - `WrapNum(10, 2, 8) = 4`
    /// - `WrapNum(4, 1, 4) = 1`
    /// - `WrapNum(-2, 0, 7) = 5`
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__WrapNum(...);
    /// **Set Custom Model Data Numbers**<br/>
    /// Sets the numerical model values
    /// on an item, used in resource packs.
    ///
    /// ## Tags
    /// - Model Value Type:
    ///   - `Floats` (Default)
    ///   - `Flags`
    ///   - `Colors`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_number` `[]` (Number):
    ///   Number model value(s)
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetModelDataNums(ModelSPECIALSpace_ValueSPECIALSpace_Type : df_string, ...);
    /// **Replace String**<br/>
    /// Searches for part of a string
    /// and replaces it.
    ///
    /// ## Tags
    /// - Regular Expressions:
    ///   - `Enable`
    ///   - `Disable` (Default)
    /// - Replacement Type:
    ///   - `First occurrence`
    ///   - `All occurrences` (Default)
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable
    ///   to set
    /// - `df_string` (String):
    ///   String to change
    /// - `df_string` (String):
    ///   String part to replace
    /// - `df_string` (String):
    ///   Replacement
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Modified string
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ReplaceString(RegularSPECIALSpace_Expressions : df_string, ReplacementSPECIALSpace_Type : df_string, ...);
    /// **Set Item Glowing**<br/>
    /// Sets whether an item is
    /// glowing regardless of
    /// its enchantments.
    ///
    /// ## Tags
    /// - Glowing:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///   - `Default`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemGlowing(Glowing : df_string, ...);
    /// **Set Item Name**<br/>
    /// Sets an item's name.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_text` `[]` (Styled Text):
    ///   Name
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALSpace_SetItemNameSPECIALSpace_(...);
    /// **Set Compass Lodestone Location**<br/>
    /// Sets a compass's lodestone location.
    ///
    /// ## Tags
    /// - Require Lodestone at Location:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_location` (Location):
    ///   Lodestone location
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetLodestoneLoc(RequireSPECIALSpace_LodestoneSPECIALSpace_atSPECIALSpace_Location : df_string, ...);
    /// **Flatten List**<br/>
    /// Sets a variable to a list with
    /// its sub-lists spread out
    /// into individual elements.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_list` `?` (List):
    ///   List to flatten
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Flattened list
    ///
    /// ## Examples
    /// - `\[\[1, 2\], 5, \[3, 4\]\] becomes \[1, 2, 5, 3, 4\]`
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__FlattenList(...);
    /// **Set Item Rarity**<br/>
    /// Sets an item's rarity, affecting its
    /// default name color.
    ///
    /// ## Tags
    /// - Rarity:
    ///   - `Default for item type` (Default)
    ///   - `Common`
    ///   - `Uncommon`
    ///   - `Rare`
    ///   - `Epic`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemRarity(Rarity : df_string, ...);
    /// **Get Block Hardness**<br/>
    /// Gets a block's hardness value.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Block to check
    /// - OR
    /// - `df_location` (Location):
    ///   Location to check
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Block hardness
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__BlockHardness(...);
    /// **Get Potion Effect Amplifier**<br/>
    /// Gets a potion effect's amplifier.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_potion` (Potion):
    ///   Potion to get
    ///   amplifier of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Potion amplifier
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetPotionAmp(...);
    /// **Get Bundle Contents**<br/>
    /// Gets the contents of a bundle.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Bundle
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Contains one Item for
    ///   each item in the bundle
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetBundleItems(...);
    /// **Get Particle Effect Amount**<br/>
    /// Gets a particle effect's particle
    /// amount.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` (Particle):
    ///   Effect to get
    ///   amount of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Particle amount
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetParticleAmount(...);
    /// **Set to Random Vector**<br/>
    /// Creates a random vector that
    /// lies on a sphere. Generated
    /// vectors will be uniformly
    /// distributed on its surface.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `?` (Number):
    ///   Vector length
    ///   - Default = 1
    ///
    /// ## Returns
    /// - `df_vector` (Vector):
    ///   A random vector
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RandomVector(...);
    /// **Get Dictionary Size**<br/>
    /// Gets the number of entries
    /// in a dictionary.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_dict` (Dict):
    ///   Dictionary to
    ///   measure
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Dictionary size
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetDictSize(...);
    /// **Set Item Stack Size**<br/>
    /// Sets an item's stack size.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_number` (Number):
    ///   Stack size
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemAmount(...);
    /// **Subtract Vectors**<br/>
    /// Sets a variable to the difference
    /// between the given vectors.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` `[]` (Vector):
    ///   Vectors to subtract
    ///
    /// ## Returns
    /// - `df_vector` (Vector):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SubtractVectors(...);
    /// **Set String Case**<br/>
    /// Sets a string's capitalization
    /// (eg. to uppercase).
    ///
    /// ## Tags
    /// - Capitalization Type:
    ///   - `UPPERCASE` (Default)
    ///   - `lowercase`
    ///   - `Proper Case`
    ///   - `iNVERT CASE`
    ///   - `RAnDoM cASe`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` `?` (String):
    ///   String to change
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Modified string
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetCase(CapitalizationSPECIALSpace_Type : df_string, ...);
    /// **Swap Vector Components**<br/>
    /// Swaps two of a vector's
    /// X, Y, or Z components.
    ///
    /// ## Tags
    /// - Components to Swap:
    ///   - `X ⇄ Y` (Default)
    ///   - `X ⇄ Z`
    ///   - `Y ⇄ Z`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` `?` (Vector):
    ///   Vector to swap
    ///   components of
    ///
    /// ## Returns
    /// - `df_vector` (Vector):
    ///   The vector with
    ///   swapped components
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SwapVectorComp(ComponentsSPECIALSpace_toSPECIALSpace_Swap : df_string, ...);
    /// **Set Particle Effect Color**<br/>
    /// Sets a particle effect's particle
    /// color and color variation.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` `?` (Particle):
    ///   Effect to
    ///   change
    /// - `df_string` (String):
    ///   Color hexadecimal
    ///   - Example: "#FF0000" (red)
    /// - `df_number` `?` (Number):
    ///   Color variation (%)
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetParticleColor(...);
    /// **Get Light Level**<br/>
    /// Gets the light level at
    /// a location.
    ///
    /// ## Tags
    /// - Light Type:
    ///   - `Combined light` (Default)
    ///   - `Sky light`
    ///   - `Block light`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Light location
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Light level
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetLight(LightSPECIALSpace_Type : df_string, ...);
    /// **Get Book Text**<br/>
    /// Gets a book's text.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Book
    /// - 
    /// - `df_number` (Number):
    ///   Page number
    /// - OR
    /// - None:
    ///   Gets list of all pages
    ///
    /// ## Returns
    /// - `df_text` (Styled Text):
    ///   Text of the specified page
    /// - OR
    /// - `df_list` (List):
    ///   Contains one Styled Text
    ///   for each page of the book
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALSpace_GetBookTextSPECIALSpace_(...);
    /// **Get Dictionary Values**<br/>
    /// Gets the list of values
    /// in this dictionary.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_dict` (Dict):
    ///   Dictionary to
    ///   pull from
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Contains one value
    ///   for each entry of the
    ///   dictionary
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetDictValues(...);
    /// **Set to Vector**<br/>
    /// Sets a variable to a vector.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - 
    /// - `df_number` (Number):
    ///   X component
    /// - `df_number` (Number):
    ///   Y component
    /// - `df_number` (Number):
    ///   Z component
    /// - OR
    /// - `df_string` (String):
    ///   Direction Name
    ///
    /// ## Returns
    /// - `df_vector` (Vector):
    ///   Vector with the
    ///   specified components
    ///   or direction.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Vector(...);
    /// **Set to Distance**<br/>
    /// Sets a variable to the distance
    /// between two locations.
    ///
    /// ## Tags
    /// - Distance Type:
    ///   - `Distance 2D (X/Z)`
    ///   - `Distance 3D (X/Y/Z)` (Default)
    ///   - `Altitude (Y)`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Location 1
    /// - `df_location` (Location):
    ///   Location 2
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Distance
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Distance(DistanceSPECIALSpace_Type : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemLore(...);
    /// **Parse MiniMessage Expression**<br/>
    /// Parses a MiniMessage expression from
    /// a string value into a styled text.
    ///
    /// ## Tags
    /// - Allowed Tags:
    ///   - `Style Only` (Default)
    ///   - `Dynamic`
    ///   - `Full`
    /// - Parse Legacy Color Codes:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` (String):
    ///   String to parse
    ///
    /// ## Returns
    /// - `df_text` (Styled Text):
    ///   Text parsed from the
    ///   expressions
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ParseMiniMessage(AllowedSPECIALSpace_Tags : df_string, ParseSPECIALSpace_LegacySPECIALSpace_ColorSPECIALSpace_Codes : df_string, ...);
    /// **Set to Root**<br/>
    /// Takes the root of a number.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `?` (Number):
    ///   Number input
    /// - `df_number` `?` (Number):
    ///   Root index
    ///   - Default = 2 (²√input)
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Root(...);
    /// **Set Custom Model Data Strings**<br/>
    /// Sets the model string values
    /// on an item, used in resource packs.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_string` `[]` (String):
    ///   String model value(s)
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetModelDataStrs(...);
    /// **Set Particle Effect Amount**<br/>
    /// Sets a particle effect's particle
    /// amount.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` `?` (Particle):
    ///   Effect to
    ///   change
    /// - `df_number` (Number):
    ///   Particle amount
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetParticleAmount(...);
    /// **Add Item Enchantment**<br/>
    /// Adds an enchantment to an item.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_string` (String):
    ///   Enchantment name
    /// - `df_number` (Number):
    ///   Enchantment level
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__AddItemEnchant(...);
    /// **Add Tool Mining Rule**<br/>
    /// Adds a rule for breaking blocks
    /// to a tool item.
    ///
    /// ## Tags
    /// - Correct Tool for Loot:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_number` (Number):
    ///   Mining speed
    /// - 
    /// - `df_item` `[]` (Block):
    ///   Blocks to apply
    ///   the rule for
    /// - OR
    /// - `df_string` (String):
    ///   Block tag ID to
    ///   apply the rule for
    ///   - Example: "#mineable/pickaxe"
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__AddItemToolRule(CorrectSPECIALSpace_ToolSPECIALSpace_forSPECIALSpace_Loot : df_string, ...);
    /// **Get Item Material**<br/>
    /// Gets an item's material.
    ///
    /// ## Tags
    /// - Return Value Type:
    ///   - `Item ID (golden_apple)` (Default)
    ///   - `Item Name (Golden Apple)`
    ///   - `Item`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get material of
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Material ID or name
    /// - OR
    /// - `df_item` (Item):
    ///   Material as item
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemType(ReturnSPECIALSpace_ValueSPECIALSpace_Type : df_string, ...);
    /// **Get Location Direction**<br/>
    /// Gets a location's rotation
    /// (pitch and yaw) as a direction.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Location to get
    ///   direction of
    ///
    /// ## Returns
    /// - `df_vector` (Vector):
    ///   Direction
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetDirection(...);
    /// **Get Item Lore Line**<br/>
    /// Gets a single line from
    /// an item's lore.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get lore from
    /// - `df_number` (Number):
    ///   Lore line to get
    ///
    /// ## Returns
    /// - `df_text` (Styled Text):
    ///   Lore text
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetLoreLine(...);
    /// **Set to Arc Tangent of 2 Numbers**<br/>
    /// Sets a variable to the
    /// arc tangent of 2 numbers.
    ///
    /// ## Tags
    /// - Output Type:
    ///   - `Degrees` (Default)
    ///   - `Radians`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` (Number):
    ///   Y
    /// - `df_number` (Number):
    ///   X
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ArcTangent2(OutputSPECIALSpace_Type : df_string, ...);
    /// **Get Particle Effect Type**<br/>
    /// Gets a particle effect's type.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` (Particle):
    ///   Effect to get
    ///   type of
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Particle type
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetParticleType(...);
    /// **Set Item Maximum Durability**<br/>
    /// Sets an item's maximum durability.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item
    /// - `df_number` `?` (Number):
    ///   Maximum durability
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemMaxDura(...);
    /// **Remove String**<br/>
    /// Searches for part of a string and
    /// removes all instances of it.
    ///
    /// ## Tags
    /// - Regular Expressions:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable
    ///   to set
    /// - `df_string` `?` (String):
    ///   String to change
    /// - `df_string` `[]` (String):
    ///   String to remove
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Modified string
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RemoveString(RegularSPECIALSpace_Expressions : df_string, ...);
    /// **Get All Block Data**<br/>
    /// Gets the block state tags
    /// at a location.
    ///
    /// ## Tags
    /// - Hide Default:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Block location
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Block data separated
    ///   with commas (,)
    ///
    /// ## Additional Info
    /// - Returns "0" if the block
    ///   has no tags.
    /// - Always returns a string value.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetAllBlockData(HideSPECIALSpace_Default : df_string, ...);
    /// **Set Crossbow Projectiles**<br/>
    /// Sets a crossbow's loaded projectiles.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Crossbow
    /// - `df_item` `[]?` (Item):
    ///   Projectiles
    ///
    /// ## Works With
    /// - `Arrows`
    /// - `Fireworks`
    ///
    /// ## Additional Info
    /// - A crossbow can hold any number of projectiles.
    /// - The projectiles will shoot out in a straight line.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetCrossbowProj(...);
    /// **Set to Maximum Number**<br/>
    /// Sets a variable to the highest
    /// number in a set.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `[]` (Number):
    ///   Number set
    ///   to compare
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__MaxNumber(...);
    /// **Get Dictionary Keys**<br/>
    /// Gets the list of keys
    /// in this dictionary.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_dict` (Dict):
    ///   Dictionary to
    ///   pull from
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Contains one String
    ///   entry for each key of the
    ///   dictionary
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetDictKeys(...);
    /// **Trim Styled Text Content**<br/>
    /// Trims the content of styled text,
    /// leaving all formatting in place.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_text` `?` (Styled Text):
    ///   Text to trim
    /// - `df_number` (Number):
    ///   Start character position
    /// - `df_number` `?` (Number):
    ///   End character position
    ///   - Default = Text length
    ///
    /// ## Returns
    /// - `df_text` (Styled Text):
    ///   The text trimmed from
    ///   the original text
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__TrimStyledText(...);
    /// **Set Particle Effect Material**<br/>
    /// Sets a particle effect's particle
    /// display material.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` `?` (Particle):
    ///   Effect to
    ///   change
    /// - `df_item` (Item):
    ///   Particle material
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetParticleMat(...);
    /// **Get Location Coordinate**<br/>
    /// Gets a location's X, Y, Z, pitch,
    /// or yaw coordinate.
    ///
    /// ## Tags
    /// - Coordinate Type:
    ///   - `Plot coordinate` (Default)
    ///   - `World coordinate`
    /// - Coordinate:
    ///   - `X` (Default)
    ///   - `Y`
    ///   - `Z`
    ///   - `Pitch`
    ///   - `Yaw`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Location to get
    ///   coordinate of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Coordinate of the specified
    ///   axis
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetCoord(CoordinateSPECIALSpace_Type : df_string, Coordinate : df_string, ...);
    /// **Remove Item Custom Tag**<br/>
    /// Removes a custom item tag.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_string` (String):
    ///   Tag name
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RemoveItemTag(...);
    /// **Get Custom Model Data Strings**<br/>
    /// Gets the model string values
    /// on an item, used in resource packs.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get model data from
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   List of model strings
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetModelDataStrs(...);
    /// **Set Particle Effect Size**<br/>
    /// Sets a particle effect's particle
    /// size and size variation.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` `?` (Particle):
    ///   Effect to
    ///   change
    /// - `df_number` (Number):
    ///   Particle size
    /// - `df_number` `?` (Number):
    ///   Size variation (%)
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetParticleSize(...);
    /// **Get Potion Effect Duration**<br/>
    /// Gets a potion effect's duration.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_potion` (Potion):
    ///   Potion to get
    ///   duration of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Potion duration
    ///   in ticks
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetPotionDur(...);
    /// **Set to Random Location**<br/>
    /// Sets the variable to a random
    /// location between two locations.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Location 1
    /// - `df_location` (Location):
    ///   Location 2
    ///
    /// ## Returns
    /// - `df_location` (Location):
    ///   Random location
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RandomLoc(...);
    /// **Set Sound Type**<br/>
    /// Sets a sound's type.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_sound` `?` (Sound):
    ///   Sound to change
    /// - `df_string` (String):
    ///   Sound name (e.g. "pling")
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetSoundType(...);
    /// **Get Compass Lodestone Location**<br/>
    /// Gets a compass's lodestone location.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Compass to get lodestone
    ///   location of
    ///
    /// ## Returns
    /// - `df_location` (Location):
    ///   Block location
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetLodestoneLoc(...);
    ///
    /// ## Tags
    /// - Direction:
    ///   - `Forwards 2D (X/Z)`
    ///   - `Forwards 3D (X/Y/Z)` (Default)
    ///   - `Sideways (-L / +R)`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ShiftDirection(Direction : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetContainerName(...);
    /// **Get Index of Substring**<br/>
    /// Searches for a string in another string
    /// and gets the index if found.
    ///
    /// ## Tags
    /// - Ignore Case:
    ///   - `True`
    ///   - `False` (Default)
    /// - Search Type:
    ///   - `First occurrence` (Default)
    ///   - `Last occurrence`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` (String):
    ///   String to search in
    /// - `df_string` (String):
    ///   String to search for
    /// - `df_number` `?` (Number):
    ///   Starting index
    ///   - Default = 1
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Value index
    ///
    /// ## Additional Info
    /// - Returns 0 if the substring is not found.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__IndexOfSubstring(IgnoreSPECIALSpace_Case : df_string, SearchSPECIALSpace_Type : df_string, ...);
    /// **Get Particle Effect Spread**<br/>
    /// Gets a particle effect's horizontal
    /// or vertical spread.
    ///
    /// ## Tags
    /// - Spread:
    ///   - `Horizontal` (Default)
    ///   - `Vertical`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` (Particle):
    ///   Effect to get
    ///   spread of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Particle spread
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetParticleSprd(Spread : df_string, ...);
    /// **Set Item Use Leftover**<br/>
    /// Sets an item's leftover item that
    /// gets replaced on use, e.g. drinking
    /// stew gives a bowl.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_item` `?` (Item):
    ///   Remainder item
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemLeftover(...);
    /// **Reflect Vector**<br/>
    /// Reflects a vector off a
    /// surface defined by another
    /// vector.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` `?` (Vector):
    ///   Vector to reflect
    /// - `df_vector` (Vector):
    ///   Surface vector
    ///
    /// ## Returns
    /// - `df_vector` (Vector):
    ///   Reflected vector
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ReflectVector(...);
    /// **Clear Item Attributes**<br/>
    /// Clears all attributes from an item.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ClearItemAttrs(...);
    /// **Get Head Owner**<br/>
    /// Gets a player head's owner
    /// name or UUID.
    ///
    /// ## Tags
    /// - Text Value:
    ///   - `Owner Name` (Default)
    ///   - `Owner UUID`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Head to get owner of
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Head owner
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetHeadOwner(TextSPECIALSpace_Value : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemEnchants(...);
    /// **Append Dictionary**<br/>
    /// Adds all entries from one
    /// dictionary into the other.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Dictionary to
    ///   add to
    /// - `df_dict` (Dict):
    ///   Dictionary
    ///   to append
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__AppendDict(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetMaxItemAmount(...);
    /// **Set Item Model**<br/>
    /// Sets an item's model.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_string` `?` (String):
    ///   Model key
    ///   - A namespaced key ("minecraft:" by default)
    ///   - Example: "stone"
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemModel(...);
    /// **Get Color Channels**<br/>
    /// Gets a color's RGB/HSB/HSL
    /// number values as a list.
    ///
    /// ## Tags
    /// - Color Channels:
    ///   - `RGB` (Default)
    ///   - `HSB`
    ///   - `HSL`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` (String):
    ///   Color hexadecimal
    ///   - Example: "#FF0000" (red)
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Contains one Number for
    ///   each color channel
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetColorChannels(ColorSPECIALSpace_Channels : df_string, ...);
    /// **Set Location Direction**<br/>
    /// Sets a location's rotation
    /// (pitch and yaw) to a direction.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` `?` (Location):
    ///   Location to change
    /// - `df_vector` (Vector):
    ///   Direction
    ///
    /// ## Returns
    /// - `df_location` (Location):
    ///   Modified location
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALSpace_SetDirectionSPECIALSpace_(...);
    /// **Set List Value**<br/>
    /// Sets a list variable's value at
    /// an index.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   List to change
    /// - `df_number` (Number):
    ///   Index
    /// - `df_opaque` (Any):
    ///   Value to set
    ///
    /// ## Additional Info
    /// - List indices start at 1.
    /// - Setting an index beyond the
    ///   bounds of the list will
    ///   clamp the index to the
    ///   list bounds.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetListValue(...);
    /// **Set Item Enchantments**<br/>
    /// Sets an item's enchantment list.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_dict` (Dict):
    ///   Enchantments
    ///   - Key of enchantment ID
    ///   - Value of enchantment level
    ///
    /// ## Additional Info
    /// - Refer to the format of the
    ///   'Get Item Enchantments' action.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALSpace_SetItemEnchantsSPECIALSpace_(...);
    /// **Set Book Text**<br/>
    /// Sets a book's text.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Book
    /// - 
    /// - `df_text` `[]` (Styled Text):
    ///   Pages
    /// - OR
    /// - `df_text` (Styled Text):
    ///   Page text
    /// - `df_number` (Number):
    ///   Page number
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetBookText(...);
    /// **Set to Random Value**<br/>
    /// Sets a variable to a random
    /// value from a set.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_opaque` `[]` (Any):
    ///   Value set
    ///   to choose from
    ///
    /// ## Returns
    /// - `df_opaque` (Any):
    ///   Randomly
    ///   chosen value
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RandomValue(...);
    ///
    /// ## Tags
    /// - Armor Trim:
    ///   - `Hide`
    ///   - `Show`
    ///   - `No Change` (Default)
    /// - Color:
    ///   - `Hide`
    ///   - `Show`
    ///   - `No Change` (Default)
    /// - Enchantments:
    ///   - `Hide`
    ///   - `Show`
    ///   - `No Change` (Default)
    /// - Attributes:
    ///   - `Hide`
    ///   - `Show`
    ///   - `No Change` (Default)
    /// - Unbreakable:
    ///   - `Hide`
    ///   - `Show`
    ///   - `No Change` (Default)
    /// - Can Destroy:
    ///   - `Hide`
    ///   - `Show`
    ///   - `No Change` (Default)
    /// - Can Place On:
    ///   - `Hide`
    ///   - `Show`
    ///   - `No Change` (Default)
    /// - Potion Effects:
    ///   - `Hide`
    ///   - `Show`
    ///   - `No Change` (Default)
    /// - Others:
    ///   - `Hide`
    ///   - `Show`
    ///   - `No Change` (Default)
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALSpace_SetItemFlagsSPECIALSpace_(ArmorSPECIALSpace_Trim : df_string, Color : df_string, Enchantments : df_string, Attributes : df_string, Unbreakable : df_string, CanSPECIALSpace_Destroy : df_string, CanSPECIALSpace_PlaceSPECIALSpace_On : df_string, PotionSPECIALSpace_Effects : df_string, Others : df_string, ...);
    /// **Set Item Material**<br/>
    /// Sets an item's material.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_string` (String):
    ///   Material
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemType(...);
    /// **Get Sound Type**<br/>
    /// Gets the given sound's type.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_sound` (Sound):
    ///   Sound to get type of
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Sound type
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetSoundType(...);
    /// **Get List Value**<br/>
    /// Gets a list variable's value at
    /// an index.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_list` (List):
    ///   List to get value of
    /// - `df_number` (Number):
    ///   Index
    ///
    /// ## Returns
    /// - `df_opaque` (Any):
    ///   List value
    ///
    /// ## Additional Info
    /// - List indices start at 1.
    /// - If the index is outside the
    ///   given list, 0 is returned.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetListValue(...);
    /// **Convert Value to JSON**<br/>
    /// Converts a value to JSON.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - 
    /// - `df_number` (Number):
    ///   Value to convert
    /// - OR
    /// - `df_string` (String):
    ///   Value to convert
    /// - OR
    /// - `df_list` (List):
    ///   Value to convert
    /// - OR
    /// - `df_dict` (Dict):
    ///   Value to convert
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   JSON string
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ValueToJson(...);
    /// **Bounce Number**<br/>
    /// Checks if a number is inside
    /// the bounds and if not, bounces
    /// it towards the other bound.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `?` (Number):
    ///   Number to bounce
    /// - `df_number` (Number):
    ///   Lower bound (inclusive)
    /// - `df_number` (Number):
    ///   Upper bound (exclusive)
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    /// ## Examples
    /// - `BounceNum(10, 2, 8) = 6`
    /// - `BounceNum(4, 1, 4) = 4`
    /// - `BounceNum(-9, 0, 7) = 5`
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__BounceNum(...);
    /// **Set to Tangent**<br/>
    /// Sets a variable to the trigonometric
    /// tangent function of a number.
    ///
    /// ## Tags
    /// - Tangent Variant:
    ///   - `Tangent` (Default)
    ///   - `Inverse tangent (arctangent)`
    ///   - `Hyperbolic tangent`
    /// - Input:
    ///   - `Degrees` (Default)
    ///   - `Radians`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` (Number):
    ///   Number input
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    /// ## Additional Info
    /// - Use the variant tag to swap between
    ///   tan, atan and tanh functions.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Tangent(TangentSPECIALSpace_Variant : df_string, Input : df_string, ...);
    /// **Get Voronoi Noise**<br/>
    /// Gets a Voronoi noise value: A cellular
    /// noise in which the value of an entire
    /// cell is calculated.
    ///
    /// ## Tags
    /// - Cell Edge Type:
    ///   - `Euclidean` (Default)
    ///   - `Manhattan`
    ///   - `Natural`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Noise location
    /// - `df_number` `?` (Number):
    ///   Cell frequency
    ///   - Default = 1
    /// - `df_number` `?` (Number):
    ///   Cell scatter
    ///   - 5-15 for cell shapes (Default = 10)
    /// - `df_number` `?` (Number):
    ///   Generation seed
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   A value between 0 and 1
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__VoronoiNoise(CellSPECIALSpace_EdgeSPECIALSpace_Type : df_string, ...);
    ///
    /// ## Tags
    /// - Face Direction:
    ///   - `Towards direction` (Default)
    ///   - `Towards opposite direction`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetDirection(FaceSPECIALSpace_Direction : df_string, ...);
    /// **Set to HSB Color**<br/>
    /// Creates a color based on hue,
    /// saturation, and brightness.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - 
    /// - `df_number` (Number):
    ///   Hue (Color circle, 0-360)
    /// - `df_number` `?` (Number):
    ///   Saturation (0-100)
    ///   - Default = 100
    /// - `df_number` `?` (Number):
    ///   Brightness (0-100)
    ///   - Default = 100
    /// - OR
    /// - `df_list` (List):
    ///   H, S, B values
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Hex code of the color
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__HSBColor(...);
    /// **Increment Number (+=)**<br/>
    /// Increments a number variable
    /// by 1 or more other numbers.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable
    ///   to increment
    /// - `df_number` `[]?` (Number):
    ///   Number(s) to
    ///   increment by
    ///   - Default = 1
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALPlus_SPECIALEqual_(...);
    /// **Interpolate Number**<br/>
    /// Interpolates two numbers
    /// based on a time value and
    /// an easing type.
    ///
    /// ## Tags
    /// - Easing:
    ///   - `Linear` (Default)
    ///   - `Quad`
    ///   - `Cubic`
    ///   - `Quart`
    ///   - `Quint`
    ///   - `Sine`
    ///   - `Expo`
    ///   - `Circ`
    /// - Direction:
    ///   - `In` (Default)
    ///   - `Out`
    ///   - `In Out`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` (Number):
    ///   Time (%)
    /// - `df_number` `?` (Number):
    ///   First number
    ///   - Default = 0
    /// - `df_number` `?` (Number):
    ///   Second number
    ///   - Default = 1
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Interpolate(Easing : df_string, Direction : df_string, ...);
    /// **Get Sound Variant**<br/>
    /// Gets the variant of a sound.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_sound` (Sound):
    ///   Sound to get variant of
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Variant name
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetSoundVariant(...);
    /// **Get Item Color**<br/>
    /// Gets a colorable item's color.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable
    ///   to set
    /// - `df_item` (Item):
    ///   Item to get color of
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Color hexadecimal
    ///   Example: "#FF0000" (red)
    ///
    /// ## Works With
    /// - `Leather Armor`
    /// - `Potions`
    /// - `Tipped Arrows`
    /// - `Filled Maps`
    /// - `Firework Stars`
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemColor(...);
    /// **Clear Formatting**<br/>
    /// Clears all formatting from the
    /// given styled text.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_text` `?` (Styled Text):
    ///   Text to change
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Plain string with no
    ///   formatting
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ClearFormatting(...);
    /// **Insert List Value**<br/>
    /// Inserts a value into a list
    /// variable at an index, shifting
    /// all values at and after it to
    /// the right.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   List to change
    /// - `df_number` (Number):
    ///   Index
    /// - `df_opaque` (Any):
    ///   Value to insert
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__InsertListValue(...);
    /// **Get Particle Effect Duration**<br/>
    /// Gets a particle effect's duration.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` (Particle):
    ///   Effect to get
    ///   duration of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Particle duration
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetParticleDur(...);
    /// **Set Sound Volume**<br/>
    /// Sets a sound's volume.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_sound` `?` (Sound):
    ///   Sound to change
    /// - `df_number` (Number):
    ///   Volume
    ///
    /// ## Additional Info
    /// - Sound volume decreases based
    ///   on distance to the sound's origin.
    /// - The maximum distance to hear a
    ///   sound is equal to 16.0 × volume.
    /// - At volumes below 1.0, the sound's
    ///   loudness at its origin decreases.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetSoundVolume(...);
    /// **Set Location Coordinate**<br/>
    /// Sets a location's X, Y, Z, pitch,
    /// or yaw coordinate.
    ///
    /// ## Tags
    /// - Coordinate Type:
    ///   - `Plot coordinate` (Default)
    ///   - `World coordinate`
    /// - Coordinate:
    ///   - `X` (Default)
    ///   - `Y`
    ///   - `Z`
    ///   - `Pitch`
    ///   - `Yaw`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` `?` (Location):
    ///   Location to change
    /// - `df_number` (Number):
    ///   Coordinate
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetCoord(CoordinateSPECIALSpace_Type : df_string, Coordinate : df_string, ...);
    /// **Add Vectors**<br/>
    /// Sets a variable to the sum
    /// of the given vectors.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` `[]` (Vector):
    ///   Vectors to add
    ///
    /// ## Returns
    /// - `df_vector` (Vector):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__AddVectors(...);
    /// **Sanitize MiniMessage Tags**<br/>
    /// Sanitizes all MiniMessage tags in a string.
    /// This is useful for using user input in
    /// the Parse MiniMessage Set Variable.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` `?` (String):
    ///   String to sanitize
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Sanitized string
    ///
    /// ## Examples
    /// - `"\<green\>Hello" becomes "\\<green\>Hello"`
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SanitizeTags(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetPitch(...);
    /// **Get Particle Effect Fade Color**<br/>
    /// Gets a particle effect's particle
    /// fade color.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` (Particle):
    ///   Effect to get
    ///   fade color of
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Color hexadecimal
    ///   Example: "#FF0000" (red)
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetParticleFade(...);
    ///
    /// ## Tags
    /// - Ignore Passable Blocks:
    ///   - `True`
    ///   - `False` (Default)
    /// - Fluid Collision:
    ///   - `Ignore fluids` (Default)
    ///   - `Detect fluids`
    ///   - `Source blocks only`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RaycastBlock(IgnoreSPECIALSpace_PassableSPECIALSpace_Blocks : df_string, FluidSPECIALSpace_Collision : df_string, ...);
    /// **Set Tool Properties**<br/>
    /// Adds tool properties to an item.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_number` `?` (Number):
    ///   Default mining speed
    ///   - Default = 1
    /// - `df_number` `?` (Number):
    ///   Damage per block
    ///   - Default = 1
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemTool(...);
    /// **Get Item Tooltip Style**<br/>
    /// Gets an item's custom tooltip.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get tooltip style of
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Tooltip key, e.g. "namespace:id"
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetTooltipStyle(...);
    /// **Get Item Enchantments**<br/>
    /// Gets an item's enchantments.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get enchantments from
    ///
    /// ## Returns
    /// - `df_dict` (Dict):
    ///   Key represents the enchantment
    ///   name as String, value represents the
    ///   enchantment level as Number
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALSpace_GetItemEnchantsSPECIALSpace_(...);
    /// **Set Head Texture**<br/>
    /// Sets a player head's texture
    /// using an owning player
    /// or custom texture.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Player head
    /// - `df_string` (String):
    ///   Owner name, UUID or
    ///   texture value
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetHeadTexture(...);
    /// **Compress to Gzip**<br/>
    /// Compresses bytes as a
    /// list of numbers to Gzip.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `[]` (Byte):
    ///   Byte(s) to compress
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Bytes as a list of numbers
    ///
    /// ## Additional Info
    /// - Returns 0 if the result is too large.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GzipCompress(...);
    /// **Get Perlin Noise**<br/>
    /// Gets a Perlin noise value: A type
    /// of fractal gradient noise.
    ///
    /// ## Tags
    /// - Fractal Type:
    ///   - `Brownian` (Default)
    ///   - `Billow (Dark edges)`
    ///   - `Rigid (Light edges)`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Noise location
    /// - `df_number` `?` (Number):
    ///   Frequency (Scale)
    ///   - Default = 1
    /// - `df_number` `?` (Number):
    ///   Octaves (Perlin layers)
    ///   - 1-8, default = 1
    /// - `df_number` `?` (Number):
    ///   Octave frequency gain
    ///   - Default = 1.5
    /// - `df_number` `?` (Number):
    ///   Octave amplitude gain
    ///   - Default = 0.75
    /// - `df_number` `?` (Number):
    ///   Generation seed
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   A value between 0 and 1
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__PerlinNoise(FractalSPECIALSpace_Type : df_string, ...);
    /// **Get Worley Noise**<br/>
    /// Gets a Worley noise value: A cellular
    /// noise in which the distance between
    /// two cells' nuclei is calculated.
    ///
    /// ## Tags
    /// - Cell Edge Type:
    ///   - `Euclidean` (Default)
    ///   - `Manhattan`
    ///   - `Natural`
    /// - Distance Calculation:
    ///   - `Primary` (Default)
    ///   - `Secondary`
    ///   - `Additive`
    ///   - `Subtractive`
    ///   - `Multiplicative`
    ///   - `Divisive`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Noise location
    /// - `df_number` `?` (Number):
    ///   Cell frequency
    ///   - Default = 1
    /// - `df_number` `?` (Number):
    ///   Cell scatter
    ///   - 5-15 for cell shapes (Default = 10)
    /// - `df_number` `?` (Number):
    ///   Generation seed
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   A value between 0 and 1
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__WorleyNoise(CellSPECIALSpace_EdgeSPECIALSpace_Type : df_string, DistanceSPECIALSpace_Calculation : df_string, ...);
    /// **Set Item Color**<br/>
    /// Sets a colorable item's color.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - 
    /// - `df_string` (String):
    ///   Color hexadecimal
    ///   - Example: "#FF0000" (red)
    /// - OR
    /// - None:
    ///   Removes color
    ///
    /// ## Works With
    /// - `Leather Armor`
    /// - `Potions`
    /// - `Tipped Arrows`
    /// - `Filled Maps`
    /// - `Firework Stars`
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemColor(...);
    /// **Get Particle Effect Color**<br/>
    /// Gets a particle effect's particle
    /// color.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` (Particle):
    ///   Effect to get
    ///   color of
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Color hexadecimal
    ///   Example: "#FF0000" (red)
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetParticleColor(...);
    /// **Set Sound Pitch**<br/>
    /// Sets a sound's pitch or note
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_sound` `?` (Sound):
    ///   Sound to change
    /// - 
    /// - `df_number` (Number):
    ///   Pitch
    /// - OR
    /// - `df_string` (String):
    ///   Note
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetSoundPitch(...);
    /// **Round Number**<br/>
    /// Rounds a number to a whole
    /// number or multiple.
    ///
    /// ## Tags
    /// - Round Mode:
    ///   - `Floor`
    ///   - `Nearest` (Default)
    ///   - `Ceiling`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `?` (Number):
    ///   Number to round
    /// - `df_number` `?` (Number):
    ///   Round multiple
    ///   - Default = 1 (Whole number)
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALSpace_RoundNumberSPECIALSpace_(RoundSPECIALSpace_Mode : df_string, ...);
    /// **Get Placeable Blocks**<br/>
    /// Gets which blocks an item
    /// can be placed on in Adventure Mode.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Contains one Item for
    ///   each placeable block
    ///
    /// ## Additional Info
    /// - Output is in arbitrary order.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetCanPlaceOn(...);
    /// **Sort List**<br/>
    /// Sorts a list variable's values.
    ///
    /// ## Tags
    /// - Sort Order:
    ///   - `Ascending` (Default)
    ///   - `Descending`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_list` `?` (List):
    ///   List to sort
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Sorted list
    ///
    /// ## Works With
    /// - `Numbers, Text, Lists`
    ///
    /// ## Additional Info
    /// - Sub-lists are sorted by
    ///   their first index.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SortList(SortSPECIALSpace_Order : df_string, ...);
    /// **Set Custom Sound Key**<br/>
    /// Sets the key of a custom sound.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_sound` `?` (Sound):
    ///   Sound to change
    /// - `df_string` `?` (String):
    ///   Sound key
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetCustomSound(...);
    /// **Remove Dictionary Entry**<br/>
    /// Removes the dictionary entry
    /// with the given key.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Dictionary to change
    /// - `df_string` (String):
    ///   Key to remove
    /// - `df_opaque` `[]?` (Any):
    ///   Expected value(s)
    ///   - When given, the entry will only
    ///     be removed if the current value
    ///     matches.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RemoveDictEntry(...);
    /// **Format Timestamp**<br/>
    /// Converts a numerical timestamp to
    /// a human-readable time/date text
    /// using a date format.
    ///
    /// ## Tags
    /// - Format:
    ///   - `Custom`
    ///   - `2020/08/17 17:20:54` (Default)
    ///   - `2020/08/17`
    ///   - `Mon, August 17`
    ///   - `Monday`
    ///   - `17:20:54`
    ///   - `5:20 PM`
    ///   - `17h20m54s`
    ///   - `54.229 seconds`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` (Number):
    ///   Time to format
    /// - `df_string` `?` (String):
    ///   Custom Format
    ///   - Used only if Tag: Format is set
    ///     to Custom
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Formatted timestamp
    ///
    /// ## Additional Info
    /// - Use Game Value: Timestamp to
    ///   generate timestamps.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__FormatTime(Format : df_string, ...);
    ///
    /// ## Tags
    /// - Hide Armor Trim:
    ///   - `True`
    ///   - `False`
    ///   - `No Change` (Default)
    /// - Hide Color:
    ///   - `True`
    ///   - `False`
    ///   - `No Change` (Default)
    /// - Hide Enchantments:
    ///   - `True`
    ///   - `False`
    ///   - `No Change` (Default)
    /// - Hide Attributes:
    ///   - `True`
    ///   - `False`
    ///   - `No Change` (Default)
    /// - Hide Unbreakable:
    ///   - `True`
    ///   - `False`
    ///   - `No Change` (Default)
    /// - Hide Can Destroy:
    ///   - `True`
    ///   - `False`
    ///   - `No Change` (Default)
    /// - Hide Can Place On:
    ///   - `True`
    ///   - `False`
    ///   - `No Change` (Default)
    /// - Hide Potion Effects:
    ///   - `True`
    ///   - `False`
    ///   - `No Change` (Default)
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemFlags(HideSPECIALSpace_ArmorSPECIALSpace_Trim : df_string, HideSPECIALSpace_Color : df_string, HideSPECIALSpace_Enchantments : df_string, HideSPECIALSpace_Attributes : df_string, HideSPECIALSpace_Unbreakable : df_string, HideSPECIALSpace_CanSPECIALSpace_Destroy : df_string, HideSPECIALSpace_CanSPECIALSpace_PlaceSPECIALSpace_On : df_string, HideSPECIALSpace_PotionSPECIALSpace_Effects : df_string, ...);
    /// **Set Particle Effect Duration**<br/>
    /// Sets a particle effect's duration.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` `?` (Particle):
    ///   Effect to
    ///   change
    /// - `df_number` (Number):
    ///   Particle duration in ticks
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetParticleDur(...);
    /// **Get String Length**<br/>
    /// Gets the number of characters
    /// a string has.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` (String):
    ///   String to measure
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   String length
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__StringLength(...);
    /// **Get Item Potion Effects**<br/>
    /// Gets the potion effects applied by
    /// an item.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get effects from
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Contains one Potion Effect
    ///   for each effect in the item
    ///
    /// ## Works With
    /// - `Potions`
    /// - `Tipped Arrows`
    /// - `Suspicious Stew`
    ///
    /// ## Additional Info
    /// - Durations of effects applied by
    ///   Tipped Arrows and Instant Potions
    ///   are multiplied by 8.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemEffects(...);
    /// **Set to Styled Text**<br/>
    /// Sets a variable to a styled text, or combines
    /// multiple values into one styled text.
    ///
    /// ## Tags
    /// - Text Value Merging:
    ///   - `Add spaces`
    ///   - `No spaces` (Default)
    /// - Inherit Styles:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_opaque` `[]?` (Any):
    ///   Text to set to
    ///
    /// ## Additional Info
    /// - All values will be converted to styled text.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__StyledText(TextSPECIALSpace_ValueSPECIALSpace_Merging : df_string, InheritSPECIALSpace_Styles : df_string, ...);
    /// **Get MiniMessage Expression**<br/>
    /// Gets the MiniMessage expression for
    /// a styled text value.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_text` (Styled Text):
    ///   Text to read
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   String containing the raw
    ///   expressions
    ///
    /// ## Additional Info
    /// - This value will often NOT match the
    ///   source expression! Meta tags such as
    ///   "\<rainbow\>" will be broken down into
    ///   several adjacent "\<color\>" tags.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetMiniMessageExpr(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetYaw(...);
    /// **Set Item Lore**<br/>
    /// Sets an item's lore list.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - 
    /// - `df_text` `[]` (Styled Text):
    ///   Lore
    /// - OR
    /// - `df_text` `?` (Styled Text):
    ///   Lore line text
    /// - `df_number` (Number):
    ///   Line number
    /// - OR
    /// - None:
    ///   Clears lore
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALSpace_SetItemLoreSPECIALSpace_(...);
    /// **Set Item Potion Effects**<br/>
    /// Sets the potion effects applied by
    /// an item.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_potion` `[]` (Potion):
    ///   Item effects
    ///
    /// ## Works With
    /// - `Potions`
    /// - `Tipped Arrows`
    /// - `Suspicious Stew`
    ///
    /// ## Additional Info
    /// - Durations of effects applied by
    ///   Tipped Arrows and Instant Potions
    ///   are divided by 8.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemEffects(...);
    /// **Decrement Number (-=)**<br/>
    /// Decrements a number variable
    /// by 1 or more other numbers.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable
    ///   to decrement
    /// - `df_number` `[]?` (Number):
    ///   Number(s) to
    ///   decrement by
    ///   - Default = 1
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SPECIALHyphen_SPECIALEqual_(...);
    /// **Get Item Custom Tag**<br/>
    /// Gets the value of a custom
    /// item tag.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get tag of
    /// - `df_string` (String):
    ///   Tag name
    ///
    /// ## Returns
    /// - `df_opaque` (Any):
    ///   Tag value
    ///
    /// ## Additional Info
    /// - Returns 0 if the tag is
    ///   not present.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemTag(...);
    /// **Create List**<br/>
    /// Creates a list from the given
    /// values.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_opaque` `[]?` (Any):
    ///   Value list
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__CreateList(...);
    /// **Append List to List**<br/>
    /// Adds a list to the end of
    /// another list variable.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   List to append to
    /// - `df_list` `[]` (List):
    ///   List(s) to append
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__AppendList(...);
    /// **Get Container Contents**<br/>
    /// Gets a container's contents
    /// at a location.
    ///
    /// ## Tags
    /// - Ignore Empty Slots:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Container location
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Contains one Item for
    ///   each item in the container
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetContainerItems(IgnoreSPECIALSpace_EmptySPECIALSpace_Slots : df_string, ...);
    /// **Get Custom Model Data Numbers**<br/>
    /// Gets the numerical model values
    /// on an item, used in resource packs.
    ///
    /// ## Tags
    /// - Model Value Type:
    ///   - `Floats` (Default)
    ///   - `Flags`
    ///   - `Colors`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get model data from
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   List of model numbers of
    ///   the selected value type
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetModelDataNums(ModelSPECIALSpace_ValueSPECIALSpace_Type : df_string, ...);
    /// **Shift Location Toward Location**<br/>
    /// Shifts a location toward another
    /// location by the given distance.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` `?` (Location):
    ///   Location to shift
    /// - `df_location` (Location):
    ///   Target location
    /// - `df_number` `?` (Number):
    ///   Shift distance
    ///   - Default = 1
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ShiftToward(...);
    /// **Trim List**<br/>
    /// Trims a list, starting and ending
    /// at the given indices.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_list` `?` (List):
    ///   List to trim
    /// - `df_number` (Number):
    ///   Start index
    /// - `df_number` `?` (Number):
    ///   End index
    ///   - Default = List length
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Trimmed list
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__TrimList(...);
    /// **Get Gradient Noise**<br/>
    /// Gets gradient noise: A type of noise
    /// based on a lattice of random gradients.
    ///
    /// ## Tags
    /// - Dimensions:
    ///   - `3D` (Default)
    ///   - `2D`
    /// - Domain Fractal:
    ///   - `Progressive` (Default)
    ///   - `Independent`
    /// - Return Type:
    ///   - `Simplex` (Default)
    ///   - `Perlin`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Noise location
    /// - `df_number` `?` (Number):
    ///   Frequency
    ///   - Modifies the scale that the
    ///     noise generates at. Default = 1
    /// - `df_number` `?` (Number):
    ///   Marbling
    ///   - Every 0.5 adds an additional ring
    ///     to the noise output. Default = 0.5
    /// - `df_number` `?` (Number):
    ///   Octaves
    ///   - Adds an additional layer of gradient noise
    ///     to the output. 1-16, Default = 1
    /// - `df_number` `?` (Number):
    ///   Lacunarity
    ///   - Every time a new octave is added,
    ///     the frequency is multiplied
    ///     by this value. Default = 1.5
    /// - `df_number` `?` (Number):
    ///   Gain
    ///   - Every time a new octave is added,
    ///     the weight of it is multiplied
    ///     by this value. Default = 0.5
    /// - `df_number` `?` (Number):
    ///   Warping
    ///   - Distorts the noise output. Default = 0
    /// - `df_number` `?` (Number):
    ///   Resonance
    ///   - Modifies the frequency of warping. Default = 1
    /// - `df_number` `?` (Number):
    ///   Domains
    ///   - Adds an additional layer of warping
    ///     to the output. Default = 1
    /// - `df_number` `?` (Number):
    ///   Domain lacunarity
    ///   - Every time a new domain is added,
    ///     the resonance is multiplied
    ///     by this value. Default = 1.5
    /// - `df_number` `?` (Number):
    ///   Domain gain
    ///   - Every time a new domain is added,
    ///     the weight of it is multiplied
    ///     by this value. Default = 0.5
    /// - `df_number` `?` (Number):
    ///   Generation seed
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   A decimal value between 0 and 1
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GradientNoise(Dimensions : df_string, DomainSPECIALSpace_Fractal : df_string, ReturnSPECIALSpace_Type : df_string, ...);
    /// **Set Tooltip Visible**<br/>
    /// Sets whether an item's tooltip
    /// is visible or not.
    ///
    /// ## Tags
    /// - Tooltip:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemHideTooltip(Tooltip : df_string, ...);
    /// **Get Block Drop Items**<br/>
    /// Gets the items dropped by a
    /// block if mined by a given tool.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Location to check
    /// - `df_item` `?` (Item):
    ///   Tool to use
    ///   - Default = Required Tool
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   List of Items dropped
    ///
    /// ## Additional Info
    /// - Setting the tool to air will
    ///   use no tool (player's fist).
    /// - Leaving the tool unset will
    ///   use the best tool to mine the
    ///   block.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetBlockDrops(...);
    /// **Get All Named Regex Groups**<br/>
    /// Gets a dictionary of all named
    /// capture groups in a Regex match.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` (String):
    ///   Input string
    /// - `df_string` (String):
    ///   Regex pattern
    ///
    /// ## Returns
    /// - `df_dict` (Dict):
    ///   Dictionary of all named capture groups
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__NamedRegexGrps(...);
    /// **Clear Item Enchantments**<br/>
    /// Removes enchantments from an item.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ClearEnchants(...);
    /// **Set to Cosine**<br/>
    /// Sets a variable to the trigonometric
    /// cosine function of a number.
    ///
    /// ## Tags
    /// - Cosine Variant:
    ///   - `Cosine` (Default)
    ///   - `Inverse cosine (arccosine)`
    ///   - `Hyperbolic cosine`
    /// - Input:
    ///   - `Degrees` (Default)
    ///   - `Radians`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` (Number):
    ///   Number input
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    /// ## Additional Info
    /// - Use the variant tag to swap between
    ///   cos, acos and cosh functions.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Cosine(CosineSPECIALSpace_Variant : df_string, Input : df_string, ...);
    /// **Get Particle Effect Opacity**<br/>
    /// Gets a particle effect's opacity.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` (Particle):
    ///   Effect to get
    ///   opacity of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Particle opacity
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetParticleOpac(...);
    /// **Get Item Break Sound**<br/>
    /// Gets which sound an item
    /// makes when it breaks.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get break sound from
    ///
    /// ## Returns
    /// - `df_sound` (Sound):
    ///   Break sound
    ///
    /// ## Additional Info
    /// - Pitch and volume are set to 1.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetBreakSound(...);
    /// **Get Items by Minecraft Tag**<br/>
    /// Gets a list of items that
    /// belongs to the specified
    /// item tag.
    ///
    /// ## Tags
    /// - Return Value Type:
    ///   - `Material ID (golden_apple)` (Default)
    ///   - `Material Name (Golden Apple)`
    ///   - `Item`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` (String):
    ///   Item tag ID
    ///   - Example: "#enchantable/sword"
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Materials
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemByMCTag(ReturnSPECIALSpace_ValueSPECIALSpace_Type : df_string, ...);
    /// **Set Particle Effect Fade Color**<br/>
    /// Sets a particle effect's particle
    /// fade color.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` `?` (Particle):
    ///   Effect to
    ///   change
    /// - `df_string` (String):
    ///   Color hexadecimal
    ///   - Example: "#FF0000" (red)
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetParticleFade(...);
    /// **Set Vector Component**<br/>
    /// Sets a vector's X, Y, or Z
    /// component.
    ///
    /// ## Tags
    /// - Component:
    ///   - `X` (Default)
    ///   - `Y`
    ///   - `Z`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` `?` (Vector):
    ///   Vector to change
    /// - `df_number` (Number):
    ///   Component
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetVectorComp(Component : df_string, ...);
    /// **Parse Number from String**<br/>
    /// Converts a string to a number.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` `?` (String):
    ///   String to convert
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Parsed number
    ///
    /// ## Additional Info
    /// - If the string is not a valid
    ///   number, the result is 0.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ParseNumber(...);
    /// **Set to Exponential**<br/>
    /// Raises a number to the power
    /// of an exponent.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `?` (Number):
    ///   Number input
    /// - `df_number` `?` (Number):
    ///   Exponent
    ///   - Default = 2 (input²)
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Exponent(...);
    ///
    /// ## Tags
    /// - Ignore Pitch:
    ///   - `True`
    ///   - `False` (Default)
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ShiftAllDirs(IgnoreSPECIALSpace_Pitch : df_string, ...);
    /// **Get List Index of Value**<br/>
    /// Searches for a value in a list
    /// variable and gets the index if
    /// found.
    ///
    /// ## Tags
    /// - Search Order:
    ///   - `Ascending (first index)` (Default)
    ///   - `Descending (last index)`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_list` (List):
    ///   List to search in
    /// - `df_opaque` (Any):
    ///   Value to search
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Value index
    ///
    /// ## Additional Info
    /// - Returns 0 if the value is not
    ///   found in the list.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetValueIndex(SearchSPECIALSpace_Order : df_string, ...);
    /// **Set Weapon Properties**<br/>
    /// Adds weapon properties to an item.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_number` `?` (Number):
    ///   Durability reduction per attack
    ///   - Default = 1
    /// - `df_number` `?` (Number):
    ///   Seconds to disable blocking for
    ///   - Default = 0
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetItemWeapon(...);
    /// **Remove Item Enchantment**<br/>
    /// Removes an enchantment from an item.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_string` (String):
    ///   Enchantment name
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RemItemEnchant(...);
    /// **Add Item Lore**<br/>
    /// Adds lines to an item's lore.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_text` `[]` (Styled Text):
    ///   Lore to add
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__AddItemLore(...);
    /// **Get Consumable Property**<br/>
    /// Gets an item's consumable property.
    ///
    /// ## Tags
    /// - Consumable Property:
    ///   - `Nutrition` (Default)
    ///   - `Saturation`
    ///   - `Use Duration`
    ///   - `Animation`
    ///   - `Sound`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get properties of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Property value
    /// - OR
    /// - `df_string` (String):
    ///   Property value
    /// - OR
    /// - `df_sound` (Sound):
    ///   Property value
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetConsumable(ConsumableSPECIALSpace_Property : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetBookText(...);
    /// **Set Particle Effect Roll**<br/>
    /// Sets a particle effect's roll.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` `?` (Particle):
    ///   Effect to
    ///   change
    /// - `df_number` (Number):
    ///   Particle roll
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetParticleRoll(...);
    /// **Set Sound Variant**<br/>
    /// Sets the variant of a sound.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_sound` `?` (Sound):
    ///   Sound to change
    /// - `df_string` `?` (String):
    ///   Variant ID (e.g. "break1")
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetSoundVariant(...);
    ///
    /// ## Tags
    /// - Shift Direction:
    ///   - `(+) Upwards / (-) Downwards`
    ///   - `(+) Forwards / (-) Backwards` (Default)
    ///   - `(+) Right / (-) Left`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ShiftLocation(ShiftSPECIALSpace_Direction : df_string, ...);
    /// **Randomize List**<br/>
    /// Randomizes the order of a
    /// list variable's values.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_list` `?` (List):
    ///   List to randomize
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Randomized list
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RandomizeList(...);
    /// **Set Item Hidden Components**<br/>
    /// Sets an item's components that should
    /// show up in its lore.
    ///
    /// ## Tags
    /// - Hiding Mode:
    ///   - `Hide`
    ///   - `Show`
    ///   - `Hide all` (Default)
    ///   - `Show all`
    ///   - `Hide all except`
    ///   - `Show all except`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_string` `[]?` (String):
    ///   Item component(s) to hide
    ///   - Item component keys,
    ///     Example: "attribute_modifiers"
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__HiddenComponents(HidingSPECIALSpace_Mode : df_string, ...);
    /// **Clamp Number**<br/>
    /// Checks if a number is between
    /// a minimum and maximum value, and
    /// if not, sets it to the nearest.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `?` (Number):
    ///   Number to clamp
    /// - `df_number` (Number):
    ///   Minimum
    /// - `df_number` (Number):
    ///   Maximum
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Result
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ClampNumber(...);
    ///
    /// ## Tags
    /// - Round Mode:
    ///   - `Floor`
    ///   - `Nearest` (Default)
    ///   - `Ceiling`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__Round(RoundSPECIALSpace_Mode : df_string, ...);
    /// **Get Item Model**<br/>
    /// Gets an item's model.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get model of
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Model key, e.g. "minecraft:stone"
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemModel(...);
    /// **Get Sound Pitch**<br/>
    /// Gets a sound's pitch or
    /// note.
    ///
    /// ## Tags
    /// - Return Value Type:
    ///   - `Pitch (number)` (Default)
    ///   - `Note (text)`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_sound` (Sound):
    ///   Sound to get pitch or
    ///   note of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Sound pitch
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetSoundPitch(ReturnSPECIALSpace_ValueSPECIALSpace_Type : df_string, ...);
    /// **Translate Legacy Colors**<br/>
    /// Converts legacy color codes written in
    /// "&" or hex format to
    /// functional color codes, or vice versa.
    ///
    /// ## Tags
    /// - Translation Type:
    ///   - `From hex to color`
    ///   - `From & to color` (Default)
    ///   - `From color to &`
    ///   - `Strip color`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` `?` (String):
    ///   String to translate
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__TranslateColors(TranslationSPECIALSpace_Type : df_string, ...);
    /// **Get Block Growth**<br/>
    /// Gets a block's current
    /// growth at a location.
    ///
    /// ## Tags
    /// - Growth Unit:
    ///   - `Growth stage number` (Default)
    ///   - `Growth percentage`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Block location
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Growth level
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetBlockGrowth(GrowthSPECIALSpace_Unit : df_string, ...);
    /// **Get All Item Custom Tags**<br/>
    /// Gets all tags registered
    /// on an item.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get tags of
    ///
    /// ## Returns
    /// - `df_dict` (Dict):
    ///   Key represents the
    ///   tag name as String, value represents
    ///   tag value
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetAllItemTags(...);
    /// **Remove List Value**<br/>
    /// Removes all matching values
    /// from a list variable.
    ///
    /// ## Tags
    /// - Items to remove:
    ///   - `All Matches` (Default)
    ///   - `First Match`
    ///   - `Last Match`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   List to change
    /// - `df_opaque` `[]` (Any):
    ///   Value(s) to
    ///   remove
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__RemoveListValue(ItemsSPECIALSpace_toSPECIALSpace_remove : df_string, ...);
    /// **Convert Bytes to String**<br/>
    /// Converts a list of numbers
    /// representing bytes to a string.
    ///
    /// ## Tags
    /// - Signed:
    ///   - `True` (Default)
    ///   - `False`
    /// - Charset:
    ///   - `UTF-8` (Default)
    ///   - `ASCII`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_number` `[]` (Byte):
    ///   Byte(s) to convert
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Converted value
    ///
    /// ## Additional Info
    /// - Returns 0 if the result is too large.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__BytesToString(Signed : df_string, Charset : df_string, ...);
    /// **Shift Location in All Directions**<br/>
    /// Shifts a location in multiple directions
    /// based on its rotation.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` `?` (Location):
    ///   Location to shift
    /// - `df_number` `?` (Number):
    ///   Forwards change
    /// - `df_number` `?` (Number):
    ///   Upwards change
    /// - `df_number` `?` (Number):
    ///   Sideways change (-L / +R)
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ShiftAllDirections(...);
    /// **Get Value Noise**<br/>
    /// Gets value noise: A type of noise
    /// based on a lattice of random values.
    ///
    /// ## Tags
    /// - Dimensions:
    ///   - `3D` (Default)
    ///   - `2D`
    /// - Domain Fractal:
    ///   - `Progressive` (Default)
    ///   - `Independent`
    /// - Return Type:
    ///   - `Value` (Default)
    ///   - `Cubic`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Noise location
    /// - `df_number` `?` (Number):
    ///   Frequency
    ///   - Modifies the scale that the
    ///     noise generates at. Default = 1
    /// - `df_number` `?` (Number):
    ///   Marbling
    ///   - Every 0.5 adds an additional ring
    ///     to the noise output. Default = 0.5
    /// - `df_number` `?` (Number):
    ///   Octaves
    ///   - Adds an additional layer of value noise
    ///     to the output. 1-16, Default = 1
    /// - `df_number` `?` (Number):
    ///   Lacunarity
    ///   - Every time a new octave is added,
    ///     the frequency is multiplied
    ///     by this value. Default = 1.5
    /// - `df_number` `?` (Number):
    ///   Gain
    ///   - Every time a new octave is added,
    ///     the weight of it is multiplied
    ///     by this value. Default = 0.5
    /// - `df_number` `?` (Number):
    ///   Warping
    ///   - Distorts the noise output. Default = 0
    /// - `df_number` `?` (Number):
    ///   Resonance
    ///   - Modifies the frequency of warping. Default = 1
    /// - `df_number` `?` (Number):
    ///   Domains
    ///   - Adds an additional layer of warping
    ///     to the output. Default = 1
    /// - `df_number` `?` (Number):
    ///   Domain lacunarity
    ///   - Every time a new domain is added,
    ///     the resonance is multiplied
    ///     by this value. Default = 1.5
    /// - `df_number` `?` (Number):
    ///   Domain gain
    ///   - Every time a new domain is added,
    ///     the weight of it is multiplied
    ///     by this value. Default = 0.5
    /// - `df_number` `?` (Number):
    ///   Generation seed
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   A decimal value between 0 and 1
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ValueNoise(Dimensions : df_string, DomainSPECIALSpace_Fractal : df_string, ReturnSPECIALSpace_Type : df_string, ...);
    /// **Get Web Response**<br/>
    /// Gets the response from a web request.
    /// 
    /// The output dictionary has 3 keys:
    /// "status": The HTTP status code.
    /// "statusText": The HTTP status message.
    /// And "body", or "json" if the response is JSON.
    ///
    /// ## Tags
    /// - Request Method:
    ///   - `Post` (Default)
    ///   - `Get`
    ///   - `Put`
    ///   - `Delete`
    /// - Content Type:
    ///   - `text/plain` (Default)
    ///   - `application/json`
    /// - Code Flow:
    ///   - `Synchronous` (Default)
    ///   - `Asynchronous`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to store response
    /// - `df_string` (String):
    ///   URL to request
    /// - `df_string` `?` (String):
    ///   Content body
    ///
    /// ## Returns
    /// - `df_dict` (Dict):
    ///   Web Response
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__WebResponse(RequestSPECIALSpace_Method : df_string, ContentSPECIALSpace_Type : df_string, CodeSPECIALSpace_Flow : df_string, ...);
    /// **Set All Item Custom Tags**<br/>
    /// Sets custom tags on an item
    /// based on the input dictionary.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_dict` (Dict):
    ///   Tags
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetAllItemTags(...);
    /// **Set Placeable Blocks**<br/>
    /// Sets which blocks an item
    /// can be placed on in Adventure Mode.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` `?` (Item):
    ///   Item to change
    /// - `df_item` `[]` (Block):
    ///   Placeable blocks
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetCanPlaceOn(...);
    /// **Get Block Material**<br/>
    /// Gets a block's material
    /// at a location.
    ///
    /// ## Tags
    /// - Return Value Type:
    ///   - `Block ID (oak_log)` (Default)
    ///   - `Block name (Oak Log)`
    ///   - `Item`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Block location
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Material ID or name
    /// - OR
    /// - `df_item` (Item):
    ///   Material as item
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetBlockType(ReturnSPECIALSpace_ValueSPECIALSpace_Type : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ParsePitch(...);
    /// **Get Dictionary Value**<br/>
    /// Get a dictionary variable's
    /// value for a key.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_dict` (Dict):
    ///   Dictionary to
    ///   pull from
    /// - `df_string` (String):
    ///   Key
    ///
    /// ## Returns
    /// - `df_opaque` (Any):
    ///   Value of the
    ///   corresponding key in the
    ///   dictionary
    ///
    /// ## Additional Info
    /// - If the dictionary does not
    ///   contain the key, 0 is returned.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetDictValue(...);
    /// **Get Container Lock**<br/>
    /// Gets a container's lock key at a
    /// location.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Container location
    ///
    /// ## Returns
    /// - `df_string` (String):
    ///   Lock key
    /// - OR
    /// - `df_number` (Number):
    ///   0 (Not lockable)
    ///
    /// ## Additional Info
    /// - Returns "none" if the container
    ///   is not locked.
    /// - Returns 0 if the container
    ///   is not lockable.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ContainerLock(...);
    /// **Get Weapon Property**<br/>
    /// Gets an item's weapon property.
    ///
    /// ## Tags
    /// - Weapon Property:
    ///   - `Durability reduction per attack` (Default)
    ///   - `Seconds to disable blocking for`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get property of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Property value
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetItemWeapon(WeaponSPECIALSpace_Property : df_string, ...);
    /// **Get Block Power**<br/>
    /// Gets a block's redstone
    /// power level.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_location` (Location):
    ///   Block location
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Power level
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetBlockPower(...);
    /// **Get Vector Length**<br/>
    /// Gets a vector's length.
    ///
    /// ## Tags
    /// - Length Type:
    ///   - `Length` (Default)
    ///   - `Length Squared`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_vector` (Vector):
    ///   Vector to get
    ///   length of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Vector length
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetVectorLength(LengthSPECIALSpace_Type : df_string, ...);
    /// **Get Text Content Length**<br/>
    /// Gets the number of characters
    /// a styled text has, ignoring all
    /// formatting tags.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_text` (Styled Text):
    ///   Text to measure
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Text length
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__ContentLength(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetModelData(...);
    /// **Convert String to Bytes**<br/>
    /// Converts a string to a list of
    /// numbers representing bytes.
    ///
    /// ## Tags
    /// - Signed:
    ///   - `True` (Default)
    ///   - `False`
    /// - Charset:
    ///   - `UTF-8` (Default)
    ///   - `ASCII`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` (String):
    ///   String to convert
    ///
    /// ## Returns
    /// - `df_list` (List):
    ///   Converted value
    ///
    /// ## Additional Info
    /// - Returns 0 if the result is too large.
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__StringToBytes(Signed : df_string, Charset : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__SetCoords(...);
    /// **Get Item Maximum Stack Size**<br/>
    /// Gets an item's maximum stack size.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item to get maximum stack
    ///   size of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Maximum stack size
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetMaxAmount(...);
    /// **Get Particle Effect Size**<br/>
    /// Gets a particle effect's particle
    /// size.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_particle` (Particle):
    ///   Effect to get
    ///   size of
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Particle size
    ///
    pub unsafe fn DF_ACTION__setSPECIALSpace_variable__GetParticleSize(...);
    /// **Set Display Rotation from Euler Angles**<br/>
    /// Sets the left or right rotation of a
    /// display entity from 3 angles on
    /// each axis.
    ///
    /// ## Tags
    /// - Rotation Type:
    ///   - `Left Rotation` (Default)
    ///   - `Right Rotation`
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Pitch (0-360)
    /// - `df_number` (Number):
    ///   Yaw (0-360)
    /// - `df_number` (Number):
    ///   Roll (0-360)
    /// - OR
    /// - `df_vector` (Vector):
    ///   Pitch/Yaw/Roll Vector
    ///
    /// ## Works With
    /// - `Any Display Entity`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DispRotationEuler(RotationSPECIALSpace_Type : df_string, ...);
    /// **Shear**<br/>
    /// Sets a mob in the sheared
    /// state.
    ///
    /// ## Works With
    /// - `Bogged`
    /// - `Mooshroom`
    /// - `Sheep`
    /// - `Snow Golem`
    ///
    /// ## Additional Info
    /// - Shearing some mobs may
    ///   turn them into different
    ///   mob types.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__Shear(...);
    /// **Set Velocity**<br/>
    /// Sets an entity's movement
    /// velocity.
    ///
    /// ## Tags
    /// - Add to Current Velocity:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_vector` (Vector):
    ///   New velocity
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetVelocity(AddSPECIALSpace_toSPECIALSpace_CurrentSPECIALSpace_Velocity : df_string, ...);
    /// **Set Glow Squid Dark Ticks**<br/>
    /// Sets the number of ticks a
    /// glow squid will stop glowing for.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Ticks
    ///
    /// ## Works With
    /// - `Glow Squid`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetGlowSquidDark(...);
    /// **Set Display Rotation from Axis-Angle**<br/>
    /// Sets the left or right rotation of
    /// a display entity from axis-angle
    /// rotation.
    ///
    /// ## Tags
    /// - Rotation Type:
    ///   - `Left Rotation` (Default)
    ///   - `Right Rotation`
    ///
    /// ## Arguments
    /// - `df_vector` (Vector):
    ///   Axis vector
    /// - `df_number` (Number):
    ///   Angle (0-360)
    ///
    /// ## Works With
    /// - `Any Display Entity`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DispRotAxisAngle(RotationSPECIALSpace_Type : df_string, ...);
    /// **Damage**<br/>
    /// Damages a mob.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Damage to inflict
    ///   - ❤ = 2 Health
    /// - 
    /// - `df_string` `?` (String):
    ///   UUID of damager entity
    /// - OR
    /// - `df_text` `?` (Styled Text):
    ///   Name of damager entity
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__Damage(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Set Sitting**<br/>
    /// Sets whether an entity
    /// is sitting.
    ///
    /// ## Tags
    /// - Is Sitting:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Camel`
    /// - `Cat`
    /// - `Fox`
    /// - `Parrot`
    /// - `Wolf`
    ///
    /// ## Additional Info
    /// - Works on both tamed
    ///   and untamed mobs.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetMobSitting(IsSPECIALSpace_Sitting : df_string, ...);
    /// **Send Mob Animation**<br/>
    /// Makes a mob perform
    /// an animation.
    ///
    /// ## Tags
    /// - Animation Type:
    ///   - `Hurt animation` (Default)
    ///   - `Crit particles`
    ///   - `Enchanted hit particles`
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SendAnimation(AnimationSPECIALSpace_Type : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DisableGlowing(...);
    /// **Set Warden Anger Level**<br/>
    /// Sets the anger level
    /// of a Warden.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Anger level
    ///   (0-150)
    /// - 
    /// - `df_string` (String):
    ///   Entity name
    /// - OR
    /// - `df_text` (Styled Text):
    ///   Entity UUID
    ///
    /// ## Works With
    /// - `Warden`
    ///
    /// ## Additional Info
    /// - If the anger level reaches
    ///   80, the warden will actively
    ///   purse the target.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetWardenAnger(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Set Horse Pattern**<br/>
    /// Sets a horse's color and pattern.
    ///
    /// ## Tags
    /// - Horse Color:
    ///   - `White`
    ///   - `Buckskin`
    ///   - `Flaxen chestnut` (Default)
    ///   - `Bay`
    ///   - `Black`
    ///   - `Dapple gray`
    ///   - `Dark bay`
    ///   - `Don't change`
    /// - Horse Markings:
    ///   - `No markings`
    ///   - `Stockings and blaze` (Default)
    ///   - `Paint`
    ///   - `Snowflake appaloosa`
    ///   - `Sooty`
    ///   - `Don't change`
    ///
    /// ## Works With
    /// - `Horse`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetHorsePattern(HorseSPECIALSpace_Color : df_string, HorseSPECIALSpace_Markings : df_string, ...);
    /// **Heal**<br/>
    /// Restores a mob's health.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Amount to heal
    ///   - ❤ = 2 Health
    /// - OR
    /// - None:
    ///   Heals to full health
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__Heal(...);
    /// **Set Panda Sad Ticks**<br/>
    /// Makes a panda sad for
    /// the specified duration.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Ticks
    ///
    /// ## Works With
    /// - `Panda`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetPandaSadTicks(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetItemOwner(...);
    /// **Set Dye Color**<br/>
    /// Sets a mob's dye color.
    ///
    /// ## Tags
    /// - Dye:
    ///   - `White` (Default)
    ///   - `Orange`
    ///   - `Magenta`
    ///   - `Light blue`
    ///   - `Yellow`
    ///   - `Lime`
    ///   - `Pink`
    ///   - `Gray`
    ///   - `Light gray`
    ///   - `Cyan`
    ///   - `Purple`
    ///   - `Blue`
    ///   - `Brown`
    ///   - `Green`
    ///   - `Red`
    ///   - `Black`
    ///
    /// ## Works With
    /// - `Sheep`
    /// - `Shulker`
    /// - `Dog (collar)`
    /// - `Cat (collar)`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetDyeColor(Dye : df_string, ...);
    /// **Launch Up**<br/>
    /// Launches an entity up or down.
    ///
    /// ## Tags
    /// - Add to Current Velocity:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Launch power
    ///
    /// ## Additional Info
    /// - A negative launch power will
    ///   launch the entity down.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__LaunchUp(AddSPECIALSpace_toSPECIALSpace_CurrentSPECIALSpace_Velocity : df_string, ...);
    /// **Set Animal Age**<br/>
    /// Sets an animal's age.
    ///
    /// ## Tags
    /// - Age Lock:
    ///   - `Enable`
    ///   - `Disable`
    ///   - `Don't change` (Default)
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Age
    ///
    /// ## Works With
    /// - `Any Breedable`
    ///
    /// ## Additional Info
    /// - 0: Target is an adult that
    ///   can breed. Age stays at 0.
    /// - -1 and below: Target is a
    ///   baby. Age goes up each tick.
    /// - 1 and above: Target is an
    ///   adult that can't breed. Age
    ///   goes down each tick.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetAge(AgeSPECIALSpace_Lock : df_string, ...);
    /// **Set Custom Name**<br/>
    /// Sets an entity's custom name.
    ///
    /// ## Tags
    /// - Name Tag Visibility:
    ///   - `Always` (Default)
    ///   - `Default`
    ///   - `Never`
    ///   - `Don't change`
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   Custom name
    /// - OR
    /// - None:
    ///   Removes custom name
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SPECIALSpace_SetNameSPECIALSpace_(NameSPECIALSpace_TagSPECIALSpace_Visibility : df_string, ...);
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__NoGravity(...);
    /// **Set Arms Raised**<br/>
    /// Sets whether a mob has its
    /// arms raised.
    ///
    /// ## Tags
    /// - Arms Raised:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Any Zombie`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetArmsRaised(ArmsSPECIALSpace_Raised : df_string, ...);
    ///
    /// ## Tags
    /// - Relative to Entity Base:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetMoveSpeed(RelativeSPECIALSpace_toSPECIALSpace_EntitySPECIALSpace_Base : df_string, ...);
    /// **Set Invulnerable**<br/>
    /// Sets whether an entity is
    /// invulnerable to damage.
    ///
    /// ## Tags
    /// - Invulnerable:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Additional Info
    /// - An invulnerable entity does not
    ///   trigger damage events.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetInvulnerable(Invulnerable : df_string, ...);
    /// **Set Friction**<br/>
    /// Changes the type of friction
    /// an entity experiences.
    ///
    /// ## Tags
    /// - Friction Type:
    ///   - `Normal`
    ///   - `No Friction` (Default)
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetFriction(FrictionSPECIALSpace_Type : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__ProjColl(...);
    ///
    /// ## Tags
    /// - Is Visible:
    ///   - `True`
    ///   - `False`
    ///   - `Don't change` (Default)
    /// - Is Marker (No Hitbox):
    ///   - `True`
    ///   - `False`
    ///   - `Don't change` (Default)
    /// - Allow Item Taking / Adding:
    ///   - `True`
    ///   - `False`
    ///   - `Don't change` (Default)
    /// - Has Physics / Updates:
    ///   - `True`
    ///   - `False`
    ///   - `Don't change` (Default)
    /// - Is Small:
    ///   - `True`
    ///   - `False`
    ///   - `Don't change` (Default)
    /// - Has Arms:
    ///   - `True`
    ///   - `False`
    ///   - `Don't change` (Default)
    /// - Has Base Plate:
    ///   - `True`
    ///   - `False`
    ///   - `Don't change` (Default)
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__ArmorStandTags(IsSPECIALSpace_Visible : df_string, IsSPECIALSpace_MarkerSPECIALSpace_SPECIALLeftparen_NoSPECIALSpace_HitboxSPECIALRightparen_ : df_string, AllowSPECIALSpace_ItemSPECIALSpace_TakingSPECIALSpace_SPECIALSlash_SPECIALSpace_Adding : df_string, HasSPECIALSpace_PhysicsSPECIALSpace_SPECIALSlash_SPECIALSpace_Updates : df_string, IsSPECIALSpace_Small : df_string, HasSPECIALSpace_Arms : df_string, HasSPECIALSpace_BaseSPECIALSpace_Plate : df_string, ...);
    /// **Set Pickup Delay**<br/>
    /// Sets the number of ticks a
    /// dropped item cannot be
    /// picked up for.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Delay
    ///
    /// ## Additional Info
    /// - If set to 0 upon spawning,
    ///   the item can be picked up
    ///   immediately.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetPickupDelay(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DropItems(...);
    /// **Set Miscellaneous Attribute**<br/>
    /// Sets one of the entity's miscellaneous
    /// attributes such as scale and
    /// burning time.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Scale` (Default)
    ///   - `Follow range`
    ///   - `Zombie spawn reinforcements`
    ///   - `Oxygen bonus`
    ///   - `Burning time`
    ///   - `Camera distance`
    ///   - `Tempt range`
    /// - Value Type:
    ///   - `Direct` (Default)
    ///   - `Percentage (Base)`
    ///   - `Percentage (Relative)`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Value
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__MiscAttribute(Attribute : df_string, ValueSPECIALSpace_Type : df_string, ...);
    /// **Set Creeper Explosion Power**<br/>
    /// Sets a creeper's explosion power.
    /// This affects the damage and area
    /// of effect.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Power (0-25)
    ///   - Default = 3 (6 for charged
    ///     creepers)
    ///
    /// ## Works With
    /// - `Creeper`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetCreeperPower(...);
    /// **Set Marker**<br/>
    /// Sets whether an armor stand
    /// is a marker.
    ///
    /// ## Tags
    /// - Marker:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetMarker(Marker : df_string, ...);
    /// **Remove Entity Custom Tag**<br/>
    /// Removes a custom tag
    /// from an entity.
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Tag name
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__RemoveCustomTag(...);
    /// **Set Absorption Health**<br/>
    /// Sets an entity's absorption
    /// health (golden hearts).
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Absorption health
    ///   - ❤ = 2 Health
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    /// ## Additional Info
    /// - The target does not need to
    ///   have an absorption effect.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetAbsorption(...);
    /// **Set Creeper Charged**<br/>
    /// Sets whether a creeper
    /// has the charged effect.
    ///
    /// ## Tags
    /// - Charged:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Creeper`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__CreeperCharged(Charged : df_string, ...);
    /// **Set Fire Ticks**<br/>
    /// Sets the remaining time an entity is on
    /// fire for.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Ticks
    ///
    /// ## Additional Info
    /// - 0 ticks mean the target is not on fire.
    /// - Ghasts, magma cubes, vexes, withers,
    ///   wither skulls, tnt, falling blocks, and
    ///   shulker bullets cannot be set on fire.
    /// - Blazes do not rely on fire ticks to
    ///   display the fire effect.
    /// - This triggers the Entity Combust event.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetFireTicks(...);
    /// **Set Combat Attribute**<br/>
    /// Sets one of the entity's combat-related
    /// attributes such as attack damage
    /// and attack speed.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Attack damage` (Default)
    ///   - `Attack knockback`
    /// - Value Type:
    ///   - `Direct` (Default)
    ///   - `Percentage (Base)`
    ///   - `Percentage (Relative)`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Value
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__CombatAttribute(Attribute : df_string, ValueSPECIALSpace_Type : df_string, ...);
    /// **Set Custom Name**<br/>
    /// Sets an entity's custom name.
    ///
    /// ## Tags
    /// - Hide Name Tag:
    ///   - `True`
    ///   - `False` (Default)
    ///   - `Don't change`
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   Custom name
    /// - OR
    /// - None:
    ///   Removes custom name
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetName(HideSPECIALSpace_NameSPECIALSpace_Tag : df_string, ...);
    /// **Jump**<br/>
    /// Causes a mob to jump.
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__Jump(...);
    /// **Set Block Display Block**<br/>
    /// Sets the displayed block
    /// of a block display.
    ///
    /// ## Arguments
    /// - `df_item` (Block):
    ///   Displayed block
    /// - `df_string` `[]?` (Block Tag):
    ///   Block data
    ///   - Example: "facing=up", "half=top"
    ///
    /// ## Works With
    /// - `Block Display`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__BDisplayBlock(...);
    /// **Set Freeze Ticks**<br/>
    /// Sets an entity's current
    /// freeze ticks.
    ///
    /// ## Tags
    /// - Ticking Locked:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Ticks
    ///   (0-140)
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetFreezeTicks(TickingSPECIALSpace_Locked : df_string, ...);
    /// **Set Text Display See-through**<br/>
    /// Sets whether a text display
    /// is visible through walls
    /// or not.
    ///
    /// ## Tags
    /// - See-through:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Text Display`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__TDisplaySeeThru(SeeSPECIALHyphen_through : df_string, ...);
    /// **Set Gliding**<br/>
    /// Sets whether an entity
    /// is gliding.
    ///
    /// ## Tags
    /// - Gliding:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    /// ## Additional Info
    /// - The entity must be
    ///   wearing an elytra.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetGliding(Gliding : df_string, ...);
    /// **Set Rotation**<br/>
    /// Changes an entity's pitch and
    /// yaw without teleporting it.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Pitch (-90 to 90)
    /// - `df_number` (Number):
    ///   Yaw (-180 to 180)
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetRotation(...);
    /// **Set Panda Rolling**<br/>
    /// Sets whether a panda is
    /// rolling or not.
    ///
    /// ## Tags
    /// - Roll Type:
    ///   - `Roll` (Default)
    ///   - `Stop Rolling`
    ///
    /// ## Works With
    /// - `Panda`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetPandaRolling(RollSPECIALSpace_Type : df_string, ...);
    /// **Set Tropical Fish Pattern**<br/>
    /// Sets a tropical fish's
    /// color and pattern.
    ///
    /// ## Tags
    /// - Pattern Color:
    ///   - `White` (Default)
    ///   - `Orange`
    ///   - `Magenta`
    ///   - `Light blue`
    ///   - `Yellow`
    ///   - `Lime`
    ///   - `Pink`
    ///   - `Gray`
    ///   - `Light gray`
    ///   - `Cyan`
    ///   - `Purple`
    ///   - `Blue`
    ///   - `Brown`
    ///   - `Green`
    ///   - `Red`
    ///   - `Black`
    ///   - `Don't change`
    /// - Body Color:
    ///   - `White` (Default)
    ///   - `Orange`
    ///   - `Magenta`
    ///   - `Light blue`
    ///   - `Yellow`
    ///   - `Lime`
    ///   - `Pink`
    ///   - `Gray`
    ///   - `Light gray`
    ///   - `Cyan`
    ///   - `Purple`
    ///   - `Blue`
    ///   - `Brown`
    ///   - `Green`
    ///   - `Red`
    ///   - `Black`
    ///   - `Don't change`
    /// - Pattern:
    ///   - `Kob` (Default)
    ///   - `Sunstreak`
    ///   - `Snooper`
    ///   - `Dasher`
    ///   - `Brinely`
    ///   - `Spotty`
    ///   - `Flopper`
    ///   - `Stripey`
    ///   - `Glitter`
    ///   - `Blockfish`
    ///   - `Betty`
    ///   - `Clayfish`
    ///   - `Don't change`
    ///
    /// ## Works With
    /// - `Tropical Fish`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetFishPattern(PatternSPECIALSpace_Color : df_string, BodySPECIALSpace_Color : df_string, Pattern : df_string, ...);
    /// **Set Temperature Type**<br/>
    /// Sets a mob's temperature variant.
    ///
    /// ## Tags
    /// - Temperature Type:
    ///   - `Cold`
    ///   - `Temperate` (Default)
    ///   - `Warm`
    ///
    /// ## Works With
    /// - `Chicken`
    /// - `Cow`
    /// - `Frog`
    /// - `Pig`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetTemperature(TemperatureSPECIALSpace_Type : df_string, ...);
    /// **Set Wolf Type**<br/>
    /// Sets a wolf's variant.
    ///
    /// ## Tags
    /// - Wolf Type:
    ///   - `Ashen` (Default)
    ///   - `Black`
    ///   - `Chestnut`
    ///   - `Pale`
    ///   - `Rusty`
    ///   - `Snowy`
    ///   - `Spotted`
    ///   - `Striped`
    ///   - `Woods`
    ///
    /// ## Works With
    /// - `Wolf`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetWolfType(WolfSPECIALSpace_Type : df_string, ...);
    /// **Set Display Interpolation**<br/>
    /// Sets the interpolation
    /// properties of a display
    /// entity.
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Interpolation duration
    ///   in ticks
    ///   - Default = 0
    /// - `df_number` `?` (Number):
    ///   Interpolation delay
    ///   in ticks
    ///   - Default = 0
    ///
    /// ## Works With
    /// - `Any Display Entity`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DispInterpolation(...);
    ///
    /// ## Tags
    /// - Hand Slot:
    ///   - `Main Hand` (Default)
    ///   - `Off Hand`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetHandItem(HandSPECIALSpace_Slot : df_string, ...);
    /// **Set Enderman Held Block**<br/>
    /// Set an enderman's held block.
    ///
    /// ## Arguments
    /// - `df_item` (Block):
    ///   Block to hold
    ///
    /// ## Works With
    /// - `Enderman`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetEndermanBlock(...);
    /// **Launch Toward Location**<br/>
    /// Launches an entity toward or away
    /// from a location.
    ///
    /// ## Tags
    /// - Add to Current Velocity:
    ///   - `True` (Default)
    ///   - `False`
    /// - Ignore Distance:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Launch destination
    /// - `df_number` `?` (Number):
    ///   Launch power
    ///
    /// ## Additional Info
    /// - A negative launch power will launch
    ///   the entity away from the location.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__LaunchToward(AddSPECIALSpace_toSPECIALSpace_CurrentSPECIALSpace_Velocity : df_string, IgnoreSPECIALSpace_Distance : df_string, ...);
    /// **Set Armor Items**<br/>
    /// Sets a mob's armor items.
    /// Place the armor in slots 1-4
    /// of the chest, with 1 being the
    /// helmet and 4 being the boots.
    ///
    /// ## Arguments
    /// - `df_item` `[]` (Item):
    ///   Armor to set
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    /// ## Additional Info
    /// - Any block or item can render
    ///   on the target's head if placed
    ///   in the 'helmet' slot.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetArmor(...);
    /// **Get Entity Custom Tag**<br/>
    /// Gets the value of a custom
    /// entity tag.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_string` (String):
    ///   Tag name
    ///
    /// ## Returns
    /// - `df_opaque` (Any):
    ///   Tag value
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__GetCustomTag(...);
    /// **Set Interaction Size**<br/>
    /// Sets the hitbox size of
    /// an interaction entity.
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Width
    ///   - Default = 1
    /// - `df_number` `?` (Number):
    ///   Height
    ///   - Default = 1
    ///
    /// ## Works With
    /// - `Interaction Entity`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__InteractionSize(...);
    /// **Face Location**<br/>
    /// Rotates an entity to look
    /// toward a location without
    /// teleporting them.
    ///
    /// ## Tags
    /// - Face Direction:
    ///   - `Toward location` (Default)
    ///   - `Away from location`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Location to face
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__FaceLocation(FaceSPECIALSpace_Direction : df_string, ...);
    /// **Set Cat Type**<br/>
    /// Sets a cat's skin type.
    ///
    /// ## Tags
    /// - Skin Type:
    ///   - `Tabby` (Default)
    ///   - `Tuxedo`
    ///   - `Red`
    ///   - `Siamese`
    ///   - `British Shorthair`
    ///   - `Calico`
    ///   - `Persian`
    ///   - `Ragdoll`
    ///   - `White`
    ///   - `Jellie`
    ///   - `Black`
    ///
    /// ## Works With
    /// - `Cat`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetCatType(SkinSPECIALSpace_Type : df_string, ...);
    /// **Set Base Arrow Damage**<br/>
    /// Sets the base damage
    /// dealt by an arrow or trident.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Base Damage
    ///
    /// ## Works With
    /// - `Arrows, Tridents`
    ///
    /// ## Additional Info
    /// - Base Arrow Damage: 2
    /// - Base Thrown Trident Damage: 8
    /// - Total Arrow Damage: ||velocity|| x base
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetArrowDamage(...);
    /// **Set Display Scale**<br/>
    /// Sets the scale of
    /// a display entity.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   X scale
    /// - `df_number` (Number):
    ///   Y scale
    /// - `df_number` (Number):
    ///   Z scale
    /// - OR
    /// - `df_vector` (Vector):
    ///   Scale vector
    ///
    /// ## Works With
    /// - `Any Display Entity`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DisplayScale(...);
    /// **Set Text Display Text Alignment**<br/>
    /// Sets the text alignment
    /// of a text display.
    ///
    /// ## Tags
    /// - Text Alignment:
    ///   - `Center` (Default)
    ///   - `Left`
    ///   - `Right`
    ///
    /// ## Works With
    /// - `Text Display`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__TDisplayAlign(TextSPECIALSpace_Alignment : df_string, ...);
    /// **Launch Projectile**<br/>
    /// Launches a projectile from a mob.
    ///
    /// ## Arguments
    /// - `df_item` (Projectile):
    ///   Projectile to launch
    /// - `df_location` `?` (Location):
    ///   Launch point
    /// - `df_text` `?` (Styled Text):
    ///   Projectile name
    /// - `df_number` `?` (Number):
    ///   Speed
    /// - `df_number` `?` (Number):
    ///   Inaccuracy
    ///   - Controls how much random
    ///     motion is applied on launch
    ///   - Default = 1
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__LaunchProj(...);
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__EnableAI(...);
    /// **Set Display Billboard**<br/>
    /// Sets how a display entity
    /// is rotated with a
    /// player's view.
    ///
    /// ## Tags
    /// - Billboard Type:
    ///   - `Fixed` (Default)
    ///   - `Vertical`
    ///   - `Horizontal`
    ///   - `Center`
    ///
    /// ## Works With
    /// - `Any Display Entity`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DisplayBillboard(BillboardSPECIALSpace_Type : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__NoProjColl(...);
    /// **Tame**<br/>
    /// Tames and sets the owner
    /// of a tameable mob.
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Owner UUID
    /// - OR
    /// - `df_text` (Styled Text):
    ///   Owner name
    /// - OR
    /// - None:
    ///   Untames mob
    ///
    /// ## Works With
    /// - `Wolf`
    /// - `Cat`
    /// - `Parrot`
    /// - `Llama`
    /// - `Any Horse`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__Tame(...);
    /// **Set Goat Screaming**<br/>
    /// Sets whether a goat
    /// screams or not.
    ///
    /// ## Tags
    /// - Screams:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Goat`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetGoatScreaming(Screams : df_string, ...);
    /// **Set Bee Has Stinger**<br/>
    /// Sets whether a bee
    /// has its stinger.
    ///
    /// ## Tags
    /// - Has Stinger:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Bee`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetBeeStinger(HasSPECIALSpace_Stinger : df_string, ...);
    /// **Disguise as Mob**<br/>
    /// Disguises an entity as a mob.
    ///
    /// ## Arguments
    /// - `df_item` (SpawnEgg):
    ///   Mob to disguise as
    /// - `df_text` `?` (Styled Text):
    ///   Display name
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__MobDisguise(...);
    /// **Set Minecart Block**<br/>
    /// Sets the block shown inside
    /// a minecart. This does not
    /// affect its functionality.
    ///
    /// ## Arguments
    /// - `df_item` (Block):
    ///   Block to show
    /// - `df_number` `?` (Number):
    ///   Block offset
    ///
    /// ## Works With
    /// - `Any Minecart`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetMinecartBlock(...);
    /// **Set Fox Sleeping**<br/>
    /// Causes a fox to start
    /// or stop sleeping.
    ///
    /// ## Tags
    /// - Sleeping:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Fox`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__FoxSleeping(Sleeping : df_string, ...);
    /// **Lock Disguise Rotation**<br/>
    /// Locks a disguise's pitch or yaw values.
    ///
    /// ## Tags
    /// - Pitch:
    ///   - `Lock`
    ///   - `Unlock`
    ///   - `No Change` (Default)
    /// - Yaw:
    ///   - `Lock`
    ///   - `Unlock`
    ///   - `No Change` (Default)
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Pitch to lock to
    ///   - Default = 0
    /// - `df_number` `?` (Number):
    ///   Yaw to lock to
    ///   - Default = 0
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__LockDisgRotation(Pitch : df_string, Yaw : df_string, ...);
    /// **Set Equipment Item**<br/>
    /// Sets the item in one of the
    /// equipment slots (including
    /// horse items) of an entity.
    ///
    /// ## Tags
    /// - Equipment Slot:
    ///   - `Main hand` (Default)
    ///   - `Off hand`
    ///   - `Head`
    ///   - `Body`
    ///   - `Legs`
    ///   - `Feet`
    ///   - `Saddle`
    ///   - `Horse armor`
    ///   - `Decor`
    ///   - `Harness`
    ///
    /// ## Arguments
    /// - `df_item` `?` (Item):
    ///   Item to set
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetEquipment(EquipmentSPECIALSpace_Slot : df_string, ...);
    /// **Set Silenced**<br/>
    /// Sets whether an entity will
    /// produce sound effects.
    ///
    /// ## Tags
    /// - Silenced:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetSilenced(Silenced : df_string, ...);
    /// **Set Bee Has Nectar**<br/>
    /// Sets if a bee has nectar
    /// on its body.
    ///
    /// ## Tags
    /// - Has Nectar:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Bee`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetBeeNectar(HasSPECIALSpace_Nectar : df_string, ...);
    /// **Attach Lead**<br/>
    /// Attaches a lead to the target,
    /// held by an entity or lead knot.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Lead holder UUID
    /// - OR
    /// - `df_text` (Styled Text):
    ///   Lead holder name
    /// - OR
    /// - `df_location` (Location):
    ///   Lead knot location
    ///   - Spawns a Leash Knot entity.
    ///     Requires a fence block.
    ///
    /// ## Works With
    /// - `Mobs`
    /// - `Boats`
    ///
    /// ## Additional Info
    /// - Max. attach range is 10 blocks.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__AttachLead(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Remove Potion Effect**<br/>
    /// Removes one or more potion
    /// effects from an entity.
    ///
    /// ## Arguments
    /// - `df_potion` `[]` (Potion):
    ///   Effect(s)
    ///   to remove
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    /// ## Additional Info
    /// - Only the potion's type needs
    ///   to match; amplifier and duration
    ///   are disregarded.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__RemovePotion(...);
    /// **Shear Sheep**<br/>
    /// Causes a sheep to
    /// be sheared.
    ///
    /// ## Works With
    /// - `Sheep`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__ShearSheep(...);
    /// **Set Armor Stand Slot Interactions**<br/>
    /// Sets the possible interactions, such
    /// as adding or removing items, of an
    /// armor stand's slot(s).
    ///
    /// ## Tags
    /// - Interactions:
    ///   - `Take, swap or place item` (Default)
    ///   - `Take or swap item`
    ///   - `Take item`
    ///   - `Place item`
    ///   - `None`
    /// - Equipment Slot:
    ///   - `All` (Default)
    ///   - `Main hand`
    ///   - `Off hand`
    ///   - `Head`
    ///   - `Chest`
    ///   - `Legs`
    ///   - `Feet`
    ///
    /// ## Works With
    /// - `Armor Stand`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__ArmorStandSlots(Interactions : df_string, EquipmentSPECIALSpace_Slot : df_string, ...);
    /// **Set Allay Dancing**<br/>
    /// Sets whether an allay is
    /// dancing or not.
    ///
    /// ## Tags
    /// - Dancing:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Allay`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetAllayDancing(Dancing : df_string, ...);
    /// **Set Rabbit Type**<br/>
    /// Sets a rabbit's skin type.
    ///
    /// ## Tags
    /// - Skin Type:
    ///   - `Brown` (Default)
    ///   - `White`
    ///   - `Black`
    ///   - `Black and White`
    ///   - `Gold`
    ///   - `Salt and Pepper`
    ///   - `Killer`
    ///
    /// ## Works With
    /// - `Rabbit`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetRabbitType(SkinSPECIALSpace_Type : df_string, ...);
    /// **Set Size**<br/>
    /// Sets the size of an entity.
    /// This may also affect its
    /// health and strength.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Size
    ///
    /// ## Works With
    /// - `Slime`
    /// - `Magma Cube`
    /// - `Phantom`
    /// - `Pufferfish`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetSize(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__ShowName(...);
    /// **Set Angry**<br/>
    /// Sets whether a mob is
    /// angry at players.
    ///
    /// ## Tags
    /// - Angry:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Bee`
    /// - `Wolf`
    /// - `Zombified Piglin`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetAngry(Angry : df_string, ...);
    /// **Undisguise**<br/>
    /// Removes an entity's disguise.
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__Undisguise(...);
    /// **Set Death Drops Enabled**<br/>
    /// Sets whether a mob drops
    /// their items when dead.
    ///
    /// ## Tags
    /// - Has Death Drops:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetDeathDrops(HasSPECIALSpace_DeathSPECIALSpace_Drops : df_string, ...);
    /// **Set Persistent**<br/>
    /// Sets whether an item
    /// or a falling block will
    /// never despawn.
    ///
    /// ## Tags
    /// - Persistent:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Dropped Item`
    /// - `Falling Block`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetPersistent(Persistent : df_string, ...);
    /// **Set Projectile Display Item**<br/>
    /// Sets the item a projectile
    /// displays as.
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   Display item
    ///
    /// ## Works With
    /// - `Snowball`
    /// - `Egg`
    /// - `Small Fireball`
    /// - `Ghast Fireball`
    /// - `Ender Pearl`
    /// - `Experience Bottle`
    /// - `Eye of Ender`
    ///
    /// ## Additional Info
    /// - Does not work with air.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__ProjectileItem(...);
    /// **Set Name Color**<br/>
    /// Sets the color an entity's
    /// name tag appears in.
    ///
    /// ## Tags
    /// - Name Color:
    ///   - `Black` (Default)
    ///   - `Dark blue`
    ///   - `Dark green`
    ///   - `Dark aqua`
    ///   - `Dark red`
    ///   - `Dark purple`
    ///   - `Gold`
    ///   - `Gray`
    ///   - `Dark gray`
    ///   - `Blue`
    ///   - `Green`
    ///   - `Aqua`
    ///   - `Red`
    ///   - `Light purple`
    ///   - `Yellow`
    ///   - `White`
    ///   - `None`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetNameColor(NameSPECIALSpace_Color : df_string, ...);
    /// **Set Carrying Chest**<br/>
    /// Sets whether a mob carries
    /// a chest, which allows its
    /// inventory to be accessed.
    ///
    /// ## Tags
    /// - Carrying Chest:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Donkey`
    /// - `Mule`
    /// - `Llama`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetCarryingChest(CarryingSPECIALSpace_Chest : df_string, ...);
    /// **Set Parrot Color**<br/>
    /// Sets a parrot's color.
    ///
    /// ## Tags
    /// - Parrot Color:
    ///   - `Red` (Default)
    ///   - `Blue`
    ///   - `Green`
    ///   - `Cyan`
    ///   - `Gray`
    ///
    /// ## Works With
    /// - `Parrot`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetParrotColor(ParrotSPECIALSpace_Color : df_string, ...);
    /// **Set Display Translation**<br/>
    /// Sets the translation values
    /// of a display entity.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   X translation
    /// - `df_number` (Number):
    ///   Y translation
    /// - `df_number` (Number):
    ///   Z translation
    /// - OR
    /// - `df_vector` (Vector):
    ///   Translation vector
    ///
    /// ## Works With
    /// - `Any Display Entity`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DispTranslation(...);
    /// **Remove**<br/>
    /// Deletes an entity.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__Remove(...);
    /// **Set Text Display Background**<br/>
    /// Sets the background color
    /// and opacity of a text display.
    ///
    /// ## Arguments
    /// - `df_string` `?` (String):
    ///   Color hexadecimal
    ///   - Example: "#FF0000" (red)
    /// - `df_number` (Number):
    ///   Opacity in percentage
    /// - OR
    /// - None:
    ///   Uses the default
    ///   background color
    ///
    /// ## Works With
    /// - `Text Display`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__TDispBackground(...);
    /// **Set Display Culling Size**<br/>
    /// Sets the culling width
    /// and height of a
    /// display entity.
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Width
    ///   - Default = 0
    /// - `df_number` `?` (Number):
    ///   Height
    ///   - Default = 0
    ///
    /// ## Works With
    /// - `Any Display Entity`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DisplayCullingSize(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__HideName(...);
    /// **Set Sheep Sheared**<br/>
    /// Sets whether a sheep
    /// has its wool.
    ///
    /// ## Tags
    /// - Sheared:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Sheep`
    ///
    #[deprecated = "This action is planned to be removed in a future update. Please use Entity Action: Shear (Appearance) instead."]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetSheepSheared(Sheared : df_string, ...);
    /// **Set Axolotl Pattern**<br/>
    /// Sets an axolotl's color.
    ///
    /// ## Tags
    /// - Axolotl Color:
    ///   - `Pink` (Default)
    ///   - `Brown`
    ///   - `Yellow`
    ///   - `Cyan`
    ///   - `Blue`
    ///
    /// ## Works With
    /// - `Axolotl`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetAxolotlColor(AxolotlSPECIALSpace_Color : df_string, ...);
    /// **Get All Entity Custom Tags**<br/>
    /// Gets all tags registered
    /// on an entity.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    ///
    /// ## Returns
    /// - `df_dict` (Dict):
    ///   Key represents the
    ///   tag name as String, value represents
    ///   tag value
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__GetAllEntityTags(...);
    /// **Set AI**<br/>
    /// Sets whether an entity is
    /// sentient and/or affected
    /// by physics.
    ///
    /// ## Tags
    /// - AI:
    ///   - `Sentient`
    ///   - `Insentient`
    ///   - `None` (Default)
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetAI(AI : df_string, ...);
    /// **Set Knockback Attribute**<br/>
    /// Sets one of the entity's knockback-related
    /// attributes such as knockback resistance.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Knockback resistance` (Default)
    ///   - `Explosion knockback resistance`
    /// - Value Type:
    ///   - `Direct` (Default)
    ///   - `Percentage (Base)`
    ///   - `Percentage (Relative)`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Value
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__KBAttribute(Attribute : df_string, ValueSPECIALSpace_Type : df_string, ...);
    /// **Set Movement Attribute**<br/>
    /// Sets one of the entity's movement-related
    /// attributes, such as walking speed
    /// and jump height.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Walking speed` (Default)
    ///   - `Flying speed`
    ///   - `Jump strength`
    ///   - `Step height`
    ///   - `Movement efficiency`
    ///   - `Water movement efficiency`
    /// - Value Type:
    ///   - `Direct` (Default)
    ///   - `Percentage (Base)`
    ///   - `Percentage (Relative)`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Value
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__MovementAttribute(Attribute : df_string, ValueSPECIALSpace_Type : df_string, ...);
    /// **Set Riptiding**<br/>
    /// Sets whether an entity
    /// is riptiding.
    ///
    /// ## Tags
    /// - Riptiding:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetRiptiding(Riptiding : df_string, ...);
    /// **Set Arrow NoClip**<br/>
    /// Sets whether an arrow
    /// will pass through blocks
    /// and through entities.
    ///
    /// ## Tags
    /// - Has NoClip:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Arrows, Tridents`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetArrowNoClip(HasSPECIALSpace_NoClip : df_string, ...);
    /// **Set Falling Attribute**<br/>
    /// Sets one of the entity's falling-related
    /// attributes, such as gravity
    /// and fall damage multiplier.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Gravity` (Default)
    ///   - `Safe fall distance`
    ///   - `Fall damage multiplier`
    /// - Value Type:
    ///   - `Direct` (Default)
    ///   - `Percentage (Base)`
    ///   - `Percentage (Relative)`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Value
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__FallingAttribute(Attribute : df_string, ValueSPECIALSpace_Type : df_string, ...);
    /// **Set Projectile Shooter**<br/>
    /// Sets the projectile source of
    /// a projectile (or removes it).
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Shooter UUID
    /// - OR
    /// - `df_text` (Styled Text):
    ///   Shooter name
    /// - OR
    /// - None:
    ///   Removes shooter
    ///
    /// ## Works With
    /// - `Any Projectile`
    /// - `Area Effect Cloud`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetProjSource(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Set Fox Leaping**<br/>
    /// Sets whether a fox appears
    /// to be leaping.
    ///
    /// ## Tags
    /// - Leaping:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Fox`
    ///
    /// ## Additional Info
    /// - Does not affect movement.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetFoxLeaping(Leaping : df_string, ...);
    /// **Set Panda Gene**<br/>
    /// Sets the gene of a panda.
    /// This affects their behavior
    /// and appearance.
    ///
    /// ## Tags
    /// - Set Gene:
    ///   - `Main gene`
    ///   - `Hidden gene`
    ///   - `Both` (Default)
    /// - Gene Type:
    ///   - `Aggressive` (Default)
    ///   - `Lazy`
    ///   - `Weak`
    ///   - `Worried`
    ///   - `Playful`
    ///   - `Normal`
    ///   - `Brown`
    ///
    /// ## Works With
    /// - `Panda`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetPandaGene(SetSPECIALSpace_Gene : df_string, GeneSPECIALSpace_Type : df_string, ...);
    /// **Set Maximum Health**<br/>
    /// Sets an entity's maximum
    /// health.
    ///
    /// ## Tags
    /// - Heal Mob to Max Health:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Maximum health
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetMaxHealth(HealSPECIALSpace_MobSPECIALSpace_toSPECIALSpace_MaxSPECIALSpace_Health : df_string, ...);
    /// **Set Fishing Wait Time**<br/>
    /// Sets the time until a fish
    /// starts to approach a
    /// fishing hook.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Wait time (ticks)
    ///
    /// ## Works With
    /// - `Fishing Hook`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetFishingTime(...);
    /// **Set End Crystal Beam**<br/>
    /// Sets the location an end
    /// crystal points its beam at.
    ///
    /// ## Arguments
    /// - `df_location` `?` (Location):
    ///   Target
    ///
    /// ## Additional Info
    /// - To remove the beam, do not
    ///   specify a location.
    /// - The target location is
    ///   rounded to the nearest
    ///   block.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__EndCrystalBeam(...);
    /// **Eat Target**<br/>
    /// Makes a frog try to eat the
    /// specified mob or player.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Target UUID
    /// - OR
    /// - `df_text` (Styled Text):
    ///   Target name
    ///
    /// ## Works With
    /// - `Frog`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__FrogEat(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Set Salmon Type**<br/>
    /// Sets a salmon's variant.
    ///
    /// ## Tags
    /// - Salmon Type:
    ///   - `Small`
    ///   - `Medium` (Default)
    ///   - `Large`
    ///
    /// ## Works With
    /// - `Salmon`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetSalmonType(SalmonSPECIALSpace_Type : df_string, ...);
    /// **Set Display Brightness**<br/>
    /// Sets the brightness
    /// of a display entity.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Block light level (0-15)
    /// - `df_number` (Number):
    ///   Sky light level (0-15)
    /// - OR
    /// - None:
    ///   Uses the light level
    ///   at entity's location
    ///
    /// ## Works With
    /// - `Any Display Entity`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DisplayBrightness(...);
    /// **Set Villager Profession**<br/>
    /// Sets a villager's profession.
    ///
    /// ## Tags
    /// - Profession:
    ///   - `Unemployed`
    ///   - `Armorer` (Default)
    ///   - `Butcher`
    ///   - `Cartographer`
    ///   - `Cleric`
    ///   - `Farmer`
    ///   - `Fisherman`
    ///   - `Fletcher`
    ///   - `Leatherworker`
    ///   - `Librarian`
    ///   - `Mason`
    ///   - `Nitwit`
    ///   - `Shepherd`
    ///   - `Toolsmith`
    ///   - `Weaponsmith`
    ///
    /// ## Works With
    /// - `Villager`
    /// - `Zombie Villager`
    ///
    /// ## Additional Info
    /// - If the villager has no
    ///   experience yet, it is given
    ///   1 experience to prevent
    ///   unemployment.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetProfession(Profession : df_string, ...);
    /// **Clear Potion Effects**<br/>
    /// Removes all active potion
    /// effects from an entity.
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__ClearPotions(...);
    /// **Set Armor Stand Parts**<br/>
    /// Sets whether an armor stand has
    /// arms and a base plate.
    ///
    /// ## Tags
    /// - Arms:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///   - `Don't change`
    /// - Base Plate:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///   - `Don't change`
    ///
    /// ## Works With
    /// - `Armor Stand`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__ArmorStandParts(Arms : df_string, BaseSPECIALSpace_Plate : df_string, ...);
    /// **Set Name Visible**<br/>
    /// Sets whether an entity's
    /// custom name is always
    /// displayed above them.
    ///
    /// ## Tags
    /// - Name Tag Visibility:
    ///   - `Always` (Default)
    ///   - `Default`
    ///   - `Never`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SPECIALSpace_SetNameVisibleSPECIALSpace_(NameSPECIALSpace_TagSPECIALSpace_Visibility : df_string, ...);
    /// **Set Target**<br/>
    /// Instructs a mob's AI to target
    /// a specific mob or player.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   Target UUID
    /// - OR
    /// - `df_text` `[]` (Styled Text):
    ///   Target name
    /// - OR
    /// - None:
    ///   Stop targeting
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetTarget(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Set Text Display Text Shadow**<br/>
    /// Sets whether the text in
    /// a text display has
    /// shadow or not.
    ///
    /// ## Tags
    /// - Text Shadow:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Text Display`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__TDisplayShadow(TextSPECIALSpace_Shadow : df_string, ...);
    /// **Set Name Visible**<br/>
    /// Sets whether an entity's
    /// custom name is always
    /// displayed above them.
    ///
    /// ## Tags
    /// - Name Tag Visible:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Additional Info
    /// - A mob's custom name is
    ///   always visible when the
    ///   cursor is placed on them.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetNameVisible(NameSPECIALSpace_TagSPECIALSpace_Visible : df_string, ...);
    /// **Set Invulnerability Ticks**<br/>
    /// Sets the currently
    /// remaining ticks until an
    /// entity can next be hurt.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Ticks
    ///
    /// ## Additional Info
    /// - This value is set to 10
    ///   upon taking damage.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetInvulTicks(...);
    /// **Set Shulker Peek Percent**<br/>
    /// Sets how far a shulker
    /// should peek up to.
    ///
    /// ## Tags
    /// - Is Silent:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Peek Percentage
    ///
    /// ## Works With
    /// - `Shulker`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetShulkerPeek(IsSPECIALSpace_Silent : df_string, ...);
    /// **Set Pose**<br/>
    /// Changes the pose of an entity.
    /// This affects their animations
    /// and/or hitbox, depending on
    /// the pose and entity type.
    ///
    /// ## Tags
    /// - Pose:
    ///   - `Standing` (Default)
    ///   - `Sleeping`
    ///   - `Swimming`
    ///   - `Sneaking`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SPECIALSpace_SetPoseSPECIALSpace_(Pose : df_string, ...);
    /// **Set Rearing**<br/>
    /// Sets whether a horse is
    /// standing on its hind legs.
    ///
    /// ## Tags
    /// - Rearing:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Any Horse`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetRearing(Rearing : df_string, ...);
    /// **Set Potion Cloud Radius**<br/>
    /// Sets an area of effect cloud's
    /// radius and shrinking speed.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Radius
    /// - `df_number` `?` (Number):
    ///   Shrinking speed
    ///   (blocks per second)
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetCloudRadius(...);
    /// **Set Gravity**<br/>
    /// Sets whether an entity
    /// is affected by gravity.
    ///
    /// ## Tags
    /// - Gravity:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetGravity(Gravity : df_string, ...);
    /// **Mimic Entity**<br/>
    /// Disguises an entity as another
    /// currently existing entity or player.
    ///
    /// ## Tags
    /// - Remove Original Entity:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   UUID of target
    ///   to disguise as
    /// - OR
    /// - `df_text` `[]` (Styled Text):
    ///   Name of target
    ///   to disguise as
    ///
    /// ## Additional Info
    /// - The disguise will match the target exactly,
    ///   including things like color or equipment.
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__Mimic(RemoveSPECIALSpace_OriginalSPECIALSpace_Entity : df_string, ...);
    /// **Set Display Teleport Duration**<br/>
    /// Sets how long a display entity takes
    /// to visually move to its destination
    /// when it teleports.
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Teleport duration
    ///   in ticks
    ///   - Default = 0
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DispTPDuration(...);
    /// **Set Invulnerability Ticks**<br/>
    /// Sets the remaining ticks of
    /// invulnerability a wither has.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Ticks
    ///
    /// ## Works With
    /// - `Wither`
    ///
    /// ## Additional Info
    /// - If AI is enabled, this value
    ///   decreases over time. At 0,
    ///   an explosion occurs.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetWitherInvul(...);
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__Silence(...);
    /// **Set Arrow Pierce**<br/>
    /// Sets how many targets an
    /// arrow can pierce through.
    /// A pierce of 1 can hit
    /// up to 2 entities.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Targets to pierce
    ///
    /// ## Works With
    /// - `Arrows`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetArrowPierce(...);
    /// **Set Display Shadow**<br/>
    /// Sets the shadow properties
    /// of a display entity.
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Shadow radius in blocks
    ///   - Default = 1
    /// - `df_number` `?` (Number):
    ///   Shadow opacity in
    ///   percentage
    ///   - Default = 100
    ///
    /// ## Works With
    /// - `Any Display Entity`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DisplayShadow(...);
    /// **Set Interaction Responsive**<br/>
    /// Sets whether an interaction
    /// entity has response when
    /// interacting with it.
    ///
    /// ## Tags
    /// - Responsive:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Interaction Entity`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__InteractResponse(Responsive : df_string, ...);
    /// **Use Item**<br/>
    /// Forces a mob to use held items
    /// such as bow or spyglass.
    ///
    /// ## Tags
    /// - Hand:
    ///   - `Main Hand` (Default)
    ///   - `Off Hand`
    /// - Use Item:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__UseItem(Hand : df_string, UseSPECIALSpace_Item : df_string, ...);
    /// **Ride Entity**<br/>
    /// Mounts an entity on top of
    /// another entity or player.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   Target UUID
    /// - OR
    /// - `df_text` `[]` (Styled Text):
    ///   Target name
    /// - OR
    /// - None:
    ///   Dismounts entity
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__RideEntity(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Set Display Transformation Matrix**<br/>
    /// Sets the affine transformation
    /// matrix of a display entity.
    ///
    /// ## Arguments
    /// - `df_list` (List):
    ///   16 numbers describing
    ///   a row-major matrix
    ///
    /// ## Works With
    /// - `Any Display Entity`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DisplayMatrix(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__NoDrops(...);
    /// **Set Wolf Sound Type**<br/>
    /// Sets a wolf's sound variant.
    ///
    /// ## Tags
    /// - Wolf Sound Type:
    ///   - `Angry` (Default)
    ///   - `Big`
    ///   - `Classic`
    ///   - `Cute`
    ///   - `Grumpy`
    ///   - `Puglin`
    ///   - `Sad`
    ///
    /// ## Works With
    /// - `Wolf`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetWolfSoundType(WolfSPECIALSpace_SoundSPECIALSpace_Type : df_string, ...);
    /// **Set Sniffer State**<br/>
    /// Forces a sniffer to perform
    /// a specific action.
    ///
    /// ## Tags
    /// - Behavior:
    ///   - `Idle` (Default)
    ///   - `Feeling Happy`
    ///   - `Scenting`
    ///   - `Sniffing`
    ///   - `Searching`
    ///   - `Digging`
    ///
    /// ## Works With
    /// - `Sniffer`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SnifferState(Behavior : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__EnableGlowing(...);
    /// **Teleport**<br/>
    /// Teleports an entity to a
    /// specified location.
    ///
    /// ## Tags
    /// - Keep Current Rotation:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   New position
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__Teleport(KeepSPECIALSpace_CurrentSPECIALSpace_Rotation : df_string, ...);
    /// **Set Display Glow Color**<br/>
    /// Sets the glowing color
    /// of a display entity.
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Color hexadecimal
    ///   - Example: "#FF0000" (red)
    /// - OR
    /// - None:
    ///   Resets glow color
    ///
    /// ## Works With
    /// - `Item Display`
    /// - `Block Display`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DisplayGlowColor(...);
    /// **Set Visual Fire**<br/>
    /// Sets whether an entity
    /// should appear on fire.
    ///
    /// ## Tags
    /// - On Fire:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Additional Info
    /// - The affected entity's
    ///   fire ticks won't change
    ///   and won't take any damage.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetVisualFire(OnSPECIALSpace_Fire : df_string, ...);
    /// **Shift Disguise Vertically**<br/>
    /// Shifts the disguise of an entity up or
    /// down relative to the entity itself.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Y-Offset
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DisguiseShiftVert(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetAgeSPECIALSlash_Size(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__LSPECIALSpace_SetArmor(...);
    /// **Set Wearing Saddle**<br/>
    /// Sets whether a mob wears
    /// a saddle.
    ///
    /// ## Tags
    /// - Saddle:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Camel`
    /// - `Pig`
    /// - `Strider`
    /// - `Any Horse`
    ///
    /// ## Additional Info
    /// - For a custom saddle item,
    ///   use 'Set Equipment Item'.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetSaddle(Saddle : df_string, ...);
    /// **Set Shulker Bullet Target**<br/>
    /// Causes a shulker bullet to start
    /// targeting the provided entity.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Target UUID
    /// - OR
    /// - `df_text` (Styled Text):
    ///   Target name
    /// - OR
    /// - None:
    ///   No Target
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetBulletTarget(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Set Text Display Line Width**<br/>
    /// Sets the maximum line width
    /// of a text display.
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Line width
    ///   - Default = 200
    ///
    /// ## Works With
    /// - `Text Display`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__TDisplayLineWidth(...);
    /// **Set Dragon Phase**<br/>
    /// Sets the behavior phase
    /// of an Ender Dragon.
    ///
    /// ## Tags
    /// - Phase:
    ///   - `Flying` (Default)
    ///   - `Hovering`
    ///   - `Breath attack`
    ///   - `Dying`
    ///
    /// ## Works With
    /// - `Ender Dragon`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetDragonPhase(Phase : df_string, ...);
    /// **Set Llama Color**<br/>
    /// Sets a llama's fur color.
    ///
    /// ## Tags
    /// - Llama Color:
    ///   - `Brown` (Default)
    ///   - `Creamy`
    ///   - `White`
    ///   - `Gray`
    ///
    /// ## Works With
    /// - `Llama`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetLlamaColor(LlamaSPECIALSpace_Color : df_string, ...);
    /// **Set Villager Biome**<br/>
    /// Sets the biome type of a
    /// villager. This affects their
    /// appearance only.
    ///
    /// ## Tags
    /// - Biome:
    ///   - `Desert` (Default)
    ///   - `Jungle`
    ///   - `Plains`
    ///   - `Savanna`
    ///   - `Snow`
    ///   - `Swamp`
    ///   - `Taiga`
    ///
    /// ## Works With
    /// - `Villager`
    /// - `Zombie villager`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetVillagerBiome(Biome : df_string, ...);
    /// **Set Creeper Fuse**<br/>
    /// Sets the starting amount
    /// of ticks it takes for a
    /// creeper to explode.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Fuse ticks
    ///
    /// ## Works With
    /// - `Creeper`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetCreeperFuse(...);
    /// **Set to Baby/Adult**<br/>
    /// Sets whether an entity
    /// is a baby (permanently).
    ///
    /// ## Tags
    /// - Baby:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Any Breedable`
    /// - `Any Zombie`
    /// - `Piglin`
    /// - `Armor Stand`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetBaby(Baby : df_string, ...);
    /// **Set Mooshroom Type**<br/>
    /// Sets a mooshroom's skin
    /// type.
    ///
    /// ## Tags
    /// - Mooshroom Variant:
    ///   - `Red` (Default)
    ///   - `Brown`
    ///
    /// ## Works With
    /// - `Mooshroom`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__MooshroomType(MooshroomSPECIALSpace_Variant : df_string, ...);
    /// **Set Invisible**<br/>
    /// Sets whether an entity
    /// is invisible.
    ///
    /// ## Tags
    /// - Invisible:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Any Mob`
    /// - `Item Frame`
    /// - `Armor Stand`
    /// - `Interaction Entity`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetInvisible(Invisible : df_string, ...);
    /// **Eat Grass**<br/>
    /// Causes a sheep to
    /// eat grass.
    ///
    /// ## Works With
    /// - `Sheep`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SheepEat(...);
    /// **Set Cat Resting**<br/>
    /// Sets whether a cat appears
    /// to be lying down.
    ///
    /// ## Tags
    /// - Resting:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Cat`
    ///
    /// ## Additional Info
    /// - Does not affect movement.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetCatResting(Resting : df_string, ...);
    /// **Give Potion Effect**<br/>
    /// Gives one or more potion
    /// effects to an entity.
    ///
    /// ## Tags
    /// - Overwrite Effect:
    ///   - `True` (Default)
    ///   - `False`
    /// - Effect Particles:
    ///   - `Regular` (Default)
    ///   - `Ambient`
    ///   - `None`
    ///
    /// ## Arguments
    /// - `df_potion` `[]` (Potion):
    ///   Effect(s)
    ///   to give
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__GivePotion(OverwriteSPECIALSpace_Effect : df_string, EffectSPECIALSpace_Particles : df_string, ...);
    /// **Set Goat Horns**<br/>
    /// Sets which goat horns
    /// are shown or hidden.
    ///
    /// ## Tags
    /// - Left Horn:
    ///   - `Show`
    ///   - `Hide`
    ///   - `No Change` (Default)
    /// - Right Horn:
    ///   - `Show`
    ///   - `Hide`
    ///   - `No Change` (Default)
    ///
    /// ## Works With
    /// - `Goat`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetGoatHorns(LeftSPECIALSpace_Horn : df_string, RightSPECIALSpace_Horn : df_string, ...);
    /// **Set Glowing**<br/>
    /// Sets whether this entity has
    /// a glowing outline that can
    /// be seen through blocks.
    ///
    /// ## Tags
    /// - Glowing:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetGlowing(Glowing : df_string, ...);
    /// **Set Panda On Back**<br/>
    /// Sets whether a panda is
    /// laying on its back or not.
    ///
    /// ## Tags
    /// - On Its Back:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Panda`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetPandaOnBack(OnSPECIALSpace_ItsSPECIALSpace_Back : df_string, ...);
    /// **Set Item Display Model Type**<br/>
    /// Sets the model type
    /// of an item display.
    ///
    /// ## Tags
    /// - Model Type:
    ///   - `None` (Default)
    ///   - `First Person Left Hand`
    ///   - `First Person Right Hand`
    ///   - `Third Person Left Hand`
    ///   - `Third Person Right Hand`
    ///   - `Head`
    ///   - `GUI`
    ///   - `Ground`
    ///   - `Fixed`
    ///
    /// ## Works With
    /// - `Item Display`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__IDisplayModelType(ModelSPECIALSpace_Type : df_string, ...);
    /// **Set Current Health**<br/>
    /// Sets an entity's current
    /// health.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Current health
    ///   - ❤ = 2 Health
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetHealth(...);
    /// **Disguise as Block**<br/>
    /// Disguises an entity as a block.
    ///
    /// ## Arguments
    /// - `df_item` (Block):
    ///   Block to disguise as
    /// - `df_text` `?` (Styled Text):
    ///   Display name
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__BlockDisguise(...);
    /// **Set Collidable**<br/>
    /// Sets whether a mob is able
    /// to collide with other entities.
    ///
    /// ## Tags
    /// - Collision:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    /// ## Additional Info
    /// - To disable collisions between
    ///   mobs and players, both must
    ///   have their collisions disabled.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetCollidable(Collision : df_string, ...);
    /// **Set Armor Stand Pose**<br/>
    /// Sets the rotation of an armor
    /// stand part.
    ///
    /// ## Tags
    /// - Armor Stand Part:
    ///   - `Head` (Default)
    ///   - `Body`
    ///   - `Left Arm`
    ///   - `Right Arm`
    ///   - `Left Leg`
    ///   - `Right Leg`
    ///
    /// ## Arguments
    /// - `df_vector` (Vector):
    ///   Direction
    /// - OR
    /// - `df_number` `?` (Number):
    ///   X Rotation (0-360)
    ///   - Default = 0
    /// - `df_number` `?` (Number):
    ///   Y Rotation (0-360)
    ///   - Default = 0
    /// - `df_number` `?` (Number):
    ///   Z Rotation (0-360)
    ///   - Default = 0
    ///
    /// ## Works With
    /// - `Armor Stand`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__ArmorStandPose(ArmorSPECIALSpace_StandSPECIALSpace_Part : df_string, ...);
    /// **Launch Forward**<br/>
    /// Launches an entity forward
    /// or backward.
    ///
    /// ## Tags
    /// - Add to Current Velocity:
    ///   - `True` (Default)
    ///   - `False`
    /// - Launch Axis:
    ///   - `Pitch and Yaw` (Default)
    ///   - `Yaw Only`
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Launch power
    ///
    /// ## Additional Info
    /// - A negative launch power will
    ///   launch the entity backward.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__LaunchFwd(AddSPECIALSpace_toSPECIALSpace_CurrentSPECIALSpace_Velocity : df_string, LaunchSPECIALSpace_Axis : df_string, ...);
    /// **Set Fall Distance**<br/>
    /// Sets an entity's fall distance,
    /// affecting fall damage upon
    /// landing.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Fall distance (blocks)
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetFallDistance(...);
    /// **Move to Location**<br/>
    /// Instructs a mob's AI to always
    /// pathfind to a certain location
    /// at a certain speed.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Target location
    /// - `df_number` (Number):
    ///   Walk speed
    /// - OR
    /// - None:
    ///   Stop moving
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__MoveToLoc(...);
    /// **Set Text Display Text Opacity**<br/>
    /// Sets the text opacity
    /// of a text display.
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Text opacity
    ///   - Default = 100
    ///
    /// ## Works With
    /// - `Text Display`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__TDisplayOpacity(...);
    /// **Set Item Display Item**<br/>
    /// Sets the displayed item
    /// of an item display.
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   Displayed item
    ///
    /// ## Works With
    /// - `Item Display`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__IDisplayItem(...);
    /// **Send Mob Attack Animation**<br/>
    /// Makes a mob perform
    /// an attack animation.
    ///
    /// ## Tags
    /// - Animation Arm:
    ///   - `Swing main arm` (Default)
    ///   - `Swing off arm`
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__AttackAnimation(AnimationSPECIALSpace_Arm : df_string, ...);
    /// **Set Snow Golem Pumpkin**<br/>
    /// Sets whether a snow golem
    /// is wearing a pumpkin.
    ///
    /// ## Tags
    /// - Pumpkin:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    /// ## Works With
    /// - `Snow Golem`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SnowmanPumpkin(Pumpkin : df_string, ...);
    /// **Set Entity Custom Tag**<br/>
    /// Sets the value of or creates
    /// a custom tag value.
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Tag name
    /// - 
    /// - `df_number` (Number):
    ///   Tag value
    /// - OR
    /// - `df_string` (String):
    ///   Tag value
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetCustomTag(...);
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__Gravity(...);
    /// **Set Display View Range**<br/>
    /// Sets the view range of a
    /// display entity.
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   View range in blocks
    ///   - Default = 64
    ///
    /// ## Works With
    /// - `Any Display Entity`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__DisplayViewRange(...);
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__NoAI(...);
    /// **Disguise as Player**<br/>
    /// Disguises an entity as a player.
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   Player name to disguise as
    /// - `df_item` `?` (Item):
    ///   Display skin
    ///
    /// ## Additional Info
    /// - To get a skin use /item head \<player name\>
    ///   OR /item mshead
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__PlayerDisguise(...);
    /// **Set Entity Item**<br/>
    /// Sets the item of
    /// an item entity.
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   New item
    ///
    /// ## Works With
    /// - `Dropped Item`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetItem(...);
    /// **Explode**<br/>
    /// Causes an entity
    /// to explode.
    ///
    /// ## Works With
    /// - `Creeper`
    /// - `Primed TNT`
    /// - `Firework`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__Explode(...);
    /// **Set Warden Digging**<br/>
    /// Makes a warden emerge
    /// or dig into the ground.
    ///
    /// ## Tags
    /// - Digging Type:
    ///   - `Emerge` (Default)
    ///   - `Dig Down`
    ///
    /// ## Works With
    /// - `Warden`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetDigging(DiggingSPECIALSpace_Type : df_string, ...);
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__MoveTo(...);
    /// **Set Arrow Hit Sound**<br/>
    /// Sets the sound an arrow
    /// plays whenever it lands.
    ///
    /// ## Arguments
    /// - `df_sound` (Sound):
    ///   Sound to play
    ///
    /// ## Works With
    /// - `Arrows, Tridents`
    ///
    /// ## Additional Info
    /// - The pitch and volume of
    ///   the sound are ignored.
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetArrowHitSound(...);
    /// **Set Vex Charging**<br/>
    /// Sets whether a vex is
    /// charging or not.
    ///
    /// ## Tags
    /// - Charging:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Vex`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetVexCharging(Charging : df_string, ...);
    /// **Set Villager Experience**<br/>
    /// Sets a villager's experience
    /// points, which affects their level.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Experience
    ///
    /// ## Works With
    /// - `Villager`
    ///
    /// ## Additional Info
    /// - Level requirements:
    ///   ■ 0: Novice
    ///   ■ 10: Apprentice
    ///   ■ 70: Journeyman
    ///   ■ 150: Expert
    ///   ■ 250: Master
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetVillagerExp(...);
    /// **Ignite Creeper**<br/>
    /// Ignites a creeper, causing
    /// it to explode after a fuse
    /// period.
    ///
    /// ## Works With
    /// - `Creeper`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__IgniteCreeper(...);
    /// **Set Celebrating**<br/>
    /// Causes a mob to start
    /// or stop celebrating.
    ///
    /// ## Tags
    /// - Celebrate:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Works With
    /// - `Any Raider`
    /// - `Piglin`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetCelebrating(Celebrate : df_string, ...);
    /// **Set Text Display Text**<br/>
    /// Sets the displayed text
    /// of a text display.
    ///
    /// ## Tags
    /// - Text Value Merging:
    ///   - `Add spaces`
    ///   - `No spaces` (Default)
    /// - Inherit Styles:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_text` `[]` (Styled Text):
    ///   Displayed text
    ///
    /// ## Works With
    /// - `Text Display`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__TDisplayText(TextSPECIALSpace_ValueSPECIALSpace_Merging : df_string, InheritSPECIALSpace_Styles : df_string, ...);
    /// **Set Horse Jump Strength**<br/>
    /// Sets a horse's jump strength.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Strength
    ///   - 0.0 (can't jump) to 2.0
    ///
    /// ## Works With
    /// - `Horse`
    /// - `Donkey`
    /// - `Mule`
    /// - `Llama`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetHorseJump(...);
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__Unsilence(...);
    /// **Set Health Attribute**<br/>
    /// Sets one of the entity's health-related
    /// attributes such as max health
    /// and armor defense points.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Maximum health` (Default)
    ///   - `Maximum absorption health`
    ///   - `Armor`
    ///   - `Armor toughness`
    /// - Value Type:
    ///   - `Direct` (Default)
    ///   - `Percentage (Base)`
    ///   - `Percentage (Relative)`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Value
    ///
    /// ## Works With
    /// - `Any Mob`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__HealthAttribute(Attribute : df_string, ValueSPECIALSpace_Type : df_string, ...);
    /// **Ram Target**<br/>
    /// Makes a goat ram the
    /// specified mob or player.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Target UUID
    /// - OR
    /// - `df_text` (Styled Text):
    ///   Target name
    ///
    /// ## Works With
    /// - `Goat`
    ///
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__Ram(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Set Fox Type**<br/>
    /// Sets a fox's fur type.
    ///
    /// ## Tags
    /// - Fox Type:
    ///   - `Red` (Default)
    ///   - `Snow`
    ///
    /// ## Works With
    /// - `Fox`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__entitySPECIALSpace_action__SetFoxType(FoxSPECIALSpace_Type : df_string, ...);
    /// **Is Vehicle**<br/>
    /// Checks if an entity
    /// is a boat or minecart.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__IsVehicle(...);
    /// **Is Grounded**<br/>
    /// Checks if an entity is
    /// supported by a block.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__IsGrounded(...);
    /// **Is Type**<br/>
    /// Checks if an entity is the
    /// given type.
    ///
    /// ## Arguments
    /// - `df_item` `[]` (EntityType):
    ///   Spawn egg,
    ///   projectile, or vehicle
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__IsType(...);
    /// **Is Projectile**<br/>
    /// Checks if an entity
    /// is a projectile.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__IsProj(...);
    /// **Is Mob**<br/>
    /// Checks if an entity
    /// is a mob.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__IsMob(...);
    /// **Has Custom Tag**<br/>
    /// Checks if an entity has a
    /// given custom tag, and (if
    /// provided) whether the tag
    /// matches the given value.
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Tag name
    /// - 
    /// - `df_number` `?` (Number):
    ///   Tag value
    /// - OR
    /// - `df_string` `?` (String):
    ///   Tag value
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__HasCustomTag(...);
    /// **Is Sheared**<br/>
    /// Checks if a sheep is
    /// sheared.
    ///
    /// ## Works With
    /// - `Sheep`
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__IsSheared(...);
    /// **Is Item**<br/>
    /// Checks if an entity
    /// is an item.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__IsItem(...);
    /// **Is Riding Entity**<br/>
    /// Checks if an entity is riding
    /// another entity.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_item` `[]` (EntityType):
    ///   Spawn egg,
    ///   projectile, or vehicle
    /// - OR
    /// - `df_string` `[]` (String):
    ///   Entity UUID
    /// - OR
    /// - `df_text` `[]` (Styled Text):
    ///   Entity name
    /// - OR
    /// - None:
    ///   Checks if riding any entity
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__SPECIALSpace_IsRidingSPECIALSpace_(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Exists**<br/>
    /// Checks if an entity still
    /// exists in the world.
    ///
    /// ## Additional Info
    /// - Entities which have been removed
    ///   still remain in selections.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__Exists(...);
    /// **Is Hitbox Near Location**<br/>
    /// Checks if an entity's hitbox is
    /// within a range of a location.
    ///
    /// ## Tags
    /// - Shape:
    ///   - `Sphere` (Default)
    ///   - `Circle`
    ///   - `Cube`
    ///   - `Square`
    ///
    /// ## Arguments
    /// - `df_location` `[]` (Location):
    ///   Center location
    /// - `df_number` `?` (Number):
    ///   Range
    ///   - Default = 5 blocks
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__IsHitboxNear(Shape : df_string, ...);
    /// **Is Near Location**<br/>
    /// Checks if an entity is within a
    /// range of a location.
    ///
    /// ## Tags
    /// - Shape:
    ///   - `Sphere` (Default)
    ///   - `Circle`
    ///   - `Cube`
    ///   - `Square`
    ///
    /// ## Arguments
    /// - `df_location` `[]` (Location):
    ///   Center location
    /// - `df_number` `?` (Number):
    ///   Range
    ///   - Default = 5 blocks
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__IsNear(Shape : df_string, ...);
    /// **Has Potion Effect**<br/>
    /// Checks if an entity has a
    /// potion effect of a certain
    /// type active.
    ///
    /// ## Tags
    /// - Check Properties:
    ///   - `None` (Default)
    ///   - `Amplifier`
    ///   - `Duration`
    ///   - `Amplifier and duration`
    /// - Check Mode:
    ///   - `Has any effect` (Default)
    ///   - `Has all effects`
    ///
    /// ## Arguments
    /// - `df_potion` `[]` (Potion):
    ///   Effect(s)
    ///   to check for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__HasPotion(CheckSPECIALSpace_Properties : df_string, CheckSPECIALSpace_Mode : df_string, ...);
    ///
    /// ## Tags
    /// - Compare Text To:
    ///   - `Entity type` (Default)
    ///   - `Name or UUID`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__IsRiding(CompareSPECIALSpace_TextSPECIALSpace_To : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__StandingOn(...);
    /// **Name Equals**<br/>
    /// Checks if an entity's name or
    /// custom name is equal to the
    /// given text.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   UUID to check for
    /// - OR
    /// - `df_text` `[]` (Styled Text):
    ///   Name to check for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__NameEquals(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Is Standing on Block**<br/>
    /// Checks if an entity is standing on
    /// the given block or location.
    ///
    /// ## Arguments
    /// - `df_item` `[]` (Block):
    ///   Block to check for
    /// - OR
    /// - `df_location` `[]` (Location):
    ///   Location to check for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_entity__SPECIALSpace_StandingOnSPECIALSpace_(...);
    /// **Set Hotbar Items**<br/>
    /// Sets items in a player's
    /// hotbar.
    ///
    /// ## Arguments
    /// - `df_item` `[]` (Item):
    ///   Item(s) to set
    ///   - Slots 1-9
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetHotbar(...);
    /// **Set Reduced Debug Info Enabled**<br/>
    /// When enabled, a player won't be
    /// able to see their coordinates,
    /// block info, or other info.
    ///
    /// ## Tags
    /// - Reduced Debug Info Enabled:
    ///   - `True` (Default)
    ///   - `False`
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetReducedDebug(ReducedSPECIALSpace_DebugSPECIALSpace_InfoSPECIALSpace_Enabled : df_string, ...);
    /// **Close Inventory**<br/>
    /// Closes a player's inventory.
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__CloseInv(...);
    /// **Give Items**<br/>
    /// Gives a player all of the
    /// items in the chest.
    ///
    /// ## Arguments
    /// - `df_item` `[]` (Item):
    ///   Item(s) to give
    /// - `df_number` `?` (Number):
    ///   Amount to give
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__GiveItems(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__NoKeepInv(...);
    /// **Set Allow Hand Crafting**<br/>
    /// Sets if a player is
    /// allowed to interact with
    /// their hand-crafting menu.
    ///
    /// ## Tags
    /// - Allow Hand Crafting:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetHandCrafting(AllowSPECIALSpace_HandSPECIALSpace_Crafting : df_string, ...);
    ///
    /// ## Tags
    /// - Bar Slot:
    ///   - `Slot 1` (Default)
    ///   - `Slot 2`
    ///   - `Slot 3`
    ///   - `Slot 4`
    ///   - `Slot 5`
    ///   - `Slot 6`
    ///   - `Slot 7`
    ///   - `Slot 8`
    ///   - `Slot 9`
    /// - Bar Style:
    ///   - `Solid` (Default)
    ///   - `6 segments`
    ///   - `10 segments`
    ///   - `12 segments`
    ///   - `20 segments`
    /// - Sky Effect:
    ///   - `None` (Default)
    ///   - `Create fog`
    ///   - `Darken sky`
    ///   - `Both`
    /// - Bar Color:
    ///   - `Red`
    ///   - `Purple` (Default)
    ///   - `Pink`
    ///   - `Blue`
    ///   - `Green`
    ///   - `Yellow`
    ///   - `White`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__BossBar(BarSPECIALSpace_Slot : df_string, BarSPECIALSpace_Style : df_string, SkySPECIALSpace_Effect : df_string, BarSPECIALSpace_Color : df_string, ...);
    /// **Display Particle Sphere**<br/>
    /// Displays a sphere of particles
    /// at a location to a player.
    ///
    /// ## Arguments
    /// - `df_particle` (Particle):
    ///   Effect
    /// - `df_location` (Location):
    ///   Center location
    /// - `df_number` `?` (Number):
    ///   Diameter
    ///   - Default = 2 blocks
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ParticleSphere(...);
    /// **Set Velocity**<br/>
    /// Sets a player's movement
    /// velocity.
    ///
    /// ## Tags
    /// - Add to Current Velocity:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_vector` (Vector):
    ///   New velocity
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetVelocity(AddSPECIALSpace_toSPECIALSpace_CurrentSPECIALSpace_Velocity : df_string, ...);
    /// **Display Particle Effect**<br/>
    /// Displays a particle effect
    /// to a player.
    ///
    /// ## Arguments
    /// - `df_particle` `[]` (Particle):
    ///   Effect
    /// - `df_location` (Location):
    ///   Effect location
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__Particle(...);
    /// **Add Inventory Menu Row**<br/>
    /// Adds a row to the bottom of
    /// a player's current inventory
    /// menu.
    ///
    /// ## Tags
    /// - New Row Position:
    ///   - `Top row`
    ///   - `Bottom row` (Default)
    ///
    /// ## Arguments
    /// - `df_item` `[]?` (Item):
    ///   Items to display
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__AddInvRow(NewSPECIALSpace_RowSPECIALSpace_Position : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__NoNatRegen(...);
    /// **Display Lightning Bolt**<br/>
    /// Displays a lightning strike
    /// effect to a player.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Strike location
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisplayLightning(...);
    /// **Damage**<br/>
    /// Damages a player.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Damage to inflict
    ///   - ❤ = 2 Health
    /// - 
    /// - `df_string` `?` (String):
    ///   UUID of damager entity
    /// - OR
    /// - `df_text` `?` (Styled Text):
    ///   Name of damager entity
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__Damage(IgnoreSPECIALSpace_Formatting : df_string, ...);
    ///
    /// ## Tags
    /// - Animation Type:
    ///   - `Hurt animation` (Default)
    ///   - `Wake up (fade effect)`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SendAnimation(AnimationSPECIALSpace_Type : df_string, ...);
    /// **Set XP Progress**<br/>
    /// Sets the XP progress bar
    /// to a certain percentage.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Progress % (0-100)
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetXPProg(...);
    /// **Set Inventory Items**<br/>
    /// Sets items in a player's
    /// upper inventory.
    ///
    /// ## Arguments
    /// - `df_item` `[]` (Item):
    ///   Item(s) to set
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetInventory(...);
    /// **Teleport Sequence**<br/>
    /// Teleports a player to multiple
    /// locations, with a delay between
    /// each teleport.
    ///
    /// ## Arguments
    /// - `df_location` `[]` (Location):
    ///   Locations to
    ///   teleport to
    /// - `df_number` `?` (Number):
    ///   Teleport delay (ticks,
    ///   default = 60)
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__TpSequence(...);
    /// **Heal**<br/>
    /// Restores a player's health.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Amount to heal
    ///   - ❤ = 2 Health
    /// - OR
    /// - None:
    ///   Heals to full health
    ///
    /// ## Additional Info
    /// - Triggers the Heal Event.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__Heal(...);
    /// **Set Spawn Point**<br/>
    /// Sets the location a player will
    /// spawn when they die and respawn.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   The new spawn location
    /// - OR
    /// - None:
    ///   Plot spawn
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetSpawnPoint(...);
    /// **Set Inventory Kept**<br/>
    /// Sets whether a player's inventory
    /// is kept after death.
    ///
    /// ## Tags
    /// - Inventory Kept:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetInventoryKept(InventorySPECIALSpace_Kept : df_string, ...);
    /// **Launch Up**<br/>
    /// Launches a player up or down.
    ///
    /// ## Tags
    /// - Add to Current Velocity:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Launch power
    ///
    /// ## Additional Info
    /// - A negative launch power will
    ///   launch the player down.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__LaunchUp(AddSPECIALSpace_toSPECIALSpace_CurrentSPECIALSpace_Velocity : df_string, ...);
    ///
    /// ## Tags
    /// - Ignore Blocks:
    ///   - `True`
    ///   - `False` (Default)
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__GetTargetEntity(IgnoreSPECIALSpace_Blocks : df_string, ...);
    /// **Force Flight Mode**<br/>
    /// Forces a player to start
    /// or stop flying.
    ///
    /// ## Tags
    /// - Flight Mode:
    ///   - `Start Flight` (Default)
    ///   - `Stop Flight`
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ForceFlight(FlightSPECIALSpace_Mode : df_string, ...);
    /// **Load Saved Inventory**<br/>
    /// Loads a player's inventory.
    ///
    /// ## Tags
    /// - Code Flow:
    ///   - `Synchronous` (Default)
    ///   - `Asynchronous`
    ///
    /// ## Additional Info
    /// - If no saved inventory exists,
    ///   the player's inventory will be
    ///   cleared.
    /// - Inventories loaded with 'Load
    ///   Saved Inventory' take several
    ///   ticks to load. Use 'Control:
    ///   Wait' if you need to check the
    ///   inventory's contents after it is
    ///   loaded.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__LoadInv(CodeSPECIALSpace_Flow : df_string, ...);
    /// **Set Chat Style**<br/>
    /// Sets a player's chat color or
    /// decoration.
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   New chat style
    /// - OR
    /// - None:
    ///   Resets chat style
    ///
    /// ## Examples
    /// - `"\<#D4D4FF\>"`
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ChatStyle(...);
    /// **Kick**<br/>
    /// Kicks a player from
    /// the plot.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__Kick(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ProjColl(...);
    /// **Set Miscellaneous Attribute**<br/>
    /// Sets one of the player's miscellaneous
    /// attributes such as scale and
    /// burning time.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Scale` (Default)
    ///   - `Luck`
    ///   - `Oxygen bonus`
    ///   - `Burning time`
    ///   - `Camera distance`
    /// - Value Type:
    ///   - `Direct` (Default)
    ///   - `Percentage (Base)`
    ///   - `Percentage (Relative)`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Value
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__MiscAttribute(Attribute : df_string, ValueSPECIALSpace_Type : df_string, ...);
    /// **Spectate Target**<br/>
    /// Makes a player spectate
    /// another player or entity.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Target UUID
    /// - OR
    /// - `df_text` (Styled Text):
    ///   Target name
    /// - OR
    /// - None:
    ///   Stops spectating
    ///
    /// ## Additional Info
    /// - The player must be in
    ///   spectator mode.
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SpectateTarget(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Send Hurt Animation**<br/>
    /// Makes a player perform
    /// a hurt animation.
    ///
    /// ## Arguments
    /// - `df_location` `?` (Location):
    ///   Damage source
    ///   - Affects the direction of
    ///     the screen tilt effect.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__HurtAnimation(...);
    /// **Set to Survival Mode**<br/>
    /// Sets a player's game
    /// mode to Survival.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SurvivalMode(...);
    /// **Display Bell Ring**<br/>
    /// Displays a bell ring animation
    /// at a location to a player.
    ///
    /// ## Tags
    /// - Ring Direction:
    ///   - `North` (Default)
    ///   - `South`
    ///   - `West`
    ///   - `East`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Block location
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisplayBellRing(RingSPECIALSpace_Direction : df_string, ...);
    /// **Set Status**<br/>
    /// Sets the player's game status,
    /// which is used to display information
    /// about what the player is doing
    /// in the game.
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   Game Status
    ///   - Limited at 50 characters
    /// - OR
    /// - None:
    ///   Clears status
    ///
    /// ## Additional Info
    /// - A player's game status be seen
    ///   when using /locate.
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetStatus(...);
    /// **Set Cursor Item**<br/>
    /// Sets the item on a
    /// player's cursor.
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   Item to set
    /// - OR
    /// - None:
    ///   Clears cursor item
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetCursorItem(...);
    /// **Set Absorption Health**<br/>
    /// Sets a player's absorption
    /// health (golden hearts).
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Absorption health
    ///   - ❤ = 2 Health
    ///
    /// ## Additional Info
    /// - The target does not need to
    ///   have an absorption effect.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetAbsorption(...);
    /// **Set Fire Ticks**<br/>
    /// Sets the remaining time a
    /// player is on fire for.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Ticks
    ///
    /// ## Additional Info
    /// - 0 ticks mean the target is
    ///   not on fire.
    /// - This triggers the Player Combust event.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetFireTicks(...);
    /// **Set Combat Attribute**<br/>
    /// Sets one of the player's combat-related
    /// attributes such as attack damage
    /// and attack speed.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Attack damage` (Default)
    ///   - `Attack speed`
    ///   - `Sweeping damage ratio`
    /// - Value Type:
    ///   - `Direct` (Default)
    ///   - `Percentage (Base)`
    ///   - `Percentage (Relative)`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Value
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__CombatAttribute(Attribute : df_string, ValueSPECIALSpace_Type : df_string, ...);
    ///
    /// ## Tags
    /// - Flight Mode:
    ///   - `Respect Gamemode` (Default)
    ///   - `Keep Original`
    /// - Gamemode:
    ///   - `Survival` (Default)
    ///   - `Creative`
    ///   - `Adventure`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetGamemode(FlightSPECIALSpace_Mode : df_string, Gamemode : df_string, ...);
    /// **Remove Inventory Menu Row**<br/>
    /// Removes the given number of
    /// rows from the bottom of a player's
    /// current inventory menu.
    ///
    /// ## Tags
    /// - Row to Remove:
    ///   - `Top row`
    ///   - `Bottom row` (Default)
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Rows to remove
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__RemoveInvRow(RowSPECIALSpace_toSPECIALSpace_Remove : df_string, ...);
    /// **Send Wake Up Animation**<br/>
    /// Displays the wake up (fade in)
    /// animation to a player.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__WakeUpAnimation(...);
    /// **Disallow Placing/Breaking Blocks**<br/>
    /// Prevents a player from placing
    /// and breaking certain blocks.
    ///
    /// ## Arguments
    /// - `df_item` `[]?` (Block):
    ///   Blocks to disallow
    ///   - If no blocks are given,
    ///     disallows all blocks
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisableBlocks(...);
    /// **Set Scoreboard Objective Name**<br/>
    /// Sets the objective name of the
    /// scoreboard sidebar.
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   Objective name
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetScoreObj(...);
    ///
    /// ## Tags
    /// - Heal Type:
    ///   - `Regular Health` (Default)
    ///   - `Absorption Health`
    ///   - `Combined Health`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__LSPECIALSpace_SetHealth(HealSPECIALSpace_Type : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ParticleEffect(...);
    /// **Clear Inventory**<br/>
    /// Empties a player's inventory.
    ///
    /// ## Tags
    /// - Clear Mode:
    ///   - `Entire inventory` (Default)
    ///   - `Main inventory`
    ///   - `Upper inventory`
    ///   - `Hotbar`
    ///   - `Armor`
    /// - Clear Crafting and Cursor:
    ///   - `True` (Default)
    ///   - `False`
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ClearInv(ClearSPECIALSpace_Mode : df_string, ClearSPECIALSpace_CraftingSPECIALSpace_andSPECIALSpace_Cursor : df_string, ...);
    /// **Set Freeze Ticks**<br/>
    /// Sets how long a player
    /// is frozen for.
    ///
    /// ## Tags
    /// - Ticking Locked:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Ticks
    ///   (0-140)
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetFreezeTicks(TickingSPECIALSpace_Locked : df_string, ...);
    /// **Set Gliding**<br/>
    /// Sets whether a player
    /// is gliding with elytra.
    ///
    /// ## Tags
    /// - Gliding:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Additional Info
    /// - The player must be
    ///   wearing an elytra.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetGliding(Gliding : df_string, ...);
    /// **Set Rotation**<br/>
    /// Changes a player's pitch and
    /// yaw.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Pitch (-90 to 90)
    /// - `df_number` (Number):
    ///   Yaw (-180 to 180)
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetRotation(...);
    /// **Clear Items**<br/>
    /// Removes all of an item from
    /// a player.
    ///
    /// ## Arguments
    /// - `df_item` `[]` (Item):
    ///   Item(s) to clear
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ClearItems(...);
    /// **Open Sign**<br/>
    /// Opens a sign for a player.
    /// Also works with client-side signs.
    ///
    /// ## Tags
    /// - Sign Side:
    ///   - `Front` (Default)
    ///   - `Back`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Sign location
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__OpenSign(SignSPECIALSpace_Side : df_string, ...);
    /// **Set Flying**<br/>
    /// Sets whether a player
    /// is flying.
    ///
    /// ## Tags
    /// - Flying:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetFlying(Flying : df_string, ...);
    /// **Display Block Opened State**<br/>
    /// Displays a container block
    /// at a location as being open
    /// or closed to a player.
    ///
    /// ## Tags
    /// - Container State:
    ///   - `Open` (Default)
    ///   - `Closed`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Block location
    ///
    /// ## Works With
    /// - `Chest`
    /// - `Trapped Chest`
    /// - `Ender Chest`
    /// - `Shulker Box`
    /// - `Barrel`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisplayBlockOpen(ContainerSPECIALSpace_State : df_string, ...);
    ///
    /// ## Tags
    /// - Hand Slot:
    ///   - `Main Hand` (Default)
    ///   - `Off Hand`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetHandItem(HandSPECIALSpace_Slot : df_string, ...);
    /// **Send Advancement**<br/>
    /// Displays a custom advancement
    /// popup to a player.
    ///
    /// ## Tags
    /// - Toast Type:
    ///   - `Advancement` (Default)
    ///   - `Goal`
    ///   - `Challenge`
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   Advancement name
    /// - `df_item` `?` (Item):
    ///   Advancement icon
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SendAdvancement(ToastSPECIALSpace_Type : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ClearChat(...);
    /// **Set Inventory Menu Item**<br/>
    /// Sets the item in a slot
    /// of a player's current
    /// inventory menu.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Slot
    /// - `df_item` `?` (Item):
    ///   Item to set
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetMenuItem(...);
    /// **Launch Toward Location**<br/>
    /// Launches a player toward or away
    /// from a location.
    ///
    /// ## Tags
    /// - Add to Current Velocity:
    ///   - `True` (Default)
    ///   - `False`
    /// - Ignore Distance:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Launch destination
    /// - `df_number` `?` (Number):
    ///   Launch power
    ///
    /// ## Additional Info
    /// - A negative launch power will launch
    ///   the player away from the location.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__LaunchToward(AddSPECIALSpace_toSPECIALSpace_CurrentSPECIALSpace_Velocity : df_string, IgnoreSPECIALSpace_Distance : df_string, ...);
    /// **Set Armor Items**<br/>
    /// Sets a player's armor items.
    /// Place the armor in slots 1-4
    /// of the chest, with 1 being the
    /// helmet and 4 being the boots.
    ///
    /// ## Arguments
    /// - `df_item` `[]` (Item):
    ///   Armor to set
    ///
    /// ## Additional Info
    /// - Any block or item can render
    ///   on the target's head if placed
    ///   in the 'helmet' slot.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetArmor(...);
    /// **Display Gateway Beam**<br/>
    /// Displays a vertical beam on
    /// an end gateway to a player.
    ///
    /// ## Tags
    /// - Animation Type:
    ///   - `Initial beam` (Default)
    ///   - `Periodic beam`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Gateway location
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisplayGateway(AnimationSPECIALSpace_Type : df_string, ...);
    /// **Give Saturation**<br/>
    /// Adds saturation to a player.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Saturation to give
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__GiveSaturation(...);
    /// **Display Equipment Change**<br/>
    /// Displays equipment on an entity
    /// to a player. Equipment goes from
    /// slots 2-7 in order of Helmet,
    /// Chestplate, Leggings, Boots,
    /// Main Hand, Off Hand.
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Entity UUID
    /// - OR
    /// - `df_text` (Styled Text):
    ///   Entity name
    /// - 
    /// - `df_item` `[]` (Item):
    ///   Equipment
    ///
    /// ## Additional Info
    /// - Any block or item can render
    ///   on the entity's head if placed
    ///   in the 'helmet' slot.
    /// - This equipment is client-side.
    ///   The actual equipment is not changed.
    /// - If the equipment slot is updated,
    ///   it will be reset.
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisplayEquipment(...);
    /// **Give Experience**<br/>
    /// Adds experience points or
    /// levels to a player.
    ///
    /// ## Tags
    /// - Give Experience:
    ///   - `Points` (Default)
    ///   - `Levels`
    ///   - `Level Percentage`
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Experience to give
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__GiveExp(GiveSPECIALSpace_Experience : df_string, ...);
    /// **Face Location**<br/>
    /// Rotates a player to look
    /// toward a location without
    /// teleporting them.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Location to face
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__FaceLocation(...);
    /// **Clear Scoreboard**<br/>
    /// Removes all scores from
    /// the scoreboard.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ClearScoreboard(...);
    /// **Show Action Bar Text**<br/>
    /// Displays text directly above
    /// a player's hotbar.
    ///
    /// ## Tags
    /// - Text Value Merging:
    ///   - `Add spaces`
    ///   - `No spaces` (Default)
    /// - Inherit Styles:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_text` `[]` (Styled Text):
    ///   Message to send
    ///
    /// ## Additional Info
    /// - Multiple variables (of any type)
    ///   will be merged into one message.
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    /// - Requires token shop purchase
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ActionBar(TextSPECIALSpace_ValueSPECIALSpace_Merging : df_string, InheritSPECIALSpace_Styles : df_string, ...);
    /// **Set Chat Tag**<br/>
    /// Sets a player's chat tag.
    ///
    /// ## Arguments
    /// - `df_text` `[]` (Styled Text):
    ///   Chat tag
    /// - OR
    /// - None:
    ///   Resets chat tag
    ///
    /// ## Additional Info
    /// - Multiple text items will be
    ///   merged into one chat tag.
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetChatTag(...);
    /// **Shift World Border**<br/>
    /// Changes a player's world border
    /// size if they have one active.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   New radius
    /// - `df_number` `?` (Number):
    ///   Blocks per second
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ShiftWorldBorder(...);
    /// **Display Sign Text**<br/>
    /// Displays text on a sign
    /// to a player.
    ///
    /// ## Tags
    /// - Sign Side:
    ///   - `Front` (Default)
    ///   - `Back`
    /// - Text Color:
    ///   - `White`
    ///   - `Orange`
    ///   - `Magenta`
    ///   - `Light blue`
    ///   - `Yellow`
    ///   - `Lime`
    ///   - `Pink`
    ///   - `Gray`
    ///   - `Light gray`
    ///   - `Cyan`
    ///   - `Purple`
    ///   - `Blue`
    ///   - `Brown`
    ///   - `Green`
    ///   - `Red`
    ///   - `Black` (Default)
    /// - Glowing:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Sign location
    /// - `df_text` `[]?` (Styled Text):
    ///   Text line(s)
    ///
    /// ## Additional Info
    /// - Displaying a sign text will
    ///   wipe any existing text on the
    ///   sign for the player.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisplaySignText(SignSPECIALSpace_Side : df_string, TextSPECIALSpace_Color : df_string, Glowing : df_string, ...);
    ///
    /// ## Tags
    /// - Speed Type:
    ///   - `Ground speed` (Default)
    ///   - `Flight speed`
    ///   - `Both`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetSpeed(SpeedSPECIALSpace_Type : df_string, ...);
    /// **Expand Inventory Menu**<br/>
    /// Adds 3 more rows to a player's
    /// current inventory menu using the
    /// contents of the chest.
    ///
    /// ## Arguments
    /// - `df_item` `[]?` (Item):
    ///   Items to display
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ExpandInv(...);
    /// **Launch Projectile**<br/>
    /// Launches a projectile from
    /// a player.
    ///
    /// ## Arguments
    /// - `df_item` (Projectile):
    ///   Projectile to
    ///   launch
    /// - `df_location` `?` (Location):
    ///   Launch point
    /// - `df_text` `?` (Styled Text):
    ///   Projectile name
    /// - `df_number` `?` (Number):
    ///   Speed
    /// - `df_number` `?` (Number):
    ///   Inaccuracy
    ///   - Controls how much random
    ///     motion is applied on launch
    ///   - Default = 1
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__LaunchProj(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__NoProjColl(...);
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ShowDisguise(...);
    /// **Display Animated Particle Cuboid**<br/>
    /// Displays an animated particle
    /// cuboid to a player.
    ///
    /// ## Tags
    /// - Fill Type:
    ///   - `Wireframe` (Default)
    ///   - `Hollow`
    ///   - `Solid`
    ///
    /// ## Arguments
    /// - `df_particle` (Particle):
    ///   Effect
    /// - `df_location` (Location):
    ///   Corner 1
    /// - `df_location` (Location):
    ///   Corner 2
    /// - `df_number` `?` (Number):
    ///   Effect spacing
    ///   - Default = 0.5 blocks
    /// - `df_number` `?` (Number):
    ///   Animation duration
    ///   - Default = 40 ticks
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ParticleCuboidA(FillSPECIALSpace_Type : df_string, ...);
    /// **Play Sound**<br/>
    /// Plays a sound for a player.
    ///
    /// ## Tags
    /// - Sound Source:
    ///   - `Master` (Default)
    ///   - `Music`
    ///   - `Jukebox/Note Blocks`
    ///   - `Weather`
    ///   - `Blocks`
    ///   - `Hostile Creatures`
    ///   - `Friendly Creatures`
    ///   - `Players`
    ///   - `Ambient/Environment`
    ///   - `Voice/Speech`
    ///   - `UI`
    ///
    /// ## Arguments
    /// - `df_sound` `[]` (Sound):
    ///   Sound to play
    /// - `df_location` `?` (Location):
    ///   Playback location
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__PlaySound(SoundSPECIALSpace_Source : df_string, ...);
    /// **Set Compass Target**<br/>
    /// Sets the location compasses
    /// point to for a player.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   New Target
    ///
    /// ## Restrictions
    /// - Requires token shop purchase
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetCompass(...);
    /// **Randomized Teleport**<br/>
    /// Teleports a player to a random
    /// location in the chest.
    ///
    /// ## Tags
    /// - Keep Current Rotation:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_location` `[]` (Location):
    ///   Locations to
    ///   choose from
    ///
    #[deprecated = "Use 'Set Variable: Set to Random Value' instead. This will likely be removed in 5.4."]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__RngTeleport(KeepSPECIALSpace_CurrentSPECIALSpace_Rotation : df_string, ...);
    /// **Disguise as Mob**<br/>
    /// Disguises a player as a mob.
    ///
    /// ## Arguments
    /// - `df_item` (SpawnEgg):
    ///   Mob to disguise as
    /// - `df_text` `?` (Styled Text):
    ///   Display name
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__MobDisguise(...);
    /// **Allow Placing/Breaking Blocks**<br/>
    /// Allows a player to place
    /// and break certain blocks.
    ///
    /// ## Arguments
    /// - `df_item` `[]?` (Block):
    ///   Blocks to allow
    ///   - If no blocks are given,
    ///     allows all blocks
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__EnableBlocks(...);
    /// **Open Container Inventory**<br/>
    /// Opens a container's inventory.
    /// Also works with crafting tables.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Container location
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__OpenBlockInv(...);
    /// **Lock Disguise Rotation**<br/>
    /// Locks a disguise's pitch or yaw values.
    ///
    /// ## Tags
    /// - Pitch:
    ///   - `Lock`
    ///   - `Unlock`
    ///   - `No Change` (Default)
    /// - Yaw:
    ///   - `Lock`
    ///   - `Unlock`
    ///   - `No Change` (Default)
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Pitch to lock to
    ///   - Default = 0
    /// - `df_number` `?` (Number):
    ///   Yaw to lock to
    ///   - Default = 0
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__LockDisgRotation(Pitch : df_string, Yaw : df_string, ...);
    /// **Display Animated Particle Circle**<br/>
    /// Displays an animated circle
    /// of particles to a player.
    ///
    /// ## Arguments
    /// - `df_particle` (Particle):
    ///   Effect
    /// - `df_location` (Location):
    ///   Center location
    /// - `df_number` `?` (Number):
    ///   Diameter
    ///   - Default = 2 blocks
    /// - `df_number` `?` (Number):
    ///   Animation duration
    ///   - Default = 40 ticks
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ParticleCircleA(...);
    ///
    /// ## Tags
    /// - Boss Bar Slot:
    ///   - `All boss bars` (Default)
    ///   - `1`
    ///   - `2`
    ///   - `3`
    ///   - `4`
    ///   - `5`
    ///   - `6`
    ///   - `7`
    ///   - `8`
    ///   - `9`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__RemoveBossBar(BossSPECIALSpace_BarSPECIALSpace_Slot : df_string, ...);
    /// **Set Equipment Item**<br/>
    /// Sets the item in one of the
    /// equipment slots (armor and
    /// held items) of a player.
    ///
    /// ## Tags
    /// - Equipment Slot:
    ///   - `Main hand` (Default)
    ///   - `Off hand`
    ///   - `Head`
    ///   - `Chest`
    ///   - `Legs`
    ///   - `Feet`
    ///
    /// ## Arguments
    /// - `df_item` `?` (Item):
    ///   Item to set
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetEquipment(EquipmentSPECIALSpace_Slot : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__GiveRngItem(...);
    /// **Set Death Drops Enabled**<br/>
    /// Sets whether a player drops
    /// their items when dead.
    ///
    /// ## Tags
    /// - Spawn Death Drops:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetDropsEnabled(SpawnSPECIALSpace_DeathSPECIALSpace_Drops : df_string, ...);
    /// **Send to Plot**<br/>
    /// Sends a player to another plot.
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Plot handle or ID
    ///
    /// ## Additional Info
    /// - The player will be prompted to
    ///   be sent to the specified plot.
    ///   The prompt may be denied by the player.
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SendToPlot(...);
    /// **Remove Potion Effect**<br/>
    /// Removes one or more potion
    /// effects from a player.
    ///
    /// ## Arguments
    /// - `df_potion` `[]` (Potion):
    ///   Effect(s)
    ///   to remove
    ///
    /// ## Additional Info
    /// - Only the potion's type needs
    ///   to match; amplifier and duration
    ///   are disregarded.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__RemovePotion(...);
    /// **Display Block Fracture**<br/>
    /// Displays a block fracture
    /// effect at a location to a
    /// player.
    ///
    /// ## Tags
    /// - Overwrite Previous Fracture:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_location` `[]` (Location):
    ///   Block(s) to
    ///   fracture
    /// - `df_number` `?` (Number):
    ///   Fracture level
    ///   - 0-10 (default = 0)
    ///
    /// ## Additional Info
    /// - Disappears after 20s.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisplayFracture(OverwriteSPECIALSpace_PreviousSPECIALSpace_Fracture : df_string, ...);
    /// **Set Entity Hidden**<br/>
    /// Sets if an entity is hidden
    /// to a target.
    ///
    /// ## Tags
    /// - Hidden:
    ///   - `Enable` (Default)
    ///   - `Disable`
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   Entity UUIDs
    /// - OR
    /// - `df_text` `[]` (Styled Text):
    ///   Entity names
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetEntityHidden(Hidden : df_string, IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Set Sidebar Visible**<br/>
    /// Sets whether the scoreboard
    /// sidebar is visible to a player.
    ///
    /// ## Tags
    /// - Sidebar:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetSidebar(Sidebar : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__AllowDrops(...);
    /// **Display Block Highlighter**<br/>
    /// Highlights a specific block for a player.
    /// The highlight is overlaid on a block,
    /// showing text on top if provided.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Block location
    /// - `df_string` `?` (String):
    ///   Color hexadecimal
    ///   - Example: "#FF0000" (red)
    /// - `df_string` `?` (String):
    ///   Name
    /// - `df_number` `?` (Number):
    ///   Opacity in percentage
    /// - `df_number` `?` (Number):
    ///   Duration (milliseconds)
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[deprecated = "Block highlighters will be removed in Minecraft client version 1.21.9."]
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisplayHighlighter(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__Vibration(...);
    /// **Set Hotbar Slot**<br/>
    /// Sets a player's selected
    /// hotbar slot.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   New slot
    ///   - 1 (left) to 9 (right)
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetSlot(...);
    /// **Display Particle Ray**<br/>
    /// Displays a ray of particles
    /// to a player.
    ///
    /// ## Arguments
    /// - `df_particle` (Particle):
    ///   Effect
    /// - `df_location` (Location):
    ///   Ray location
    /// - `df_vector` (Vector):
    ///   Ray vector
    /// - `df_number` `?` (Number):
    ///   Effect spacing
    ///   - Default = 0.5 blocks
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ParticleRay(...);
    /// **Display Particle Cuboid**<br/>
    /// Displays a particle cuboid as a
    /// solid, hollow or wireframe
    /// shape to a player.
    ///
    /// ## Tags
    /// - Fill Type:
    ///   - `Wireframe` (Default)
    ///   - `Hollow`
    ///   - `Solid`
    ///
    /// ## Arguments
    /// - `df_particle` (Particle):
    ///   Effect
    /// - `df_location` (Location):
    ///   Corner 1
    /// - `df_location` (Location):
    ///   Corner 2
    /// - `df_number` `?` (Number):
    ///   Effect spacing
    ///   - Default = 0.5 blocks
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ParticleCuboid(FillSPECIALSpace_Type : df_string, ...);
    /// **Send Message Sequence**<br/>
    /// Sends a series of messages
    /// in chat to a player, with a
    /// delay after each message.
    ///
    /// ## Tags
    /// - Alignment Mode:
    ///   - `Regular` (Default)
    ///   - `Centered`
    ///
    /// ## Arguments
    /// - `df_text` `[]` (Styled Text):
    ///   Messages to send
    /// - `df_number` `?` (Number):
    ///   Message delay ticks
    ///   - Default = 60
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    /// - Requires token shop purchase
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SendMessageSeq(AlignmentSPECIALSpace_Mode : df_string, ...);
    /// **Set Name Prefix / Suffix**<br/>
    /// Sets the prefix or suffix
    /// for the player's name.
    ///
    /// ## Tags
    /// - Text Type:
    ///   - `Prefix` (Default)
    ///   - `Suffix`
    ///
    /// ## Arguments
    /// - `df_text` `?` (Styled Text):
    ///   Prefix/suffix text
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetNamePrefix(TextSPECIALSpace_Type : df_string, ...);
    /// **Clear Display Block**<br/>
    /// Displays the real block at a
    /// location to a player, effectively
    /// removing any client-side blocks.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Block location,
    ///   or start of region
    /// - `df_location` `?` (Location):
    ///   End of region
    ///   - Region size limit: 500 blocks
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ClearDispBlock(...);
    /// **Set Rain Level**<br/>
    /// Sets the heaviness of rain and
    /// storm visible to a player.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Rain level (%)
    /// - `df_number` (Number):
    ///   Storm level (%)
    ///
    /// ## Additional Info
    /// - The player's weather must be
    ///   set to Downfall to take effect.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetRainLevel(...);
    /// **Undisguise**<br/>
    /// Removes a player's disguise.
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__Undisguise(...);
    /// **Display Animated Particle Spiral**<br/>
    /// Displays an animated spiral of
    /// particles to a player.
    ///
    /// ## Arguments
    /// - `df_particle` (Particle):
    ///   Effect
    /// - `df_location` (Location):
    ///   Base location
    /// - `df_number` `?` (Number):
    ///   Length
    ///   - Default = 10 blocks
    /// - `df_number` `?` (Number):
    ///   Diameter
    ///   - Default = 2 blocks
    /// - `df_number` `?` (Number):
    ///   Effect count
    ///   - Default = 50
    /// - `df_number` `?` (Number):
    ///   Rotations
    ///   - Default = 4
    /// - `df_number` `?` (Number):
    ///   Animation duration
    ///   - Default = 40 ticks
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ParticleSpiralA(...);
    /// **Set Instant Respawn**<br/>
    /// Sets if a player is instantly 
    /// respawned upon dying.
    ///
    /// ## Tags
    /// - Instant Respawn:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Additional Info
    /// - Also respawns a player if
    ///   they are dead.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__InstantRespawn(InstantSPECIALSpace_Respawn : df_string, ...);
    /// **Set Scoreboard Score**<br/>
    /// Sets a score on the
    /// scoreboard.
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   Score name
    /// - `df_number` `?` (Number):
    ///   Score value
    ///   - Default = 0
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetScore(...);
    /// **Set Name Color**<br/>
    /// Sets the color a player's
    /// name tag appears in.
    ///
    /// ## Tags
    /// - Name Color:
    ///   - `Black` (Default)
    ///   - `Dark blue`
    ///   - `Dark green`
    ///   - `Dark aqua`
    ///   - `Dark red`
    ///   - `Dark purple`
    ///   - `Gold`
    ///   - `Gray`
    ///   - `Dark gray`
    ///   - `Blue`
    ///   - `Green`
    ///   - `Aqua`
    ///   - `Red`
    ///   - `Light purple`
    ///   - `Yellow`
    ///   - `White`
    ///   - `None`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetNameColor(NameSPECIALSpace_Color : df_string, ...);
    /// **Set Reach Attribute**<br/>
    /// Sets one of the player's reach-related
    /// attributes such as block and
    /// entity interaction ranges.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Block interaction range` (Default)
    ///   - `Entity interaction range`
    /// - Value Type:
    ///   - `Direct` (Default)
    ///   - `Percentage (Base)`
    ///   - `Percentage (Relative)`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Value
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ReachAttribute(Attribute : df_string, ValueSPECIALSpace_Type : df_string, ...);
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetAtkSpeed(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisablePvp(...);
    /// **Set Tick Rate**<br/>
    /// Changes the tick rate of a player.
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Ticks per second (0-20)
    ///   - Default = 20
    ///
    /// ## Additional Info
    /// - Lower tick rate makes the player
    ///   move in slower speed, and makes
    ///   various animations slower.
    /// - Tick rate of 0 does not affect
    ///   the player's movement and entities
    ///   will appear to be broken.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetTickRate(...);
    /// **Play Sound from Entity**<br/>
    /// Plays a sound that follows a
    /// moving entity or player.
    ///
    /// ## Tags
    /// - Sound Source:
    ///   - `Master` (Default)
    ///   - `Music`
    ///   - `Jukebox/Note Blocks`
    ///   - `Weather`
    ///   - `Blocks`
    ///   - `Hostile Creatures`
    ///   - `Friendly Creatures`
    ///   - `Players`
    ///   - `Ambient/Environment`
    ///   - `Voice/Speech`
    ///   - `UI`
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_sound` `[]` (Sound):
    ///   Sound to play
    /// - 
    /// - `df_string` `[]` (String):
    ///   Target UUID
    /// - OR
    /// - `df_text` `[]` (Styled Text):
    ///   Target name
    ///
    /// ## Additional Info
    /// - Some sounds are not supported
    ///   to be played from an entity.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__PlayEntitySound(SoundSPECIALSpace_Source : df_string, IgnoreSPECIALSpace_Formatting : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ReplaceProj(...);
    /// **Set Experience**<br/>
    /// Sets a player's experience
    /// level, points or progress.
    ///
    /// ## Tags
    /// - Set Experience:
    ///   - `Points`
    ///   - `Level` (Default)
    ///   - `Level Percentage`
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Experience to set
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetExp(SetSPECIALSpace_Experience : df_string, ...);
    /// **Set Mining Attribute**<br/>
    /// Sets one of the player's mining-related
    /// attributes such as break speed
    /// and mining efficiency.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Block break speed` (Default)
    ///   - `Mining efficiency`
    ///   - `Submerged mining speed`
    /// - Value Type:
    ///   - `Direct` (Default)
    ///   - `Percentage (Base)`
    ///   - `Percentage (Relative)`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Value
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__MiningAttribute(Attribute : df_string, ValueSPECIALSpace_Type : df_string, ...);
    /// **Set Knockback Attribute**<br/>
    /// Sets one of the player's knockback-related
    /// attributes such as knockback resistance.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Knockback resistance` (Default)
    ///   - `Explosion knockback resistance`
    /// - Value Type:
    ///   - `Direct` (Default)
    ///   - `Percentage (Base)`
    ///   - `Percentage (Relative)`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Value
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__KBAttribute(Attribute : df_string, ValueSPECIALSpace_Type : df_string, ...);
    /// **Set Movement Attribute**<br/>
    /// Sets one of the player's movement-related
    /// attributes, such as walking speed
    /// and jump height.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Walking speed` (Default)
    ///   - `Flying speed`
    ///   - `Jump strength`
    ///   - `Sneaking speed`
    ///   - `Step height`
    ///   - `Movement efficiency`
    ///   - `Water movement efficiency`
    /// - Value Type:
    ///   - `Direct` (Default)
    ///   - `Percentage (Base)`
    ///   - `Percentage (Relative)`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Value
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__MovementAttribute(Attribute : df_string, ValueSPECIALSpace_Type : df_string, ...);
    /// **Display Particle Spiral**<br/>
    /// Displays a spiral of particles
    /// at a location to a player.
    ///
    /// ## Arguments
    /// - `df_particle` (Particle):
    ///   Effect
    /// - `df_location` (Location):
    ///   Base location
    /// - `df_number` `?` (Number):
    ///   Length
    ///   - Default = 10 blocks
    /// - `df_number` `?` (Number):
    ///   Diameter
    ///   - Default = 2 blocks
    /// - `df_number` `?` (Number):
    ///   Effect count
    ///   - Default = 50
    /// - `df_number` `?` (Number):
    ///   Rotations
    ///   - Default = 4
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ParticleSpiral(...);
    /// **Set Falling Attribute**<br/>
    /// Sets one of the player's falling-related
    /// attributes, such as gravity
    /// and fall damage multiplier.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Gravity` (Default)
    ///   - `Safe fall distance`
    ///   - `Fall damage multiplier`
    /// - Value Type:
    ///   - `Direct` (Default)
    ///   - `Percentage (Base)`
    ///   - `Percentage (Relative)`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Value
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__FallingAttribute(Attribute : df_string, ValueSPECIALSpace_Type : df_string, ...);
    /// **Set Allow Flight**<br/>
    /// Sets whether a player
    /// is able to enter and exit
    /// flight mode by double
    /// tapping jump.
    ///
    /// ## Tags
    /// - Allow Flight:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetAllowFlight(AllowSPECIALSpace_Flight : df_string, ...);
    /// **Set Maximum Health**<br/>
    /// Sets a player's maximum
    /// health.
    ///
    /// ## Tags
    /// - Heal Player to Max Health:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Maximum health
    ///   - ❤ = 2 Health
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetMaxHealth(HealSPECIALSpace_PlayerSPECIALSpace_toSPECIALSpace_MaxSPECIALSpace_Health : df_string, ...);
    /// **Remove Boss Bar**<br/>
    /// Removes a boss health bar
    /// from a player's screen.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Boss bar position
    /// - OR
    /// - None:
    ///   Removes all boss bars
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SPECIALSpace_RemoveBossBarSPECIALSpace_(...);
    /// **Set Fog Distance**<br/>
    /// Sets how far the fog is
    /// displayed to a player.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Fog distance in chunks
    ///   (2-7)
    /// - OR
    /// - None:
    ///   Removes fog
    ///
    /// ## Additional Info
    /// - Fog distance cannot be higher
    ///   than the client's View Distance.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetFogDistance(...);
    /// **Set to Adventure Mode**<br/>
    /// Sets a player's game
    /// mode to Adventure.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__AdventureMode(...);
    /// **Set to Spectator Mode**<br/>
    /// Sets a player's game
    /// mode to Spectator.
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SpectatorMode(...);
    /// **Display Head Texture**<br/>
    /// Changes a head's texture at
    /// a location for a player.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Head location
    /// - 
    /// - `df_item` (Item):
    ///   Player Head
    /// - OR
    /// - `df_string` (String):
    ///   Head owner
    ///   - Player name, UUID, or texture
    ///
    /// ## Additional Info
    /// - The texture is client-side but
    ///   will NOT revert if interacted with.
    /// - Requires a player head to
    ///   already be placed.
    /// - Works on client-side player heads.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DispHeadTexture(...);
    /// **Clear Potion Effects**<br/>
    /// Removes all active potion
    /// effects from a player.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ClearPotions(...);
    /// **Set Player List Info**<br/>
    /// Sets the text to be displayed
    /// above or below a player's player
    /// list shown when pressing Tab.
    ///
    /// ## Tags
    /// - Player List Field:
    ///   - `Header` (Default)
    ///   - `Footer`
    /// - Text Value Merging:
    ///   - `Add spaces`
    ///   - `No spaces` (Default)
    /// - Inherit Styles:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_text` `[]?` (Styled Text):
    ///   Header/footer text
    ///
    /// ## Additional Info
    /// - Multiple variables (of any type)
    ///   will be merged into one message.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetTabListInfo(PlayerSPECIALSpace_ListSPECIALSpace_Field : df_string, TextSPECIALSpace_ValueSPECIALSpace_Merging : df_string, InheritSPECIALSpace_Styles : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__EnablePvp(...);
    /// **Prompt Purchase**<br/>
    /// Prompts the player to
    /// purchase a plot product.
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Product ID
    ///
    /// ## Additional Info
    /// - Plot products must be
    ///   approved before they
    ///   can be sold.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__PromptPurchase(...);
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__HideDisguise(...);
    /// **Set Scoreboard Line Number Format**<br/>
    /// Sets the number format of a
    /// single line in the player's
    /// scoreboard.
    ///
    /// ## Tags
    /// - Number Format:
    ///   - `Fixed` (Default)
    ///   - `Styled`
    ///   - `Blank`
    ///   - `Reset`
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   Score name
    /// - 
    /// - `df_text` (Styled Text):
    ///   Content or style
    /// - OR
    /// - None:
    ///
    /// ## Additional Info
    /// - Put an appropriate value in the chest
    ///   depending on the number format type.
    /// - Line number format overrides the default
    ///   number format of the whole scoreboard.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ScoreLineFormat(NumberSPECIALSpace_Format : df_string, ...);
    /// **Set Boss Bar**<br/>
    /// Creates or modifies a custom boss
    /// health bar at the top of a player's
    /// screen.
    ///
    /// ## Tags
    /// - Bar Style:
    ///   - `Solid` (Default)
    ///   - `6 segments`
    ///   - `10 segments`
    ///   - `12 segments`
    ///   - `20 segments`
    /// - Sky Effect:
    ///   - `None` (Default)
    ///   - `Create fog`
    ///   - `Darken sky`
    ///   - `Both`
    /// - Bar Color:
    ///   - `Red`
    ///   - `Purple` (Default)
    ///   - `Pink`
    ///   - `Blue`
    ///   - `Green`
    ///   - `Yellow`
    ///   - `White`
    ///
    /// ## Arguments
    /// - `df_text` `?` (Styled Text):
    ///   Title
    /// - `df_number` `?` (Number):
    ///   Current health
    /// - `df_number` `?` (Number):
    ///   Maximum health
    ///   - Default = 100
    /// - `df_number` `?` (Number):
    ///   Boss bar position
    ///   - Default = 1 (top)
    ///
    /// ## Additional Info
    /// - Boss bar positions are relative;
    ///   unused positions are not shown.
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SPECIALSpace_SetBossBarSPECIALSpace_(BarSPECIALSpace_Style : df_string, SkySPECIALSpace_Effect : df_string, BarSPECIALSpace_Color : df_string, ...);
    /// **Set Skin**<br/>
    /// Sets the player's skin.
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   Player head
    /// - OR
    /// - None:
    ///   Clears Skin
    ///
    /// ## Additional Info
    /// - To get a skin use
    ///   /item head \<player name\>
    ///   OR /item mshead
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetSkin(...);
    /// **Set Spectator Collision**<br/>
    /// Toggles whether a player
    /// collides with blocks in
    /// spectator mode.
    ///
    /// ## Tags
    /// - Spectator Collision:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SpectatorCollision(SpectatorSPECIALSpace_Collision : df_string, ...);
    /// **Set Name Tag Visible**<br/>
    /// Sets whether a player's
    /// name tag is visible.
    ///
    /// ## Tags
    /// - Name Tag Visible:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetNameVisible(NameSPECIALSpace_TagSPECIALSpace_Visible : df_string, ...);
    /// **Set Invulnerability Ticks**<br/>
    /// Sets the currently
    /// remaining ticks until a
    /// player can next be hurt.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Ticks
    ///
    /// ## Additional Info
    /// - This value is set to 10
    ///   upon taking damage.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetInvulTicks(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__EnableFlight(...);
    /// **Mimic Entity**<br/>
    /// Disguises a player as another
    /// currently existing entity or player.
    ///
    /// ## Tags
    /// - Remove Original Entity:
    ///   - `Enable` (Default)
    ///   - `Disable`
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   UUID of target
    ///   to disguise as
    /// - OR
    /// - `df_text` `[]` (Styled Text):
    ///   Name of target
    ///   to disguise as
    ///
    /// ## Additional Info
    /// - The disguise will match the target exactly,
    ///   including things like color or equipment.
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__Mimic(RemoveSPECIALSpace_OriginalSPECIALSpace_Entity : df_string, ...);
    /// **Set Bee Stings Stuck**<br/>
    /// Sets the amount of bee stings
    /// sticking out of a player's
    /// character model.
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Sting Count
    ///   - Default = 0
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetStingsStuck(...);
    /// **Remove Scoreboard Score**<br/>
    /// Removes a score from
    /// the scoreboard.
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   Score name
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__RemoveScore(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisallowDrops(...);
    /// **Set Exhaustion Level**<br/>
    /// Sets a player's exhaustion level.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Exhaustion level
    ///   (0-4)
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetExhaustion(...);
    /// **Display Particle Circle**<br/>
    /// Displays a circle of particles
    /// to a player.
    ///
    /// ## Arguments
    /// - `df_particle` (Particle):
    ///   Effect
    /// - `df_location` (Location):
    ///   Center location
    /// - `df_number` `?` (Number):
    ///   Diameter
    ///   - Default = 2 blocks
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ParticleCircle(...);
    /// **Display Block**<br/>
    /// Displays a block at a location to
    /// a player.
    ///
    /// ## Arguments
    /// - `df_item` (Block):
    ///   Block to display
    /// - `df_location` (Location):
    ///   Block location,
    ///   or start of region
    /// - `df_location` `?` (Location):
    ///   End of region
    ///   - Region size limit: 250,000 blocks
    /// - `df_string` `[]?` (Block Tag):
    ///   Block data
    ///   - Example: "facing=up", "half=top"
    ///
    /// ## Additional Info
    /// - This block is client-side. The
    ///   actual block is not changed.
    /// - Interacting with a client-side
    ///   block causes it to update.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisplayBlock(...);
    /// **Ride Entity**<br/>
    /// Mounts a player on top of
    /// another player or entity.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   Target UUID
    /// - OR
    /// - `df_text` `[]` (Styled Text):
    ///   Target name
    /// - OR
    /// - None:
    ///   Dismounts player
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__RideEntity(IgnoreSPECIALSpace_Formatting : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__WeatherRain(...);
    /// **Remove World Border**<br/>
    /// Removes a player's world border.
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__RmWorldBorder(...);
    /// **Send Resource Pack**<br/>
    /// Send a resource pack to a player.
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Resource Pack URL
    ///   - Must link directly to a .zip file.
    ///
    /// ## Additional Info
    /// - Resource packs may only be
    ///   sent once per player.
    /// - Resource packs are limited to
    ///   10 MiB in size, but can be
    ///   upgraded to 40 MiB with /plot addons.
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ResourcePack(...);
    /// **Set Inventory Menu Name**<br/>
    /// Renames a player's current
    /// inventory menu.
    ///
    /// ## Tags
    /// - Alignment Mode:
    ///   - `Regular` (Default)
    ///   - `Centered`
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   Inventory name
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SPECIALSpace_SetInvNameSPECIALSpace_(AlignmentSPECIALSpace_Mode : df_string, ...);
    /// **Give Exhaustion**<br/>
    /// Adds exhaustion to a player.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Exhaustion to give
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__GiveExhaustion(...);
    /// **Teleport**<br/>
    /// Teleports a player to
    /// a location.
    ///
    /// ## Tags
    /// - Keep Current Rotation:
    ///   - `True`
    ///   - `False` (Default)
    /// - Keep Velocity:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   New position
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__Teleport(KeepSPECIALSpace_CurrentSPECIALSpace_Rotation : df_string, KeepSPECIALSpace_Velocity : df_string, ...);
    /// **Set PvP Allowed**<br/>
    /// Sets whether a player can
    /// hurt or be hurt by other
    /// players.
    ///
    /// ## Tags
    /// - PVP:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetAllowPVP(PVP : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisableFlight(...);
    /// **Set Visual Fire**<br/>
    /// Sets whether a player
    /// should appear on fire.
    ///
    /// ## Tags
    /// - On Fire:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Additional Info
    /// - The affected player's
    ///   fire ticks won't change
    ///   and won't take any damage.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetVisualFire(OnSPECIALSpace_Fire : df_string, ...);
    /// **Set Own Disguise Visibility**<br/>
    /// Sets a player's ability to
    /// see their own disguise. It
    /// is recommended that it is
    /// almost always hidden.
    ///
    /// ## Tags
    /// - Disguise Visible:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetDisguiseVisible(DisguiseSPECIALSpace_Visible : df_string, ...);
    /// **Shift Disguise Vertically**<br/>
    /// Shifts the disguise of a player up or
    /// down relative to the player.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Y-Offset
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisguiseShiftVert(...);
    /// **Set Arrows Stuck**<br/>
    /// Sets the amount of arrows
    /// sticking out of a player's
    /// character model.
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Arrow Count
    ///   - Default = 0
    ///
    /// ## Additional Info
    /// - These arrows cannot be
    ///   used or picked up.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetArrowsStuck(...);
    /// **Get Item Cooldown**<br/>
    /// Gets the remaining cooldown
    /// on an item type.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to set
    /// - `df_item` (Item):
    ///   Item type to check
    ///
    /// ## Returns
    /// - `df_number` (Number):
    ///   Cooldown left in ticks
    ///
    /// ## Additional Info
    /// - The cooldown applies to all items
    ///   of the given type.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__GetItemCooldown(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetItems(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__KeepInv(...);
    /// **Replace Items**<br/>
    /// Replaces items in a player's
    /// inventory with the given item.
    ///
    /// ## Arguments
    /// - `df_item` `[]?` (Item):
    ///   Item(s) to replace
    /// - `df_item` (Item):
    ///   Item to replace with
    /// - `df_number` `?` (Number):
    ///   Amount of items to
    ///   replace
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ReplaceItems(...);
    /// **Send Message**<br/>
    /// Sends a chat message to a
    /// player.
    ///
    /// ## Tags
    /// - Alignment Mode:
    ///   - `Regular` (Default)
    ///   - `Centered`
    /// - Text Value Merging:
    ///   - `Add spaces` (Default)
    ///   - `No spaces`
    /// - Inherit Styles:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_text` `[]?` (Styled Text):
    ///   Message to send
    ///
    /// ## Additional Info
    /// - Multiple values (of any type)
    ///   will be merged together.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SendMessage(AlignmentSPECIALSpace_Mode : df_string, TextSPECIALSpace_ValueSPECIALSpace_Merging : df_string, InheritSPECIALSpace_Styles : df_string, ...);
    /// **Set Item in Slot**<br/>
    /// Sets the item in a slot
    /// of a player's inventory.
    ///
    /// ## Arguments
    /// - `df_item` `?` (Item):
    ///   Item to set
    /// - `df_number` (Number):
    ///   Slot to set
    ///   - 1-9 Hotbar
    ///   - 10-36 Inventory (Top to bottom)
    ///   - 37-40 Armor (foot to head)
    ///   - 41 Offhand
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetSlotItem(...);
    /// **Play Sound Sequence**<br/>
    /// Plays a sequence of sounds
    /// to a player, with a delay
    /// between each sound.
    ///
    /// ## Tags
    /// - Sound Source:
    ///   - `Master` (Default)
    ///   - `Music`
    ///   - `Jukebox/Note Blocks`
    ///   - `Weather`
    ///   - `Blocks`
    ///   - `Hostile Creatures`
    ///   - `Friendly Creatures`
    ///   - `Players`
    ///   - `Ambient/Environment`
    ///   - `Voice/Speech`
    ///   - `UI`
    ///
    /// ## Arguments
    /// - `df_sound` `[]` (Sound):
    ///   Sounds to play
    /// - `df_number` `?` (Number):
    ///   Sound delay (ticks,
    ///   default = 60)
    /// - `df_location` `?` (Location):
    ///   Playback location
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__PlaySoundSeq(SoundSPECIALSpace_Source : df_string, ...);
    /// **Display Animated Particle Line**<br/>
    /// Displays an animated line of
    /// particles between two locations
    /// to a player.
    ///
    /// ## Arguments
    /// - `df_particle` (Particle):
    ///   Effect
    /// - `df_location` (Location):
    ///   Start location
    /// - `df_location` (Location):
    ///   End location
    /// - `df_number` `?` (Number):
    ///   Effect spacing
    ///   - Default = 0.5 blocks
    /// - `df_number` `?` (Number):
    ///   Animation duration
    ///   - Default = 40 ticks
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ParticleLineA(...);
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__Respawn(...);
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetInvName(...);
    /// **Set Item Cooldown**<br/>
    /// Applies a cooldown visual effect
    /// to an item type.
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   Item type to affect
    /// - `df_number` (Number):
    ///   Cooldown in ticks
    ///
    /// ## Additional Info
    /// - The cooldown applies to all items
    ///   of the given type.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetItemCooldown(...);
    /// **Set Player Weather**<br/>
    /// Sets the type of weather
    /// visible to a player.
    ///
    /// ## Tags
    /// - Weather:
    ///   - `Clear`
    ///   - `Downfall` (Default)
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetPlayerWeather(Weather : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SendHover(...);
    /// **Set Visual Shoulder Parrot**<br/>
    /// Displays a parrot on the targets'
    /// shoulders.
    ///
    /// ## Tags
    /// - Shoulder:
    ///   - `Left` (Default)
    ///   - `Right`
    /// - Type:
    ///   - `Remove` (Default)
    ///   - `Red`
    ///   - `Blue`
    ///   - `Green`
    ///   - `Cyan`
    ///   - `Gray`
    ///
    /// ## Additional Info
    /// - Parrot will not dismount
    ///   automatically.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetShoulder(Shoulder : df_string, Type : df_string, ...);
    /// **Set Remaining Air**<br/>
    /// Sets a player's remaining
    /// breath ticks.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Breath ticks
    ///
    /// ## Additional Info
    /// - Each bubble is equal
    ///   to 30 air ticks.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetAirTicks(...);
    /// **Display Pick Up Animation**<br/>
    /// Displays a pickup animation
    /// of one entity being collected
    /// by another entity.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Entity UUID
    /// - OR
    /// - `df_text` (Styled Text):
    ///   Entity name
    /// - 
    /// - `df_string` (String):
    ///   Collector UUID
    /// - OR
    /// - `df_text` (Styled Text):
    ///   Collector name
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisplayPickup(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Set World Border**<br/>
    /// Creates a world border only
    /// visible to a player.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Center position
    /// - `df_number` (Number):
    ///   Radius in blocks
    /// - `df_number` `?` (Number):
    ///   Warning distance
    ///   - Default = 0
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetWorldBorder(...);
    /// **Set Player Time**<br/>
    /// Sets the time of day visible
    /// to a player.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Daylight ticks
    ///   - Day: 1000
    ///   - Noon: 6000
    ///   - Night: 13000
    ///   - Midnight: 18000
    /// - OR
    /// - None:
    ///   Resets player time
    ///
    /// ## Additional Info
    /// - Add 24000 ticks to add
    ///   one moon phase.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetPlayerTime(...);
    /// **Give Food**<br/>
    /// Adds food to a player.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Food to give
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__GiveFood(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__NatRegen(...);
    /// **Give Potion Effect**<br/>
    /// Gives one or more potion
    /// effects to a player.
    ///
    /// ## Tags
    /// - Show Icon:
    ///   - `True` (Default)
    ///   - `False`
    /// - Overwrite Effect:
    ///   - `True` (Default)
    ///   - `False`
    /// - Effect Particles:
    ///   - `Regular` (Default)
    ///   - `Ambient`
    ///   - `None`
    ///
    /// ## Arguments
    /// - `df_potion` `[]` (Potion):
    ///   Effect(s)
    ///   to give
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__GivePotion(ShowSPECIALSpace_Icon : df_string, OverwriteSPECIALSpace_Effect : df_string, EffectSPECIALSpace_Particles : df_string, ...);
    /// **Remove Items**<br/>
    /// Removes items from a player.
    ///
    /// ## Arguments
    /// - `df_item` `[]` (Item):
    ///   Item(s) to remove
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__RemoveItems(...);
    /// **Boost Elytra**<br/>
    /// Boosts a player's elytra
    /// using a firework rocket.
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   Firework
    ///
    /// ## Additional Info
    /// - The player must be
    ///   gliding with an elytra.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__BoostElytra(...);
    /// **Save Current Inventory**<br/>
    /// Saves a player's inventory.
    /// It can be loaded later with
    /// 'Load Saved Inventory'.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SaveInv(...);
    /// **Open Book**<br/>
    /// Opens a written book
    /// menu for a player.
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   Book item
    ///
    /// ## Additional Info
    /// - Opened book and quills
    ///   cannot be edited.
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__OpenBook(...);
    /// **Set Current Health**<br/>
    /// Sets a player's current
    /// health.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Current health
    ///   - ❤ = 2 Health
    ///
    /// ## Additional Info
    /// - Does not trigger a heal
    ///   or damage event.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetHealth(...);
    /// **Disguise as Block**<br/>
    /// Disguises a player as a block.
    ///
    /// ## Arguments
    /// - `df_item` (Block):
    ///   Block to disguise as
    /// - `df_text` `?` (Styled Text):
    ///   Display name
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__BlockDisguise(...);
    /// **Roll Back Block Changes**<br/>
    /// Undoes the interactions with
    /// blocks by a player.
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Rollback time
    ///
    /// ## Additional Info
    /// - The rollback time argument
    ///   specifies how far back
    ///   in time block changes should
    ///   be reverted.
    /// - Please note that the rollback
    ///   time argument is in
    ///   SECONDS!
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__RollbackBlocks(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__NoDeathDrops(...);
    /// **Set Walk Speed**<br/>
    /// Sets a player's walk
    /// speed.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   % of normal
    ///   walk speed (0 to 500)
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__WalkSpeed(...);
    /// **Clear Block Highlighters**<br/>
    /// Clears all block highlighters
    /// displayed to a player.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[deprecated = "Block highlighters will be removed in Minecraft client version 1.21.9."]
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ClearHighlighters(...);
    /// **Set Collidable**<br/>
    /// Sets whether a player is
    /// able to collide with other
    /// entities.
    ///
    /// ## Tags
    /// - Collision:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    /// ## Additional Info
    /// - Useful in conjunction
    ///   with projectiles.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetCollidable(Collision : df_string, ...);
    /// **Launch Forward**<br/>
    /// Launches a player forward
    /// or backward.
    ///
    /// ## Tags
    /// - Add to Current Velocity:
    ///   - `True` (Default)
    ///   - `False`
    /// - Launch Axis:
    ///   - `Pitch and Yaw` (Default)
    ///   - `Yaw Only`
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Launch power
    ///
    /// ## Additional Info
    /// - A negative launch power will
    ///   launch the player backward.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__LaunchFwd(AddSPECIALSpace_toSPECIALSpace_CurrentSPECIALSpace_Velocity : df_string, LaunchSPECIALSpace_Axis : df_string, ...);
    /// **Set Fall Distance**<br/>
    /// Sets a player's fall distance,
    /// affecting fall damage upon
    /// landing.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Fall distance (blocks)
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetFallDistance(...);
    /// **Set to Creative Mode**<br/>
    /// Sets a player's game
    /// mode to Creative.
    ///
    /// ## Restrictions
    /// - Requires token shop purchase
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__CreativeMode(...);
    /// **Send Player Attack Animation**<br/>
    /// Makes a player perform
    /// an attack animation.
    ///
    /// ## Tags
    /// - Animation Arm:
    ///   - `Swing main arm` (Default)
    ///   - `Swing off arm`
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__AttackAnimation(AnimationSPECIALSpace_Arm : df_string, ...);
    /// **Display Hologram**<br/>
    /// Displays a floating name tag
    /// at a location to a player.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Display location
    /// - 
    /// - `df_text` (Styled Text):
    ///   Text to display
    /// - OR
    /// - None:
    ///   Removes hologram
    ///
    /// ## Additional Info
    /// - If the player already has
    ///   a hologram displayed at the
    ///   location, its text is replaced.
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DisplayHologram(...);
    /// **Set Absorption Health**<br/>
    /// Sets a player's absorption
    /// health (golden hearts).
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Absorption health
    ///   - ❤ = 2 Health
    ///
    /// ## Additional Info
    /// - The target does not need to
    ///   have an absorption effect.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SPECIALSpace_SetAbsorptionSPECIALSpace_(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__DeathDrops(...);
    /// **Show Inventory Menu**<br/>
    /// Opens a custom inventory
    /// for a player.
    ///
    /// ## Arguments
    /// - `df_item` `[]?` (Item):
    ///   Items to display
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ShowInv(...);
    /// **Set Food Level**<br/>
    /// Sets a player's food hunger level.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Food level
    ///   (1-20)
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetFoodLevel(...);
    /// **Disguise as Player**<br/>
    /// Disguises a player as another
    /// player.
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   Player name to disguise as
    /// - `df_item` `?` (Item):
    ///   Display skin
    ///
    /// ## Additional Info
    /// - To get a skin use /item head \<player name\>
    ///   OR /item mshead
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__PlayerDisguise(...);
    /// **Set Saturation Level**<br/>
    /// Sets a player's saturation
    /// level.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Saturation level
    ///   (1-20)
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SetSaturation(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__WeatherClear(...);
    /// **Show Title Text**<br/>
    /// Displays text in the center
    /// of a player's screen.
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   Title text
    /// - `df_text` `?` (Styled Text):
    ///   Subtitle text
    /// - `df_number` `?` (Number):
    ///   Title duration
    ///   - Default = 60 ticks
    /// - `df_number` `?` (Number):
    ///   Fade in length
    ///   - Default = 20 ticks
    /// - `df_number` `?` (Number):
    ///   Fade out length
    ///   - Default = 20 ticks
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    /// - Requires token shop purchase
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__SendTitle(...);
    /// **Set Scoreboard Default Number Format**<br/>
    /// Sets the default number format
    /// of the player's scoreboard.
    ///
    /// ## Tags
    /// - Number Format:
    ///   - `Fixed` (Default)
    ///   - `Styled`
    ///   - `Blank`
    ///   - `Reset`
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   Content or style
    /// - OR
    /// - None:
    ///
    /// ## Additional Info
    /// - Put an appropriate value in the chest
    ///   depending on the number format type.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ScoreDefFormat(NumberSPECIALSpace_Format : df_string, ...);
    /// **Stop Sounds**<br/>
    /// Stops all or specific sounds
    /// for a player.
    ///
    /// ## Tags
    /// - Sound Source:
    ///   - `Master` (Default)
    ///   - `Music`
    ///   - `Jukebox/Note Blocks`
    ///   - `Weather`
    ///   - `Blocks`
    ///   - `Hostile Creatures`
    ///   - `Friendly Creatures`
    ///   - `Players`
    ///   - `Ambient/Environment`
    ///   - `Voice/Speech`
    ///   - `UI`
    ///
    /// ## Arguments
    /// - `df_sound` `[]?` (Sound):
    ///   Sounds to stop
    ///
    /// ## Additional Info
    /// - Stopping sounds from the
    ///   Master source stops every
    ///   sound effect.
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__StopSound(SoundSPECIALSpace_Source : df_string, ...);
    /// **Set Health Attribute**<br/>
    /// Sets one of the player's health-related
    /// attributes such as max health
    /// and armor defense points.
    ///
    /// ## Tags
    /// - Attribute:
    ///   - `Maximum health` (Default)
    ///   - `Maximum absorption health`
    ///   - `Armor`
    ///   - `Armor toughness`
    /// - Value Type:
    ///   - `Direct` (Default)
    ///   - `Percentage (Base)`
    ///   - `Percentage (Relative)`
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Value
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__HealthAttribute(Attribute : df_string, ValueSPECIALSpace_Type : df_string, ...);
    /// **Display Particle Line**<br/>
    /// Displays a line of particles
    /// between two locations to
    /// a player.
    ///
    /// ## Arguments
    /// - `df_particle` (Particle):
    ///   Effect
    /// - `df_location` (Location):
    ///   Start location
    /// - `df_location` (Location):
    ///   End location
    /// - `df_number` `?` (Number):
    ///   Effect spacing
    ///   - Default = 0.5 blocks
    ///
    pub unsafe fn DF_ACTION__playerSPECIALSpace_action__ParticleLine(...);
    /// **Number ≤**<br/>
    /// Checks if a number value is
    /// less than or equal to another
    /// number.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Number to check
    /// - `df_number` (Number):
    ///   Number to compare to
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__SPECIALLessthan_SPECIALEqual_(...);
    /// **Item Has Enchantment**<br/>
    /// Checks if an item has a
    /// given enchantment, or,
    /// if no enchantment is specified,
    /// checks if it has any.
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   Item to check
    /// - `df_string` `?` (String):
    ///   Enchantment
    /// - `df_number` `?` (Number):
    ///   Level
    ///
    /// ## Additional Info
    /// - A level of 0 will work
    ///   for any level.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__ItemHasEnchant(...);
    /// **Item Is Block**<br/>
    /// Checks if an item is
    /// able to be placed.
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   Item to check
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__ItemIsBlock(...);
    /// **Dictionary Value Equals**<br/>
    /// Checks if a dictionary's value
    /// for the given key is equal to
    /// any of the given values.
    ///
    /// ## Arguments
    /// - `df_dict` (Dict):
    ///   Dictionary to check
    /// - `df_string` (String):
    ///   Key to check
    /// - `df_opaque` `[]` (Any):
    ///   Values to
    ///   compare with
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__DictValueEquals(...);
    /// **Item Has Custom Tag**<br/>
    /// Checks if an item has a
    /// given custom tag, and (if
    /// provided) whether the tag
    /// matches the given value.
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   Item to check
    /// - `df_string` (String):
    ///   Tag name
    /// - 
    /// - `df_number` `?` (Number):
    ///   Tag value
    /// - OR
    /// - `df_string` `?` (String):
    ///   Tag value
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__ItemHasTag(...);
    /// **List Size Equals**<br/>
    /// Checks if a list is of a given size.
    ///
    /// ## Arguments
    /// - `df_list` (List):
    ///   List to check
    /// - `df_number` `?` (Number):
    ///   Size to compare
    ///   - Default = 0
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__ListSizeEquals(...);
    /// **String Matches**<br/>
    /// Checks if a string value matches
    /// other values.
    ///
    /// ## Tags
    /// - Regular Expressions:
    ///   - `Enable`
    ///   - `Disable` (Default)
    /// - Ignore Case:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   String or source expression to match
    /// - `df_string` `[]` (String):
    ///   String to compare
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__StringMatches(RegularSPECIALSpace_Expressions : df_string, IgnoreSPECIALSpace_Case : df_string, ...);
    /// **String Is Filtered**<br/>
    /// Checks if the input string
    /// would be filtered by the
    /// selected chat filters.
    ///
    /// ## Tags
    /// - Link Filter:
    ///   - `True` (Default)
    ///   - `False`
    /// - Swear Filter:
    ///   - `True` (Default)
    ///   - `False`
    /// - Caps Filter:
    ///   - `True` (Default)
    ///   - `False`
    /// - Character Spacing Filter:
    ///   - `True` (Default)
    ///   - `False`
    /// - Character Drag Filter:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   String to check
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__IsFiltered(LinkSPECIALSpace_Filter : df_string, SwearSPECIALSpace_Filter : df_string, CapsSPECIALSpace_Filter : df_string, CharacterSPECIALSpace_SpacingSPECIALSpace_Filter : df_string, CharacterSPECIALSpace_DragSPECIALSpace_Filter : df_string, ...);
    /// **String Starts With**<br/>
    /// Checks if the first part of
    /// a string value matches a
    /// certain string.
    ///
    /// ## Tags
    /// - Ignore Case:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   String to check
    /// - `df_string` `[]` (String):
    ///   String to start with
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__StartsWith(IgnoreSPECIALSpace_Case : df_string, ...);
    /// **List Value Equals**<br/>
    /// Checks if a list's value at an
    /// index is equal to a value.
    ///
    /// ## Arguments
    /// - `df_list` (List):
    ///   List to check in
    /// - `df_number` (Number):
    ///   Index to check at
    /// - `df_opaque` `[]` (Any):
    ///   Variable to
    ///   compare to
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__ListValueEq(...);
    /// **Value Is Type**<br/>
    /// Checks if a value is of a
    /// certain type.
    ///
    /// ## Tags
    /// - Variable Type:
    ///   - `Number` (Default)
    ///   - `String`
    ///   - `Styled Text`
    ///   - `Location`
    ///   - `Item`
    ///   - `List`
    ///   - `Potion effect`
    ///   - `Sound`
    ///   - `Particle`
    ///   - `Vector`
    ///   - `Dictionary`
    ///
    /// ## Arguments
    /// - `df_opaque` (Any):
    ///   Value to check
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__VarIsType(VariableSPECIALSpace_Type : df_string, ...);
    ///
    /// ## Tags
    /// - Regular Expressions:
    ///   - `Enable`
    ///   - `Disable` (Default)
    /// - Ignore Case:
    ///   - `True`
    ///   - `False` (Default)
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__TextMatches(RegularSPECIALSpace_Expressions : df_string, IgnoreSPECIALSpace_Case : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__IsNear(...);
    /// **Value Is Within Range**<br/>
    /// Checks if a number value is
    /// in between 2 other numbers or
    /// a location value is within the
    /// region of 2 other locations.
    ///
    /// ## Tags
    /// - Location Handling:
    ///   - `Block`
    ///   - `Exact` (Default)
    ///
    /// ## Arguments
    /// - `df_opaque` (Any):
    ///   Check value
    /// - `df_opaque` (Any):
    ///   Minimum value
    /// - `df_opaque` (Any):
    ///   Maximum value
    ///
    /// ## Works With
    /// - `Number`
    /// - `Location`
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__SPECIALSpace_InRangeSPECIALSpace_(LocationSPECIALSpace_Handling : df_string, ...);
    /// **Value Is Empty**<br/>
    /// Checks if a value is empty.
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   String to check if empty
    /// - OR
    /// - `df_text` (Styled Text):
    ///   Text to compare
    ///   content length to 0
    /// - OR
    /// - `df_list` (List):
    ///   List to compare size to 0
    /// - OR
    /// - `df_dict` (Dict):
    ///   Dictionary to
    ///   compare size to 0
    /// - OR
    /// - `df_item` (Item):
    ///   Item to compare to air
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__ValueIsEmpty(...);
    /// **Variable Exists**<br/>
    /// Checks if a variable exists.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Variable to check
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__VarExists(...);
    /// **Dictionary Has Keys**<br/>
    /// Checks if a dictionary has
    /// the given keys.
    ///
    /// ## Tags
    /// - Check Mode:
    ///   - `Has Any Key` (Default)
    ///   - `Has All Keys`
    ///
    /// ## Arguments
    /// - `df_dict` (Dict):
    ///   Dictionary to check
    /// - `df_string` `[]` (String):
    ///   Key(s) to look for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__DictHasKeys(CheckSPECIALSpace_Mode : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__LegacySPECIALSpace_SPECIALExclamation_SPECIALEqual_(...);
    /// **Block Is Solid**<br/>
    /// Checks if a material will collide
    /// with entities.
    ///
    /// ## Arguments
    /// - `df_item` (Block):
    ///   Block to check for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__BlockIsSolid(...);
    /// **Item Equals**<br/>
    /// Works the same as Value =
    /// but has a few extra options
    /// for item comparison.
    ///
    /// ## Tags
    /// - Comparison Mode:
    ///   - `Exactly equals` (Default)
    ///   - `Ignore stack size`
    ///   - `Ignore durability and stack size`
    ///   - `Material only`
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   Item to check
    /// - `df_item` `[]?` (Item):
    ///   Item(s) to compare to
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__ItemEquals(ComparisonSPECIALSpace_Mode : df_string, ...);
    /// **List Contains Value**<br/>
    /// Checks if any of a list's contents
    /// match the given value.
    ///
    /// ## Tags
    /// - Check Mode:
    ///   - `Has Any Value` (Default)
    ///   - `Has All Values`
    ///
    /// ## Arguments
    /// - `df_list` (List):
    ///   List to check in
    /// - `df_opaque` `[]` (Any):
    ///   Value to find
    ///
    /// ## Additional Info
    /// - If multiple values are given, the
    ///   condition will return true if one
    ///   of the values are found.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__ListContains(CheckSPECIALSpace_Mode : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__LegacySPECIALSpace_SPECIALEqual_(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__InRange(...);
    /// **Location Is Near**<br/>
    /// Checks if a location is
    /// near another location.
    ///
    /// ## Tags
    /// - Shape:
    ///   - `Sphere` (Default)
    ///   - `Circle`
    ///   - `Cube`
    ///   - `Square`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Location to check
    /// - `df_location` `[]` (Location):
    ///   Location(s) to
    ///   compare to
    /// - `df_number` (Number):
    ///   Radius
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__LocIsNear(Shape : df_string, ...);
    /// **Item Is Unbreakable**<br/>
    /// Checks if an item is unbreakable.
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   Item to check
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__ItemIsUnbreakable(...);
    /// **String Contains**<br/>
    /// Checks if a string value
    /// contains another string.
    ///
    /// ## Tags
    /// - Ignore Case:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   String to check
    /// - `df_string` `[]` (String):
    ///   String to check for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__Contains(IgnoreSPECIALSpace_Case : df_string, ...);
    /// **Value !=**<br/>
    /// Checks if a value does not
    /// equal another value.
    ///
    /// ## Arguments
    /// - `df_opaque` (Any):
    ///   Value to check
    /// - `df_opaque` `[]` (Any):
    ///   Values to compare to
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__SPECIALExclamation_SPECIALEqual_(...);
    /// **Number \<**<br/>
    /// Checks if a number value is
    /// less than another number.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Number to check
    /// - `df_number` (Number):
    ///   Number to compare to
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__SPECIALLessthan_(...);
    /// **Value =**<br/>
    /// Checks if a value is equal
    /// to one of the given values.
    ///
    /// ## Arguments
    /// - `df_opaque` (Any):
    ///   Value to check
    /// - `df_opaque` `[]` (Any):
    ///   Values to compare to
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__SPECIALEqual_(...);
    /// **Number \>**<br/>
    /// Checks if a number value is
    /// greater than another number.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Number to check
    /// - `df_number` (Number):
    ///   Number to compare to
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__SPECIALGreaterthan_(...);
    /// **String Ends With**<br/>
    /// Checks if the last part of
    /// a string value matches a
    /// certain string.
    ///
    /// ## Tags
    /// - Ignore Case:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   String to check
    /// - `df_string` `[]` (String):
    ///   String to end with
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__EndsWith(IgnoreSPECIALSpace_Case : df_string, ...);
    /// **Number ≥**<br/>
    /// Checks if a number value
    /// is greater than or equal to
    /// another number.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Number to check
    /// - `df_number` (Number):
    ///   Number to compare to
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__SPECIALGreaterthan_SPECIALEqual_(...);
    /// **Dictionary Has Key**<br/>
    /// Checks if a dictionary has
    /// the given key.
    ///
    /// ## Arguments
    /// - `df_dict` (Dict):
    ///   Dictionary to check
    /// - `df_string` (String):
    ///   Key to look for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_variable__DictHasKey(...);
    /// **Has Room For Item**<br/>
    /// Checks if a player's inventory
    /// has room for one or more
    /// items to be given.
    ///
    /// ## Tags
    /// - Check Mode:
    ///   - `Has Room for Any Item` (Default)
    ///   - `Has Room for All Items`
    /// - Checked Slots:
    ///   - `Entire inventory`
    ///   - `Main inventory` (Default)
    ///   - `Upper inventory`
    ///   - `Hotbar`
    ///   - `Armor`
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   Item(s) to check with
    /// - OR
    /// - None:
    ///   Checks for empty slot
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__HasRoomForItem(CheckSPECIALSpace_Mode : df_string, CheckedSPECIALSpace_Slots : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsHoldingOff(...);
    /// **Item Is Not On Cooldown**<br/>
    /// Checks if a player does not have a
    /// cooldown applied to an item type.
    ///
    /// ## Arguments
    /// - `df_item` `[]` (Item):
    ///   Item type(s) to check
    ///
    /// ## Additional Info
    /// - The check will succeed if any
    ///   of the given items are not on
    ///   cooldown.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__NoItemCooldown(...);
    /// **Is Using Item**<br/>
    /// Checks if a player is currently
    /// using an item (eg. bow).
    ///
    /// ## Arguments
    /// - `df_item` `[]?` (Item):
    ///   Item(s) to check
    ///   - Checks material only
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsUsingItem(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__HasAllItems(...);
    /// **Has Item**<br/>
    /// Checks if a player has an item
    /// in their inventory.
    ///
    /// ## Tags
    /// - Check Mode:
    ///   - `Has Any Item` (Default)
    ///   - `Has All Items`
    ///
    /// ## Arguments
    /// - `df_item` `[]` (Item):
    ///   Item(s) to check for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__HasItem(CheckSPECIALSpace_Mode : df_string, ...);
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__BlockEquals(...);
    /// **Is Wearing Item**<br/>
    /// Checks if a player is wearing
    /// an item.
    ///
    /// ## Tags
    /// - Check Mode:
    ///   - `Is Wearing Some` (Default)
    ///   - `Is Wearing All`
    ///
    /// ## Arguments
    /// - `df_item` `[]` (Item):
    ///   Item(s) to check for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsWearing(CheckSPECIALSpace_Mode : df_string, ...);
    /// **Is Near Location**<br/>
    /// Checks if a player is within a
    /// range of a location.
    ///
    /// ## Tags
    /// - Shape:
    ///   - `Sphere` (Default)
    ///   - `Circle`
    ///   - `Cube`
    ///   - `Square`
    ///
    /// ## Arguments
    /// - `df_location` `[]` (Location):
    ///   Center location
    /// - `df_number` `?` (Number):
    ///   Radius
    ///   - Default = 5 blocks
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsNear(Shape : df_string, ...);
    ///
    /// ## Tags
    /// - Compare Text To:
    ///   - `Entity type` (Default)
    ///   - `Name or UUID`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsRiding(CompareSPECIALSpace_TextSPECIALSpace_To : df_string, ...);
    ///
    /// ## Tags
    /// - Check Mode:
    ///   - `Check Entire Command` (Default)
    ///   - `Check First Word`
    /// - Ignore Case:
    ///   - `True` (Default)
    ///   - `False`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__CmdEquals(CheckSPECIALSpace_Mode : df_string, IgnoreSPECIALSpace_Case : df_string, ...);
    /// **Cursor Item Equals**<br/>
    /// Checks if the item that is being moved
    /// with a player's cursor is the given item.
    ///
    /// ## Arguments
    /// - `df_item` `[]?` (Item):
    ///   Items(s) to check for
    ///
    /// ## Additional Info
    /// - If multiple items are in the chest,
    ///   the target can have any of them on
    ///   their cursor.
    /// - When used on the Player Click Item
    ///   in Own Inventory Event, 'Cursor Item
    ///   Equals' checks the previous cursor
    ///   item, not the item that was clicked.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__CursorItem(...);
    /// **Hotbar Slot Equals**<br/>
    /// Checks if a player's currently
    /// selected hotbar slot equals the
    /// given slot ID.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Slot ID to check
    ///   - 1 (left) to 9 (right)
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__SlotEquals(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__ItemEquals(...);
    /// **Inventory Menu Slot Equals**<br/>
    /// Checks if a player's currently
    /// open inventory menu contains
    /// an item in the given slot.
    ///
    /// ## Arguments
    /// - `df_number` `[]` (Number):
    ///   Slot(s) to check
    /// - `df_item` `[]?` (Item):
    ///   Item(s) to check for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__MenuSlotEquals(...);
    /// **Has Plot Permission**<br/>
    /// Checks if a player has a certain
    /// level of access on this plot, such
    /// as builder or owner.
    ///
    /// ## Tags
    /// - Permission:
    ///   - `Owner`
    ///   - `Developer`
    ///   - `Builder`
    ///   - `Developer or builder` (Default)
    ///   - `Whitelisted`
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__HasPermission(Permission : df_string, ...);
    /// **Is Riding Entity**<br/>
    /// Checks if a player is riding
    /// another entity.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_item` `[]` (EntityType):
    ///   Spawn egg,
    ///   projectile, or vehicle
    /// - OR
    /// - `df_string` `[]` (String):
    ///   Entity UUID
    /// - OR
    /// - `df_text` `[]` (Styled Text):
    ///   Entity name
    /// - OR
    /// - None:
    ///   Checks if riding any entity
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__SPECIALSpace_IsRidingSPECIALSpace_(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Main Hand Equals**<br/>
    /// Checks if a player's main hand
    /// is their left or right hand.
    ///
    /// ## Tags
    /// - Main Hand:
    ///   - `Left Hand` (Default)
    ///   - `Right Hand`
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__MainHandEquals(MainSPECIALSpace_Hand : df_string, ...);
    /// **Is Sneaking**<br/>
    /// Checks if a player is sneaking.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsSneaking(...);
    /// **Is Movement Key Pressed**<br/>
    /// Checks if a player is holding
    /// a specific movement key.
    ///
    /// ## Tags
    /// - Forward (W):
    ///   - `Pressed`
    ///   - `Released`
    ///   - `Don't check` (Default)
    /// - Backward (S):
    ///   - `Pressed`
    ///   - `Released`
    ///   - `Don't check` (Default)
    /// - Left (A):
    ///   - `Pressed`
    ///   - `Released`
    ///   - `Don't check` (Default)
    /// - Right (D):
    ///   - `Pressed`
    ///   - `Released`
    ///   - `Don't check` (Default)
    /// - Jump (Space):
    ///   - `Pressed`
    ///   - `Released`
    ///   - `Don't check` (Default)
    /// - Sneak (Left Shift):
    ///   - `Pressed`
    ///   - `Released`
    ///   - `Don't check` (Default)
    /// - Sprint (Left Control):
    ///   - `Pressed`
    ///   - `Released`
    ///   - `Don't check` (Default)
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__MovementKey(ForwardSPECIALSpace_SPECIALLeftparen_WSPECIALRightparen_ : df_string, BackwardSPECIALSpace_SPECIALLeftparen_SSPECIALRightparen_ : df_string, LeftSPECIALSpace_SPECIALLeftparen_ASPECIALRightparen_ : df_string, RightSPECIALSpace_SPECIALLeftparen_DSPECIALRightparen_ : df_string, JumpSPECIALSpace_SPECIALLeftparen_SpaceSPECIALRightparen_ : df_string, SneakSPECIALSpace_SPECIALLeftparen_LeftSPECIALSpace_ShiftSPECIALRightparen_ : df_string, SprintSPECIALSpace_SPECIALLeftparen_LeftSPECIALSpace_ControlSPECIALRightparen_ : df_string, ...);
    /// **Is Flying**<br/>
    /// Checks if a player is flying.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsFlying(...);
    /// **Inventory Type Open**<br/>
    /// Checks if a player has a
    /// certain inventory type open.
    ///
    /// ## Tags
    /// - Inventory Type:
    ///   - `Any Inventory` (Default)
    ///   - `Plot Menu`
    ///   - `Crafting Table`
    ///   - `Chest`
    ///   - `Double Chest`
    ///   - `Ender Chest`
    ///   - `Shulker Box`
    ///   - `Barrel`
    ///   - `Furnace (any)`
    ///   - `Furnace`
    ///   - `Blast Furnace`
    ///   - `Smoker`
    ///   - `Dropper`
    ///   - `Dispenser`
    ///   - `Beacon`
    ///   - `Hopper`
    ///   - `Anvil`
    ///   - `Brewing Stand`
    ///   - `Cartography Table`
    ///   - `Smithing Table`
    ///   - `Loom`
    ///   - `Grindstone`
    ///   - `Stonecutter`
    ///   - `Enchanting Table`
    ///   - `Trader Menu (any)`
    ///   - `Villager Menu`
    ///   - `Wandering Trader Menu`
    ///   - `Horse Inventory`
    ///   - `Llama Inventory`
    ///
    /// ## Additional Info
    /// - Does not work with special
    ///   screens such as the death
    ///   screen or the player's own
    ///   inventory.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__InvOpen(InventorySPECIALSpace_Type : df_string, ...);
    /// **Has Item in Slot**<br/>
    /// Checks if a player has an item
    /// in the given inventory slot.
    ///
    /// ## Arguments
    /// - `df_number` `[]` (Number):
    ///   Slot(s) to check
    /// - `df_item` `[]?` (Item):
    ///   Item(s) to check for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__HasSlotItem(...);
    /// **Is Sprinting**<br/>
    /// Checks if a player is sprinting
    /// or using the sprint key to swim.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsSprinting(...);
    /// **Is Gliding**<br/>
    /// Checks if a player is
    /// gliding with elytra.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsGliding(...);
    ///
    /// ## Tags
    /// - Ignore Case:
    ///   - `True` (Default)
    ///   - `False`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__CmdArgEquals(IgnoreSPECIALSpace_Case : df_string, ...);
    /// **Is Looking at Block**<br/>
    /// Checks if a player is looking at
    /// the given block or location.
    ///
    /// ## Tags
    /// - Fluid Mode:
    ///   - `Ignore fluids` (Default)
    ///   - `Detect fluids`
    ///
    /// ## Arguments
    /// - `df_item` `[]?` (Block):
    ///   Block to check for
    /// - OR
    /// - `df_location` `[]` (Location):
    ///   Location to check for
    /// - `df_number` `?` (Number):
    ///   Maximum distance from
    ///   target block/location
    ///   - This is distance from the player's
    ///     selected block 
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsLookingAt(FluidSPECIALSpace_Mode : df_string, ...);
    /// **Is In World Border**<br/>
    /// Checks if a player (or a location)
    /// is within their world border.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Location to check
    /// - OR
    /// - None:
    ///   Checks player location
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__InWorldBorder(...);
    /// **Is in Game Mode**<br/>
    /// Checks if a player is in
    /// a specific game mode.
    ///
    /// ## Tags
    /// - Game Mode:
    ///   - `Survival` (Default)
    ///   - `Creative`
    ///   - `Adventure`
    ///   - `Spectator`
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsInGameMode(GameSPECIALSpace_Mode : df_string, ...);
    /// **Is Using Resource Pack**<br/>
    /// Checks if a player is
    /// using a plot resource
    /// pack.
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__UsingPack(...);
    /// **Is Swimming**<br/>
    /// Checks if a player
    /// is in water or lava.
    ///
    /// ## Additional Info
    /// - Use 'Is Sprinting'
    ///   to check if a player
    ///   is swimming with the
    ///   swimming animation.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsSwimming(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__StandingOn(...);
    /// **Is Standing on Block**<br/>
    /// Checks if a player is standing on
    /// the given block or location.
    ///
    /// ## Arguments
    /// - `df_item` `[]?` (Block):
    ///   Block to check for
    /// - OR
    /// - `df_location` `[]` (Location):
    ///   Location to check for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__SPECIALSpace_StandingOnSPECIALSpace_(...);
    /// **Is Grounded**<br/>
    /// Checks if a player is
    /// supported by a block.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsGrounded(...);
    /// **Owns Product**<br/>
    /// Checks if a player owns a plot
    /// product or is a developer or
    /// builder.
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Product ID
    ///
    /// ## Additional Info
    /// - Only works with one-time
    ///   purchase products.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__OwnsProduct(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsHoldingMain(...);
    /// **Is Holding Item**<br/>
    /// Checks if a player is holding
    /// an item in their hand.
    ///
    /// ## Tags
    /// - Hand Slot:
    ///   - `Either hand` (Default)
    ///   - `Main hand`
    ///   - `Off hand`
    ///
    /// ## Arguments
    /// - `df_item` `[]?` (Item):
    ///   Item(s) to check for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsHolding(HandSPECIALSpace_Slot : df_string, ...);
    /// **Is Blocking**<br/>
    /// Checks if a player is
    /// blocking with a shield.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsBlocking(...);
    /// **Is Hitbox Near Location**<br/>
    /// Checks if a player's hitbox is
    /// within a range of a location.
    ///
    /// ## Tags
    /// - Shape:
    ///   - `Sphere` (Default)
    ///   - `Circle`
    ///   - `Cube`
    ///   - `Square`
    ///
    /// ## Arguments
    /// - `df_location` `[]` (Location):
    ///   Center location
    /// - `df_number` `?` (Number):
    ///   Range
    ///   - Default = 5 blocks
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__IsHitboxNear(Shape : df_string, ...);
    /// **Has Potion Effect**<br/>
    /// Checks if a player has a
    /// potion effect of the given
    /// type active.
    ///
    /// ## Tags
    /// - Check Properties:
    ///   - `None` (Default)
    ///   - `Amplifier`
    ///   - `Duration`
    ///   - `Amplifier and duration`
    /// - Check Mode:
    ///   - `Has any effect` (Default)
    ///   - `Has all effects`
    ///
    /// ## Arguments
    /// - `df_potion` `[]` (Potion):
    ///   Effect(s)
    ///   to check for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__HasPotion(CheckSPECIALSpace_Properties : df_string, CheckSPECIALSpace_Mode : df_string, ...);
    /// **Name Equals**<br/>
    /// Checks if a player's username is
    /// equal to one of the given
    /// usernames (case insensitive).
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   Name(s) to check for
    ///
    /// ## Additional Info
    /// - Works with UUIDs.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__NameEquals(...);
    /// **Can Fly**<br/>
    /// Checks if a player can fly.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_player__CanFly(...);
    /// **Repeat Adjacently**<br/>
    /// Repeats code once for each
    /// block adjacent to a location.
    ///
    /// ## Tags
    /// - Change Location Rotation:
    ///   - `True`
    ///   - `False` (Default)
    /// - Include Origin Block:
    ///   - `True`
    ///   - `False` (Default)
    /// - Pattern:
    ///   - `Cardinal (4 blocks)`
    ///   - `Square (8 blocks)`
    ///   - `Adjacent (6 blocks)` (Default)
    ///   - `Cube (26 blocks)`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Gets the current
    ///   block location each iteration
    /// - `df_location` (Location):
    ///   Center block
    ///
    pub unsafe fn DF_ACTION__repeat__Adjacent(ChangeSPECIALSpace_LocationSPECIALSpace_Rotation : df_string, IncludeSPECIALSpace_OriginSPECIALSpace_Block : df_string, Pattern : df_string, ...);
    /// **Repeat On Path**<br/>
    /// Repeats code once for
    /// each interpolated point in
    /// a path of locations.
    ///
    /// ## Tags
    /// - Rotate Location:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Gets the current
    ///   path location each iteration
    /// - `df_location` `[]` (Location):
    ///   Path locations
    /// - `df_number` `?` (Number):
    ///   Point spacing
    ///   - Default = 0.5 blocks
    ///
    pub unsafe fn DF_ACTION__repeat__Path(RotateSPECIALSpace_Location : df_string, ...);
    /// **Repeat Multiple Times**<br/>
    /// Repeats code multiple times.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` `?` (Variable):
    ///   Gets the
    ///   current index each iteration
    /// - `df_number` (Number):
    ///   Amount
    ///
    pub unsafe fn DF_ACTION__repeat__Multiple(...);
    /// **Repeat On Grid**<br/>
    /// Repeats code once for each
    /// block in a region in order:
    /// X → Z → Y. Iterates from the
    /// first to the second location.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Gets the current
    ///   block location each iteration
    /// - `df_location` (Location):
    ///   Start of region
    /// - `df_location` (Location):
    ///   End of region
    ///
    pub unsafe fn DF_ACTION__repeat__Grid(...);
    /// **Repeat Do While**<br/>
    /// Repeats code as long as a
    /// condition is true.
    /// The condition is evaluated
    /// at the end of each
    /// loop iteration.
    ///
    pub unsafe fn DF_ACTION__repeat__DoWhile(...);
    /// **Repeat While**<br/>
    /// Repeats code as long as a
    /// condition is true.
    /// The condition is evaluated
    /// before each loop iteration.
    ///
    pub unsafe fn DF_ACTION__repeat__While(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__repeat__Range(...);
    /// **Repeat For Each in List**<br/>
    /// Repeats code once for each
    /// index of a list.
    ///
    /// ## Tags
    /// - Allow List Changes:
    ///   - `True` (Default)
    ///   - `False (copy list)`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Gets the current
    ///   value each iteration
    /// - `df_list` (List):
    ///   List to repeat through
    ///
    pub unsafe fn DF_ACTION__repeat__ForEach(AllowSPECIALSpace_ListSPECIALSpace_Changes : df_string, ...);
    /// **Repeat On Sphere**<br/>
    /// Repeats code once for every
    /// evenly distributed sphere point.
    ///
    /// ## Tags
    /// - Point Locations Inwards:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Gets the current
    ///   sphere location each iteration
    /// - `df_location` (Location):
    ///   Sphere center
    /// - `df_number` (Number):
    ///   Sphere radius
    /// - `df_number` `?` (Number):
    ///   Sphere points
    ///
    pub unsafe fn DF_ACTION__repeat__Sphere(PointSPECIALSpace_LocationsSPECIALSpace_Inwards : df_string, ...);
    /// **Repeat Forever**<br/>
    /// Repeats code indefinitely.
    ///
    /// ## Additional Info
    /// - The Control: Wait block can
    ///   be used for a delay.
    ///
    pub unsafe fn DF_ACTION__repeat__Forever(...);
    /// **Repeat On Range**<br/>
    /// Repeats code once for each
    /// number on a number line.
    ///
    /// ## Arguments
    /// - `*mut df_opaque` `?` (Variable):
    ///   Gets the current
    ///   number each iteration
    /// - `df_number` (Number):
    ///   Start of range
    /// - `df_number` (Number):
    ///   End of range
    /// - `df_number` `?` (Number):
    ///   Step
    ///   - Default = 1
    ///
    /// ## Additional Info
    /// - To iterate in the negative direction,
    ///   you must use a negative step size.
    ///
    pub unsafe fn DF_ACTION__repeat__SPECIALSpace_RangeSPECIALSpace_(...);
    /// **Repeat For Each Dictionary Entry**<br/>
    /// Repeats code once per entry in
    /// a dictionary
    ///
    /// ## Arguments
    /// - `*mut df_opaque` (Variable):
    ///   Gets the current key
    ///   each iteration
    /// - `*mut df_opaque` (Variable):
    ///   Gets the current value
    ///   each iteration
    /// - `df_dict` (Dict):
    ///   Dictionary to
    ///   repeat through
    ///
    pub unsafe fn DF_ACTION__repeat__ForEachEntry(...);
    ///
    /// ## Tags
    /// - Sign Line:
    ///   - `1`
    ///   - `2`
    ///   - `3`
    ///   - `4`
    ///   - `All lines` (Default)
    /// - Check Mode:
    ///   - `Contains` (Default)
    ///   - `Equals`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__SignHasTxt(SignSPECIALSpace_Line : df_string, CheckSPECIALSpace_Mode : df_string, ...);
    /// **Container Has Room For Item**<br/>
    /// Checks if the container at a
    /// location has room for one or
    /// more items to be given.
    ///
    /// ## Tags
    /// - Check Mode:
    ///   - `Has Room for Any Item` (Default)
    ///   - `Has Room for All Items`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Container location
    /// - 
    /// - `df_item` (Item):
    ///   Item(s) to check with
    /// - OR
    /// - None:
    ///   Checks for empty slot
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__HasRoomForItem(CheckSPECIALSpace_Mode : df_string, ...);
    /// **Event Block Equals**<br/>
    /// Checks if the block in a block
    /// related event is the given block.
    ///
    /// ## Arguments
    /// - `df_item` `[]` (Block):
    ///   Block(s) to check for
    ///
    /// ## Works With
    /// - `Block Events`
    /// - `Click Events`
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__EventBlockEquals(...);
    /// **Command Equals**<br/>
    /// Checks if the command entered
    /// in the Command Event is equal
    /// to the given string.
    ///
    /// ## Tags
    /// - Check Mode:
    ///   - `Check entire command` (Default)
    ///   - `Check beginning`
    /// - Ignore Case:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   String(s) to check for
    ///
    /// ## Works With
    /// - `Command Event`
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__CommandEquals(CheckSPECIALSpace_Mode : df_string, IgnoreSPECIALSpace_Case : df_string, ...);
    /// **Event Item Equals**<br/>
    /// Checks if the item in a item
    /// related event is the given item.
    ///
    /// ## Tags
    /// - Comparison Mode:
    ///   - `Exactly equals`
    ///   - `Ignore stack size/durability` (Default)
    ///   - `Material only`
    ///
    /// ## Arguments
    /// - `df_item` `[]` (Item):
    ///   Item(s) to check for
    ///
    /// ## Works With
    /// - `Item Events`
    /// - `Right Click Event`
    /// - `Left Click Event`
    /// - `Change Slot Event`
    /// - `Place Block Event`
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__EventItemEquals(ComparisonSPECIALSpace_Mode : df_string, ...);
    /// **Sign Contains Text**<br/>
    /// Checks if the sign at a location
    /// has the given text.
    ///
    /// ## Tags
    /// - Sign Line:
    ///   - `1`
    ///   - `2`
    ///   - `3`
    ///   - `4`
    ///   - `All lines` (Default)
    /// - Sign Side:
    ///   - `Front` (Default)
    ///   - `Back`
    /// - Check Mode:
    ///   - `Contains` (Default)
    ///   - `Equals`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Sign location
    /// - `df_text` `[]` (Styled Text):
    ///   Text to check for
    ///   - Formatting is ignored.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__SPECIALSpace_SignHasTxtSPECIALSpace_(SignSPECIALSpace_Line : df_string, SignSPECIALSpace_Side : df_string, CheckSPECIALSpace_Mode : df_string, ...);
    /// **Attack Is Critical**<br/>
    /// Checks if an event attack
    /// is critical.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__AttackIsCrit(...);
    /// **Container Has Item**<br/>
    /// Checks if the container at a
    /// location has the given item.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Container location
    /// - `df_item` `[]` (Item):
    ///   Item(s) to check for
    ///
    /// ## Additional Info
    /// - If multiple items are in the
    ///   chest, the container only
    ///   needs to have one of them.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__ContainerHas(...);
    /// **Block Equals**<br/>
    /// Checks if the block at a location
    /// is the given block.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Check location
    /// - `df_item` `[]?` (Block):
    ///   Block(s) to check for
    /// - `df_string` `[]?` (Block Tag):
    ///   Block data
    ///   - Example: "facing=up"
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__BlockEquals(...);
    /// **Location Is In Block**<br/>
    /// Checks if a location collides with
    /// the hitbox of the nearest block.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Check location
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__InBlock(...);
    /// **Event Movement Key Equals**<br/>
    /// Checks if specific movement keys
    /// changed state in the current event.
    ///
    /// ## Tags
    /// - Movement Key:
    ///   - `Forward (W)` (Default)
    ///   - `Backward (S)`
    ///   - `Left (A)`
    ///   - `Right (D)`
    ///   - `Jump (Space)`
    ///   - `Sneak (Left Shift)`
    ///   - `Sprint (Left Control)`
    /// - Action:
    ///   - `Just pressed` (Default)
    ///   - `Just released`
    ///
    /// ## Works With
    /// - `Movement Key Change Event`
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__MovementKey(MovementSPECIALSpace_Key : df_string, Action : df_string, ...);
    /// **Block Is Powered**<br/>
    /// Checks if the block at a location
    /// is powered by redstone.
    ///
    /// ## Tags
    /// - Redstone Power Mode:
    ///   - `Direct power` (Default)
    ///   - `Indirect power`
    ///
    /// ## Arguments
    /// - `df_location` `[]` (Location):
    ///   Check location(s)
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__BlockPowered(RedstoneSPECIALSpace_PowerSPECIALSpace_Mode : df_string, ...);
    /// **Game Has Player**<br/>
    /// Checks if there is currently
    /// a player in the game with the
    /// given name or UUID.
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   Name or UUID
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__HasPlayer(...);
    /// **Container Has All Items**<br/>
    /// Checks if the container at a
    /// location has all of the given
    /// items.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Container location
    /// - `df_item` `[]` (Item):
    ///   Item(s) to check for
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__ContainerHasAll(...);
    /// **Command Argument Equals**<br/>
    /// Checks if a part of the command
    /// entered in the Command Event
    /// is equal to the given string.
    ///
    /// ## Tags
    /// - Ignore Case:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   String(s) to check for
    /// - `df_number` (Number):
    ///   Argument number
    ///
    /// ## Works With
    /// - `Command Event`
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__CmdArgEquals(IgnoreSPECIALSpace_Case : df_string, ...);
    /// **Event Is Cancelled**<br/>
    /// Checks if the current
    /// event is cancelled.
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__EventCancelled(...);
    /// **Is Chunk Loaded**<br/>
    /// Checks if the chunk at a location
    /// is currently loaded.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Location in chunk
    ///
    pub unsafe fn DF_ACTION__ifSPECIALSpace_game__IsChunkLoaded(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__LastMob(...);
    /// **Select Random Player**<br/>
    /// Creates a selection using
    /// one or more random
    /// players in the game.
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Selection size
    ///   - Default = 1
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__RandomPlayer(...);
    /// **Select Last-Spawned Entity**<br/>
    /// Creates a selection using
    /// the most recently spawned
    /// entity.
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__LastEntity(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__Shooter(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__AllMobs(...);
    /// **Select Entities by Name**<br/>
    /// Creates a selection using all
    /// entities in the game whose
    /// name or UUID matches.
    ///
    /// ## Tags
    /// - Ignore Formatting:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   UUID to check for
    /// - OR
    /// - `df_text` `[]` (Styled Text):
    ///   Name to check for
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__EntityName(IgnoreSPECIALSpace_Formatting : df_string, ...);
    /// **Filter Selection Randomly**<br/>
    /// Filters the selection by
    /// randomly picking one or
    /// more objects from it.
    ///
    /// ## Arguments
    /// - `df_number` `?` (Number):
    ///   Selection size
    ///   - Default = 1
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__FilterRandom(...);
    /// **Select Entities by UUID**<br/>
    /// Creates a selection of all
    /// entities in the game with
    /// the given UUIDs.
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   UUID(s) to check for
    ///
    /// ## Additional Info
    /// - This action selects entities
    ///   directly and is much faster
    ///   than normal selections.
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__EntityUUID(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__DefaultEntity(...);
    /// **Select Players by Name**<br/>
    /// Creates a selection using all
    /// players in the game whose
    /// name or UUID matches.
    ///
    /// ## Arguments
    /// - `df_string` `[]` (String):
    ///   Name or UUID
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__PlayerName(...);
    /// **Select All Entities**<br/>
    /// Creates a selection of
    /// all entities in the game.
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__AllEntities(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__Damager(...);
    /// **Filter Selection by Distance**<br/>
    /// Filters the selection to the
    /// objects that are nearest
    /// or farthest to a location.
    ///
    /// ## Tags
    /// - Ignore Y-Axis:
    ///   - `True`
    ///   - `False` (Default)
    /// - Compare Mode:
    ///   - `Nearest` (Default)
    ///   - `Farthest`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Location to
    ///   compare to
    /// - `df_number` `?` (Number):
    ///   Selection size
    ///   - Default = 1
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__FilterDistance(IgnoreSPECIALSpace_YSPECIALHyphen_Axis : df_string, CompareSPECIALSpace_Mode : df_string, ...);
    /// **Filter Selection by Raycast**<br/>
    /// Filters the selected objects
    /// to the objects that intersect
    /// a ray.
    ///
    /// ## Tags
    /// - Block Collision:
    ///   - `All blocks`
    ///   - `Non-fluid blocks`
    ///   - `Solid blocks` (Default)
    ///   - `None`
    ///
    /// ## Arguments
    /// - `*mut df_opaque` `?` (Variable):
    ///   Gets the end or
    ///   final hit location
    /// - `df_location` (Location):
    ///   Ray origin
    /// - `df_number` (Number):
    ///   Ray distance
    /// - `df_number` `?` (Number):
    ///   Ray width
    ///   - Default = 0.0
    /// - `df_number` `?` (Number):
    ///   Selection size
    ///   - Default = 1
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__FilterRay(BlockSPECIALSpace_Collision : df_string, ...);
    /// **Reset Selection**<br/>
    /// Deactivates the selection.
    /// Code that follows will run
    /// normally with event targets.
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__Reset(...);
    /// **Select Event Target**<br/>
    /// Creates a selection using
    /// a target involved in this
    /// Event.
    ///
    /// ## Tags
    /// - Event Target:
    ///   - `Default` (Default)
    ///   - `Killer`
    ///   - `Damager`
    ///   - `Victim`
    ///   - `Shooter`
    ///   - `Projectile`
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__EventTarget(EventSPECIALSpace_Target : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__Killer(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__Victim(...);
    /// **Select Entities by Condition**<br/>
    /// Creates a selection of
    /// all entities in the game
    /// that meet a condition.
    ///
    /// ## Additional Info
    /// - Can be inverted with the
    ///   NOT Arrow.
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__EntitiesCond(...);
    /// **Select All Players**<br/>
    /// Creates a selection of
    /// all players in the game.
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__AllPlayers(...);
    /// **Invert Selection**<br/>
    /// Creates a new selection by
    /// inverting the selection that
    /// is currently active.
    ///
    /// ## Additional Info
    /// - If the current selection
    ///   contains players, all other
    ///   players are selected.
    /// - If the current selection
    ///   contains entities, all other
    ///   entities are selected.
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__Invert(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__RandomEntity(...);
    /// **Filter Selection by Condition**<br/>
    /// Filters the selection to the
    /// objects that meet a certain
    /// condition.
    ///
    /// ## Additional Info
    /// - Can be inverted with the
    ///   NOT Arrow.
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__FilterCondition(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__MobsCond(...);
    /// **Filter Selection by Sort**<br/>
    /// Filters the selection by
    /// sorting the value of each
    /// object in order.
    ///
    /// ## Tags
    /// - Sort Order:
    ///   - `Ascending` (Default)
    ///   - `Descending`
    ///
    /// ## Arguments
    /// - `df_opaque` (Any):
    ///   Value to
    ///   compare
    /// - `df_number` `?` (Number):
    ///   Selection size
    ///   - Default = 1
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__FilterSort(SortSPECIALSpace_Order : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__Projectile(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__DefaultPlayer(...);
    /// **Select Players by Condition**<br/>
    /// Creates a selection of
    /// all players in the game
    /// that meet a condition.
    ///
    /// ## Additional Info
    /// - Can be inverted with the
    ///   NOT Arrow.
    ///
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__PlayersCond(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__selectSPECIALSpace_object__MobName(...);
    /// **Fill Container**<br/>
    /// Fills the container at a location
    /// with items.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Container location
    /// - `df_item` `[]` (Item):
    ///   Item(s) to fill with
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__FillContainer(...);
    /// **Break Block**<br/>
    /// Breaks the block at a location
    /// as if it was broken by a player.
    ///
    /// ## Arguments
    /// - `df_location` `[]` (Location):
    ///   Block(s) to break
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__BreakBlock(...);
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__LSPECIALSpace_PFXSPECIALSpace_Spiral(...);
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ParticleSphere(...);
    /// **Change Sign Text**<br/>
    /// Changes a line of text
    /// on a sign.
    ///
    /// ## Tags
    /// - Sign Side:
    ///   - `Front` (Default)
    ///   - `Back`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Sign location
    /// - `df_number` (Number):
    ///   Line number
    /// - `df_text` `?` (Styled Text):
    ///   New text
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ChangeSign(SignSPECIALSpace_Side : df_string, ...);
    /// **Send Web Request**<br/>
    /// Sends a web request to a URL.
    ///
    /// ## Tags
    /// - Request Method:
    ///   - `Post` (Default)
    ///   - `Get`
    ///   - `Put`
    ///   - `Delete`
    /// - Content Type:
    ///   - `text/plain` (Default)
    ///   - `application/json`
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   URL to request
    /// - `df_string` `?` (String):
    ///   Content body
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__WebRequest(RequestSPECIALSpace_Method : df_string, ContentSPECIALSpace_Type : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ClearScBoard(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__HideSidebar(...);
    /// **Set Event Death Message**<br/>
    /// Sets the death message in
    /// this event.
    ///
    /// ## Arguments
    /// - `df_text` (Styled Text):
    ///   New death message
    ///
    /// ## Works With
    /// - `Player Death Event`
    /// - `Player Kill Player Event`
    /// - `Mob Kill Player Event`
    ///
    /// ## Additional Info
    /// - Also sets the message that
    ///   appears in the death screen.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetEventDeathMsg(...);
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnItemDisplay(...);
    /// **Write Transaction**<br/>
    /// Adds blocks to the next transaction; a method
    /// of queuing up block operations so that
    /// they can be sent simultaneously.
    ///
    /// ## Arguments
    /// - `df_item` (Block):
    ///   Block to set
    /// - `df_location` (Location):
    ///   Corner 1
    /// - `df_location` (Location):
    ///   Corner 2
    /// - `df_string` `?` (Block Tag):
    ///   Block data
    ///   (comma separated)
    ///   - Example: "facing=up,half=top"
    ///
    /// ## Additional Info
    /// - Requires the Game Action: Apply Transaction
    ///   in order to apply changes to the world.
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__WriteTransaction(...);
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ParticleSpiral(...);
    /// **Set Block Data**<br/>
    /// Sets a data tag value of
    /// the block at a location.
    ///
    /// ## Tags
    /// - Overwrite Existing Data:
    ///   - `True`
    ///   - `False` (Default)
    ///
    /// ## Arguments
    /// - `df_location` `[]` (Location):
    ///   Location
    /// - `df_string` `[]` (Block Tag):
    ///   Block data
    ///   - Example: "facing=up"
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetBlockData(OverwriteSPECIALSpace_ExistingSPECIALSpace_Data : df_string, ...);
    /// **Spawn Firework**<br/>
    /// Launches a firework
    /// rocket at a location.
    ///
    /// ## Tags
    /// - Instant:
    ///   - `True`
    ///   - `False` (Default)
    /// - Movement:
    ///   - `Upwards` (Default)
    ///   - `Directional`
    ///
    /// ## Arguments
    /// - `df_item` (Item):
    ///   Firework rocket
    /// - `df_location` (Location):
    ///   Spawn location
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__Firework(Instant : df_string, Movement : df_string, ...);
    /// **Set Event Damage**<br/>
    /// Sets the damage dealt in
    /// this event.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   New damage amount
    ///   - ❤ = 2 Health
    ///
    /// ## Works With
    /// - `Damage Events`
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetEventDamage(...);
    /// **Spawn Item**<br/>
    /// Spawns an item at a location.
    ///
    /// ## Tags
    /// - Apply Item Motion:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_item` `[]` (Item):
    ///   Item(s) to spawn
    /// - `df_location` (Location):
    ///   Spawn location
    /// - `df_text` `?` (Styled Text):
    ///   Custom name
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnItem(ApplySPECIALSpace_ItemSPECIALSpace_Motion : df_string, ...);
    /// **Set Sign Text Color**<br/>
    /// Changes the text color
    /// of a sign.
    ///
    /// ## Tags
    /// - Sign Side:
    ///   - `Front` (Default)
    ///   - `Back`
    /// - Text Color:
    ///   - `White`
    ///   - `Orange`
    ///   - `Magenta`
    ///   - `Light blue`
    ///   - `Yellow`
    ///   - `Lime`
    ///   - `Pink`
    ///   - `Gray`
    ///   - `Light gray`
    ///   - `Cyan`
    ///   - `Purple`
    ///   - `Blue`
    ///   - `Brown`
    ///   - `Green`
    ///   - `Red`
    ///   - `Black` (Default)
    /// - Glowing:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Sign location
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SignColor(SignSPECIALSpace_Side : df_string, TextSPECIALSpace_Color : df_string, Glowing : df_string, ...);
    /// **Spawn Shulker Bullet**<br/>
    /// Spawns a shulker bullet at a location.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Spawn Location
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ShulkerBullet(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__FireworkEffect(...);
    /// **Set Container Contents**<br/>
    /// Sets the contents of the container
    /// at a location.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Container location
    /// - `df_item` `[]` (Item):
    ///   Item(s) to set
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetContainer(...);
    /// **Spawn Interaction Entity**<br/>
    /// Spawns an invisible hitbox
    /// with the specified size.
    ///
    /// ## Tags
    /// - Responsive:
    ///   - `Enable`
    ///   - `Disable` (Default)
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Spawn location
    /// - `df_number` `?` (Number):
    ///   Hitbox width
    /// - `df_number` `?` (Number):
    ///   Hitbox height
    ///
    /// ## Additional Info
    /// - Detect interactions with:
    ///   ⏵ Player Damage Entity Event
    ///   ⏵ Player Right Click Entity Event
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnInteraction(Responsive : df_string, ...);
    /// **Set Item in Container Slot**<br/>
    /// Sets the item in a slot of the
    /// container at a location.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Container location
    /// - `df_item` `?` (Item):
    ///   Item to set
    /// - `df_number` (Number):
    ///   Slot
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetItemInSlot(...);
    /// **Clone Region**<br/>
    /// Copies a region of blocks to another
    /// region, including air.
    ///
    /// ## Tags
    /// - Ignore Air:
    ///   - `True`
    ///   - `False` (Default)
    /// - Clone Block Entities:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Corner 1
    /// - `df_location` (Location):
    ///   Corner 2
    /// - `df_location` (Location):
    ///   Position to copy from
    /// - `df_location` (Location):
    ///   Position to paste to
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__CloneRegion(IgnoreSPECIALSpace_Air : df_string, CloneSPECIALSpace_BlockSPECIALSpace_Entities : df_string, ...);
    /// **Uncancel Event**<br/>
    /// Uncancels the initial event that
    /// triggered this line of code.
    ///
    /// ## Additional Info
    /// - Events cannot be uncancelled
    ///   after a Control: Wait block.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__UncancelEvent(...);
    /// **Set Lectern Book**<br/>
    /// Sets the book and the
    /// displayed page of a Lectern.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Lectern location
    /// - `df_item` `?` (Item):
    ///   Book to put
    /// - `df_number` `?` (Number):
    ///   Displayed page
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetLecternBook(...);
    /// **Spawn Armor Stand**<br/>
    /// Spawns an armor stand at a
    /// location.
    ///
    /// ## Tags
    /// - Visibility:
    ///   - `Visible` (Default)
    ///   - `Visible (No hitbox)`
    ///   - `Invisible`
    ///   - `Invisible (No hitbox)`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Spawn location
    /// - `df_text` `?` (Styled Text):
    ///   Custom name
    /// - `df_item` `[]?` (Item):
    ///   Equipment
    ///
    /// ## Additional Info
    /// - Options to set the pose and tags
    ///   are in Entity Action » Appearance.
    /// - Equipment goes from the bottom
    ///   left corner towards the right:
    ///   
    ///   Helmet, Chestplate, Leggings,
    ///   Boots, Right Hand, Left Hand
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnArmorStand(Visibility : df_string, ...);
    /// **Spawn Block Display**<br/>
    /// Spawns a block display entity.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Spawn location
    /// - `df_item` (Block):
    ///   Displayed block
    /// - `df_string` `[]?` (Block Tag):
    ///   Block data
    ///   - Example: "facing=up", "half=top"
    ///
    /// ## Additional Info
    /// - Notable defaults:
    ///   ⏵ Billboard = Fixed
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnBlockDisp(...);
    /// **Clear Container**<br/>
    /// Empties a container at a location.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Container location
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ClearContainer(...);
    /// **Cancel Event**<br/>
    /// Cancels the initial event that
    /// triggered this line of code.
    ///
    /// ## Additional Info
    /// - Events cannot be cancelled
    ///   after a Control: Wait block.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__CancelEvent(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ParticleEffect(...);
    /// **Spawn Evoker Fangs**<br/>
    /// Spawns evoker fangs at a
    /// location.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Spawn location
    /// - `df_text` `?` (Styled Text):
    ///   Custom name
    ///
    /// ## Additional Info
    /// - Evoker Fangs deal damage
    ///   and remove themselves a
    ///   second later.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnFangs(...);
    /// **Set Event Sound**<br/>
    /// Sets the sound to play for
    /// this event, replacing the
    /// original sound.
    ///
    /// ## Arguments
    /// - `df_sound` (Sound):
    ///   New sound
    ///
    /// ## Works With
    /// - `Entity Death Events`
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetEventSound(...);
    /// **Set Event Experience**<br/>
    /// Sets the amount of experience
    /// this event should drop.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   Experience
    ///
    /// ## Works With
    /// - `Entity Death Events`
    /// - `Fish Event`
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetEventXP(...);
    /// **Set Container Lock**<br/>
    /// Sets the lock key of the container
    /// at a location.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Container location
    /// - 
    /// - `df_string` (String):
    ///   Lock key
    /// - OR
    /// - None:
    ///   Unlocks the container
    ///
    /// ## Additional Info
    /// - Lock keys determine the name of
    ///   the item used to open the container.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__LockContainer(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__RemoveScore(...);
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__CreateHologram(...);
    /// **Set Event Exhaustion**<br/>
    /// Sets the exhaustion 
    /// gained in this event.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   New exhaustion amount
    ///
    /// ## Works With
    /// - `Exhaustion Events`
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetExhaustion(...);
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ParticleCircle(...);
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__PFXSPECIALSpace_LineSPECIALSpace_SPECIALLeftbracket_ASPECIALRightbracket_(...);
    /// **Clear Container Items**<br/>
    /// Removes all of an item from
    /// the container at a location.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Container location
    /// - `df_item` `[]` (Item):
    ///   Item(s) to clear
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ClearItems(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__StartLoop(...);
    /// **Set Furnace Cook Time**<br/>
    /// Sets the amount of ticks it
    /// takes for a furnace block
    /// to cook an item.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Furnace location
    /// - `df_number` (Number):
    ///   Ticks
    ///
    /// ## Works With
    /// - `Furnace`
    /// - `Blast Furnace`
    /// - `Smoker`
    ///
    /// ## Additional Info
    /// - By default, a furnace cooks
    ///   in 200 ticks.
    /// - Fuel duration is unaffected
    ///   by cooking time.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetFurnaceSpeed(...);
    /// **Enable Block Drops**<br/>
    /// Enables blocks dropping
    /// as items when broken.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__BlockDropsOn(...);
    /// **Bone Meal Block**<br/>
    /// Applies bone meal to a block.
    ///
    /// ## Tags
    /// - Show Particles:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_location` `[]` (Location):
    ///   Block(s) to bone meal
    /// - `df_number` `?` (Number):
    ///   Number of uses
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    /// - Requires token shop purchase
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__BoneMeal(ShowSPECIALSpace_Particles : df_string, ...);
    ///
    /// ## Restrictions
    /// - Requires **Dev** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_dev"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_dev")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__DebugStackTrace(...);
    /// **Spawn Falling Block**<br/>
    /// Spawns a falling block at a
    /// location.
    ///
    /// ## Tags
    /// - Hurt Hit Entities:
    ///   - `True`
    ///   - `False` (Default)
    /// - Reform on Impact:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Block location
    /// - 
    /// - `df_item` (Block):
    ///   Block material
    /// - `df_string` `[]?` (Block Tag):
    ///   Block data
    /// - OR
    /// - None:
    ///   Converts the block at
    ///   the location to a falling block
    /// - 
    ///
    /// ## Additional Info
    /// - Falling blocks automatically
    ///   disappear after 30 seconds.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__FallingBlock(HurtSPECIALSpace_HitSPECIALSpace_Entities : df_string, ReformSPECIALSpace_onSPECIALSpace_Impact : df_string, ...);
    /// **Send Discord Webhook Message**<br/>
    /// Sends a message to a Discord
    /// webhook.
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Webhook URL
    /// - `df_string` (String):
    ///   Message content
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__DiscordWebhook(...);
    /// **Random Tick Block**<br/>
    /// Causes a block to get "random
    /// ticked", which could cause a
    /// block update.
    ///
    /// ## Arguments
    /// - `df_location` `[]` (Location):
    ///   Block(s) to tick
    /// - `df_number` `?` (Number):
    ///   Number of ticks
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__TickBlock(...);
    /// **Replace Container Items**<br/>
    /// Replaces items in the container
    /// at a location with the given item.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Container location
    /// - `df_item` `[]?` (Item):
    ///   Item(s) to replace
    /// - `df_item` (Item):
    ///   Item to replace with
    /// - `df_number` `?` (Number):
    ///   Amount of items to
    ///   replace
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ReplaceItems(...);
    /// **Set Event Projectile**<br/>
    /// Replaces the projectile fired in
    /// the Shoot Bow Event.
    ///
    /// ## Arguments
    /// - `df_item` `?` (Projectile):
    ///   Projectile to launch
    ///
    /// ## Works With
    /// - `Shoot Bow Event`
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetEventProj(...);
    /// **Create Explosion**<br/>
    /// Creates an explosion at a location.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Explosion
    ///   location
    /// - `df_number` `?` (Number):
    ///   Explosion power (0-4)
    ///   - Default = 4
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__Explosion(...);
    /// **Spawn Mob**<br/>
    /// Spawns a mob at a location.
    ///
    /// ## Arguments
    /// - `df_item` (SpawnEgg):
    ///   Mob type
    /// - `df_location` (Location):
    ///   Spawn location
    /// - `df_number` `?` (Number):
    ///   Health
    /// - `df_text` `?` (Styled Text):
    ///   Custom name
    /// - `df_potion` `[]?` (Potion):
    ///   Effect(s)
    /// - `df_item` `[]?` (Item):
    ///   Equipment
    ///
    /// ## Additional Info
    /// - Equipment goes from the bottom
    ///   left corner towards the right:
    ///   
    ///   Main Hand, Helmet, Chestplate,
    ///   Leggings, Boots, Off Hand
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnMob(...);
    /// **Set Item in Brushable Block**<br/>
    /// Sets the item buried in a
    /// suspicious sand or gravel.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Block location
    /// - `df_item` `?` (Item):
    ///   Item
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetBrushableItem(...);
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ParticleLineA(...);
    /// **Spawn Eye of Ender**<br/>
    /// Spawns an eye of ender at a
    /// location, which (if specified) will
    /// float towards its destination.
    ///
    /// ## Tags
    /// - End of Lifespan:
    ///   - `Drop item`
    ///   - `Shatter`
    ///   - `Random` (Default)
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Location to spawn at
    /// - `df_location` `?` (Location):
    ///   Destination
    /// - `df_number` `?` (Number):
    ///   Lifespan (ticks)
    /// - `df_text` `?` (Styled Text):
    ///   Custom name
    ///
    /// ## Additional Info
    /// - If the destination is further
    ///   than 12 blocks away, the eye's
    ///   path always goes upwards.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnEnderEye(EndSPECIALSpace_ofSPECIALSpace_Lifespan : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ShowSidebar(...);
    /// **Spawn Area of Effect Cloud**<br/>
    /// Spawns a lingering potion cloud
    /// at a location that imbues effects
    /// onto entities who enter it.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Spawn location
    /// - `df_potion` `[]?` (Potion):
    ///   Effect to apply
    /// - `df_text` `?` (Styled Text):
    ///   Custom name
    /// - `df_number` `?` (Number):
    ///   Radius (blocks)
    ///   - Default = 3
    /// - `df_number` `?` (Number):
    ///   Duration (ticks)
    ///   - Default = 200
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnPotionCloud(...);
    /// **Launch Game Projectile**<br/>
    /// Launches a projectile.
    ///
    /// ## Arguments
    /// - `df_item` (Projectile):
    ///   Projectile to launch
    /// - `df_location` (Location):
    ///   Launch point
    /// - `df_text` `?` (Styled Text):
    ///   Custom name
    /// - `df_number` `?` (Number):
    ///   Speed
    /// - `df_number` `?` (Number):
    ///   Inaccuracy
    ///   - Controls how much random
    ///     motion is applied on launch
    ///   - Default = 1
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__LaunchProj(...);
    /// **Set Block**<br/>
    /// Sets the block at a location.
    ///
    /// ## Arguments
    /// - `df_item` (Block):
    ///   Block to set
    /// - `df_location` `[]` (Location):
    ///   Block location(s)
    /// - `df_string` `[]?` (Block Tag):
    ///   Block data
    ///   - Example: "facing=up", "half=top"
    ///
    /// ## Additional Info
    /// - Will cause block updates. Use SetRegion
    ///   to not update nearby blocks and
    ///   save CPU.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SPECIALSpace_SetBlockSPECIALSpace_(...);
    /// **Spawn Item Display**<br/>
    /// Spawns an item display entity.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Spawn location
    /// - `df_item` (Item):
    ///   Displayed item
    ///
    /// ## Additional Info
    /// - Notable defaults:
    ///   ⏵ Billboard = Fixed
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnItemDisp(...);
    /// **Set Block Growth**<br/>
    /// Sets the growth stage of the block
    /// (eg. carrots) at a location.
    ///
    /// ## Tags
    /// - Growth Unit:
    ///   - `Growth Stage Number` (Default)
    ///   - `Growth Percentage`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Block location
    /// - `df_number` `?` (Number):
    ///   Growth stage
    ///   - Default = 0
    ///
    /// ## Additional Info
    /// - Most crops have growth stages
    ///   from 0 to 7.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetBlockGrowth(GrowthSPECIALSpace_Unit : df_string, ...);
    ///
    /// ## Tags
    /// - Delay Unit:
    ///   - `Ticks` (Default)
    ///   - `Seconds`
    ///   - `Minutes`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__Wait(DelaySPECIALSpace_Unit : df_string, ...);
    /// **Set Container Name**<br/>
    /// Sets the name of the container
    /// at a location.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Container location
    /// - `df_text` (Styled Text):
    ///   Name
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetContainerName(...);
    /// **Set Player Head**<br/>
    /// Sets the block at a location
    /// to a player head.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Head location
    /// - 
    /// - `df_item` (Item):
    ///   Player Head
    /// - OR
    /// - `df_string` (String):
    ///   Head owner
    ///   - Player name or UUID
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetHead(...);
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__RemoveHologram(...);
    /// **Remove Container Items**<br/>
    /// Removes items from the container
    /// at a location.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Container location
    /// - `df_item` `[]` (Item):
    ///   Item(s) to remove
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__RemoveItems(...);
    ///
    /// ## Tags
    /// - Apply Item Motion:
    ///   - `True` (Default)
    ///   - `False`
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnRngItem(ApplySPECIALSpace_ItemSPECIALSpace_Motion : df_string, ...);
    /// **Set Region**<br/>
    /// Fills a region with a type of block.
    ///
    /// ## Arguments
    /// - `df_item` (Block):
    ///   Block to set
    /// - `df_location` (Location):
    ///   Corner 1
    /// - `df_location` (Location):
    ///   Corner 2
    /// - `df_string` `?` (Block Tag):
    ///   Block data
    ///   (comma separated)
    ///   - Example: "facing=up,half=top"
    ///
    /// ## Additional Info
    /// - Can set up to 100,000 blocks per
    ///   action.
    /// - Will not cause block updates. A 1x1x1
    ///   SetRegion may save CPU compared to
    ///   SetBlock in certain situations.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetRegion(...);
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ParticleCircleA(...);
    /// **Spawn Primed TNT**<br/>
    /// Spawns primed TNT at a location.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Spawn location
    /// - `df_number` `?` (Number):
    ///   TNT power (0-4)
    ///   - Default = 4
    /// - `df_number` `?` (Number):
    ///   Fuse duration
    ///   - Default = 80
    /// - `df_text` `?` (Styled Text):
    ///   Custom name
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnTNT(...);
    /// **Spawn Experience Orb**<br/>
    /// Spawns an experience orb at
    /// a location.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Spawn location
    /// - `df_number` `?` (Number):
    ///   Experience amount
    /// - `df_text` `?` (Styled Text):
    ///   Custom name
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnExpOrb(...);
    /// **Set Biome**<br/>
    /// Sets the biome of a region.
    ///
    /// ## Arguments
    /// - `df_string` (String):
    ///   Biome to set
    ///   - Example: "basalt_deltas", "plains"
    /// - `df_location` (Location):
    ///   Corner 1
    /// - `df_location` (Location):
    ///   Corner 2
    ///
    /// ## Additional Info
    /// - Can set up to 100,000 blocks per
    ///   action.
    /// - Players may have to reload the chunks
    ///   on their client to see a change.
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetBiome(...);
    /// **Set Event Heal Amount**<br/>
    /// Sets the amount of health
    /// regained in this event.
    ///
    /// ## Arguments
    /// - `df_number` (Number):
    ///   New healing amount
    ///   - ❤ = 2 Health
    ///
    /// ## Works With
    /// - `Player Heal Event`
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetEventHeal(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__PFXSPECIALSpace_Path(...);
    /// **Apply Transaction**<br/>
    /// Applies the current transaction
    /// and generates a new one.
    ///
    /// ## Additional Info
    /// - Can set up to 1,000,000 blocks
    ///   per transaction.
    /// - Only 1 transaction can be active at a time.
    /// - Transactions are asynchronous, and
    ///   may override blocks set after them.
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ApplyTransaction(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ParticleRay(...);
    /// **Generate Tree**<br/>
    /// Generates a tree at a location.
    ///
    /// ## Tags
    /// - Tree Type:
    ///   - `Oak Tree` (Default)
    ///   - `Big Oak Tree`
    ///   - `Swamp Tree`
    ///   - `Spruce Tree`
    ///   - `Slightly Taller Spruce Tree`
    ///   - `Big Spruce Tree`
    ///   - `Birch Tree`
    ///   - `Tall Birch Tree`
    ///   - `Jungle Tree`
    ///   - `Big Jungle Tree`
    ///   - `Jungle Bush`
    ///   - `Acacia Tree`
    ///   - `Dark Oak Tree`
    ///   - `Mangrove Tree`
    ///   - `Tall Mangrove Tree`
    ///   - `Cherry Tree`
    ///   - `Azalea Tree`
    ///   - `Red Mushroom`
    ///   - `Brown Mushroom`
    ///   - `Crimson Fungus`
    ///   - `Warped Fungus`
    ///   - `Chorus Plant`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Tree location (bottom
    ///   log block)
    ///
    /// ## Additional Info
    /// - The location is the northwest
    ///   corner of 2x2 trees.
    /// - Trees will not grow on blocks
    ///   that do not normally support
    ///   tree growth.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__GenerateTree(TreeSPECIALSpace_Type : df_string, ...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__StopLoop(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetScObj(...);
    /// **Spawn End Crystal**<br/>
    /// Spawns an end crystal at a
    /// location.
    ///
    /// ## Tags
    /// - Show Bottom:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Spawn location
    /// - `df_text` `?` (Styled Text):
    ///   Custom name
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnCrystal(ShowSPECIALSpace_Bottom : df_string, ...);
    /// **Set Campfire Item**<br/>
    /// Sets the item being cooked in
    /// one of a campfire's slots.
    ///
    /// ## Tags
    /// - Campfire Slot:
    ///   - `1` (Default)
    ///   - `2`
    ///   - `3`
    ///   - `4`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Campfire location
    /// - `df_item` (Item):
    ///   Campfire item
    /// - `df_number` `?` (Number):
    ///   Cooking time (ticks)
    ///   - Default = 600
    ///
    /// ## Additional Info
    /// - After the cooking time, the
    ///   item drops from the campfire
    ///   (unless the campfire is unlit).
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetCampfireItem(CampfireSPECIALSpace_Slot : df_string, ...);
    /// **Spawn Text Display**<br/>
    /// Spawns a text display entity.
    ///
    /// ## Tags
    /// - Text Value Merging:
    ///   - `Add spaces`
    ///   - `No spaces` (Default)
    /// - Inherit Styles:
    ///   - `True` (Default)
    ///   - `False`
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Spawn location
    /// - `df_text` `[]` (Styled Text):
    ///   Displayed text
    ///
    /// ## Additional Info
    /// - Notable defaults:
    ///   ⏵ Line Width = 200
    ///   ⏵ Text Alignment = Center
    ///   ⏵ Billboard = Center
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnTextDisplay(TextSPECIALSpace_ValueSPECIALSpace_Merging : df_string, InheritSPECIALSpace_Styles : df_string, ...);
    /// **Spawn Vehicle**<br/>
    /// Spawns a vehicle at a location.
    ///
    /// ## Arguments
    /// - `df_item` (Vehicle):
    ///   Vehicle type
    /// - `df_location` (Location):
    ///   Spawn location
    /// - `df_text` `?` (Styled Text):
    ///   Custom name
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SpawnVehicle(...);
    /// **Summon Lightning**<br/>
    /// Strikes lightning at a location.
    ///
    /// ## Arguments
    /// - `df_location` (Location):
    ///   Impact location
    ///
    /// ## Additional Info
    /// - The lightning's damage can be
    ///   detected using the "lightning"
    ///   damage event cause.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__Lightning(...);
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ParticleSpiralA(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__SetScore(...);
    ///
    /// ## Restrictions
    /// - Requires **Noble** rank
    ///
    #[deprecated]
    #[cfg(any(doc, feature = "rank_noble"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_noble")))]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ParticleCluster(...);
    /// **Disable Block Drops**<br/>
    /// Disables blocks dropping
    /// as items when broken.
    ///
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__BlockDropsOff(...);
    ///
    #[deprecated]
    pub unsafe fn DF_ACTION__gameSPECIALSpace_action__ParticleLine(...);
}
