PRAGMA foreign_keys = off;
BEGIN TRANSACTION;

-- Table: AccessoryDefaultLoc
CREATE TABLE AccessoryDefaultLoc (
    GroupID     INT32 PRIMARY KEY,
    Description TEXT4,
    Pos_X       REAL,
    Pos_Y       REAL,
    Pos_Z       REAL,
    Rot_X       REAL,
    Rot_Y       REAL,
    Rot_Z       REAL
);


-- Table: Activities
CREATE TABLE Activities (
    ActivityID              INT32    PRIMARY KEY,
    locStatus               INT32,
    instanceMapID           INT32    REFERENCES ZoneTable (zoneID),
    minTeams                INT32,
    maxTeams                INT32,
    minTeamSize             INT32,
    maxTeamSize             INT32,
    waitTime                INT32,
    startDelay              INT32,
    requiresUniqueData      INT_BOOL,
    leaderboardType         INT32,
    localize                INT_BOOL,
    optionalCostLOT         INT32    REFERENCES Objects (id),
    optionalCostCount       INT32,
    showUIRewards           INT_BOOL,
    CommunityActivityFlagID INT32,
    gate_version            TEXT4    REFERENCES FeatureGating (featureName),
    noTeamLootOnDeath       INT_BOOL,
    optionalPercentage      REAL
);


-- Table: ActivityRewards
CREATE TABLE ActivityRewards (
    objectTemplate      INT32 REFERENCES Objects (id),
    ActivityRewardIndex INT32,
    activityRating      INT32,
    LootMatrixIndex     INT32 REFERENCES LootMatrix (LootMatrixIndex),
    CurrencyIndex       INT32,
    ChallengeRating     INT32,
    description         TEXT4,
    PRIMARY KEY (
        objectTemplate,
        ActivityRewardIndex
    )
);


-- Table: ActivityText
CREATE TABLE ActivityText (
    activityID   INT32    REFERENCES Activities (ActivityID),
    type         TEXT4,
    localize     INT_BOOL,
    locStatus    INT32,
    gate_version TEXT4    REFERENCES FeatureGating (featureName),
    PRIMARY KEY (
        activityID,
        type
    )
);


-- Table: AICombatRoles
CREATE TABLE AICombatRoles (
    id                     INT32 PRIMARY KEY,
    preferredRole          INT32,
    specifiedMinRangeNOUSE REAL,
    specifiedMaxRangeNOUSE REAL,
    specificMinRange       REAL,
    specificMaxRange       REAL
);


-- Table: AnimationIndex
CREATE TABLE AnimationIndex (
    animationGroupID INT32 PRIMARY KEY,
    description      TEXT4,
    groupType        TEXT4
);


-- Table: Animations
CREATE TABLE Animations (
    animationGroupID    INT32    REFERENCES AnimationIndex (animationGroupID),
    animation_type      TEXT4,
    animation_name      TEXT4,
    chance_to_play      REAL,
    min_loops           INT32,
    max_loops           INT32,
    animation_length    REAL,
    hideEquip           INT_BOOL,
    ignoreUpperBody     INT_BOOL,
    restartable         INT_BOOL,
    face_animation_name TEXT4,
    priority            REAL,
    blendTime           REAL,
    PRIMARY KEY (
        animationGroupID,
        animation_type,
        animation_name
    )
);


-- Table: BaseCombatAIComponent
CREATE TABLE BaseCombatAIComponent (
    id                INT32    PRIMARY KEY,
    behaviorType      INT32,
    combatRoundLength REAL,
    combatRole        INT32,
    minRoundLength    REAL,
    maxRoundLength    REAL,
    tetherSpeed       REAL,
    pursuitSpeed      REAL,
    combatStartDelay  REAL,
    softTetherRadius  REAL,
    hardTetherRadius  REAL,
    spawnTimer        REAL,
    tetherEffectID    INT32,
    ignoreMediator    INT_BOOL,
    aggroRadius       REAL,
    ignoreStatReset   INT_BOOL,
    ignoreParent      INT_BOOL
);


-- Table: BehaviorEffect
CREATE TABLE BehaviorEffect (
    effectID           INT32,
    effectType         TEXT4,
    effectName         TEXT4,
    trailID            INT32,
    pcreateDuration    REAL,
    animationName      TEXT4,
    attachToObject     INT_BOOL,
    boneName           TEXT4,
    useSecondary       INT_BOOL,
    cameraEffectType   INT32,
    cameraDuration     REAL,
    cameraFrequency    REAL,
    cameraXAmp         REAL,
    cameraYAmp         REAL,
    cameraZAmp         REAL,
    cameraRotFrequency REAL,
    cameraRoll         REAL,
    cameraPitch        REAL,
    cameraYaw          REAL,
    AudioEventGUID     TEXT4,
    renderEffectType   INT32,
    renderEffectTime   REAL,
    renderStartVal     REAL,
    renderEndVal       REAL,
    renderDelayVal     REAL,
    renderValue1       REAL,
    renderValue2       REAL,
    renderValue3       REAL,
    renderRGBA         TEXT4,
    renderShaderVal    INT32,
    motionID           INT32,
    meshID             INT32,
    meshDuration       REAL,
    meshLockedNode     TEXT4,
    PRIMARY KEY (
        effectID,
        effectType
    )
);


-- Table: BehaviorParameter
CREATE TABLE BehaviorParameter (
    behaviorID  INT32 REFERENCES BehaviorTemplate (behaviorID),
    parameterID TEXT4,
    value       REAL,
    PRIMARY KEY (
        behaviorID,
        parameterID
    )
);


-- Table: BehaviorTemplate
CREATE TABLE BehaviorTemplate (
    behaviorID   INT32 PRIMARY KEY,
    templateID   INT32 REFERENCES BehaviorTemplateName (templateID),
    effectID     INT32,
    effectHandle TEXT4
);


-- Table: BehaviorTemplateName
CREATE TABLE BehaviorTemplateName (
    templateID INT32 PRIMARY KEY,
    name       TEXT4
);


-- Table: Blueprints
CREATE TABLE Blueprints (
    id          INT64    PRIMARY KEY,
    name        TEXT4,
    description TEXT4,
    accountid   INT64,
    characterid INT64,
    price       INT32,
    rating      INT32,
    categoryid  INT32,
    lxfpath     TEXT4,
    deleted     INT_BOOL,
    created     INT64,
    modified    INT64
);


-- Table: brickAttributes
CREATE TABLE brickAttributes (
    ID            INT32 PRIMARY KEY,
    icon_asset    TEXT4,
    display_order INT32,
    locStatus     INT32
);


-- Table: BrickColors
CREATE TABLE BrickColors (
    id              INT32    PRIMARY KEY,
    red             REAL,
    green           REAL,
    blue            REAL,
    alpha           REAL,
    legopaletteid   INT32,
    description     TEXT4,
    validTypes      INT32,
    validCharacters INT32,
    factoryValid    INT_BOOL
);


-- Table: BrickIDTable
CREATE TABLE BrickIDTable (
    NDObjectID  INT32 REFERENCES Objects (id)
                      PRIMARY KEY,
    LEGOBrickID INT32
);


-- Table: BuffDefinitions
CREATE TABLE BuffDefinitions (
    ID       INT32 PRIMARY KEY,
    Priority REAL,
    UIIcon   TEXT4
);


-- Table: BuffParameters
CREATE TABLE BuffParameters (
    BuffID        INT32 REFERENCES BuffDefinitions (ID),
    ParameterName TEXT4,
    NumberValue   REAL,
    StringValue   TEXT4,
    EffectID      INT32,
    PRIMARY KEY (
        BuffID,
        ParameterName
    )
);


