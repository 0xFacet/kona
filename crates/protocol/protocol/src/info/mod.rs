//! Module containing L1 Attributes types (aka the L1 block info transaction).

mod variant;
pub use variant::L1BlockInfoTx;

mod isthmus;
pub use isthmus::L1BlockInfoIsthmus;

mod bedrock;
pub use bedrock::L1BlockInfoBedrock;

mod ecotone;
pub use ecotone::L1BlockInfoEcotone;

mod facet;
pub use facet::L1BlockInfoFacet;

mod errors;
pub use errors::{BlockInfoError, DecodeError};
