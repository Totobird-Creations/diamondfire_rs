use crate::*;
unsafe extern "C" {
    /// **Absorption**<br/>
    /// Grants 4 × level absorption
    /// (shield) health
    ///
    pub safe fn DF_POTION__Absorption(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Conduit Power**<br/>
    /// Increases underwater
    /// vision, mining speed, and
    /// prevents drowning
    ///
    pub safe fn DF_POTION__ConduitSPECIALSpace_Power(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Dolphin's Grace**<br/>
    /// Increases swimming
    /// speed by 40%
    ///
    pub safe fn DF_POTION__DolphinSPECIALApostrophe_sSPECIALSpace_Grace(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Fire Resistance**<br/>
    /// Grants immunity to
    /// fire and lava damage
    ///
    pub safe fn DF_POTION__FireSPECIALSpace_Resistance(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Haste**<br/>
    /// Increases mining speed
    /// by 20% × level
    ///
    pub safe fn DF_POTION__Haste(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Health Boost**<br/>
    /// Increases maximum
    /// health by 4 × level
    ///
    pub safe fn DF_POTION__HealthSPECIALSpace_Boost(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Hero of the Village**<br/>
    /// Causes nearby villagers
    /// to throw gifts and to
    /// offer a trade discount
    /// of 23.75 + (6.25 × level)%
    ///
    pub safe fn DF_POTION__HeroSPECIALSpace_ofSPECIALSpace_theSPECIALSpace_Village(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Instant Health**<br/>
    /// Instantly heals
    /// 2 × 2 ^ level health
    ///
    pub safe fn DF_POTION__InstantSPECIALSpace_Health(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Invisibility**<br/>
    /// Causes the entity
    /// to disappear
    ///
    pub safe fn DF_POTION__Invisibility(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Jump Boost**<br/>
    /// Increases jump
    /// height to 0.5 + level
    ///
    pub safe fn DF_POTION__JumpSPECIALSpace_Boost(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Luck**<br/>
    /// Improves drops
    /// from loot tables based
    /// on level
    ///
    pub safe fn DF_POTION__Luck(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Night Vision**<br/>
    /// Enables vision at full
    /// brightness everywhere
    ///
    pub safe fn DF_POTION__NightSPECIALSpace_Vision(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Regeneration**<br/>
    /// Heals 1 health every
    /// 2.5 ÷ level seconds
    ///
    pub safe fn DF_POTION__Regeneration(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Resistance**<br/>
    /// Reduces damage
    /// taken by 20% × level
    ///
    pub safe fn DF_POTION__Resistance(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Saturation**<br/>
    /// Instantly replenishes 
    /// 1 × level hunger and
    /// 2 × level saturation, or
    /// does so each tick
    ///
    pub safe fn DF_POTION__Saturation(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Slow Falling**<br/>
    /// Reduces falling speed
    /// by 70% and prevents fall
    /// damage
    ///
    pub safe fn DF_POTION__SlowSPECIALSpace_Falling(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Strength**<br/>
    /// Increases melee-dealt
    /// damage by 3 × level
    ///
    pub safe fn DF_POTION__Strength(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Speed**<br/>
    /// Increases walking
    /// speed by 20% × level
    ///
    pub safe fn DF_POTION__Speed(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Water Breathing**<br/>
    /// Prevents the loss of
    /// breath underwater
    ///
    pub safe fn DF_POTION__WaterSPECIALSpace_Breathing(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Bad Omen**<br/>
    /// Causes an ominous event
    /// upon entering a village
    /// or a trial chamber
    ///
    pub safe fn DF_POTION__BadSPECIALSpace_Omen(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Blindness**<br/>
    /// Obscures the player's
    /// vision with black fog and
    /// prevents sprinting
    ///
    pub safe fn DF_POTION__Blindness(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Darkness**<br/>
    /// Obscures the player's
    /// vision with a dark vignette
    ///
    pub safe fn DF_POTION__Darkness(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Glowing**<br/>
    /// Draws an outline around
    /// the entity that is
    /// visible through walls
    ///
    pub safe fn DF_POTION__Glowing(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Hunger**<br/>
    /// Increases food exhaustion
    /// by 0.1 × level per second
    ///
    pub safe fn DF_POTION__Hunger(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Infested**<br/>
    /// Spawns between 1 and
    /// 3 silverfish with a 10%
    /// chance when hurt
    ///
    pub safe fn DF_POTION__Infested(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Instant Damage**<br/>
    /// Instantly inflicts
    /// 3 × 2 ^ level damage
    ///
    pub safe fn DF_POTION__InstantSPECIALSpace_Damage(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Levitation**<br/>
    /// Levitates the player
    /// upwards by 0.875 × level 
    /// blocks per second
    ///
    pub safe fn DF_POTION__Levitation(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Mining Fatigue**<br/>
    /// Greatly reduces mining
    /// speed, and attack speed
    /// by 10% × level
    ///
    pub safe fn DF_POTION__MiningSPECIALSpace_Fatigue(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Nausea**<br/>
    /// Wobbles and warps
    /// the player's vision
    ///
    pub safe fn DF_POTION__Nausea(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Oozing**<br/>
    /// Spawns 2 slimes upon
    /// death
    ///
    pub safe fn DF_POTION__Oozing(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Poison**<br/>
    /// Deals 1 damage every
    /// 1.25 ÷ level seconds
    ///
    pub safe fn DF_POTION__Poison(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Raid Omen**<br/>
    /// Starts a raid when the
    /// player enters a village
    ///
    pub safe fn DF_POTION__RaidSPECIALSpace_Omen(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Slowness**<br/>
    /// Reduces walking speed
    /// by 15% × level
    ///
    pub safe fn DF_POTION__Slowness(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Trial Omen**<br/>
    /// Transforms nearby trial
    /// spawners into ominous
    /// spawners
    ///
    pub safe fn DF_POTION__TrialSPECIALSpace_Omen(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Bad Luck**<br/>
    /// Deteriorates drops
    /// from loot tables based
    /// on level
    ///
    pub safe fn DF_POTION__BadSPECIALSpace_Luck(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Weakness**<br/>
    /// Reduces melee-dealt
    /// damage by 4 × level
    ///
    pub safe fn DF_POTION__Weakness(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Weaving**<br/>
    /// Spreads cobweb blocks
    /// around the entity upon death
    ///
    pub safe fn DF_POTION__Weaving(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Wind Charged**<br/>
    /// Emits a burst of wind
    /// upon death
    ///
    pub safe fn DF_POTION__WindSPECIALSpace_Charged(amplifier : df_number, duration : df_number) -> df_potion;
    /// **Wither**<br/>
    /// Deals 1 damage every
    /// 2 ÷ level seconds (and
    /// can kill)
    ///
    pub safe fn DF_POTION__Wither(amplifier : df_number, duration : df_number) -> df_potion;
}