-- Table: Camera
CREATE TABLE Camera (
    camera_name                          TEXT4 PRIMARY KEY,
    pitch_angle_tolerance                REAL,
    starting_zoom                        REAL,
    zoom_return_modifier                 REAL,
    pitch_return_modifier                REAL,
    tether_out_return_modifier           REAL,
    tether_in_return_multiplier          REAL,
    verticle_movement_dampening_modifier REAL,
    return_from_incline_modifier         REAL,
    horizontal_return_modifier           REAL,
    yaw_behavior_speed_multiplier        REAL,
    camera_collision_padding             REAL,
    glide_speed                          REAL,
    fade_player_min_range                REAL,
    min_movement_delta_tolerance         REAL,
    min_glide_distance_tolerance         REAL,
    look_forward_offset                  REAL,
    look_up_offset                       REAL,
    minimum_vertical_dampening_distance  REAL,
    maximum_vertical_dampening_distance  REAL,
    minimum_ignore_jump_distance         REAL,
    maximum_ignore_jump_distance         REAL,
    maximum_auto_glide_angle             REAL,
    minimum_tether_glide_distance        REAL,
    yaw_sign_correction                  REAL,
    set_1_look_forward_offset            REAL,
    set_1_look_up_offset                 REAL,
    set_2_look_forward_offset            REAL,
    set_2_look_up_offset                 REAL,
    set_0_speed_influence_on_dir         REAL,
    set_1_speed_influence_on_dir         REAL,
    set_2_speed_influence_on_dir         REAL,
    set_0_angular_relaxation             REAL,
    set_1_angular_relaxation             REAL,
    set_2_angular_relaxation             REAL,
    set_0_position_up_offset             REAL,
    set_1_position_up_offset             REAL,
    set_2_position_up_offset             REAL,
    set_0_position_forward_offset        REAL,
    set_1_position_forward_offset        REAL,
    set_2_position_forward_offset        REAL,
    set_0_FOV                            REAL,
    set_1_FOV                            REAL,
    set_2_FOV                            REAL,
    set_0_max_yaw_angle                  REAL,
    set_1_max_yaw_angle                  REAL,
    set_2_max_yaw_angle                  REAL,
    set_1_fade_in_camera_set_change      INT32,
    set_1_fade_out_camera_set_change     INT32,
    set_2_fade_in_camera_set_change      INT32,
    set_2_fade_out_camera_set_change     INT32,
    input_movement_scalar                REAL,
    input_rotation_scalar                REAL,
    input_zoom_scalar                    REAL,
    minimum_pitch_desired                REAL,
    maximum_pitch_desired                REAL,
    minimum_zoom                         REAL,
    maximum_zoom                         REAL,
    horizontal_rotate_tolerance          REAL,
    horizontal_rotate_modifier           REAL
);


-- Table: CelebrationParameters
CREATE TABLE CelebrationParameters (
    id               INT32 PRIMARY KEY,
    animation        TEXT4,
    backgroundObject INT32 REFERENCES Objects (id),
    duration         REAL,
    subText          TEXT4,
    mainText         TEXT4,
    iconID           INT32 REFERENCES Icons (IconID),
    celeLeadIn       REAL,
    celeLeadOut      REAL,
    cameraPathLOT    INT32 REFERENCES Objects (id),
    pathNodeName     TEXT4,
    ambientR         REAL,
    ambientG         REAL,
    ambientB         REAL,
    directionalR     REAL,
    directionalG     REAL,
    directionalB     REAL,
    specularR        REAL,
    specularG        REAL,
    specularB        REAL,
    lightPositionX   REAL,
    lightPositionY   REAL,
    lightPositionZ   REAL,
    blendTime        REAL,
    fogColorR        REAL,
    fogColorG        REAL,
    fogColorB        REAL,
    musicCue         TEXT4,
    soundGUID        TEXT4,
    mixerProgram     TEXT4
);


-- Table: ChoiceBuildComponent
CREATE TABLE ChoiceBuildComponent (
    id                  INT32 PRIMARY KEY,
    selections          TEXT4,
    imaginationOverride INT32
);


-- Table: CollectibleComponent
CREATE TABLE CollectibleComponent (
    id                  INT32 PRIMARY KEY,
    requirement_mission INT32 REFERENCES Missions (id) 
);


-- Table: ComponentsRegistry
CREATE TABLE ComponentsRegistry (
    id             INT32,
    component_type INT32,
    component_id   INT32,
    PRIMARY KEY (
        id,
        component_type
    )
);


-- Table: ControlSchemes
CREATE TABLE ControlSchemes (
    control_scheme                    INT32 PRIMARY KEY,
    scheme_name                       TEXT4,
    rotation_speed                    REAL,
    walk_forward_speed                REAL,
    walk_backward_speed               REAL,
    walk_strafe_speed                 REAL,
    walk_strafe_forward_speed         REAL,
    walk_strafe_backward_speed        REAL,
    run_backward_speed                REAL,
    run_strafe_speed                  REAL,
    run_strafe_forward_speed          REAL,
    run_strafe_backward_speed         REAL,
    keyboard_zoom_sensitivity         REAL,
    keyboard_pitch_sensitivity        REAL,
    keyboard_yaw_sensitivity          REAL,
    mouse_zoom_wheel_sensitivity      REAL,
    x_mouse_move_sensitivity_modifier REAL,
    y_mouse_move_sensitivity_modifier REAL,
    freecam_speed_modifier            REAL,
    freecam_slow_speed_multiplier     REAL,
    freecam_fast_speed_multiplier     REAL,
    freecam_mouse_modifier            REAL,
    gamepad_pitch_rot_sensitivity     REAL,
    gamepad_yaw_rot_sensitivity       REAL,
    gamepad_trigger_sensitivity       REAL
);


-- Table: CurrencyDenominations
CREATE TABLE CurrencyDenominations (
    value    INT32 PRIMARY KEY,
    objectid INT32 REFERENCES Objects (id) 
);


-- Table: CurrencyTable
CREATE TABLE CurrencyTable (
    currencyIndex INT32,
    npcminlevel   INT32,
    minvalue      INT32,
    maxvalue      INT32,
    id            INT32 PRIMARY KEY
);


-- Table: DBExclude
CREATE TABLE DBExclude (
    [table]  TEXT4,
    [column] TEXT4,
    PRIMARY KEY (
        [table],
        [column]
    )
);


-- Table: DeletionRestrictions
CREATE TABLE DeletionRestrictions (
    id           INT32    PRIMARY KEY,
    restricted   INT_BOOL,
    ids          TEXT4,
    checkType    INT32,
    localize     INT_BOOL,
    locStatus    INT32,
    gate_version TEXT4    REFERENCES FeatureGating (featureName)
);


-- Table: DestructibleComponent
CREATE TABLE DestructibleComponent (
    id              INT32    PRIMARY KEY,
    faction         INT32    REFERENCES Factions (faction),
    factionList     TEXT4,
    life            INT32,
    imagination     INT32,
    LootMatrixIndex INT32    REFERENCES LootMatrix (LootMatrixIndex),
    CurrencyIndex   INT32,
    level           INT32,
    armor           REAL,
    death_behavior  INT32,
    isnpc           INT_BOOL,
    attack_priority INT32,
    isSmashable     INT_BOOL,
    difficultyLevel INT32
);


-- Table: DevModelBehaviors
CREATE TABLE DevModelBehaviors (
    ModelID    INT32 PRIMARY KEY,
    BehaviorID INT32
);


-- Table: dtproperties
CREATE TABLE dtproperties (
    id       INT32 PRIMARY KEY,
    objectid INT32,
    property TEXT4,
    value    TEXT4,
    uvalue   TEXT4,
    lvalue   BLOB_NONE,
    version  INT32
);


-- Table: Emotes
CREATE TABLE Emotes (
    id            INT32    PRIMARY KEY,
    animationName TEXT4,
    iconFilename  TEXT4,
    channel       TEXT4,
    command       TEXT4,
    locked        INT_BOOL,
    localize      INT_BOOL,
    locStatus     INT32,
    gate_version  TEXT4    REFERENCES FeatureGating (featureName)
);


-- Table: EventGating
CREATE TABLE EventGating (
    eventName  TEXT4 PRIMARY KEY,
    date_start INT64,
    date_end   INT64
);


