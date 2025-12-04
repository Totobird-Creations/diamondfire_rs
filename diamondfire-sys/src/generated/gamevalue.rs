use crate::*;
unsafe extern "C" {
    /// **Current Health**<br/>
    /// Gets a target's remaining
    /// health points.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0.0 (dead) up
    /// to the target's maximum
    /// health (20.0 by default)
    ///
    /// ## Additional Info
    /// - ❤ = 2 Health
    ///
    pub safe fn DF_GAMEVALUE__CurrentSPECIALSpace_Health(target : df_string) -> df_number;
    /// **Maximum Health**<br/>
    /// Gets a target's maximum
    /// health points.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Maximum
    /// health, 1.0 or above
    ///
    /// ## Additional Info
    /// - ❤ = 2 Health
    ///
    pub safe fn DF_GAMEVALUE__MaximumSPECIALSpace_Health(target : df_string) -> df_number;
    /// **Absorption Health**<br/>
    /// Gets a target's
    /// absorption health
    /// (golden hearts).
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Absorption
    /// health
    ///
    /// ## Additional Info
    /// - ❤ = 2 Health
    ///
    pub safe fn DF_GAMEVALUE__AbsorptionSPECIALSpace_Health(target : df_string) -> df_number;
    /// **Food Level**<br/>
    /// Gets a target's remaining
    /// food points.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0 (starving)
    /// to 20 (full bar)
    ///
    pub safe fn DF_GAMEVALUE__FoodSPECIALSpace_Level(target : df_string) -> df_number;
    /// **Food Saturation**<br/>
    /// Gets a target's saturation level,
    /// which depends on the types
    /// of food consumed.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0.0 (minimum), up to
    /// the player's food level
    ///
    /// ## Additional Info
    /// - If saturation is \> 0.0, the
    ///   player's food level will not drop.
    ///
    pub safe fn DF_GAMEVALUE__FoodSPECIALSpace_Saturation(target : df_string) -> df_number;
    /// **Food Exhaustion**<br/>
    /// Gets a target's exhaustion
    /// level, which is increased
    /// by the player's actions.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0.0 (minimum) to
    /// 4.0 (reset point)
    ///
    /// ## Additional Info
    /// - When exhaustion resets
    ///   from 4.0 to 0.0, a player's
    ///   saturation decreases by 1.
    ///
    pub safe fn DF_GAMEVALUE__FoodSPECIALSpace_Exhaustion(target : df_string) -> df_number;
    /// **Attack Damage**<br/>
    /// Gets a target's attack damage,
    /// which has a base value that
    /// can be altered by items.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0.0 or higher
    /// (more damage)
    ///
    /// ## Additional Info
    /// - Default base value = 1.0
    ///
    pub safe fn DF_GAMEVALUE__AttackSPECIALSpace_Damage(target : df_string) -> df_number;
    /// **Attack Speed**<br/>
    /// Gets a target's attack speed,
    /// which has a base value that
    /// can be altered by items.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0.0 or higher
    /// (faster)
    ///
    /// ## Additional Info
    /// - Default base value = 4.0
    ///
    pub safe fn DF_GAMEVALUE__AttackSPECIALSpace_Speed(target : df_string) -> df_number;
    /// **Attack Cooldown**<br/>
    /// Gets a target's current attack
    /// cooldown as a percentage of the
    /// way to fully charged
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0.0 (fully uncharged)
    /// to 100.0 (fully charged)
    ///
    pub safe fn DF_GAMEVALUE__AttackSPECIALSpace_Cooldown(target : df_string) -> df_number;
    /// **Attack Cooldown Ticks**<br/>
    /// Gets the number of ticks
    /// it will take to fully charge a
    /// target's attack cooldown after
    /// attacking with their held item.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0.0 (instant)
    /// or above
    ///
    /// ## Additional Info
    /// - This value is set to -1
    ///   if it will never recharge.
    ///
    pub safe fn DF_GAMEVALUE__AttackSPECIALSpace_CooldownSPECIALSpace_Ticks(target : df_string) -> df_number;
    /// **Armor Points**<br/>
    /// Gets a target's armor points,
    /// which has a base value that
    /// can be altered by items.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0.0 (no armor)
    /// to 20.0 (full bar)
    ///
    /// ## Additional Info
    /// - Default base value = 0.0
    ///
    pub safe fn DF_GAMEVALUE__ArmorSPECIALSpace_Points(target : df_string) -> df_number;
    /// **Armor Toughness**<br/>
    /// Gets a target's armor toughness,
    /// which has a base value that
    /// can be altered by items.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0.0 or above (full
    /// set of diamond armor = 8.0)
    ///
    /// ## Additional Info
    /// - Armor Toughness increases 
    ///   the amount of damage required
    ///   to penetrate one armor point.
    /// - Default base value = 0.0
    ///
    pub safe fn DF_GAMEVALUE__ArmorSPECIALSpace_Toughness(target : df_string) -> df_number;
    /// **Invulnerability Ticks**<br/>
    /// Gets a target's remaining
    /// ticks of invulnerability.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0 (can be hurt)
    /// or above (invulnerable)
    ///
    /// ## Additional Info
    /// - This value is set to 10
    ///   upon taking damage.
    ///
    pub safe fn DF_GAMEVALUE__InvulnerabilitySPECIALSpace_Ticks(target : df_string) -> df_number;
    /// **Experience Level**<br/>
    /// Gets a target's experience
    /// level.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0 (no levels)
    /// or above
    ///
    pub safe fn DF_GAMEVALUE__ExperienceSPECIALSpace_Level(target : df_string) -> df_number;
    /// **Experience Progress**<br/>
    /// Gets a target's experience
    /// progress percentage to
    /// the next level.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0.0% (no progress)
    /// to 100.0% (next level)
    ///
    pub safe fn DF_GAMEVALUE__ExperienceSPECIALSpace_Progress(target : df_string) -> df_number;
    /// **Fire Ticks**<br/>
    /// Gets the remaining ticks a
    /// target is on fire for.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0 (not on fire)
    /// or above (burning)
    ///
    pub safe fn DF_GAMEVALUE__FireSPECIALSpace_Ticks(target : df_string) -> df_number;
    /// **Freeze Ticks**<br/>
    /// Gets the remaining ticks a
    /// target is freezing for.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0 (not frozen)
    /// or above (freezing)
    ///
    pub safe fn DF_GAMEVALUE__FreezeSPECIALSpace_Ticks(target : df_string) -> df_number;
    /// **Remaining Air**<br/>
    /// Gets a target's remaining
    /// air ticks.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0 (drowning)
    /// to 300 (maximum air)
    ///
    /// ## Additional Info
    /// - One breath bubble is
    ///   equal to 30 air ticks.
    ///
    pub safe fn DF_GAMEVALUE__RemainingSPECIALSpace_Air(target : df_string) -> df_number;
    /// **Fall Distance**<br/>
    /// Gets a target's distance fallen
    /// in blocks.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0.0 (not falling)
    /// or higher (falling down)
    ///
    /// ## Additional Info
    /// - Resets to 0 upon landing.
    /// - Works on the Damage Event.
    ///
    pub safe fn DF_GAMEVALUE__FallSPECIALSpace_Distance(target : df_string) -> df_number;
    /// **Held Slot**<br/>
    /// Gets a target's selected
    /// hotbar slot index.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 1 (leftmost slot)
    /// to 9 (rightmost slot)
    ///
    pub safe fn DF_GAMEVALUE__HeldSPECIALSpace_Slot(target : df_string) -> df_number;
    /// **Ping**<br/>
    /// Gets the latency between
    /// a player and the server
    /// in milliseconds.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Player ping
    /// ,,╻╻┃ 0 to 149
    /// ,,╻╻┃ 150 to 299
    /// ,,╻╻┃ 300 to 599
    /// ,,╻╻┃ 600 to 999
    /// ,,╻╻┃ over 1000
    ///
    pub safe fn DF_GAMEVALUE__Ping(target : df_string) -> df_number;
    /// **Steer Sideways Movement**<br/>
    /// While a player is steering
    /// an entity, gets the sideways
    /// movement of the steering.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// -1 (right), 1 (left), or 0 (none)
    ///
    pub safe fn DF_GAMEVALUE__SteerSPECIALSpace_SidewaysSPECIALSpace_Movement(target : df_string) -> df_number;
    /// **Steer Forward Movement**<br/>
    /// While a player is steering
    /// an entity, gets the forward
    /// movement of the steering.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// -1 (backward), 1 (forward), or 0 (none)
    ///
    pub safe fn DF_GAMEVALUE__SteerSPECIALSpace_ForwardSPECIALSpace_Movement(target : df_string) -> df_number;
    /// **Item Usage Progress**<br/>
    /// Gets the progress percentage
    /// of a target using their held
    /// item, such as food.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0 (not using an item),
    /// or 0.0% (start) to 100.0%
    ///
    /// ## Additional Info
    /// - Bows, crossbows and tridents
    ///   remain at 100.0% until they
    ///   are released.
    ///
    pub safe fn DF_GAMEVALUE__ItemSPECIALSpace_UsageSPECIALSpace_Progress(target : df_string) -> df_number;
    /// **Flight Speed**<br/>
    /// Gets a target's flight
    /// speed as a percentage.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Flight speed
    /// percentage (0% to 1000%)
    ///
    pub safe fn DF_GAMEVALUE__FlightSPECIALSpace_Speed(target : df_string) -> df_number;
    /// **Walk Speed**<br/>
    /// Gets a target's walk
    /// speed as a percentage.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Walk speed
    /// percentage (0% to 500%)
    ///
    pub safe fn DF_GAMEVALUE__WalkSPECIALSpace_Speed(target : df_string) -> df_number;
    /// **Entity Width**<br/>
    /// Gets the width of an entity's
    /// bounding box.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Width in blocks
    ///
    pub safe fn DF_GAMEVALUE__EntitySPECIALSpace_Width(target : df_string) -> df_number;
    /// **Entity Height**<br/>
    /// Gets the height of an entity's
    /// bounding box.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Height in blocks
    ///
    pub safe fn DF_GAMEVALUE__EntitySPECIALSpace_Height(target : df_string) -> df_number;
    /// **Location**<br/>
    /// Gets a target's location.
    ///
    /// ## Returns
    /// `df_location` (Location):
    /// Location and
    /// rotation, at feet height
    ///
    pub safe fn DF_GAMEVALUE__Location(target : df_string) -> df_location;
    /// **Target Block Location**<br/>
    /// Gets the location of the
    /// block a target is looking at.
    ///
    /// ## Returns
    /// `df_location` (Location):
    /// Block center
    ///
    pub safe fn DF_GAMEVALUE__TargetSPECIALSpace_BlockSPECIALSpace_Location(target : df_string) -> df_location;
    /// **Target Block Side**<br/>
    /// Gets the side of the
    /// block a target is looking
    /// at as a direction.
    ///
    /// ## Returns
    /// `df_vector` (Vector):
    /// Block side
    ///
    pub safe fn DF_GAMEVALUE__TargetSPECIALSpace_BlockSPECIALSpace_Side(target : df_string) -> df_vector;
    /// **Eye Location**<br/>
    /// Gets a target's location, but
    /// adjusted to its eye height.
    ///
    /// ## Returns
    /// `df_location` (Location):
    /// Eye location
    /// and rotation
    ///
    pub safe fn DF_GAMEVALUE__EyeSPECIALSpace_Location(target : df_string) -> df_location;
    /// **X-Coordinate**<br/>
    /// Gets the X coordinate
    /// of a target's position.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Coordinate
    ///
    pub safe fn DF_GAMEVALUE__XSPECIALHyphen_Coordinate(target : df_string) -> df_number;
    /// **Y-Coordinate**<br/>
    /// Gets the Y coordinate
    /// of a target's position.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Coordinate
    ///
    pub safe fn DF_GAMEVALUE__YSPECIALHyphen_Coordinate(target : df_string) -> df_number;
    /// **Z-Coordinate**<br/>
    /// Gets the Z coordinate
    /// of a target's position.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Coordinate
    ///
    pub safe fn DF_GAMEVALUE__ZSPECIALHyphen_Coordinate(target : df_string) -> df_number;
    /// **Midpoint Location**<br/>
    /// Gets the location of the center of
    /// the target's bounding box.
    ///
    /// ## Returns
    /// `df_location` (Location):
    /// Location and
    /// rotation, at midpoint
    ///
    pub safe fn DF_GAMEVALUE__MidpointSPECIALSpace_Location(target : df_string) -> df_location;
    /// **Pitch**<br/>
    /// Gets the pitch (up/down
    /// rotation) of a target's
    /// position.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// -90.0 to 90.0
    ///
    /// ## Additional Info
    /// - -90.0° = up
    /// - 90.0° = down
    ///
    pub safe fn DF_GAMEVALUE__Pitch(target : df_string) -> df_number;
    /// **Yaw**<br/>
    /// Gets the yaw (left/right
    /// rotation) of a target's
    /// position.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// -180.0 to 180.0
    ///
    /// ## Additional Info
    /// - -180.0° & 180.0° = north
    /// - -90.0° = east
    /// - 0.0° = south
    /// - 90.0° = west
    ///
    pub safe fn DF_GAMEVALUE__Yaw(target : df_string) -> df_number;
    /// **Body Yaw**<br/>
    /// Gets the yaw (left/right
    /// rotation) of a target's
    /// body.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// -180.0 to 180.0
    ///
    /// ## Additional Info
    /// - -180.0° & 180.0° = north
    /// - -90.0° = east
    /// - 0.0° = south
    /// - 90.0° = west
    ///
    pub safe fn DF_GAMEVALUE__BodySPECIALSpace_Yaw(target : df_string) -> df_number;
    /// **Standing Block Location**<br/>
    /// Gets the location of the block
    /// that is supporting the player.
    ///
    /// ## Returns
    /// `df_location` (Location):
    /// Block center
    ///
    /// ## Additional Info
    /// - When not grounded, this will
    ///   return the location of the block
    ///   at the target's location.
    ///
    pub safe fn DF_GAMEVALUE__StandingSPECIALSpace_BlockSPECIALSpace_Location(target : df_string) -> df_location;
    /// **Spawn Location**<br/>
    /// Gets a target's original spawn
    /// location.
    ///
    /// ## Returns
    /// `df_location` (Location):
    /// Location this
    /// entity was created at
    ///
    pub safe fn DF_GAMEVALUE__SpawnSPECIALSpace_Location(target : df_string) -> df_location;
    /// **Velocity**<br/>
    /// Gets the speed at which a
    /// target is moving (not walking)
    /// in each direction.
    ///
    /// ## Returns
    /// `df_vector` (Vector):
    /// Movement velocity
    ///
    /// ## Additional Info
    /// - When grounded, a target may
    ///   still have a downward velocity
    ///   due to how gravity is applied.
    ///
    pub safe fn DF_GAMEVALUE__Velocity(target : df_string) -> df_vector;
    /// **Direction**<br/>
    /// Gets the looking direction
    /// of a target's location as
    /// a vector.
    ///
    /// ## Returns
    /// `df_vector` (Vector):
    /// Direction vector
    /// (length of 1.0)
    ///
    pub safe fn DF_GAMEVALUE__Direction(target : df_string) -> df_vector;
    /// **Main Hand Item**<br/>
    /// Gets a target's currently
    /// held item.
    ///
    /// ## Returns
    /// `df_item` (Item):
    /// Item in the
    /// selected hotbar slot
    ///
    pub safe fn DF_GAMEVALUE__MainSPECIALSpace_HandSPECIALSpace_Item(target : df_string) -> df_item;
    /// **Off Hand Item**<br/>
    /// Gets a target's currently
    /// held off hand item.
    ///
    /// ## Returns
    /// `df_item` (Item):
    /// Item in the
    /// offhand slot
    ///
    pub safe fn DF_GAMEVALUE__OffSPECIALSpace_HandSPECIALSpace_Item(target : df_string) -> df_item;
    /// **Armor Items**<br/>
    /// Gets the items in a target's
    /// armor slots.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one Item
    /// entry for each armor
    /// slot (air if empty, 4 in
    /// total)
    ///
    /// ## Additional Info
    /// - Armor slots are ordered
    ///   from helmet to boots.
    ///
    pub safe fn DF_GAMEVALUE__ArmorSPECIALSpace_Items(target : df_string) -> df_list;
    /// **Hotbar Items**<br/>
    /// Gets a target's current
    /// hotbar items.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one Item
    /// entry for each hotbar
    /// slot (air if empty, 9 in
    /// total)
    ///
    pub safe fn DF_GAMEVALUE__HotbarSPECIALSpace_Items(target : df_string) -> df_list;
    /// **Inventory Items**<br/>
    /// Gets a target's inventory
    /// items (includes hotbar).
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one Item
    /// entry for each inventory
    /// slot (air if empty, 36 in
    /// total)
    ///
    pub safe fn DF_GAMEVALUE__InventorySPECIALSpace_Items(target : df_string) -> df_list;
    /// **Cursor Item**<br/>
    /// Gets the item on a target's
    /// cursor (used when moving
    /// items in the inventory).
    ///
    /// ## Returns
    /// `df_item` (Item):
    /// Cursor item
    ///
    pub safe fn DF_GAMEVALUE__CursorSPECIALSpace_Item(target : df_string) -> df_item;
    /// **Inventory Menu Items**<br/>
    /// Gets a target's current
    /// inventory menu items.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one Item
    /// entry for each menu
    /// slot (air if empty)
    ///
    /// ## Additional Info
    /// - Works with container
    ///   inventories.
    ///
    pub safe fn DF_GAMEVALUE__InventorySPECIALSpace_MenuSPECIALSpace_Items(target : df_string) -> df_list;
    /// **Saddle Item**<br/>
    /// Gets a target's currently
    /// worn saddle or carpet.
    ///
    /// ## Returns
    /// `df_item` (Item):
    /// Item in the
    /// saddle/decor slot
    ///
    /// ## Works With
    /// - `Horses`
    /// - `Pigs`
    /// - `Llamas (decor)`
    ///
    pub safe fn DF_GAMEVALUE__SaddleSPECIALSpace_Item(target : df_string) -> df_item;
    /// **Entity Item**<br/>
    /// The item form of the
    /// target.
    ///
    /// ## Returns
    /// `df_item` (Item):
    /// The entity item
    ///
    /// ## Works With
    /// - `Dropped Item`
    /// - `Potion`
    /// - `Firework`
    /// - `Arrow`
    /// - `Trident`
    /// - `Eye of Ender`
    /// - `Falling Block`
    ///
    pub safe fn DF_GAMEVALUE__EntitySPECIALSpace_Item(target : df_string) -> df_item;
    /// **Name **<br/>
    /// Gets a target's name.
    ///
    /// ## Returns
    /// `df_text` (Styled Text):
    /// Target name
    ///
    pub safe fn DF_GAMEVALUE__NameSPECIALSpace_(target : df_string) -> df_text;
    /// **UUID**<br/>
    /// Gets a target's universally
    /// unique identifier.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Target UUID
    ///
    pub safe fn DF_GAMEVALUE__UUID(target : df_string) -> df_string;
    /// **Entity Type**<br/>
    /// Gets a target's entity type.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Entity type, e.g.
    /// "tipped_arrow" or "cow"
    ///
    pub safe fn DF_GAMEVALUE__EntitySPECIALSpace_Type(target : df_string) -> df_string;
    /// **Game Mode**<br/>
    /// Gets a player's game mode.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Game mode:
    /// "survival", "creative", "adventure", 
    /// "spectator"
    ///
    pub safe fn DF_GAMEVALUE__GameSPECIALSpace_Mode(target : df_string) -> df_string;
    /// **Open Inventory Title **<br/>
    /// Gets the title of a target's
    /// opened inventory.
    ///
    /// ## Returns
    /// `df_text` (Styled Text):
    /// Inventory title, or
    /// "none" if either no menu
    /// or the player's regular
    /// inventory is open
    ///
    pub safe fn DF_GAMEVALUE__OpenSPECIALSpace_InventorySPECIALSpace_TitleSPECIALSpace_(target : df_string) -> df_text;
    /// **Potion Effects**<br/>
    /// Gets a target's active
    /// potion effects.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one 
    /// Potion Effect entry
    /// for each active
    /// effect on the target
    ///
    pub safe fn DF_GAMEVALUE__PotionSPECIALSpace_Effects(target : df_string) -> df_list;
    /// **Vehicle**<br/>
    /// Gets the UUID of the entity
    /// that the target is riding.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// UUID of the ridden
    /// entity, or "none" if the
    /// target is not riding one
    ///
    /// ## Additional Info
    /// - The ridden entity does not
    ///   need to be of vehicular type.
    /// - In a stack of entities, the
    ///   vehicle is the bottom entity.
    ///
    pub safe fn DF_GAMEVALUE__Vehicle(target : df_string) -> df_string;
    /// **Passengers **<br/>
    /// Gets the UUIDs of any
    /// entities riding a target.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one String
    /// entry (UUID) for each
    /// passenger riding the target
    /// (Empty List if the target
    /// has no passengers)
    ///
    pub safe fn DF_GAMEVALUE__PassengersSPECIALSpace_(target : df_string) -> df_list;
    /// **Lead Holder**<br/>
    /// Gets the entity that is holding
    /// a target on a lead.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Lead holder UUID,
    /// or "none" if the target
    /// is not on a lead
    ///
    pub safe fn DF_GAMEVALUE__LeadSPECIALSpace_Holder(target : df_string) -> df_string;
    /// **Attached Leads**<br/>
    /// Gets all entities attached to
    /// to a target by a lead.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one String
    /// entry (UUID) for each
    /// leashed entity
    /// (Empty List if the target
    /// holds no leads)
    ///
    pub safe fn DF_GAMEVALUE__AttachedSPECIALSpace_Leads(target : df_string) -> df_list;
    /// **Targeted Entity UUID**<br/>
    /// Gets the UUID of the entity
    /// that the target is targeting.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// UUID of the targeted
    /// entity, or "none" if the
    /// target is not chasing one
    ///
    /// ## Additional Info
    /// - The targeted entity is the
    ///   entity the target is chasing.
    ///
    pub safe fn DF_GAMEVALUE__TargetedSPECIALSpace_EntitySPECIALSpace_UUID(target : df_string) -> df_string;
    /// **Projectile Shooter UUID**<br/>
    /// Gets the UUID of a projectile's
    /// shooter, or "none" if not set.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Shooter UUID
    ///
    pub safe fn DF_GAMEVALUE__ProjectileSPECIALSpace_ShooterSPECIALSpace_UUID(target : df_string) -> df_string;
    /// **Display Entity Translation**<br/>
    /// Gets the translation of a
    /// display entity's transformation.
    ///
    /// ## Returns
    /// `df_vector` (Vector):
    /// Translation
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub safe fn DF_GAMEVALUE__DisplaySPECIALSpace_EntitySPECIALSpace_Translation(target : df_string) -> df_vector;
    /// **Display Entity Scale**<br/>
    /// Gets the scale of a display
    /// entity's transformation.
    ///
    /// ## Returns
    /// `df_vector` (Vector):
    /// Scale
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub safe fn DF_GAMEVALUE__DisplaySPECIALSpace_EntitySPECIALSpace_Scale(target : df_string) -> df_vector;
    /// **Pose**<br/>
    /// Gets the target's pose.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Pose:
    /// "dying" (dead), 
    /// "fall_flying" (gliding), 
    /// "sleeping", "sneaking", 
    /// "spin_attack" (riptiding with a trident), 
    /// "standing" (standing normally), 
    /// "swimming" (swimming/crawling)
    /// Camel Only:
    /// "sitting"
    /// Frog Only:
    /// "croaking", "long_jumping", 
    /// "using_tongue"
    /// Warden Only:
    /// "digging", "emerging", "roaring", "sniffing"
    /// Breeze Only:
    /// "inhaling", "shooting", "sliding"
    ///
    pub safe fn DF_GAMEVALUE__Pose(target : df_string) -> df_string;
    /// **Weather Type**<br/>
    /// Gets a player's weather type.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Weather: 
    /// "clear", "downfall"
    ///
    pub safe fn DF_GAMEVALUE__WeatherSPECIALSpace_Type(target : df_string) -> df_string;
    /// **Pick Entity Result**<br/>
    /// Gets the item that would be created
    /// when a player middle clicks
    /// the target in Creative mode.
    ///
    /// ## Returns
    /// `df_item` (SpawnEgg):
    /// Item or air
    ///
    pub safe fn DF_GAMEVALUE__PickSPECIALSpace_EntitySPECIALSpace_Result(target : df_string) -> df_item;
    /// **Particle Visibility Status**<br/>
    /// Gets the target's Particles setting.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Particle Visibility Status:
    /// "all", "decreased", "minimal"
    ///
    pub safe fn DF_GAMEVALUE__ParticleSPECIALSpace_VisibilitySPECIALSpace_Status(target : df_string) -> df_string;
    /// **Pressed Movement Keys**<br/>
    /// Gets a list of all movement
    /// keys that are currently
    /// pressed by a player.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one String
    /// value for each movement type:
    /// "forward", "backward", "left", 
    /// "right", "jump", "sneak", 
    /// "sprint"
    ///
    pub safe fn DF_GAMEVALUE__PressedSPECIALSpace_MovementSPECIALSpace_Keys(target : df_string) -> df_list;
    /// **Event Block Location**<br/>
    /// Gets the location of
    /// the block in this event.
    ///
    /// ## Returns
    /// `df_location` (Location):
    /// Block center
    ///
    /// ## Works With
    /// - `Right/Left Click Event`
    /// - `Place/Break Block Event`
    /// - `Projectile Hit Event`
    /// - `Player Combust Event`
    /// - `Entity Combust Event`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_BlockSPECIALSpace_Location(target : df_string) -> df_location;
    /// **Event Block Side**<br/>
    /// Gets the side of the block
    /// that was hit in this event
    /// as a direction.
    ///
    /// ## Returns
    /// `df_vector` (Vector):
    /// Block side
    ///
    /// ## Works With
    /// - `Right/Left Click Event`
    /// - `Place/Break Block Event`
    /// - `Projectile Hit Event`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_BlockSPECIALSpace_Side(target : df_string) -> df_vector;
    /// **Event Damage**<br/>
    /// Gets the amount of damage
    /// dealt in this event.
    /// Includes damage reduction.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0.0 or above
    ///
    /// ## Works With
    /// - `Damage Events`
    /// - `Death Events`
    ///
    /// ## Additional Info
    /// - ❤ = 2 Health
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_Damage(target : df_string) -> df_number;
    /// **Damage Event Cause**<br/>
    /// Gets the type of damage taken or dealt
    /// in this event.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Damage Cause:
    /// "block_explosion", "contact" (cactus), 
    /// "cramming", "custom" (damage action), 
    /// "dragon_breath", "drowning", "dryout", 
    /// (fish on land), "entity_attack", 
    /// "entity_explosion", "entity_sweep_attack", 
    /// "fall", "falling_block", "fire" (in fire block), 
    /// "fire_tick", "fly_into_wall", "freeze", 
    /// "hot_floor" (magma block), "kill", "lava", 
    /// "magic", "melting" (snowman), "poison", 
    /// "projectile", "starvation", "suffocation", "thorns", 
    /// "void", "wither", "world_border"
    ///
    /// ## Works With
    /// - `Damage Events`
    /// - `Death Events`
    ///
    pub safe fn DF_GAMEVALUE__DamageSPECIALSpace_EventSPECIALSpace_Cause(target : df_string) -> df_string;
    /// **Raw Event Damage**<br/>
    /// Gets the amount of damage
    /// dealt in this event
    /// before any damage reductions.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0.0 or above
    ///
    /// ## Works With
    /// - `Damage Events`
    /// - `Death Events`
    ///
    /// ## Additional Info
    /// - ❤ = 2 Health
    ///
    pub safe fn DF_GAMEVALUE__RawSPECIALSpace_EventSPECIALSpace_Damage(target : df_string) -> df_number;
    /// **Event Death Message **<br/>
    /// Gets the death message for
    /// this death event.
    ///
    /// ## Returns
    /// `df_text` (Styled Text):
    /// Death message
    ///
    /// ## Works With
    /// - `Player Death Event`
    /// - `Player Kill Player Event`
    /// - `Mob Kill Player Event`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_DeathSPECIALSpace_MessageSPECIALSpace_(target : df_string) -> df_text;
    /// **Event Heal Amount**<br/>
    /// Gets the amount of health
    /// regained in this event.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Health regained
    ///
    /// ## Works With
    /// - `Player Heal Event`
    ///
    /// ## Additional Info
    /// - ❤ = 2 Health
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_HealSPECIALSpace_Amount(target : df_string) -> df_number;
    /// **Heal Event Cause**<br/>
    /// Gets the reason the target
    /// regained health in this event.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Healing Cause:
    /// "natural" (natural regeneration), 
    /// "magic" (instant health effect), 
    /// "magic_regen" (regen effect), 
    /// "custom" (code)
    ///
    /// ## Works With
    /// - `Player Heal Event`
    ///
    pub safe fn DF_GAMEVALUE__HealSPECIALSpace_EventSPECIALSpace_Cause(target : df_string) -> df_string;
    /// **Event Explosion Affected Blocks**<br/>
    /// Gets the locations of blocks
    /// affected by the explosion
    /// in this event.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one Location
    /// entry for each block
    ///
    /// ## Works With
    /// - `Entity Explode Event`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_ExplosionSPECIALSpace_AffectedSPECIALSpace_Blocks(target : df_string) -> df_list;
    /// **Event Power**<br/>
    /// Gets the percentage of
    /// power this event was
    /// executed with.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0.0% to 100.0%
    ///
    /// ## Works With
    /// - `Shoot Bow Event`
    /// - `Horse Jump Event`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_Power(target : df_string) -> df_number;
    /// **Event Command**<br/>
    /// Gets the entire command line
    /// entered in this event.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Command, with the
    /// first "@" excluded
    ///
    /// ## Works With
    /// - `Command Event`
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_Command(target : df_string) -> df_string;
    /// **Event Command Arguments**<br/>
    /// Gets the separated parts
    /// of the event command.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one String
    /// entry for each word in the
    /// command (split by " ")
    ///
    /// ## Works With
    /// - `Command Event`
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_CommandSPECIALSpace_Arguments(target : df_string) -> df_list;
    /// **Event Item**<br/>
    /// Gets the item in an item
    /// related event.
    ///
    /// ## Returns
    /// `df_item` (Item):
    /// Main item in
    /// event
    ///
    /// ## Works With
    /// - `Item-related Events`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_Item(target : df_string) -> df_item;
    /// **Event Hotbar Slot**<br/>
    /// Gets the hotbar slot being
    /// changed to in this event.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 1 (leftmost slot)
    /// to 9 (rightmost slot)
    ///
    /// ## Works With
    /// - `Change Slot Event`
    /// - `Click Slot Events (swap_hotbar)`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_HotbarSPECIALSpace_Slot(target : df_string) -> df_number;
    /// **Event Clicked Slot Index**<br/>
    /// Gets the index of the
    /// clicked inventory slot
    /// in this event.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// From 1 (first slot)
    /// up to the inventory's size
    ///
    /// ## Works With
    /// - `Click Menu Slot Event`
    /// - `Click Inv Slot Event`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_ClickedSPECIALSpace_SlotSPECIALSpace_Index(target : df_string) -> df_number;
    /// **Event Clicked Slot Item**<br/>
    /// Gets the inventory item
    /// clicked on in this event.
    ///
    /// ## Returns
    /// `df_item` (Item):
    /// Item in slot
    /// (before the click event)
    ///
    /// ## Works With
    /// - `Click Menu Slot Event`
    /// - `Click Inv Slot Event`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_ClickedSPECIALSpace_SlotSPECIALSpace_Item(target : df_string) -> df_item;
    /// **Event Clicked Slot New Item**<br/>
    /// Gets the inventory item
    /// clicked with in this event.
    ///
    /// ## Returns
    /// `df_item` (Item):
    /// Item in slot
    /// (after the click event)
    ///
    /// ## Works With
    /// - `Click Menu Slot Event`
    /// - `Click Inv Slot Event`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_ClickedSPECIALSpace_SlotSPECIALSpace_NewSPECIALSpace_Item(target : df_string) -> df_item;
    /// **Close Inventory Event Cause**<br/>
    /// Gets the reason the target's
    /// inventory was closed in this
    /// event.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Close Cause:
    /// "player", "code", "open_new", 
    /// "teleport", "unloaded", 
    /// "cant_use", "disconnect", 
    /// "death", "unknown"
    ///
    /// ## Works With
    /// - `Close Inventory Event`
    ///
    /// ## Restrictions
    /// - Requires **Emperor** rank
    ///
    #[cfg(any(doc, feature = "rank_emperor"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_emperor")))]
    pub safe fn DF_GAMEVALUE__CloseSPECIALSpace_InventorySPECIALSpace_EventSPECIALSpace_Cause(target : df_string) -> df_string;
    /// **Inventory Event Click Type**<br/>
    /// Gets the click type in this
    /// inventory click event.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Click Type:
    /// "left", "shift_left", "double_left", 
    /// "right", "shift_right", 
    /// "drop" (Q), "drop_stack" (Ctrl+Q), 
    /// "copy" (middle click), "creative", 
    /// "swap_offhand" (F), 
    /// "swap_hotbar" (1 ... 9)
    ///
    /// ## Works With
    /// - `Click Menu Slot Event`
    /// - `Click Inv Slot Event`
    ///
    /// ## Restrictions
    /// - Requires token shop purchase
    ///
    pub safe fn DF_GAMEVALUE__InventorySPECIALSpace_EventSPECIALSpace_ClickSPECIALSpace_Type(target : df_string) -> df_string;
    /// **Fish Event Cause**<br/>
    /// Gets the cause of this
    /// fish event.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Event Type:
    /// "cast", "bite", "catch", 
    /// "cancel", "cancel_block", 
    /// "miss", "pull_entity"
    ///
    /// ## Works With
    /// - `Fish Event`
    ///
    pub safe fn DF_GAMEVALUE__FishSPECIALSpace_EventSPECIALSpace_Cause(target : df_string) -> df_string;
    /// **Teleport Event Cause**<br/>
    /// Gets the reason the player
    /// was teleported in this
    /// event.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Teleport Cause:
    /// "code", "ender_pearl", 
    /// "chorus_fruit", "unknown"
    ///
    /// ## Works With
    /// - `Player Teleport Event`
    ///
    pub safe fn DF_GAMEVALUE__TeleportSPECIALSpace_EventSPECIALSpace_Cause(target : df_string) -> df_string;
    /// **Teleport Location**<br/>
    /// Gets the location that will be
    /// teleported to in this event.
    ///
    /// ## Returns
    /// `df_location` (Location):
    /// Location
    ///
    /// ## Works With
    /// - `Teleport Event`
    ///
    pub safe fn DF_GAMEVALUE__TeleportSPECIALSpace_Location(target : df_string) -> df_location;
    /// **Exhaustion Event Cause**<br/>
    /// Gets the reason the target
    /// became exhausted in this
    /// event.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Exhaustion Cause:
    /// "attack", "block_mined", "custom", 
    /// "crouch", "damaged", "hunger_effect", 
    /// "jump", "jump_sprint", "none", 
    /// "regen", "sprint", "swim", "walk", 
    /// "walk_on_water", "walk_underwater"
    ///
    /// ## Works With
    /// - `Exhaustion Event`
    ///
    pub safe fn DF_GAMEVALUE__ExhaustionSPECIALSpace_EventSPECIALSpace_Cause(target : df_string) -> df_string;
    /// **Event Exhaustion**<br/>
    /// Gets the amount of exhaustion
    /// gained in this event.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Number
    ///
    /// ## Works With
    /// - `Exhaustion Event`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_Exhaustion(target : df_string) -> df_number;
    /// **Transform Event Cause**<br/>
    /// Gets the reason the target
    /// transformed in this event.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Transform Cause:
    /// "cured" (Zombie Villager → Villager), 
    /// "drowned" (Zombie → Drowned), 
    /// "frozen" (Skeleton → Stray), 
    /// "infection" (Villager → Zombie Villager), 
    /// "metamorphosis" (Tadpole → Frog), 
    /// "sheared" (Mooshroom → Cow), 
    /// "split" (Slime → Small Slimes), 
    /// "lightning", "piglin_zombified", 
    /// "unknown"
    ///
    /// ## Works With
    /// - `Entity Transform Event`
    ///
    pub safe fn DF_GAMEVALUE__TransformSPECIALSpace_EventSPECIALSpace_Cause(target : df_string) -> df_string;
    /// **Event Transform Entities**<br/>
    /// Gets the entities an
    /// entity transforms into.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// UUIDs of the new entity/entities
    ///
    /// ## Works With
    /// - `Entity Transform Event`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_TransformSPECIALSpace_Entities(target : df_string) -> df_list;
    /// **Event Hit Type**<br/>
    /// Gets the type of object that
    /// the projectile collided with
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Hit Type:
    /// "block", "entity"
    ///
    /// ## Works With
    /// - `Projectile Hit Event`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_HitSPECIALSpace_Type(target : df_string) -> df_string;
    /// **Product ID**<br/>
    /// Gets the ID of the
    /// product purchased.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Product ID
    ///
    /// ## Works With
    /// - `Purchase Product Event`
    ///
    pub safe fn DF_GAMEVALUE__ProductSPECIALSpace_ID(target : df_string) -> df_string;
    /// **Event Message**<br/>
    /// The message sent in this
    /// event
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Chat message
    ///
    /// ## Works With
    /// - `Chat Event`
    ///
    /// ## Restrictions
    /// - Requires **Mythic** rank
    ///
    #[cfg(any(doc, feature = "rank_mythic"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_mythic")))]
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_Message(target : df_string) -> df_string;
    /// **Event Sign Text**<br/>
    /// Gets the sign text
    /// in this event.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one String
    /// entry for each sign line.
    ///
    /// ## Works With
    /// - `Change Sign Event`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_SignSPECIALSpace_Text(target : df_string) -> df_list;
    /// **Event Sign Side**<br/>
    /// Gets the sign side modified
    /// in this event.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// "front" or "back"
    ///
    /// ## Works With
    /// - `Change Sign Event`
    ///
    pub safe fn DF_GAMEVALUE__EventSPECIALSpace_SignSPECIALSpace_Side(target : df_string) -> df_string;
    /// **Combust Event Duration**<br/>
    /// Gets the duration of fire inflicted
    /// in this event.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Fire duration in ticks
    ///
    /// ## Works With
    /// - `Player Combust Event`
    /// - `Entity Combust Event`
    ///
    pub safe fn DF_GAMEVALUE__CombustSPECIALSpace_EventSPECIALSpace_Duration(target : df_string) -> df_number;
    /// **Combust Event Cause**<br/>
    /// Gets the reason the target caught
    /// on fire in this event.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Combust Cause:
    /// "player", "entity", "block", "code", "unknown"
    ///
    /// ## Works With
    /// - `Player Combust Event`
    /// - `Entity Combust Event`
    ///
    pub safe fn DF_GAMEVALUE__CombustSPECIALSpace_EventSPECIALSpace_Cause(target : df_string) -> df_string;
    /// **Player Count**<br/>
    /// Gets the amount of players
    /// playing on the plot.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Player count
    ///
    pub safe fn DF_GAMEVALUE__PlayerSPECIALSpace_Count(target : df_string) -> df_number;
    /// **CPU Usage**<br/>
    /// Gets the percentage of the
    /// plot's CPU being used this
    /// instant.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Usage as a percentage.
    /// Can go above 100%.
    ///
    pub safe fn DF_GAMEVALUE__CPUSPECIALSpace_Usage(target : df_string) -> df_number;
    /// **Server TPS**<br/>
    /// Gets the amount of game Ticks
    /// Per Second the server is
    /// currently able to handle.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 20.0 (no server
    /// lag) or below (decreases
    /// with more lag)
    ///
    pub safe fn DF_GAMEVALUE__ServerSPECIALSpace_TPS(target : df_string) -> df_number;
    /// **Timestamp**<br/>
    /// Gets the current time as
    /// one number in seconds.
    /// E.g.: 1418840496.5 means
    /// Dec 17, 2014, 18:21:36.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Current time
    ///
    /// ## Additional Info
    /// - The number represents
    ///   the total seconds passed
    ///   since 1970 (Unix Time).
    ///
    pub safe fn DF_GAMEVALUE__Timestamp(target : df_string) -> df_number;
    /// **Selection Size**<br/>
    /// Gets the amount of targets
    /// in the selection.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// 0 (no targets)
    /// or above
    ///
    pub safe fn DF_GAMEVALUE__SelectionSPECIALSpace_Size(target : df_string) -> df_number;
    /// **Selection Target Names**<br/>
    /// Gets the name of each target
    /// in the selection.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one String
    /// entry (name) for each
    /// target
    ///
    pub safe fn DF_GAMEVALUE__SelectionSPECIALSpace_TargetSPECIALSpace_Names(target : df_string) -> df_list;
    /// **Selection Target UUIDs**<br/>
    /// Gets the UUID of each target
    /// in the selection.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one String
    /// entry (UUID) for each
    /// target
    ///
    pub safe fn DF_GAMEVALUE__SelectionSPECIALSpace_TargetSPECIALSpace_UUIDs(target : df_string) -> df_list;
    /// **Plot ID**<br/>
    /// Gets the id of the plot as a string.
    ///
    /// ## Returns
    /// `df_string` (String):
    /// Plot ID
    ///
    pub safe fn DF_GAMEVALUE__PlotSPECIALSpace_ID(target : df_string) -> df_string;
    /// **Plot Name**<br/>
    /// Gets the name of the plot
    /// as a styled text.
    ///
    /// ## Returns
    /// `df_text` (Styled Text):
    /// Plot Name
    ///
    pub safe fn DF_GAMEVALUE__PlotSPECIALSpace_Name(target : df_string) -> df_text;
    /// **Plot Size**<br/>
    /// Gets the size of the plot.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Plot size in blocks
    ///
    /// ## Additional Info
    /// - A basic plot will return 51.
    /// - A large plot will return 101.
    /// - A massive plot will return 301.
    /// - A mega plot will return 1001.
    ///
    pub safe fn DF_GAMEVALUE__PlotSPECIALSpace_Size(target : df_string) -> df_number;
    /// **Microseconds Since Startup**<br/>
    /// Gets the number of microseconds
    /// elapsed since the first player
    /// joined the plot. The decimal
    /// part represents nanoseconds.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Microseconds
    ///
    pub safe fn DF_GAMEVALUE__MicrosecondsSPECIALSpace_SinceSPECIALSpace_Startup(target : df_string) -> df_number;
    /// **Ticks Since Startup**<br/>
    /// Gets the number of ticks
    /// that elapsed since the
    /// first player joined the plot.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Ticks
    ///
    pub safe fn DF_GAMEVALUE__TicksSPECIALSpace_SinceSPECIALSpace_Startup(target : df_string) -> df_number;
    /// **Active Block Transactions**<br/>
    /// Gets the number of block
    /// transactions a plot is executing.
    ///
    /// ## Returns
    /// `df_number` (Number):
    /// Active Block Transactions
    ///
    /// ## Restrictions
    /// - Requires **Overlord** rank
    ///
    #[cfg(any(doc, feature = "rank_overlord"))]
    #[cfg_attr(doc, doc(cfg(feature = "rank_overlord")))]
    pub safe fn DF_GAMEVALUE__ActiveSPECIALSpace_BlockSPECIALSpace_Transactions(target : df_string) -> df_number;
    /// **Plot Player Names**<br/>
    /// Gets the name of each player
    /// on the plot.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one String
    /// entry (name) for each
    /// player
    ///
    pub safe fn DF_GAMEVALUE__PlotSPECIALSpace_PlayerSPECIALSpace_Names(target : df_string) -> df_list;
    /// **Plot Player UUIDs**<br/>
    /// Gets the UUID of each player
    /// on the plot.
    ///
    /// ## Returns
    /// `df_list` (List):
    /// Contains one String
    /// entry (UUID) for each
    /// player
    ///
    pub safe fn DF_GAMEVALUE__PlotSPECIALSpace_PlayerSPECIALSpace_UUIDs(target : df_string) -> df_list;
}
