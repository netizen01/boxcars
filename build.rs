extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("generated.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(&mut file, "use attributes::{{AttributeDecoder, AttributeDecodeFn}};\n ").unwrap();
    write!(&mut file, "use phf;\n ").unwrap();
    write!(&mut file, "use network::SpawnTrajectory;\n ").unwrap();

    write!(&mut file, "pub static SPAWN_STATS: phf::Map<&'static str, SpawnTrajectory> = ").unwrap();
    phf_codegen::Map::new()
        .entry("TAGame.Ball_Breakout_TA", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.Ball.Ball_Breakout", "SpawnTrajectory::LocationAndRotation")
        .entry("TAGame.Ball_TA", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.Ball.Ball_BasketBall_Mutator", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.Ball.Ball_BasketBall", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.Ball.Ball_Default", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.Ball.Ball_Puck", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.Ball.CubeBall", "SpawnTrajectory::LocationAndRotation")
        .entry("TAGame.Car_Season_TA", "SpawnTrajectory::LocationAndRotation")
        .entry("TAGame.Car_TA", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.Car.Car_Default", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.GameEvent.GameEvent_Season:CarArchetype", "SpawnTrajectory::LocationAndRotation")

        .entry("TAGame.CameraSettingsActor_TA", "SpawnTrajectory::Location")
        .entry("TAGame.CarComponent_Boost_TA", "SpawnTrajectory::Location")
        .entry("TAGame.CarComponent_Dodge_TA", "SpawnTrajectory::Location")
        .entry("TAGame.CarComponent_DoubleJump_TA", "SpawnTrajectory::Location")
        .entry("TAGame.CarComponent_FlipCar_TA", "SpawnTrajectory::Location")
        .entry("TAGame.CarComponent_Jump_TA", "SpawnTrajectory::Location")
        .entry("TAGame.GameEvent_Season_TA", "SpawnTrajectory::Location")
        .entry("TAGame.GameEvent_Soccar_TA", "SpawnTrajectory::Location")
        .entry("TAGame.GameEvent_SoccarPrivate_TA", "SpawnTrajectory::Location")
        .entry("TAGame.GameEvent_SoccarSplitscreen_TA", "SpawnTrajectory::Location")
        .entry("TAGame.GRI_TA", "SpawnTrajectory::Location")
        .entry("TAGame.PRI_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_BallCarSpring_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_BallFreeze_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_BallGravity_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_BallLasso_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_BallVelcro_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_Batarang_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_BoostOverride_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_GrapplingHook_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_HitForce_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_Swapper_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_Tornado_TA", "SpawnTrajectory::Location")
        .entry("TAGame.Team_Soccar_TA", "SpawnTrajectory::Location")
        .entry("Archetypes.CarComponents.CarComponent_Boost", "SpawnTrajectory::Location")
        .entry("Archetypes.CarComponents.CarComponent_Dodge", "SpawnTrajectory::Location")
        .entry("Archetypes.CarComponents.CarComponent_DoubleJump", "SpawnTrajectory::Location")
        .entry("Archetypes.CarComponents.CarComponent_FlipCar", "SpawnTrajectory::Location")
        .entry("Archetypes.CarComponents.CarComponent_Jump", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_Basketball", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_BasketballPrivate", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_BasketballSplitscreen", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_Breakout", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_Hockey", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_HockeyPrivate", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_HockeySplitscreen", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_Items", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_Season", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_Soccar", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_SoccarPrivate", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_SoccarSplitscreen", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallFreeze", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallGrapplingHook", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallLasso", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallSpring", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallVelcro", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_Batarang", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BoostOverride", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_CarSpring", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_GravityWell", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_StrongHit", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_Swapper", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_Tornado", "SpawnTrajectory::Location")
        .entry("Archetypes.Teams.Team0", "SpawnTrajectory::Location")
        .entry("Archetypes.Teams.Team1", "SpawnTrajectory::Location")
        .entry("GameInfo_Basketball.GameInfo.GameInfo_Basketball:GameReplicationInfoArchetype", "SpawnTrajectory::Location")
        .entry("GameInfo_Breakout.GameInfo.GameInfo_Breakout:GameReplicationInfoArchetype", "SpawnTrajectory::Location")
        .entry("Gameinfo_Hockey.GameInfo.Gameinfo_Hockey:GameReplicationInfoArchetype", "SpawnTrajectory::Location")
        .entry("GameInfo_Items.GameInfo.GameInfo_Items:GameReplicationInfoArchetype", "SpawnTrajectory::Location")
        .entry("GameInfo_Season.GameInfo.GameInfo_Season:GameReplicationInfoArchetype", "SpawnTrajectory::Location")
        .entry("GameInfo_Soccar.GameInfo.GameInfo_Soccar:GameReplicationInfoArchetype", "SpawnTrajectory::Location")
        .entry("TAGame.Default__CameraSettingsActor_TA", "SpawnTrajectory::Location")
        .entry("TAGame.Default__PRI_TA", "SpawnTrajectory::Location")
        .entry("TheWorld:PersistentLevel.BreakOutActor_Platform_TA", "SpawnTrajectory::Location")
        .entry("TheWorld:PersistentLevel.CrowdActor_TA", "SpawnTrajectory::Location")
        .entry("TheWorld:PersistentLevel.CrowdManager_TA", "SpawnTrajectory::Location")
        .entry("TheWorld:PersistentLevel.InMapScoreboard_TA", "SpawnTrajectory::Location")
        .entry("TheWorld:PersistentLevel.VehiclePickup_Boost_TA", "SpawnTrajectory::Location")
        .build(&mut file)
        .unwrap();

    write!(&mut file, ";\n").unwrap();


    write!(&mut file, "pub static ATTRIBUTES: phf::Map<&'static str, AttributeDecodeFn> = ").unwrap();
    phf_codegen::Map::new()
        .entry("Engine.Actor:bBlockActors", "AttributeDecoder::decode_boolean")
        .entry("Engine.Actor:bCollideActors", "AttributeDecoder::decode_boolean")
        .entry("Engine.Actor:bHidden", "AttributeDecoder::decode_boolean")
        .entry("Engine.Actor:DrawScale", "AttributeDecoder::decode_float")
        .entry("Engine.Actor:Role", "AttributeDecoder::decode_enum")
        .entry("Engine.GameReplicationInfo:bMatchIsOver", "AttributeDecoder::decode_boolean")
        .entry("Engine.GameReplicationInfo:GameClass", "AttributeDecoder::decode_flagged")
        .entry("Engine.GameReplicationInfo:ServerName", "AttributeDecoder::decode_string")
        .entry("Engine.Pawn:PlayerReplicationInfo", "AttributeDecoder::decode_flagged")
        .entry("Engine.PlayerReplicationInfo:bBot", "AttributeDecoder::decode_boolean")
        .entry("Engine.PlayerReplicationInfo:bIsSpectator", "AttributeDecoder::decode_boolean")
        .entry("Engine.PlayerReplicationInfo:bReadyToPlay", "AttributeDecoder::decode_boolean")
        .entry("Engine.PlayerReplicationInfo:bWaitingPlayer", "AttributeDecoder::decode_boolean")
        .entry("Engine.PlayerReplicationInfo:Ping", "AttributeDecoder::decode_byte")
        .entry("Engine.PlayerReplicationInfo:PlayerID", "AttributeDecoder::decode_int")
        .entry("Engine.PlayerReplicationInfo:PlayerName", "AttributeDecoder::decode_string")
        .entry("Engine.PlayerReplicationInfo:RemoteUserData", "AttributeDecoder::decode_string")
        .entry("Engine.PlayerReplicationInfo:Score", "AttributeDecoder::decode_int")
        .entry("Engine.PlayerReplicationInfo:Team", "AttributeDecoder::decode_flagged")
        .entry("Engine.PlayerReplicationInfo:UniqueId", "AttributeDecoder::decode_unique_id")
        .entry("Engine.TeamInfo:Score", "AttributeDecoder::decode_int")
        .entry("ProjectX.GRI_X:bGameStarted", "AttributeDecoder::decode_boolean")
        .entry("ProjectX.GRI_X:GameServerID", "AttributeDecoder::decode_qword")
        .entry("ProjectX.GRI_X:MatchGUID", "AttributeDecoder::decode_string")
        .entry("ProjectX.GRI_X:ReplicatedGameMutatorIndex", "AttributeDecoder::decode_int")
        .entry("ProjectX.GRI_X:ReplicatedGamePlaylist", "AttributeDecoder::decode_int")
        .entry("ProjectX.GRI_X:Reservations", "AttributeDecoder::decode_reservation")
        .entry("TAGame.Ball_Breakout_TA:AppliedDamage", "AttributeDecoder::decode_applied_damage")
        .entry("TAGame.Ball_Breakout_TA:DamageIndex", "AttributeDecoder::decode_int")
        .entry("TAGame.Ball_Breakout_TA:LastTeamTouch", "AttributeDecoder::decode_byte")
        .entry("TAGame.Ball_TA:GameEvent", "AttributeDecoder::decode_flagged")
        .entry("TAGame.Ball_TA:HitTeamNum", "AttributeDecoder::decode_byte")
        .entry("TAGame.Ball_TA:ReplicatedAddedCarBounceScale", "AttributeDecoder::decode_float")
        .entry("TAGame.Ball_TA:ReplicatedBallMaxLinearSpeedScale", "AttributeDecoder::decode_float")
        .entry("TAGame.Ball_TA:ReplicatedBallScale", "AttributeDecoder::decode_float")
        .entry("TAGame.Ball_TA:ReplicatedExplosionData", "AttributeDecoder::decode_explosion")
        .entry("TAGame.Ball_TA:ReplicatedExplosionDataExtended", "AttributeDecoder::decode_extended_explosion")
        .entry("TAGame.Ball_TA:ReplicatedWorldBounceScale", "AttributeDecoder::decode_float")
        .entry("TAGame.BreakOutActor_Platform_TA:DamageState", "AttributeDecoder::decode_damage_state")
        .entry("TAGame.CameraSettingsActor_TA:bUsingBehindView", "AttributeDecoder::decode_boolean")
        .entry("TAGame.CameraSettingsActor_TA:bUsingSecondaryCamera", "AttributeDecoder::decode_boolean")
        .entry("TAGame.CameraSettingsActor_TA:CameraPitch", "AttributeDecoder::decode_byte")
        .entry("TAGame.CameraSettingsActor_TA:CameraYaw", "AttributeDecoder::decode_byte")
        .entry("TAGame.CameraSettingsActor_TA:PRI", "AttributeDecoder::decode_flagged")
        .entry("TAGame.CameraSettingsActor_TA:ProfileSettings", "AttributeDecoder::decode_cam_settings")
        .entry("TAGame.Car_TA:AddedBallForceMultiplier", "AttributeDecoder::decode_float")
        .entry("TAGame.Car_TA:AddedCarForceMultiplier", "AttributeDecoder::decode_float")
        .entry("TAGame.Car_TA:AttachedPickup", "AttributeDecoder::decode_flagged")
        .entry("TAGame.Car_TA:ClubColors", "AttributeDecoder::decode_club_colors")
        .entry("TAGame.Car_TA:ReplicatedDemolish", "AttributeDecoder::decode_demolish")
        .entry("TAGame.Car_TA:TeamPaint", "AttributeDecoder::decode_team_paint")
        .entry("TAGame.CarComponent_Boost_TA:bNoBoost", "AttributeDecoder::decode_boolean")
        .entry("TAGame.CarComponent_Boost_TA:BoostModifier", "AttributeDecoder::decode_float")
        .entry("TAGame.CarComponent_Boost_TA:bUnlimitedBoost", "AttributeDecoder::decode_boolean")
        .entry("TAGame.CarComponent_Boost_TA:RechargeDelay", "AttributeDecoder::decode_float")
        .entry("TAGame.CarComponent_Boost_TA:RechargeRate", "AttributeDecoder::decode_float")
        .entry("TAGame.CarComponent_Boost_TA:ReplicatedBoostAmount", "AttributeDecoder::decode_byte")
        .entry("TAGame.CarComponent_Boost_TA:UnlimitedBoostRefCount", "AttributeDecoder::decode_int")
        .entry("TAGame.CarComponent_Dodge_TA:DodgeTorque", "AttributeDecoder::decode_location")
        .entry("TAGame.CarComponent_FlipCar_TA:bFlipRight", "AttributeDecoder::decode_boolean")
        .entry("TAGame.CarComponent_FlipCar_TA:FlipCarTime", "AttributeDecoder::decode_float")
        .entry("TAGame.CarComponent_TA:ReplicatedActive", "AttributeDecoder::decode_byte")
        .entry("TAGame.CarComponent_TA:ReplicatedActivityTime", "AttributeDecoder::decode_float")
        .entry("TAGame.CarComponent_TA:Vehicle", "AttributeDecoder::decode_flagged")
        .entry("TAGame.CrowdActor_TA:GameEvent", "AttributeDecoder::decode_flagged")
        .entry("TAGame.CrowdActor_TA:ModifiedNoise", "AttributeDecoder::decode_float")
        .entry("TAGame.CrowdActor_TA:ReplicatedCountDownNumber", "AttributeDecoder::decode_int")
        .entry("TAGame.CrowdActor_TA:ReplicatedOneShotSound", "AttributeDecoder::decode_flagged")
        .entry("TAGame.CrowdActor_TA:ReplicatedRoundCountDownNumber", "AttributeDecoder::decode_int")
        .entry("TAGame.CrowdManager_TA:GameEvent", "AttributeDecoder::decode_flagged")
        .entry("TAGame.CrowdManager_TA:ReplicatedGlobalOneShotSound", "AttributeDecoder::decode_int")
        .entry("TAGame.GameEvent_Soccar_TA:bBallHasBeenHit", "AttributeDecoder::decode_boolean")
        .entry("TAGame.GameEvent_Soccar_TA:bOverTime", "AttributeDecoder::decode_boolean")
        .entry("TAGame.GameEvent_Soccar_TA:GameTime", "AttributeDecoder::decode_int")
        .entry("TAGame.GameEvent_Soccar_TA:ReplicatedMusicStinger", "AttributeDecoder::decode_music_stinger")
        .entry("TAGame.GameEvent_Soccar_TA:ReplicatedScoredOnTeam", "AttributeDecoder::decode_byte")
        .entry("TAGame.GameEvent_Soccar_TA:RoundNum", "AttributeDecoder::decode_int")
        .entry("TAGame.GameEvent_Soccar_TA:SecondsRemaining", "AttributeDecoder::decode_int")
        .entry("TAGame.GameEvent_Soccar_TA:SubRulesArchetype", "AttributeDecoder::decode_flagged")
        .entry("TAGame.GameEvent_SoccarPrivate_TA:MatchSettings", "AttributeDecoder::decode_private_match_settings")
        .entry("TAGame.GameEvent_TA:bCanVoteToForfeit", "AttributeDecoder::decode_boolean")
        .entry("TAGame.GameEvent_TA:bHasLeaveMatchPenalty", "AttributeDecoder::decode_boolean")
        .entry("TAGame.GameEvent_TA:BotSkill", "AttributeDecoder::decode_int")
        .entry("TAGame.GameEvent_TA:GameMode", "AttributeDecoder::decode_game_mode")
        .entry("TAGame.GameEvent_TA:MatchTypeClass", "AttributeDecoder::decode_flagged")
        .entry("TAGame.GameEvent_TA:ReplicatedGameStateTimeRemaining", "AttributeDecoder::decode_int")
        .entry("TAGame.GameEvent_TA:ReplicatedStateIndex", "AttributeDecoder::decode_byte")
        .entry("TAGame.GameEvent_TA:ReplicatedStateName", "AttributeDecoder::decode_int")
        .entry("TAGame.GameEvent_Team_TA:bForfeit", "AttributeDecoder::decode_boolean")
        .entry("TAGame.GameEvent_Team_TA:MaxTeamSize", "AttributeDecoder::decode_int")
        .entry("TAGame.GRI_TA:NewDedicatedServerIP", "AttributeDecoder::decode_string")
        .entry("TAGame.PRI_TA:bIsInSplitScreen", "AttributeDecoder::decode_boolean")
        .entry("TAGame.PRI_TA:bMatchMVP", "AttributeDecoder::decode_boolean")
        .entry("TAGame.PRI_TA:bOnlineLoadoutSet", "AttributeDecoder::decode_boolean")
        .entry("TAGame.PRI_TA:bOnlineLoadoutsSet", "AttributeDecoder::decode_boolean")
        .entry("TAGame.PRI_TA:BotProductName", "AttributeDecoder::decode_int")
        .entry("TAGame.PRI_TA:bReady", "AttributeDecoder::decode_boolean")
        .entry("TAGame.PRI_TA:bUsingBehindView", "AttributeDecoder::decode_boolean")
        .entry("TAGame.PRI_TA:bUsingItems", "AttributeDecoder::decode_boolean")
        .entry("TAGame.PRI_TA:bUsingSecondaryCamera", "AttributeDecoder::decode_boolean")
        .entry("TAGame.PRI_TA:CameraPitch", "AttributeDecoder::decode_byte")
        .entry("TAGame.PRI_TA:CameraSettings", "AttributeDecoder::decode_cam_settings")
        .entry("TAGame.PRI_TA:CameraYaw", "AttributeDecoder::decode_byte")
        .entry("TAGame.PRI_TA:ClientLoadout", "AttributeDecoder::decode_loadout")
        .entry("TAGame.PRI_TA:ClientLoadoutOnline", "AttributeDecoder::decode_loadout_online")
        .entry("TAGame.PRI_TA:ClientLoadouts", "AttributeDecoder::decode_team_loadout")
        .entry("TAGame.PRI_TA:ClientLoadoutsOnline", "AttributeDecoder::decode_loadouts_online")
        .entry("TAGame.PRI_TA:MatchAssists", "AttributeDecoder::decode_int")
        .entry("TAGame.PRI_TA:MatchBreakoutDamage", "AttributeDecoder::decode_int")
        .entry("TAGame.PRI_TA:MatchGoals", "AttributeDecoder::decode_int")
        .entry("TAGame.PRI_TA:MatchSaves", "AttributeDecoder::decode_int")
        .entry("TAGame.PRI_TA:MatchScore", "AttributeDecoder::decode_int")
        .entry("TAGame.PRI_TA:MatchShots", "AttributeDecoder::decode_int")
        .entry("TAGame.PRI_TA:MaxTimeTillItem", "AttributeDecoder::decode_int")
        .entry("TAGame.PRI_TA:PartyLeader", "AttributeDecoder::decode_party_leader")
        .entry("TAGame.PRI_TA:PawnType", "AttributeDecoder::decode_byte")
        .entry("TAGame.PRI_TA:PersistentCamera", "AttributeDecoder::decode_flagged")
        .entry("TAGame.PRI_TA:PlayerHistoryValid", "AttributeDecoder::decode_boolean")
        .entry("TAGame.PRI_TA:ReplicatedGameEvent", "AttributeDecoder::decode_flagged")
        .entry("TAGame.PRI_TA:SteeringSensitivity", "AttributeDecoder::decode_float")
        .entry("TAGame.PRI_TA:TimeTillItem", "AttributeDecoder::decode_int")
        .entry("TAGame.PRI_TA:Title", "AttributeDecoder::decode_int")
        .entry("TAGame.PRI_TA:TotalXP", "AttributeDecoder::decode_int")
        .entry("TAGame.RBActor_TA:bFrozen", "AttributeDecoder::decode_boolean")
        .entry("TAGame.RBActor_TA:bIgnoreSyncing", "AttributeDecoder::decode_boolean")
        .entry("TAGame.RBActor_TA:bReplayActor", "AttributeDecoder::decode_boolean")
        .entry("TAGame.RBActor_TA:ReplicatedRBState", "AttributeDecoder::decode_rigid_body")
        .entry("TAGame.RBActor_TA:WeldedInfo", "AttributeDecoder::decode_welded")
        .entry("TAGame.SpecialPickup_BallFreeze_TA:RepOrigSpeed", "AttributeDecoder::decode_float")
        .entry("TAGame.SpecialPickup_BallVelcro_TA:AttachTime", "AttributeDecoder::decode_float")
        .entry("TAGame.SpecialPickup_BallVelcro_TA:bBroken", "AttributeDecoder::decode_boolean")
        .entry("TAGame.SpecialPickup_BallVelcro_TA:bHit", "AttributeDecoder::decode_boolean")
        .entry("TAGame.SpecialPickup_BallVelcro_TA:BreakTime", "AttributeDecoder::decode_float")
        .entry("TAGame.SpecialPickup_Targeted_TA:Targeted", "AttributeDecoder::decode_flagged")
        .entry("TAGame.Team_Soccar_TA:GameScore", "AttributeDecoder::decode_int")
        .entry("TAGame.Team_TA:ClubColors", "AttributeDecoder::decode_club_colors")
        .entry("TAGame.Team_TA:CustomTeamName", "AttributeDecoder::decode_string")
        .entry("TAGame.Team_TA:GameEvent", "AttributeDecoder::decode_flagged")
        .entry("TAGame.Team_TA:LogoData", "AttributeDecoder::decode_flagged")
        .entry("TAGame.Vehicle_TA:bDriving", "AttributeDecoder::decode_boolean")
        .entry("TAGame.Vehicle_TA:bReplicatedHandbrake", "AttributeDecoder::decode_boolean")
        .entry("TAGame.Vehicle_TA:ReplicatedSteer", "AttributeDecoder::decode_byte")
        .entry("TAGame.Vehicle_TA:ReplicatedThrottle", "AttributeDecoder::decode_byte")
        .entry("TAGame.VehiclePickup_TA:bNoPickup", "AttributeDecoder::decode_boolean")
        .entry("TAGame.VehiclePickup_TA:ReplicatedPickupData", "AttributeDecoder::decode_pickup")
        .build(&mut file)
        .unwrap();

    write!(&mut file, ";\n").unwrap();


    write!(&mut file, "pub static OBJECT_CLASSES: phf::Map<&'static str, &'static str> = ").unwrap();

    phf_codegen::Map::new()
        .entry("Archetypes.Ball.Ball_BasketBall_Mutator", "\"TAGame.Ball_TA\"")
        .entry("Archetypes.Ball.Ball_Basketball", "\"TAGame.Ball_TA\"")
        .entry("Archetypes.Ball.Ball_Breakout", "\"TAGame.Ball_Breakout_TA\"")
        .entry("Archetypes.Ball.Ball_Default", "\"TAGame.Ball_TA\"")
        .entry("Archetypes.Ball.Ball_Puck", "\"TAGame.Ball_TA\"")
        .entry("Archetypes.Ball.CubeBall", "\"TAGame.Ball_TA\"")
        .entry("Archetypes.Car.Car_Default", "\"TAGame.Car_TA\"")
        .entry("Archetypes.CarComponents.CarComponent_Boost", "\"TAGame.CarComponent_Boost_TA\"")
        .entry("Archetypes.CarComponents.CarComponent_Dodge", "\"TAGame.CarComponent_Dodge_TA\"")
        .entry("Archetypes.CarComponents.CarComponent_DoubleJump", "\"TAGame.CarComponent_DoubleJump_TA\"")
        .entry("Archetypes.CarComponents.CarComponent_FlipCar", "\"TAGame.CarComponent_FlipCar_TA\"")
        .entry("Archetypes.CarComponents.CarComponent_Jump", "\"TAGame.CarComponent_Jump_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_Basketball", "\"TAGame.GameEvent_Soccar_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_BasketballPrivate", "\"TAGame.GameEvent_SoccarPrivate_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_BasketballSplitscreen", "\"TAGame.GameEvent_SoccarSplitscreen_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_Breakout", "\"TAGame.GameEvent_Soccar_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_Hockey", "\"TAGame.GameEvent_Soccar_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_HockeyPrivate", "\"TAGame.GameEvent_SoccarPrivate_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_HockeySplitscreen", "\"TAGame.GameEvent_SoccarSplitscreen_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_Items", "\"TAGame.GameEvent_Soccar_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_Season:CarArchetype", "\"TAGame.Car_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_Season", "\"TAGame.GameEvent_Season_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_Soccar", "\"TAGame.GameEvent_Soccar_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_SoccarPrivate", "\"TAGame.GameEvent_SoccarPrivate_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_SoccarSplitscreen", "\"TAGame.GameEvent_SoccarSplitscreen_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallFreeze", "\"TAGame.SpecialPickup_BallFreeze_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallGrapplingHook", "\"TAGame.SpecialPickup_GrapplingHook_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallLasso", "\"TAGame.SpecialPickup_BallLasso_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallSpring", "\"TAGame.SpecialPickup_BallCarSpring_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallVelcro", "\"TAGame.SpecialPickup_BallVelcro_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_Batarang", "\"TAGame.SpecialPickup_Batarang_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BoostOverride", "\"TAGame.SpecialPickup_BoostOverride_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_CarSpring", "\"TAGame.SpecialPickup_BallCarSpring_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_GravityWell", "\"TAGame.SpecialPickup_BallGravity_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_StrongHit", "\"TAGame.SpecialPickup_HitForce_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_Swapper", "\"TAGame.SpecialPickup_Swapper_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_Tornado", "\"TAGame.SpecialPickup_Tornado_TA\"")
        .entry("Archetypes.Teams.Team0", "\"TAGame.Team_Soccar_TA\"")
        .entry("Archetypes.Teams.Team1", "\"TAGame.Team_Soccar_TA\"")
        .entry("GameInfo_Basketball.GameInfo.GameInfo_Basketball:GameReplicationInfoArchetype", "\"TAGame.GRI_TA\"")
        .entry("GameInfo_Breakout.GameInfo.GameInfo_Breakout:GameReplicationInfoArchetype", "\"TAGame.GRI_TA\"")
        .entry("Gameinfo_Hockey.GameInfo.Gameinfo_Hockey:GameReplicationInfoArchetype", "\"TAGame.GRI_TA\"")
        .entry("GameInfo_Items.GameInfo.GameInfo_Items:GameReplicationInfoArchetype", "\"TAGame.GRI_TA\"")
        .entry("GameInfo_Season.GameInfo.GameInfo_Season:GameReplicationInfoArchetype", "\"TAGame.GRI_TA\"")
        .entry("GameInfo_Soccar.GameInfo.GameInfo_Soccar:GameReplicationInfoArchetype", "\"TAGame.GRI_TA\"")
        .entry("TAGame.Default__CameraSettingsActor_TA", "\"TAGame.CameraSettingsActor_TA\"")
        .entry("TAGame.Default__PRI_TA", "\"TAGame.PRI_TA\"")
        .entry("TheWorld:PersistentLevel.BreakOutActor_Platform_TA", "\"TAGame.BreakOutActor_Platform_TA\"")
        .entry("TheWorld:PersistentLevel.CrowdActor_TA", "\"TAGame.CrowdActor_TA\"")
        .entry("TheWorld:PersistentLevel.CrowdManager_TA", "\"TAGame.CrowdManager_TA\"")
        .entry("TheWorld:PersistentLevel.InMapScoreboard_TA", "\"TAGame.InMapScoreboard_TA\"")
        .entry("TheWorld:PersistentLevel.VehiclePickup_Boost_TA", "\"TAGame.VehiclePickup_Boost_TA\"")
        .build(&mut file)
        .unwrap();

    write!(&mut file, ";\n").unwrap();


    write!(&mut file, "pub static PARENT_CLASSES: phf::Map<&'static str, &'static str> = ").unwrap();

    phf_codegen::Map::new()
        .entry("Engine.Actor", "\"Core.Object\"")
        .entry("Engine.GameReplicationInfo", "\"Engine.ReplicationInfo\"")
        .entry("Engine.Info", "\"Engine.Actor\"")
        .entry("Engine.Pawn", "\"Engine.Actor\"")
        .entry("Engine.PlayerReplicationInfo", "\"Engine.ReplicationInfo\"")
        .entry("Engine.ReplicationInfo", "\"Engine.Info\"")
        .entry("Engine.TeamInfo", "\"Engine.ReplicationInfo\"")
        .entry("ProjectX.GRI_X", "\"Engine.GameReplicationInfo\"")
        .entry("ProjectX.Pawn_X", "\"Engine.Pawn\"")
        .entry("ProjectX.PRI_X", "\"Engine.PlayerReplicationInfo\"")
        .entry("TAGame.Ball_TA", "\"TAGame.RBActor_TA\"")
        .entry("TAGame.CameraSettingsActor_TA", "\"Engine.ReplicationInfo\"")
        .entry("TAGame.Car_Season_TA", "\"TAGame.PRI_TA\"")
        .entry("TAGame.Car_TA", "\"TAGame.Vehicle_TA\"")
        .entry("TAGame.CarComponent_Boost_TA", "\"TAGame.CarComponent_TA\"")
        .entry("TAGame.CarComponent_Dodge_TA", "\"TAGame.CarComponent_TA\"")
        .entry("TAGame.CarComponent_DoubleJump_TA", "\"TAGame.CarComponent_TA\"")
        .entry("TAGame.CarComponent_FlipCar_TA", "\"TAGame.CarComponent_TA\"")
        .entry("TAGame.CarComponent_Jump_TA", "\"TAGame.CarComponent_TA\"")
        .entry("TAGame.CarComponent_TA", "\"Engine.ReplicationInfo\"")
        .entry("TAGame.CrowdActor_TA", "\"Engine.ReplicationInfo\"")
        .entry("TAGame.CrowdManager_TA", "\"Engine.ReplicationInfo\"")
        .entry("TAGame.GameEvent_Season_TA", "\"TAGame.GameEvent_Soccar_TA\"")
        .entry("TAGame.GameEvent_Soccar_TA", "\"TAGame.GameEvent_Team_TA\"")
        .entry("TAGame.GameEvent_SoccarPrivate_TA", "\"TAGame.GameEvent_Soccar_TA\"")
        .entry("TAGame.GameEvent_SoccarSplitscreen_TA", "\"TAGame.GameEvent_SoccarPrivate_TA\"")
        .entry("TAGame.GameEvent_TA", "\"Engine.ReplicationInfo\"")
        .entry("TAGame.GameEvent_Team_TA", "\"TAGame.GameEvent_TA\"")
        .entry("TAGame.GRI_TA", "\"ProjectX.GRI_X\"")
        .entry("TAGame.InMapScoreboard_TA", "\"Engine.Actor\"")
        .entry("TAGame.PRI_TA", "\"ProjectX.PRI_X\"")
        .entry("TAGame.RBActor_TA", "\"ProjectX.Pawn_X\"")
        .entry("TAGame.SpecialPickup_BallCarSpring_TA", "\"TAGame.SpecialPickup_Spring_TA\"")
        .entry("TAGame.SpecialPickup_BallFreeze_TA", "\"TAGame.SpecialPickup_Targeted_TA\"")
        .entry("TAGame.SpecialPickup_BallGravity_TA", "\"TAGame.SpecialPickup_TA\"")
        .entry("TAGame.SpecialPickup_BallLasso_TA", "\"TAGame.SpecialPickup_GrapplingHook_TA\"")
        .entry("TAGame.SpecialPickup_BallVelcro_TA", "\"TAGame.SpecialPickup_TA\"")
        .entry("TAGame.SpecialPickup_Batarang_TA", "\"TAGame.SpecialPickup_BallLasso_TA\"")
        .entry("TAGame.SpecialPickup_BoostOverride_TA", "\"TAGame.SpecialPickup_Targeted_TA\"")
        .entry("TAGame.SpecialPickup_GrapplingHook_TA", "\"TAGame.SpecialPickup_Targeted_TA\"")
        .entry("TAGame.SpecialPickup_HitForce_TA", "\"TAGame.SpecialPickup_TA\"")
        .entry("TAGame.SpecialPickup_Spring_TA", "\"TAGame.SpecialPickup_Targeted_TA\"")
        .entry("TAGame.SpecialPickup_Swapper_TA", "\"TAGame.SpecialPickup_Targeted_TA\"")
        .entry("TAGame.SpecialPickup_TA", "\"TAGame.CarComponent_TA\"")
        .entry("TAGame.SpecialPickup_Targeted_TA", "\"TAGame.SpecialPickup_TA\"")
        .entry("TAGame.SpecialPickup_Tornado_TA", "\"TAGame.SpecialPickup_TA\"")
        .entry("TAGame.Team_Soccar_TA", "\"TAGame.Team_TA\"")
        .entry("TAGame.Team_TA", "\"Engine.TeamInfo\"")
        .entry("TAGame.Vehicle_TA", "\"TAGame.RBActor_TA\"")
        .entry("TAGame.VehiclePickup_Boost_TA", "\"TAGame.VehiclePickup_TA\"")
        .entry("TAGame.VehiclePickup_TA", "\"Engine.ReplicationInfo\"")
        .build(&mut file)
        .unwrap();

    write!(&mut file, ";\n").unwrap();
}