-- Table: ExhibitComponent
CREATE TABLE ExhibitComponent (
    id                        INT32 PRIMARY KEY,
    length                    REAL,
    width                     REAL,
    height                    REAL,
    offsetX                   REAL,
    offsetY                   REAL,
    offsetZ                   REAL,
    fReputationSizeMultiplier REAL,
    fImaginationCost          REAL
);


-- Table: Factions
CREATE TABLE Factions (
    faction             INT32    PRIMARY KEY,
    factionList         TEXT4,
    factionListFriendly INT_BOOL,
    friendList          TEXT4,
    enemyList           TEXT4
);


-- Table: FeatureGating
CREATE TABLE FeatureGating (
    featureName TEXT4 PRIMARY KEY,
    major       INT32,
    [current]   INT32,
    minor       INT32,
    description TEXT4
);


-- Table: FlairTable
CREATE TABLE FlairTable (
    id    INT32 PRIMARY KEY,
    asset TEXT4
);


-- Table: Icons
CREATE TABLE Icons (
    IconID   INT32 PRIMARY KEY,
    IconPath TEXT4,
    IconName TEXT4
);


-- Table: InventoryComponent
CREATE TABLE InventoryComponent (
    id     INT32,
    itemid INT32    REFERENCES Objects (id),
    count  INT32,
    equip  INT_BOOL,
    PRIMARY KEY (
        id,
        itemid
    )
);


-- Table: ItemComponent
CREATE TABLE ItemComponent (
    id                     INT32    PRIMARY KEY,
    equipLocation          TEXT4,
    baseValue              INT32,
    isKitPiece             INT_BOOL,
    rarity                 INT32,
    itemType               INT32    REFERENCES mapItemTypes (id),
    itemInfo               INT64,
    inLootTable            INT_BOOL,
    inVendor               INT_BOOL,
    isUnique               INT_BOOL,
    isBOP                  INT_BOOL,
    isBOE                  INT_BOOL,
    reqFlagID              INT32,
    reqSpecialtyID         INT32,
    reqSpecRank            INT32,
    reqAchievementID       INT32,
    stackSize              INT32,
    color1                 INT32,
    decal                  INT32,
    offsetGroupID          INT32,
    buildTypes             INT32,
    reqPrecondition        TEXT4,
    animationFlag          INT32,
    equipEffects           INT32,
    readyForQA             INT_BOOL,
    itemRating             INT32,
    isTwoHanded            INT_BOOL,
    minNumRequired         INT32,
    delResIndex            INT32,
    currencyLOT            INT32    REFERENCES Objects (id),
    altCurrencyCost        INT32,
    subItems               TEXT4,
    audioEventUse          TEXT4,
    noEquipAnimation       INT_BOOL,
    commendationLOT        INT32    REFERENCES Objects (id),
    commendationCost       INT32,
    audioEquipMetaEventSet TEXT4,
    currencyCosts          TEXT4,
    ingredientInfo         TEXT4,
    locStatus              INT32,
    forgeType              INT32,
    SellMultiplier         REAL
);


-- Table: ItemEggData
CREATE TABLE ItemEggData (
    id              INT32 PRIMARY KEY,
    chassie_type_id INT32
);


-- Table: ItemFoodData
CREATE TABLE ItemFoodData (
    id               INT32 PRIMARY KEY,
    element_1        INT32,
    element_1_amount INT32,
    element_2        INT32,
    element_2_amount INT32,
    element_3        INT32,
    element_3_amount INT32,
    element_4        INT32,
    element_4_amount INT32
);


-- Table: ItemSets
CREATE TABLE ItemSets (
    setID         INT32    PRIMARY KEY,
    locStatus     INT32,
    itemIDs       TEXT4,
    kitType       INT32,
    kitRank       INT32,
    kitImage      INT32    REFERENCES Icons (IconID),
    skillSetWith2 INT32,
    skillSetWith3 INT32,
    skillSetWith4 INT32,
    skillSetWith5 INT32,
    skillSetWith6 INT32,
    localize      INT_BOOL,
    gate_version  TEXT4    REFERENCES FeatureGating (featureName),
    kitID         INT32,
    priority      REAL
);


-- Table: ItemSetSkills
CREATE TABLE ItemSetSkills (
    SkillSetID    INT32,
    SkillID       INT32 REFERENCES SkillBehavior (skillID),
    SkillCastType INT32,
    PRIMARY KEY (
        SkillSetID,
        SkillID
    )
);


-- Table: JetPackPadComponent
CREATE TABLE JetPackPadComponent (
    id               INT32 PRIMARY KEY,
    xDistance        REAL,
    yDistance        REAL,
    warnDistance     REAL,
    lotBlocker       INT32 REFERENCES Objects (id),
    lotWarningVolume INT32 REFERENCES Objects (id) 
);


-- Table: LanguageType
CREATE TABLE LanguageType (
    LanguageID          INT32 PRIMARY KEY,
    LanguageDescription TEXT4
);


-- Table: LevelProgressionLookup
CREATE TABLE LevelProgressionLookup (
    id             INT32 PRIMARY KEY,
    requiredUScore INT32,
    BehaviorEffect TEXT4
);


-- Table: LootMatrix
CREATE TABLE LootMatrix (
    LootMatrixIndex  INT32 REFERENCES LootMatrixIndex (LootMatrixIndex),
    LootTableIndex   INT32 REFERENCES LootTable (LootTableIndex),
    RarityTableIndex INT32,
    percent          REAL,
    minToDrop        INT32,
    maxToDrop        INT32,
    id               INT32,
    flagID           INT32,
    gate_version     TEXT4 REFERENCES FeatureGating (featureName),
    PRIMARY KEY (
        LootMatrixIndex,
        LootTableIndex
    )
);


-- Table: LootMatrixIndex
CREATE TABLE LootMatrixIndex (
    LootMatrixIndex INT32    PRIMARY KEY,
    inNpcEditor     INT_BOOL
);


-- Table: LootTable
CREATE TABLE LootTable (
    itemid         INT32    REFERENCES Objects (id),
    LootTableIndex INT32    REFERENCES LootTableIndex (LootTableIndex),
    id             INT32    PRIMARY KEY,
    MissionDrop    INT_BOOL,
    sortPriority   INT32
);


-- Table: LootTableIndex
CREATE TABLE LootTableIndex (
    LootTableIndex INT32 PRIMARY KEY
);


-- Table: LUPExhibitComponent
CREATE TABLE LUPExhibitComponent (
    id      INT32 PRIMARY KEY,
    minXZ   REAL,
    maxXZ   REAL,
    maxY    REAL,
    offsetX REAL,
    offsetY REAL,
    offsetZ REAL
);


-- Table: LUPExhibitModelData
CREATE TABLE LUPExhibitModelData (
    LOT         INT32 REFERENCES Objects (id)
                      PRIMARY KEY,
    minXZ       REAL,
    maxXZ       REAL,
    maxY        REAL,
    description TEXT4,
    owner       TEXT4
);


-- Table: LUPZoneIDs
CREATE TABLE LUPZoneIDs (
    zoneID INT32 REFERENCES ZoneTable (zoneID)
               PRIMARY KEY
);


-- Table: map_BlueprintCategory
CREATE TABLE map_BlueprintCategory (
    id          INT32 PRIMARY KEY,
    description TEXT4,
    enabled     INT_BOOL
);


-- Table: mapAnimationPriorities
CREATE TABLE mapAnimationPriorities (
    id       INT32 PRIMARY KEY,
    name     TEXT4,
    priority REAL
);


-- Table: mapAssetType
CREATE TABLE mapAssetType (
    id        INT32 PRIMARY KEY,
    label     TEXT4,
    pathdir   TEXT4,
    typelabel TEXT4
);


-- Table: mapIcon
CREATE TABLE mapIcon (
    LOT       INT32 REFERENCES Objects (id),
    iconID    INT32 REFERENCES Icons (IconID),
    iconState INT32,
    PRIMARY KEY (
        LOT,
        iconID,
        iconState
    )
);


