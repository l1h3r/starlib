use std::collections::HashMap;

use starlib_proto::ResponseGameInfo;
use starlib_proto::StartRaw;

use crate::bridge::InterfaceOptions;
use crate::bridge::PlayerInfo;
use crate::core::PlayerTag;
use crate::core::PointF;
use crate::core::PointI;
use crate::core::RectI;
use crate::core::SizeI;
use crate::grid::PixelGrid;
use crate::grid::pixels;
use crate::traits::FromProto;
use crate::traits::IntoSc2;

#[derive(Debug)]
pub struct GameInfo {
  // ---------------------------------------------------------------------------
  // Player Info
  // ---------------------------------------------------------------------------
  participants: HashMap<PlayerTag, PlayerInfo>,
  player_point: PointF,
  enemy_points: Vec<PointF>,
  // ---------------------------------------------------------------------------
  // Map Info
  // ---------------------------------------------------------------------------
  map_name: String,
  map_path: String,
  map_size: SizeI,
  map_region: RectI,
  map_center: PointI,
  // ---------------------------------------------------------------------------
  // Map Info - Pixel Maps
  // ---------------------------------------------------------------------------
  pathing_grid: PixelGrid<pixels::Block>,
  placement_grid: PixelGrid<pixels::Block>,
  terrain_height: PixelGrid<pixels::Height>,
  // ---------------------------------------------------------------------------
  // Misc. Configuration
  // ---------------------------------------------------------------------------
  mod_names: Vec<String>,
  interface: InterfaceOptions,
}

impl GameInfo {
  #[inline]
  pub(crate) fn update(&mut self, point: PointF) {
    self.player_point = point;
  }

  // ---------------------------------------------------------------------------
  // Player Info
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn participants(&self) -> &HashMap<PlayerTag, PlayerInfo> {
    &self.participants
  }

  #[inline]
  pub fn player_point(&self) -> PointF {
    self.player_point
  }

  #[inline]
  pub fn enemy_points(&self) -> &[PointF] {
    self.enemy_points.as_slice()
  }

  // ---------------------------------------------------------------------------
  // Map Info
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn map_name(&self) -> &str {
    self.map_name.as_str()
  }

  #[inline]
  pub const fn map_path(&self) -> &str {
    self.map_path.as_str()
  }

  #[inline]
  pub const fn map_size(&self) -> SizeI {
    self.map_size
  }

  #[inline]
  pub const fn map_region(&self) -> RectI {
    self.map_region
  }

  #[inline]
  pub const fn map_center(&self) -> PointI {
    self.map_center
  }

  // ---------------------------------------------------------------------------
  // Map Info - Pixel Maps
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn pathing_grid(&self) -> &PixelGrid<pixels::Block> {
    &self.pathing_grid
  }

  #[inline]
  pub const fn placement_grid(&self) -> &PixelGrid<pixels::Block> {
    &self.placement_grid
  }

  #[inline]
  pub const fn terrain_height(&self) -> &PixelGrid<pixels::Height> {
    &self.terrain_height
  }

  // ---------------------------------------------------------------------------
  // Misc. Configuration
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn mod_names(&self) -> &[String] {
    self.mod_names.as_slice()
  }

  #[inline]
  pub const fn interface(&self) -> &InterfaceOptions {
    &self.interface
  }
}

impl FromProto<ResponseGameInfo> for GameInfo {
  fn from_proto(other: ResponseGameInfo) -> Self {
    let raw: StartRaw = other.start_raw.unwrap_or_default();

    let participants: HashMap<PlayerTag, PlayerInfo> = other
      .player_info
      .into_iter()
      .map(IntoSc2::into_sc2)
      .map(|info: PlayerInfo| (info.id(), info))
      .collect();

    let map_region: RectI = raw.playable_area.unwrap_or_default().into_sc2();
    let map_center: PointI = map_region.center();

    Self {
      participants,
      player_point: Default::default(),
      enemy_points: raw.start_locations.into_sc2(),

      map_name: other.map_name,
      map_path: other.local_map_path,
      map_size: raw.map_size.unwrap_or_default().into_sc2(),
      map_region,
      map_center,

      pathing_grid: raw.pathing_grid.unwrap_or_default().into_sc2(),
      placement_grid: raw.placement_grid.unwrap_or_default().into_sc2(),
      terrain_height: raw.terrain_height.unwrap_or_default().into_sc2(),

      mod_names: other.mod_names,
      interface: other.options.unwrap_or_default().into_sc2(),
    }
  }
}
