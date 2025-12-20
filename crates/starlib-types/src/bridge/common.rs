use crate::core::Bytes;
use crate::geom::SizeI;
use crate::stable::AbilityId;

define_bridge_message! {
  #[bridge(AvailableAbility)]
  pub struct Ability {
    ability_id: AbilityId = proto::ability_id,
    requires_point: bool = proto::requires_point,
  }
}

define_bridge_message! {
  #[bridge(ImageData)]
  pub struct Image {
    bpp: i32 = proto::bits_per_pixel,
    size: SizeI = proto::size,
    data: Bytes = proto::data, // width * height * bpp / 8
  }
}

define_bridge_enum! {
  #[bridge(Race)]
  pub enum Race {
    NoRace,
    Terran,
    Zerg,
    Protoss,
    Random,
  }
}