-- Table: mapItemTypes
CREATE TABLE mapItemTypes (
    id            INT32 PRIMARY KEY,
    description   TEXT4,
    equipLocation TEXT4
);


-- Table: mapRenderEffects
CREATE TABLE mapRenderEffects (
    id          INT32 PRIMARY KEY,
    gameID      INT32,
    description TEXT4
);


-- Table: mapShaders
CREATE TABLE mapShaders (
    id        INT32 PRIMARY KEY,
    label     TEXT4,
    gameValue INT32,
    priority  INT32
);


-- Table: mapTextureResource
CREATE TABLE mapTextureResource (
    id          INT32 PRIMARY KEY,
    texturepath TEXT4,
    SurfaceType INT32
);


-- Table: MinifigComponent
CREATE TABLE MinifigComponent (
    id           INT32 PRIMARY KEY,
    head         INT32,
    chest        INT32,
    legs         INT32,
    hairstyle    INT32,
    haircolor    INT32,
    chestdecal   INT32,
    headcolor    INT32,
    lefthand     INT32,
    righthand    INT32,
    eyebrowstyle INT32 REFERENCES MinifigDecals_Eyebrows (ID),
    eyesstyle    INT32 REFERENCES MinifigDecals_Eyes (ID),
    mouthstyle   INT32 REFERENCES MinifigDecals_Mouths (ID) 
);


-- Table: MinifigDecals_Eyebrows
CREATE TABLE MinifigDecals_Eyebrows (
    ID                   INT32    PRIMARY KEY,
    High_path            TEXT4,
    Low_path             TEXT4,
    CharacterCreateValid INT_BOOL,
    male                 INT_BOOL,
    female               INT_BOOL
);


-- Table: MinifigDecals_Eyes
CREATE TABLE MinifigDecals_Eyes (
    ID                   INT32    PRIMARY KEY,
    High_path            TEXT4,
    Low_path             TEXT4,
    CharacterCreateValid INT_BOOL,
    male                 INT_BOOL,
    female               INT_BOOL
);


-- Table: MinifigDecals_Legs
CREATE TABLE MinifigDecals_Legs (
    ID        INT32 PRIMARY KEY,
    High_path TEXT4
);


-- Table: MinifigDecals_Mouths
CREATE TABLE MinifigDecals_Mouths (
    ID                   INT32    PRIMARY KEY,
    High_path            TEXT4,
    Low_path             TEXT4,
    CharacterCreateValid INT_BOOL,
    male                 INT_BOOL,
    female               INT_BOOL
);


-- Table: MinifigDecals_Torsos
CREATE TABLE MinifigDecals_Torsos (
    ID                   INT32    PRIMARY KEY,
    High_path            TEXT4,
    CharacterCreateValid INT_BOOL,
    male                 INT_BOOL,
    female               INT_BOOL
);


-- Table: MissionEmail
CREATE TABLE MissionEmail (
    ID                INT32    PRIMARY KEY,
    messageType       INT32,
    notificationGroup INT32,
    missionID         INT32    REFERENCES Missions (id),
    attachmentLOT     INT32    REFERENCES Objects (id),
    localize          INT_BOOL,
    locStatus         INT32,
    gate_version      TEXT4    REFERENCES FeatureGating (featureName)
);


-- Table: MissionNPCComponent
CREATE TABLE MissionNPCComponent (
    id             INT32,
    missionID      INT32    REFERENCES Missions (id),
    offersMission  INT_BOOL,
    acceptsMission INT_BOOL,
    gate_version   TEXT4    REFERENCES FeatureGating (featureName),
    PRIMARY KEY (
        id,
        missionID
    )
);


-- Table: Missions
CREATE TABLE Missions (
    id                         INT32    PRIMARY KEY,
    defined_type               TEXT4,
    defined_subtype            TEXT4,
    UISortOrder                INT32,
    offer_objectID             INT32    REFERENCES Objects (id),
    target_objectID            INT32    REFERENCES Objects (id),
    reward_currency            INT64,
    LegoScore                  INT32,
    reward_reputation          INT64,
    isChoiceReward             INT_BOOL,
    reward_item1               INT32    REFERENCES Objects (id),
    reward_item1_count         INT32,
    reward_item2               INT32    REFERENCES Objects (id),
    reward_item2_count         INT32,
    reward_item3               INT32    REFERENCES Objects (id),
    reward_item3_count         INT32,
    reward_item4               INT32    REFERENCES Objects (id),
    reward_item4_count         INT32,
    reward_emote               INT32    REFERENCES Emotes (id),
    reward_emote2              INT32    REFERENCES Emotes (id),
    reward_emote3              INT32    REFERENCES Emotes (id),
    reward_emote4              INT32    REFERENCES Emotes (id),
    reward_maximagination      INT32,
    reward_maxhealth           INT32,
    reward_maxinventory        INT32,
    reward_maxmodel            INT32,
    reward_maxwidget           INT32,
    reward_maxwallet           INT64,
    repeatable                 INT_BOOL,
    reward_currency_repeatable INT64,
    reward_item1_repeatable    INT32    REFERENCES Objects (id),
    reward_item1_repeat_count  INT32,
    reward_item2_repeatable    INT32    REFERENCES Objects (id),
    reward_item2_repeat_count  INT32,
    reward_item3_repeatable    INT32    REFERENCES Objects (id),
    reward_item3_repeat_count  INT32,
    reward_item4_repeatable    INT32    REFERENCES Objects (id),
    reward_item4_repeat_count  INT32,
    time_limit                 INT32,
    isMission                  INT_BOOL,
    missionIconID              INT32    REFERENCES Icons (IconID),
    prereqMissionID            TEXT4,
    localize                   INT_BOOL,
    inMOTD                     INT_BOOL,
    cooldownTime               INT64,
    isRandom                   INT_BOOL,
    randomPool                 TEXT4,
    UIPrereqID                 INT32,
    gate_version               TEXT4    REFERENCES FeatureGating (featureName),
    HUDStates                  TEXT4,
    locStatus                  INT32,
    reward_bankinventory       INT32
);


-- Table: MissionTasks
CREATE TABLE MissionTasks (
    id              INT32    REFERENCES Missions (id),
    locStatus       INT32,
    taskType        INT32,
    target          INT32,
    targetGroup     TEXT4,
    targetValue     INT32,
    taskParam1      TEXT4,
    largeTaskIcon   TEXT4,
    IconID          INT32    REFERENCES Icons (IconID),
    uid             INT32    PRIMARY KEY,
    largeTaskIconID INT32    REFERENCES Icons (IconID),
    localize        INT_BOOL,
    gate_version    TEXT4    REFERENCES FeatureGating (featureName)
);


-- Table: MissionText
CREATE TABLE MissionText (
    id                                 INT32    REFERENCES Missions (id)
                                                PRIMARY KEY,
    story_icon                         TEXT4,
    missionIcon                        TEXT4,
    offerNPCIcon                       TEXT4,
    IconID                             INT32    REFERENCES Icons (IconID),
    state_1_anim                       TEXT4,
    state_2_anim                       TEXT4,
    state_3_anim                       TEXT4,
    state_4_anim                       TEXT4,
    state_3_turnin_anim                TEXT4,
    state_4_turnin_anim                TEXT4,
    onclick_anim                       TEXT4,
    CinematicAccepted                  TEXT4,
    CinematicAcceptedLeadin            REAL,
    CinematicCompleted                 TEXT4,
    CinematicCompletedLeadin           REAL,
    CinematicRepeatable                TEXT4,
    CinematicRepeatableLeadin          REAL,
    CinematicRepeatableCompleted       TEXT4,
    CinematicRepeatableCompletedLeadin REAL,
    AudioEventGUID_Interact            TEXT4,
    AudioEventGUID_OfferAccept         TEXT4,
    AudioEventGUID_OfferDeny           TEXT4,
    AudioEventGUID_Completed           TEXT4,
    AudioEventGUID_TurnIn              TEXT4,
    AudioEventGUID_Failed              TEXT4,
    AudioEventGUID_Progress            TEXT4,
    AudioMusicCue_OfferAccept          TEXT4,
    AudioMusicCue_TurnIn               TEXT4,
    turnInIconID                       INT32    REFERENCES Icons (IconID),
    localize                           INT_BOOL,
    locStatus                          INT32,
    gate_version                       TEXT4    REFERENCES FeatureGating (featureName)
);


