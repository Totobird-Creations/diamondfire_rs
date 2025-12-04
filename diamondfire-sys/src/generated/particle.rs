use crate::*;
unsafe extern "C" {
    /// **Rain**<br/>
    ///
    pub safe fn DF_PARTICLE__rain__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Underwater**<br/>
    ///
    /// ## Additional Info
    /// - Disappears when
    ///   outside of water
    ///
    pub safe fn DF_PARTICLE__underwater__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Ash**<br/>
    ///
    pub safe fn DF_PARTICLE__ash__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **White Ash**<br/>
    ///
    pub safe fn DF_PARTICLE__whiteSPECIALUnderscore_ash__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Crimson Spore**<br/>
    ///
    pub safe fn DF_PARTICLE__crimsonSPECIALUnderscore_spore__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Warped Spore**<br/>
    ///
    pub safe fn DF_PARTICLE__warpedSPECIALUnderscore_spore__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Angry Villager**<br/>
    ///
    pub safe fn DF_PARTICLE__angrySPECIALUnderscore_villager__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Happy Villager**<br/>
    ///
    pub safe fn DF_PARTICLE__happySPECIALUnderscore_villager__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Spit**<br/>
    ///
    pub safe fn DF_PARTICLE__spit__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Sneeze**<br/>
    ///
    pub safe fn DF_PARTICLE__sneeze__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Heart**<br/>
    ///
    pub safe fn DF_PARTICLE__heart__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Witch**<br/>
    ///
    pub safe fn DF_PARTICLE__witch__MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion_variation : df_number) -> df_particle;
    /// **Raid Omen**<br/>
    ///
    pub safe fn DF_PARTICLE__raidSPECIALUnderscore_omen__MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion_variation : df_number) -> df_particle;
    /// **Trial Omen**<br/>
    ///
    pub safe fn DF_PARTICLE__trialSPECIALUnderscore_omen__MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion_variation : df_number) -> df_particle;
    /// **Explosion**<br/>
    ///
    pub safe fn DF_PARTICLE__explosion__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Explosion Emitter**<br/>
    ///
    /// ## Additional Info
    /// - Creates 6 Explosion
    ///   particles each tick
    ///
    pub safe fn DF_PARTICLE__explosionSPECIALUnderscore_emitter__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Gust**<br/>
    ///
    pub safe fn DF_PARTICLE__gust__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Gust Emitter (Small)**<br/>
    ///
    /// ## Additional Info
    /// - Creates 3 Gust
    ///   particles every 3 ticks
    ///
    pub safe fn DF_PARTICLE__gustSPECIALUnderscore_emitterSPECIALUnderscore_small__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Gust Emitter (Large)**<br/>
    ///
    /// ## Additional Info
    /// - Creates 3 Gust
    ///   particles each tick
    ///
    pub safe fn DF_PARTICLE__gustSPECIALUnderscore_emitterSPECIALUnderscore_large__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Flash**<br/>
    ///
    pub safe fn DF_PARTICLE__flash__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Splash**<br/>
    ///
    pub safe fn DF_PARTICLE__splash__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Fishing**<br/>
    ///
    pub safe fn DF_PARTICLE__fishing__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Effect**<br/>
    ///
    pub safe fn DF_PARTICLE__effect__MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion_variation : df_number) -> df_particle;
    /// **Instant Effect**<br/>
    ///
    pub safe fn DF_PARTICLE__instantSPECIALUnderscore_effect__MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion_variation : df_number) -> df_particle;
    /// **Entity Effect**<br/>
    ///
    pub safe fn DF_PARTICLE__entitySPECIALUnderscore_effect__Colour_Opacity_MotionVariation_ColourVariation(amount : df_number, spread_x : df_number, spread_y : df_number, colour : df_string, opacity : df_number, motion_variation : df_number, colour_variation : df_number) -> df_particle;
    /// **Dolphin**<br/>
    ///
    pub safe fn DF_PARTICLE__dolphin__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Glowing Squid Glow**<br/>
    ///
    pub safe fn DF_PARTICLE__glow__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Falling Nectar**<br/>
    ///
    pub safe fn DF_PARTICLE__fallingSPECIALUnderscore_nectar__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Infested**<br/>
    ///
    pub safe fn DF_PARTICLE__infested__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Small Gust**<br/>
    ///
    pub safe fn DF_PARTICLE__smallSPECIALUnderscore_gust__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Firework**<br/>
    ///
    pub safe fn DF_PARTICLE__firework__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Bubble**<br/>
    ///
    /// ## Additional Info
    /// - Disappears when
    ///   outside of water
    ///
    pub safe fn DF_PARTICLE__bubble__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Bubble Pop**<br/>
    ///
    pub safe fn DF_PARTICLE__bubbleSPECIALUnderscore_pop__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Snowflake**<br/>
    ///
    pub safe fn DF_PARTICLE__snowflake__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Snowball**<br/>
    ///
    pub safe fn DF_PARTICLE__itemSPECIALUnderscore_snowball__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Slime**<br/>
    ///
    pub safe fn DF_PARTICLE__itemSPECIALUnderscore_slime__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Cobweb**<br/>
    ///
    pub safe fn DF_PARTICLE__itemSPECIALUnderscore_cobweb__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Item**<br/>
    ///
    pub safe fn DF_PARTICLE__item__Motion_Material_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, material : df_string, motion_variation : df_number) -> df_particle;
    /// **Ominous Spawning**<br/>
    ///
    pub safe fn DF_PARTICLE__ominousSPECIALUnderscore_spawning__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Critical Hit**<br/>
    ///
    pub safe fn DF_PARTICLE__crit__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Enchanted Hit**<br/>
    ///
    pub safe fn DF_PARTICLE__enchantedSPECIALUnderscore_hit__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Damage Indicator**<br/>
    ///
    pub safe fn DF_PARTICLE__damageSPECIALUnderscore_indicator__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Sweep Attack**<br/>
    ///
    pub safe fn DF_PARTICLE__sweepSPECIALUnderscore_attack__Size_SizeVariation(amount : df_number, spread_x : df_number, spread_y : df_number, size : df_number, size_variation : df_number) -> df_particle;
    /// **Squid Ink**<br/>
    ///
    pub safe fn DF_PARTICLE__squidSPECIALUnderscore_ink__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Glowing Squid Ink**<br/>
    ///
    pub safe fn DF_PARTICLE__glowSPECIALUnderscore_squidSPECIALUnderscore_ink__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Poof**<br/>
    ///
    pub safe fn DF_PARTICLE__poof__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Elder Guardian**<br/>
    ///
    pub safe fn DF_PARTICLE__elderSPECIALUnderscore_guardian__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Dragon Breath**<br/>
    ///
    pub safe fn DF_PARTICLE__dragonSPECIALUnderscore_breath__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Totem of Undying**<br/>
    ///
    pub safe fn DF_PARTICLE__totemSPECIALUnderscore_ofSPECIALUnderscore_undying__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Cloud**<br/>
    ///
    pub safe fn DF_PARTICLE__cloud__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Sonic Boom**<br/>
    ///
    pub safe fn DF_PARTICLE__sonicSPECIALUnderscore_boom__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Dust Pillar**<br/>
    ///
    pub safe fn DF_PARTICLE__dustSPECIALUnderscore_pillar__Material(amount : df_number, spread_x : df_number, spread_y : df_number, material : df_string) -> df_particle;
    /// **Dripping Obsidian Tear**<br/>
    ///
    /// ## Additional Info
    /// - Creates a Falling
    ///   Obsidian Tear particle
    ///
    pub safe fn DF_PARTICLE__drippingSPECIALUnderscore_obsidianSPECIALUnderscore_tear__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Falling Obsidian Tear**<br/>
    ///
    /// ## Additional Info
    /// - Creates a Landing
    ///   Obsidian Tear particle
    ///   upon impact
    ///
    pub safe fn DF_PARTICLE__fallingSPECIALUnderscore_obsidianSPECIALUnderscore_tear__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Landing Obsidian Tear**<br/>
    ///
    pub safe fn DF_PARTICLE__landingSPECIALUnderscore_obsidianSPECIALUnderscore_tear__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Dripping Water**<br/>
    ///
    /// ## Additional Info
    /// - Creates a Falling
    ///   Water particle
    ///
    pub safe fn DF_PARTICLE__drippingSPECIALUnderscore_water__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Dripstone Dripping Water**<br/>
    ///
    /// ## Additional Info
    /// - Creates a Falling
    ///   Dripstone Water particle
    ///
    pub safe fn DF_PARTICLE__drippingSPECIALUnderscore_dripstoneSPECIALUnderscore_water__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Falling Water**<br/>
    ///
    /// ## Additional Info
    /// - Creates a Splash
    ///   particle upon impact
    ///
    pub safe fn DF_PARTICLE__fallingSPECIALUnderscore_water__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Dripstone Falling Water**<br/>
    ///
    /// ## Additional Info
    /// - Produces sound on landing
    ///
    pub safe fn DF_PARTICLE__fallingSPECIALUnderscore_dripstoneSPECIALUnderscore_water__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Dripping Lava**<br/>
    ///
    /// ## Additional Info
    /// - Creates a Falling
    ///   Lava particle
    ///
    pub safe fn DF_PARTICLE__drippingSPECIALUnderscore_lava__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Dripstone Dripping Lava**<br/>
    ///
    /// ## Additional Info
    /// - Creates a Falling
    ///   Dripstone Lava particle
    ///
    pub safe fn DF_PARTICLE__drippingSPECIALUnderscore_dripstoneSPECIALUnderscore_lava__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Falling Lava**<br/>
    ///
    /// ## Additional Info
    /// - Creates a Landing
    ///   Lava particle upon
    ///   impact
    ///
    pub safe fn DF_PARTICLE__fallingSPECIALUnderscore_lava__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Dripstone Falling Lava**<br/>
    ///
    /// ## Additional Info
    /// - Produces sound on landing
    ///
    pub safe fn DF_PARTICLE__fallingSPECIALUnderscore_dripstoneSPECIALUnderscore_lava__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Landing Lava**<br/>
    ///
    pub safe fn DF_PARTICLE__landingSPECIALUnderscore_lava__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Dripping Honey**<br/>
    ///
    /// ## Additional Info
    /// - Creates a Falling
    ///   Honey particle
    ///
    pub safe fn DF_PARTICLE__drippingSPECIALUnderscore_honey__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Falling Honey**<br/>
    ///
    /// ## Additional Info
    /// - Creates a Landing
    ///   Honey particle upon
    ///   impact
    ///
    pub safe fn DF_PARTICLE__fallingSPECIALUnderscore_honey__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Landing Honey**<br/>
    ///
    pub safe fn DF_PARTICLE__landingSPECIALUnderscore_honey__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Lava**<br/>
    ///
    /// ## Additional Info
    /// - Creates a Smoke
    ///   particle each tick
    ///
    pub safe fn DF_PARTICLE__lava__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Mycelium**<br/>
    ///
    pub safe fn DF_PARTICLE__mycelium__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Spore Blossom Fall**<br/>
    ///
    pub safe fn DF_PARTICLE__fallingSPECIALUnderscore_sporeSPECIALUnderscore_blossom__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Spore Blossom Air**<br/>
    ///
    pub safe fn DF_PARTICLE__sporeSPECIALUnderscore_blossomSPECIALUnderscore_air__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Portal**<br/>
    ///
    pub safe fn DF_PARTICLE__portal__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Reverse Portal**<br/>
    ///
    pub safe fn DF_PARTICLE__reverseSPECIALUnderscore_portal__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Enchant**<br/>
    ///
    pub safe fn DF_PARTICLE__enchant__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Small Flame**<br/>
    ///
    pub safe fn DF_PARTICLE__smallSPECIALUnderscore_flame__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Flame**<br/>
    ///
    pub safe fn DF_PARTICLE__flame__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Soul Flame**<br/>
    ///
    pub safe fn DF_PARTICLE__soulSPECIALUnderscore_fireSPECIALUnderscore_flame__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Nautilus**<br/>
    ///
    pub safe fn DF_PARTICLE__nautilus__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **End Rod**<br/>
    ///
    pub safe fn DF_PARTICLE__endSPECIALUnderscore_rod__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Falling Dust**<br/>
    ///
    pub safe fn DF_PARTICLE__fallingSPECIALUnderscore_dust__Material(amount : df_number, spread_x : df_number, spread_y : df_number, material : df_string) -> df_particle;
    /// **Whirlpool Bubble Column**<br/>
    ///
    /// ## Additional Info
    /// - Disappears when
    ///   outside of water
    ///
    pub safe fn DF_PARTICLE__currentSPECIALUnderscore_down__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Upward Bubble Column**<br/>
    ///
    /// ## Additional Info
    /// - Disappears when
    ///   outside of water
    ///
    pub safe fn DF_PARTICLE__bubbleSPECIALUnderscore_columnSPECIALUnderscore_up__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Campfire Smoke**<br/>
    ///
    pub safe fn DF_PARTICLE__campfireSPECIALUnderscore_cosySPECIALUnderscore_smoke__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Campfire Signal Smoke**<br/>
    ///
    pub safe fn DF_PARTICLE__campfireSPECIALUnderscore_signalSPECIALUnderscore_smoke__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Smoke**<br/>
    ///
    pub safe fn DF_PARTICLE__smoke__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Large Smoke**<br/>
    ///
    pub safe fn DF_PARTICLE__largeSPECIALUnderscore_smoke__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Note**<br/>
    ///
    pub safe fn DF_PARTICLE__note__Colour_ColourVariation(amount : df_number, spread_x : df_number, spread_y : df_number, colour : df_string, colour_variation : df_number) -> df_particle;
    /// **Wax On**<br/>
    ///
    pub safe fn DF_PARTICLE__waxSPECIALUnderscore_on__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Wax Off**<br/>
    ///
    pub safe fn DF_PARTICLE__waxSPECIALUnderscore_off__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Scrape Oxidization**<br/>
    ///
    pub safe fn DF_PARTICLE__scrape__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Composter**<br/>
    ///
    pub safe fn DF_PARTICLE__composter__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Block Marker**<br/>
    ///
    pub safe fn DF_PARTICLE__blockSPECIALUnderscore_marker__Material(amount : df_number, spread_x : df_number, spread_y : df_number, material : df_string) -> df_particle;
    /// **Dust**<br/>
    ///
    /// ## Additional Info
    /// - Duration scales
    ///   with size
    ///
    pub safe fn DF_PARTICLE__dust__Colour_Size_ColourVariation_SizeVariation(amount : df_number, spread_x : df_number, spread_y : df_number, colour : df_string, size : df_number, colour_variation : df_number, size_variation : df_number) -> df_particle;
    /// **Fade Dust**<br/>
    ///
    /// ## Additional Info
    /// - Duration scales
    ///   with size
    ///
    pub safe fn DF_PARTICLE__dustSPECIALUnderscore_colorSPECIALUnderscore_transition__Colour_FadeColour_Size_ColourVariation_SizeVariation(amount : df_number, spread_x : df_number, spread_y : df_number, colour : df_string, fade_colour : df_string, size : df_number, colour_variation : df_number, size_variation : df_number) -> df_particle;
    /// **Firefly**<br/>
    ///
    /// ## Additional Info
    /// - Only the Y motion affects
    ///   the particle movement
    ///
    pub safe fn DF_PARTICLE__firefly__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Tinted Leaves**<br/>
    ///
    pub safe fn DF_PARTICLE__tintedSPECIALUnderscore_leaves__Colour_ColourVariation(amount : df_number, spread_x : df_number, spread_y : df_number, colour : df_string, colour_variation : df_number) -> df_particle;
    /// **Cherry Leaves**<br/>
    ///
    pub safe fn DF_PARTICLE__cherrySPECIALUnderscore_leaves__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Pale Oak Leaves**<br/>
    ///
    pub safe fn DF_PARTICLE__paleSPECIALUnderscore_oakSPECIALUnderscore_leaves__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Trail**<br/>
    ///
    pub safe fn DF_PARTICLE__trail__Motion_Colour_Duration_ColourVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, colour : df_string, duration : df_number, colour_variation : df_number) -> df_particle;
    /// **Vibration**<br/>
    ///
    pub safe fn DF_PARTICLE__vibration__Motion_Duration(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, duration : df_number) -> df_particle;
    /// **Sculk Soul**<br/>
    ///
    pub safe fn DF_PARTICLE__sculkSPECIALUnderscore_soul__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Shriek**<br/>
    ///
    pub safe fn DF_PARTICLE__shriek__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Sculk Charge**<br/>
    ///
    pub safe fn DF_PARTICLE__sculkSPECIALUnderscore_charge__Motion_Roll_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, roll : df_number, motion_variation : df_number) -> df_particle;
    /// **Sculk Charge Pop**<br/>
    ///
    pub safe fn DF_PARTICLE__sculkSPECIALUnderscore_chargeSPECIALUnderscore_pop__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Soul**<br/>
    ///
    pub safe fn DF_PARTICLE__soul__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Block**<br/>
    ///
    pub safe fn DF_PARTICLE__block__Material(amount : df_number, spread_x : df_number, spread_y : df_number, material : df_string) -> df_particle;
    /// **Block Crumble**<br/>
    ///
    pub safe fn DF_PARTICLE__blockSPECIALUnderscore_crumble__Material(amount : df_number, spread_x : df_number, spread_y : df_number, material : df_string) -> df_particle;
    /// **Electric Spark**<br/>
    ///
    pub safe fn DF_PARTICLE__electricSPECIALUnderscore_spark__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Egg Crack**<br/>
    ///
    pub safe fn DF_PARTICLE__eggSPECIALUnderscore_crack__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Dust Plume**<br/>
    ///
    pub safe fn DF_PARTICLE__dustSPECIALUnderscore_plume__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **White Smoke**<br/>
    ///
    pub safe fn DF_PARTICLE__whiteSPECIALUnderscore_smoke__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Trial Spawner Detection**<br/>
    ///
    pub safe fn DF_PARTICLE__trialSPECIALUnderscore_spawnerSPECIALUnderscore_detection__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Trial Spawner Detection (Ominous)**<br/>
    ///
    pub safe fn DF_PARTICLE__trialSPECIALUnderscore_spawnerSPECIALUnderscore_detectionSPECIALUnderscore_ominous__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Vault Connection**<br/>
    ///
    pub safe fn DF_PARTICLE__vaultSPECIALUnderscore_connection__Motion_MotionVariation(amount : df_number, spread_x : df_number, spread_y : df_number, motion : df_vector, motion_variation : df_number) -> df_particle;
    /// **Barrier**<br/>
    ///
    pub safe fn DF_PARTICLE__blockSPECIALUnderscore_marker__(amount : df_number, spread_x : df_number, spread_y : df_number) -> df_particle;
    /// **Ambient Entity Effect**<br/>
    ///
    pub safe fn DF_PARTICLE__entitySPECIALUnderscore_effect__Colour_MotionVariation_ColourVariation(amount : df_number, spread_x : df_number, spread_y : df_number, colour : df_string, motion_variation : df_number, colour_variation : df_number) -> df_particle;
}