-- Table: ModelBehavior
CREATE TABLE ModelBehavior (
    id                    INT32 PRIMARY KEY,
    definitionXMLfilename TEXT4
);


-- Table: ModularBuildComponent
CREATE TABLE ModularBuildComponent (
    id                      INT32    PRIMARY KEY,
    buildType               INT32,
    xml                     TEXT_XML,
    createdLOT              INT32    REFERENCES Objects (id),
    createdPhysicsID        INT32,
    AudioEventGUID_Snap     TEXT4,
    AudioEventGUID_Complete TEXT4,
    AudioEventGUID_Present  TEXT4
);


-- Table: ModuleComponent
CREATE TABLE ModuleComponent (
    id                INT32    PRIMARY KEY,
    partCode          INT32,
    buildType         INT32,
    xml               TEXT_XML,
    primarySoundGUID  TEXT4,
    assembledEffectID INT32
);


-- Table: MotionFX
CREATE TABLE MotionFX (
    id            INT32 PRIMARY KEY,
    typeID        INT32,
    slamVelocity  REAL,
    addVelocity   REAL,
    duration      REAL,
    destGroupName TEXT4,
    startScale    REAL,
    endScale      REAL,
    velocity      REAL,
    distance      REAL
);


-- Table: MovementAIComponent
CREATE TABLE MovementAIComponent (
    id             INT32 PRIMARY KEY,
    MovementType   TEXT4,
    WanderChance   REAL,
    WanderDelayMin REAL,
    WanderDelayMax REAL,
    WanderSpeed    REAL,
    WanderRadius   REAL,
    attachedPath   TEXT4
);


-- Table: MovingPlatforms
CREATE TABLE MovingPlatforms (
    id                    INT32    PRIMARY KEY,
    platformIsSimpleMover INT_BOOL,
    platformMoveX         REAL,
    platformMoveY         REAL,
    platformMoveZ         REAL,
    platformMoveTime      REAL,
    platformStartAtEnd    INT_BOOL,
    description           TEXT4
);


-- Table: NpcIcons
CREATE TABLE NpcIcons (
    id                             INT32    PRIMARY KEY,
    color                          INT32,
    [offset]                       REAL,
    LOT                            INT32    REFERENCES Objects (id),
    Texture                        TEXT4,
    isClickable                    INT_BOOL,
    scale                          REAL,
    rotateToFace                   INT_BOOL,
    compositeHorizOffset           REAL,
    compositeVertOffset            REAL,
    compositeScale                 REAL,
    compositeConnectionNode        TEXT4,
    compositeLOTMultiMission       INT32    REFERENCES Objects (id),
    compositeLOTMultiMissionVentor INT32    REFERENCES Objects (id),
    compositeIconTexture           TEXT4
);


-- Table: ObjectBehaviors
CREATE TABLE ObjectBehaviors (
    BehaviorID INT64 PRIMARY KEY,
    xmldata    TEXT_XML
);


-- Table: ObjectBehaviorXREF
CREATE TABLE ObjectBehaviorXREF (
    LOT         INT32 PRIMARY KEY,
    behaviorID1 INT64,
    behaviorID2 INT64,
    behaviorID3 INT64,
    behaviorID4 INT64,
    behaviorID5 INT64,
    type        INT32
);


-- Table: Objects
CREATE TABLE Objects (
    id                  INT32    PRIMARY KEY,
    name                TEXT4,
    placeable           INT_BOOL,
    type                TEXT4,
    description         TEXT4,
    localize            INT_BOOL,
    npcTemplateID       INT32,
    displayName         TEXT4,
    interactionDistance REAL,
    nametag             INT_BOOL,
    _internalNotes      TEXT4,
    locStatus           INT32,
    gate_version        TEXT4    REFERENCES FeatureGating (featureName),
    HQ_valid            INT_BOOL
);


-- Table: ObjectSkills
CREATE TABLE ObjectSkills (
    objectTemplate INT32 REFERENCES Objects (id),
    skillID        INT32 REFERENCES SkillBehavior (skillID),
    castOnType     INT32,
    AICombatWeight INT32,
    PRIMARY KEY (
        objectTemplate,
        skillID
    )
);


-- Table: PackageComponent
CREATE TABLE PackageComponent (
    id              INT32 PRIMARY KEY,
    LootMatrixIndex INT32 REFERENCES LootMatrix (LootMatrixIndex),
    packageType     INT32
);


-- Table: PetAbilities
CREATE TABLE PetAbilities (
    id              INT32 PRIMARY KEY,
    AbilityName     TEXT4,
    ImaginationCost INT32,
    locStatus       INT32
);


-- Table: PetComponent
CREATE TABLE PetComponent (
    id                   INT32 PRIMARY KEY,
    minTameUpdateTime    REAL,
    maxTameUpdateTime    REAL,
    percentTameChance    REAL,
    tamability           REAL,
    elementType          INT32,
    walkSpeed            REAL,
    runSpeed             REAL,
    sprintSpeed          REAL,
    idleTimeMin          REAL,
    idleTimeMax          REAL,
    petForm              INT32,
    imaginationDrainRate REAL,
    AudioMetaEventSet    TEXT4,
    buffIDs              TEXT4
);


-- Table: PetNestComponent
CREATE TABLE PetNestComponent (
    id            INT32 PRIMARY KEY,
    ElementalType INT32
);


-- Table: PhysicsComponent
CREATE TABLE PhysicsComponent (
    id                 INT32 PRIMARY KEY,
    static             REAL,
    physics_asset      TEXT4,
    jump               REAL,
    doublejump         REAL,
    speed              REAL,
    rotSpeed           REAL,
    playerHeight       REAL,
    playerRadius       REAL,
    pcShapeType        INT32,
    collisionGroup     INT32,
    airSpeed           REAL,
    boundaryAsset      TEXT4,
    jumpAirSpeed       REAL,
    friction           REAL,
    gravityVolumeAsset TEXT4
);


-- Table: PlayerFlags
CREATE TABLE PlayerFlags (
    id              INT32    PRIMARY KEY,
    SessionOnly     INT_BOOL,
    OnlySetByServer INT_BOOL,
    SessionZoneOnly INT_BOOL
);


-- Table: PlayerStatistics
CREATE TABLE PlayerStatistics (
    statID       INT32 PRIMARY KEY,
    sortOrder    INT32,
    locStatus    INT32,
    gate_version TEXT4 REFERENCES FeatureGating (featureName)
);


-- Table: PossessableComponent
CREATE TABLE PossessableComponent (
    id                     INT32    PRIMARY KEY,
    controlSchemeID        INT32    REFERENCES ControlSchemes (control_scheme),
    minifigAttachPoint     TEXT4,
    minifigAttachAnimation TEXT4,
    minifigDetachAnimation TEXT4,
    mountAttachAnimation   TEXT4,
    mountDetachAnimation   TEXT4,
    attachOffsetFwd        REAL,
    attachOffsetRight      REAL,
    possessionType         INT32,
    wantBillboard          INT_BOOL,
    billboardOffsetUp      REAL,
    depossessOnHit         INT_BOOL,
    hitStunTime            REAL,
    skillSet               INT32
);


-- Table: Preconditions
CREATE TABLE Preconditions (
    id            INT32    PRIMARY KEY,
    type          INT32,
    targetLOT     TEXT4,
    targetGroup   TEXT4,
    targetCount   INT32,
    iconID        INT32    REFERENCES Icons (IconID),
    localize      INT_BOOL,
    validContexts INT64,
    locStatus     INT32,
    gate_version  TEXT4    REFERENCES FeatureGating (featureName)
);


-- Table: PropertyEntranceComponent
CREATE TABLE PropertyEntranceComponent (
    id           INT32    PRIMARY KEY,
    mapID        INT32    REFERENCES ZoneTable (zoneID),
    propertyName TEXT4,
    isOnProperty INT_BOOL,
    groupType    TEXT4
);


-- Table: PropertyTemplate
CREATE TABLE PropertyTemplate (
    id                  INT32    PRIMARY KEY,
    mapID               INT32    REFERENCES ZoneTable (zoneID),
    vendorMapID         INT32    REFERENCES ZoneTable (zoneID),
    spawnName           TEXT4,
    type                INT32,
    sizecode            INT32,
    minimumPrice        INT32,
    rentDuration        INT32,
    path                TEXT4,
    cloneLimit          INT32,
    durationType        INT32,
    achievementRequired INT32,
    zoneX               REAL,
    zoneY               REAL,
    zoneZ               REAL,
    maxBuildHeight      REAL,
    localize            INT_BOOL,
    reputationPerMinute INT32,
    locStatus           INT32,
    gate_version        TEXT4    REFERENCES FeatureGating (featureName)
);


-- Table: ProximityMonitorComponent
CREATE TABLE ProximityMonitorComponent (
    id           INT32    PRIMARY KEY,
    Proximities  TEXT4,
    LoadOnClient INT_BOOL,
    LoadOnServer INT_BOOL
);


-- Table: ProximityTypes
CREATE TABLE ProximityTypes (
    id             INT32    PRIMARY KEY,
    Name           TEXT4,
    Radius         INT32,
    CollisionGroup INT32,
    PassiveChecks  INT_BOOL,
    IconID         INT32    REFERENCES Icons (IconID),
    LoadOnClient   INT_BOOL,
    LoadOnServer   INT_BOOL
);


-- Table: RacingModuleComponent
CREATE TABLE RacingModuleComponent (
    id           INT32 PRIMARY KEY,
    topSpeed     REAL,
    acceleration REAL,
    handling     REAL,
    stability    REAL,
    imagination  REAL
);


-- Table: RailActivatorComponent
CREATE TABLE RailActivatorComponent (
    id                INT32    PRIMARY KEY,
    startAnim         TEXT4,
    loopAnim          TEXT4,
    stopAnim          TEXT4,
    startSound        TEXT4,
    loopSound         TEXT4,
    stopSound         TEXT4,
    effectIDs         TEXT4,
    preconditions     TEXT4,
    playerCollision   INT_BOOL,
    cameraLocked      INT_BOOL,
    StartEffectID     TEXT4,
    StopEffectID      TEXT4,
    DamageImmune      INT_BOOL,
    NoAggro           INT_BOOL,
    ShowNameBillboard INT_BOOL
);


-- Table: RarityTable
CREATE TABLE RarityTable (
    id               INT32 PRIMARY KEY,
    randmax          REAL,
    rarity           INT32,
    RarityTableIndex INT32 REFERENCES RarityTableIndex (RarityTableIndex) 
);


-- Table: RarityTableIndex
CREATE TABLE RarityTableIndex (
    RarityTableIndex INT32 PRIMARY KEY
);


-- Table: RebuildComponent
CREATE TABLE RebuildComponent (
    id                    INT32    PRIMARY KEY,
    reset_time            REAL,
    complete_time         REAL,
    take_imagination      INT32,
    interruptible         INT_BOOL,
    self_activator        INT_BOOL,
    custom_modules        TEXT4,
    activityID            INT32    REFERENCES Activities (ActivityID),
    post_imagination_cost INT32,
    time_before_smash     REAL
);


-- Table: RebuildSections
CREATE TABLE RebuildSections (
    id            INT32    PRIMARY KEY,
    rebuildID     INT32,
    objectID      INT32    REFERENCES Objects (id),
    offset_x      REAL,
    offset_y      REAL,
    offset_z      REAL,
    fall_angle_x  REAL,
    fall_angle_y  REAL,
    fall_angle_z  REAL,
    fall_height   REAL,
    requires_list TEXT4,
    size          INT32,
    bPlaced       INT_BOOL
);


-- Table: Release_Version
CREATE TABLE Release_Version (
    ReleaseVersion TEXT4 PRIMARY KEY,
    ReleaseDate    INT64
);


-- Table: RenderComponent
CREATE TABLE RenderComponent (
    id                     INT32    PRIMARY KEY,
    render_asset           TEXT4,
    icon_asset             TEXT4,
    IconID                 INT32    REFERENCES Icons (IconID),
    shader_id              INT32,
    effect1                INT32,
    effect2                INT32,
    effect3                INT32,
    effect4                INT32,
    effect5                INT32,
    effect6                INT32,
    animationGroupIDs      TEXT4,
    fade                   INT_BOOL,
    usedropshadow          INT_BOOL,
    preloadAnimations      INT_BOOL,
    fadeInTime             REAL,
    maxShadowDistance      REAL,
    ignoreCameraCollision  INT_BOOL,
    renderComponentLOD1    INT32,
    renderComponentLOD2    INT32,
    gradualSnap            INT_BOOL,
    animationFlag          INT32,
    AudioMetaEventSet      TEXT4,
    billboardHeight        REAL,
    chatBubbleOffset       REAL,
    staticBillboard        INT_BOOL,
    LXFMLFolder            TEXT4,
    attachIndicatorsToNode INT_BOOL
);


-- Table: RenderComponentFlash
CREATE TABLE RenderComponentFlash (
    id          INT32,
    interactive INT_BOOL,
    animated    INT_BOOL,
    nodeName    TEXT4,
    flashPath   TEXT4,
    elementName TEXT4,
    _uid        INT32    PRIMARY KEY
);


-- Table: RenderComponentWrapper
CREATE TABLE RenderComponentWrapper (
    id                  INT32 PRIMARY KEY,
    defaultWrapperAsset TEXT4
);


-- Table: RenderIconAssets
CREATE TABLE RenderIconAssets (
    id           INT32 PRIMARY KEY,
    icon_asset   TEXT4,
    blank_column TEXT4
);


-- Table: ReputationRewards
CREATE TABLE ReputationRewards (
    repLevel   INT32,
    sublevel   INT32,
    reputation REAL,
    PRIMARY KEY (
        repLevel,
        sublevel
    )
);


-- Table: RewardCodes
CREATE TABLE RewardCodes (
    id            INT32 PRIMARY KEY,
    code          TEXT4,
    attachmentLOT INT32 REFERENCES Objects (id),
    locStatus     INT32,
    gate_version  TEXT4 REFERENCES FeatureGating (featureName)
);


-- Table: Rewards
CREATE TABLE Rewards (
    id         INT32 PRIMARY KEY,
    LevelID    INT32,
    MissionID  INT32,
    RewardType INT32,
    value      INT32,
    count      INT32
);


-- Table: RocketLaunchpadControlComponent
CREATE TABLE RocketLaunchpadControlComponent (
    id                        INT32    PRIMARY KEY,
    targetZone                INT32    REFERENCES ZoneTable (zoneID),
    defaultZoneID             INT32    REFERENCES ZoneTable (zoneID),
    targetScene               TEXT4,
    gmLevel                   INT32,
    playerAnimation           TEXT4,
    rocketAnimation           TEXT4,
    launchMusic               TEXT4,
    useLaunchPrecondition     INT_BOOL,
    useAltLandingPrecondition INT_BOOL,
    launchPrecondition        TEXT4,
    altLandingPrecondition    TEXT4,
    altLandingSpawnPointName  TEXT4
);


-- Table: SceneTable
CREATE TABLE SceneTable (
    sceneID   INT32 PRIMARY KEY,
    sceneName TEXT4
);


-- Table: ScriptComponent
CREATE TABLE ScriptComponent (
    id                 INT32 PRIMARY KEY,
    script_name        TEXT4,
    client_script_name TEXT4
);


-- Table: SkillBehavior
CREATE TABLE SkillBehavior (
    skillID             INT32    PRIMARY KEY,
    locStatus           INT32,
    behaviorID          INT32    REFERENCES BehaviorTemplate (behaviorID),
    imaginationcost     INT32,
    cooldowngroup       INT32,
    cooldown            REAL,
    inNpcEditor         INT_BOOL,
    skillIcon           INT32    REFERENCES Icons (IconID),
    oomSkillID          TEXT4,
    oomBehaviorEffectID INT32,
    castTypeDesc        INT32,
    imBonusUI           INT32,
    lifeBonusUI         INT32,
    armorBonusUI        INT32,
    damageUI            INT32,
    hideIcon            INT_BOOL,
    localize            INT_BOOL,
    gate_version        TEXT4    REFERENCES FeatureGating (featureName),
    cancelType          INT32
);


-- Table: SmashableChain
CREATE TABLE SmashableChain (
    chainIndex       INT32,
    chainLevel       INT32,
    lootMatrixID     INT32 REFERENCES LootMatrix (LootMatrixIndex),
    rarityTableIndex INT32 REFERENCES RarityTable (id),
    currencyIndex    INT32,
    currencyLevel    INT32,
    smashCount       INT32,
    timeLimit        INT32,
    chainStepID      INT32,
    PRIMARY KEY (
        chainIndex,
        chainLevel
    )
);


-- Table: SmashableChainIndex
CREATE TABLE SmashableChainIndex (
    id          INT32 PRIMARY KEY,
    targetGroup TEXT4,
    description TEXT4,
    continuous  INT32
);


-- Table: SmashableComponent
CREATE TABLE SmashableComponent (
    id              INT32 PRIMARY KEY,
    LootMatrixIndex INT32 REFERENCES LootMatrix (LootMatrixIndex) 
);


-- Table: SmashableElements
CREATE TABLE SmashableElements (
    elementID  INT32 PRIMARY KEY,
    dropWeight INT32
);


-- Table: SpeedchatMenu
CREATE TABLE SpeedchatMenu (
    id           INT32    PRIMARY KEY,
    parentId     INT32,
    emoteId      INT32    REFERENCES Emotes (id),
    imageName    TEXT4,
    localize     INT_BOOL,
    locStatus    INT32,
    gate_version TEXT4    REFERENCES FeatureGating (featureName)
);


-- Table: SubscriptionPricing
CREATE TABLE SubscriptionPricing (
    id               INT32    PRIMARY KEY,
    countryCode      TEXT4,
    monthlyFeeGold   TEXT4,
    monthlyFeeSilver TEXT4,
    monthlyFeeBronze TEXT4,
    monetarySymbol   INT32,
    symbolIsAppended INT_BOOL
);


-- Table: SurfaceType
CREATE TABLE SurfaceType (
    SurfaceType                     INT32 PRIMARY KEY,
    FootstepNDAudioMetaEventSetName TEXT4
);


-- Table: sysdiagrams
CREATE TABLE sysdiagrams (
    name         TEXT4 PRIMARY KEY,
    principal_id INT32,
    diagram_id   INT32,
    version      INT32,
    definition   TEXT4
);


-- Table: TamingBuildPuzzles
CREATE TABLE TamingBuildPuzzles (
    id               INT32 PRIMARY KEY,
    PuzzleModelLot   INT32 REFERENCES Objects (id),
    NPCLot           INT32 REFERENCES Objects (id),
    ValidPiecesLXF   TEXT4,
    InvalidPiecesLXF TEXT4,
    Difficulty       INT32,
    Timelimit        INT32,
    NumValidPieces   INT32,
    TotalNumPieces   INT32,
    ModelName        TEXT4,
    FullModelLXF     TEXT4,
    Duration         REAL,
    imagCostPerBuild INT32
);


-- Table: TextDescription
CREATE TABLE TextDescription (
    TextID          INT32 PRIMARY KEY,
    TestDescription TEXT4
);


-- Table: TextLanguage
CREATE TABLE TextLanguage (
    TextID     INT32,
    LanguageID INT32,
    Text       TEXT4,
    PRIMARY KEY (
        TextID,
        LanguageID
    )
);


-- Table: TrailEffects
CREATE TABLE TrailEffects (
    trailID       INT32 PRIMARY KEY,
    textureName   TEXT4,
    blendmode     INT32,
    cardlifetime  REAL,
    colorlifetime REAL,
    minTailFade   REAL,
    tailFade      REAL,
    max_particles INT32,
    birthDelay    REAL,
    deathDelay    REAL,
    bone1         TEXT4,
    bone2         TEXT4,
    texLength     REAL,
    texWidth      REAL,
    startColorR   REAL,
    startColorG   REAL,
    startColorB   REAL,
    startColorA   REAL,
    middleColorR  REAL,
    middleColorG  REAL,
    middleColorB  REAL,
    middleColorA  REAL,
    endColorR     REAL,
    endColorG     REAL,
    endColorB     REAL,
    endColorA     REAL
);


-- Table: UGBehaviorSounds
CREATE TABLE UGBehaviorSounds (
    id           INT32    PRIMARY KEY,
    guid         TEXT4,
    localize     INT_BOOL,
    locStatus    INT32,
    gate_version TEXT4    REFERENCES FeatureGating (featureName)
);


-- Table: VehiclePhysics
CREATE TABLE VehiclePhysics (
    id                               INT32    PRIMARY KEY,
    hkxFilename                      TEXT4,
    fGravityScale                    REAL,
    fMass                            REAL,
    fChassisFriction                 REAL,
    fMaxSpeed                        REAL,
    fEngineTorque                    REAL,
    fBrakeFrontTorque                REAL,
    fBrakeRearTorque                 REAL,
    fBrakeMinInputToBlock            REAL,
    fBrakeMinTimeToBlock             REAL,
    fSteeringMaxAngle                REAL,
    fSteeringSpeedLimitForMaxAngle   REAL,
    fSteeringMinAngle                REAL,
    fFwdBias                         REAL,
    fFrontTireFriction               REAL,
    fRearTireFriction                REAL,
    fFrontTireFrictionSlide          REAL,
    fRearTireFrictionSlide           REAL,
    fFrontTireSlipAngle              REAL,
    fRearTireSlipAngle               REAL,
    fWheelWidth                      REAL,
    fWheelRadius                     REAL,
    fWheelMass                       REAL,
    fReorientPitchStrength           REAL,
    fReorientRollStrength            REAL,
    fSuspensionLength                REAL,
    fSuspensionStrength              REAL,
    fSuspensionDampingCompression    REAL,
    fSuspensionDampingRelaxation     REAL,
    iChassisCollisionGroup           INT32,
    fNormalSpinDamping               REAL,
    fCollisionSpinDamping            REAL,
    fCollisionThreshold              REAL,
    fTorqueRollFactor                REAL,
    fTorquePitchFactor               REAL,
    fTorqueYawFactor                 REAL,
    fInertiaRoll                     REAL,
    fInertiaPitch                    REAL,
    fInertiaYaw                      REAL,
    fExtraTorqueFactor               REAL,
    fCenterOfMassFwd                 REAL,
    fCenterOfMassUp                  REAL,
    fCenterOfMassRight               REAL,
    fWheelHardpointFrontFwd          REAL,
    fWheelHardpointFrontUp           REAL,
    fWheelHardpointFrontRight        REAL,
    fWheelHardpointRearFwd           REAL,
    fWheelHardpointRearUp            REAL,
    fWheelHardpointRearRight         REAL,
    fInputTurnSpeed                  REAL,
    fInputDeadTurnBackSpeed          REAL,
    fInputAccelSpeed                 REAL,
    fInputDeadAccelDownSpeed         REAL,
    fInputDecelSpeed                 REAL,
    fInputDeadDecelDownSpeed         REAL,
    fInputSlopeChangePointX          REAL,
    fInputInitialSlope               REAL,
    fInputDeadZone                   REAL,
    fAeroAirDensity                  REAL,
    fAeroFrontalArea                 REAL,
    fAeroDragCoefficient             REAL,
    fAeroLiftCoefficient             REAL,
    fAeroExtraGravity                REAL,
    fBoostTopSpeed                   REAL,
    fBoostCostPerSecond              REAL,
    fBoostAccelerateChange           REAL,
    fBoostDampingChange              REAL,
    fPowerslideNeutralAngle          REAL,
    fPowerslideTorqueStrength        REAL,
    iPowerslideNumTorqueApplications INT32,
    fImaginationTankSize             REAL,
    fSkillCost                       REAL,
    fWreckSpeedBase                  REAL,
    fWreckSpeedPercent               REAL,
    fWreckMinAngle                   REAL,
    AudioEventEngine                 TEXT4,
    AudioEventSkid                   TEXT4,
    AudioEventLightHit               TEXT4,
    AudioSpeedThresholdLightHit      REAL,
    AudioTimeoutLightHit             REAL,
    AudioEventHeavyHit               TEXT4,
    AudioSpeedThresholdHeavyHit      REAL,
    AudioTimeoutHeavyHit             REAL,
    AudioEventStart                  TEXT4,
    AudioEventTreadConcrete          TEXT4,
    AudioEventTreadSand              TEXT4,
    AudioEventTreadWood              TEXT4,
    AudioEventTreadDirt              TEXT4,
    AudioEventTreadPlastic           TEXT4,
    AudioEventTreadGrass             TEXT4,
    AudioEventTreadGravel            TEXT4,
    AudioEventTreadMud               TEXT4,
    AudioEventTreadWater             TEXT4,
    AudioEventTreadSnow              TEXT4,
    AudioEventTreadIce               TEXT4,
    AudioEventTreadMetal             TEXT4,
    AudioEventTreadLeaves            TEXT4,
    AudioEventLightLand              TEXT4,
    AudioAirtimeForLightLand         REAL,
    AudioEventHeavyLand              TEXT4,
    AudioAirtimeForHeavyLand         REAL,
    bWheelsVisible                   INT_BOOL
);


-- Table: VehicleStatMap
CREATE TABLE VehicleStatMap (
    id                       INT32,
    ModuleStat               TEXT4,
    HavokStat                TEXT4 PRIMARY KEY,
    HavokChangePerModuleStat REAL
);


-- Table: VendorComponent
CREATE TABLE VendorComponent (
    id                 INT32 PRIMARY KEY,
    buyScalar          REAL,
    sellScalar         REAL,
    refreshTimeSeconds REAL,
    LootMatrixIndex    INT32 REFERENCES LootMatrix (LootMatrixIndex) 
);


-- Table: WhatsCoolItemSpotlight
CREATE TABLE WhatsCoolItemSpotlight (
    id           INT32    PRIMARY KEY,
    itemID       INT32    REFERENCES Objects (id),
    localize     INT_BOOL,
    gate_version TEXT4    REFERENCES FeatureGating (featureName),
    locStatus    INT32
);


-- Table: WhatsCoolNewsAndTips
CREATE TABLE WhatsCoolNewsAndTips (
    id           INT32    PRIMARY KEY,
    iconID       INT32    REFERENCES Icons (IconID),
    type         INT32,
    localize     INT_BOOL,
    gate_version TEXT4    REFERENCES FeatureGating (featureName),
    locStatus    INT32
);


-- Table: WorldConfig
CREATE TABLE WorldConfig (
    WorldConfigID                             INT32 PRIMARY KEY,
    pegravityvalue                            REAL,
    pebroadphaseworldsize                     REAL,
    pegameobjscalefactor                      REAL,
    character_rotation_speed                  REAL,
    character_walk_forward_speed              REAL,
    character_walk_backward_speed             REAL,
    character_walk_strafe_speed               REAL,
    character_walk_strafe_forward_speed       REAL,
    character_walk_strafe_backward_speed      REAL,
    character_run_backward_speed              REAL,
    character_run_strafe_speed                REAL,
    character_run_strafe_forward_speed        REAL,
    character_run_strafe_backward_speed       REAL,
    global_cooldown                           REAL,
    characterGroundedTime                     REAL,
    characterGroundedSpeed                    REAL,
    globalImmunityTime                        REAL,
    character_max_slope                       REAL,
    defaultrespawntime                        REAL,
    mission_tooltip_timeout                   REAL,
    vendor_buy_multiplier                     REAL,
    pet_follow_radius                         REAL,
    character_eye_height                      REAL,
    flight_vertical_velocity                  REAL,
    flight_airspeed                           REAL,
    flight_fuel_ratio                         REAL,
    flight_max_airspeed                       REAL,
    fReputationPerVote                        REAL,
    nPropertyCloneLimit                       INT32,
    defaultHomespaceTemplate                  INT32,
    coins_lost_on_death_percent               REAL,
    coins_lost_on_death_min                   INT32,
    coins_lost_on_death_max                   INT32,
    character_votes_per_day                   INT32,
    property_moderation_request_approval_cost INT32,
    property_moderation_request_review_cost   INT32,
    propertyModRequestsAllowedSpike           INT32,
    propertyModRequestsAllowedInterval        INT32,
    propertyModRequestsAllowedTotal           INT32,
    propertyModRequestsSpikeDuration          INT32,
    propertyModRequestsIntervalDuration       INT32,
    modelModerateOnCreate                     INT_BOOL,
    defaultPropertyMaxHeight                  REAL,
    reputationPerVoteCast                     REAL,
    reputationPerVoteReceived                 REAL,
    showcaseTopModelConsiderationBattles      INT32,
    reputationPerBattlePromotion              REAL,
    coins_lost_on_death_min_timeout           REAL,
    coins_lost_on_death_max_timeout           REAL,
    mail_base_fee                             INT32,
    mail_percent_attachment_fee               REAL,
    propertyReputationDelay                   INT32,
    LevelCap                                  INT32,
    LevelUpBehaviorEffect                     TEXT4,
    CharacterVersion                          INT32,
    LevelCapCurrencyConversion                INT32
);


-- Table: ZoneLoadingTips
CREATE TABLE ZoneLoadingTips (
    id            INT32    PRIMARY KEY,
    zoneid        INT32    REFERENCES ZoneTable (zoneID),
    imagelocation TEXT4,
    localize      INT_BOOL,
    gate_version  TEXT4    REFERENCES FeatureGating (featureName),
    locStatus     INT32,
    weight        INT32,
    targetVersion TEXT4    REFERENCES FeatureGating (featureName)
);


-- Table: ZoneSummary
CREATE TABLE ZoneSummary (
    zoneID    INT32 REFERENCES ZoneTable (zoneID),
    type      INT32,
    value     INT32,
    _uniqueID INT32 PRIMARY KEY
);


-- Table: ZoneTable
CREATE TABLE ZoneTable (
    zoneID                 INT32    PRIMARY KEY,
    locStatus              INT32,
    zoneName               TEXT4,
    scriptID               INT32    REFERENCES ScriptComponent (id),
    ghostdistance_min      REAL,
    ghostdistance          REAL,
    population_soft_cap    INT32,
    population_hard_cap    INT32,
    DisplayDescription     TEXT4,
    mapFolder              TEXT4,
    smashableMinDistance   REAL,
    smashableMaxDistance   REAL,
    mixerProgram           TEXT4,
    clientPhysicsFramerate TEXT4,
    serverPhysicsFramerate TEXT4,
    zoneControlTemplate    INT32,
    widthInChunks          INT32,
    heightInChunks         INT32,
    petsAllowed            INT_BOOL,
    localize               INT_BOOL,
    fZoneWeight            REAL,
    thumbnail              TEXT4,
    PlayerLoseCoinsOnDeath INT_BOOL,
    disableSaveLoc         INT_BOOL,
    teamRadius             REAL,
    gate_version           TEXT4    REFERENCES FeatureGating (featureName),
    mountsAllowed          INT_BOOL
);


COMMIT TRANSACTION;
PRAGMA foreign_keys = on;
