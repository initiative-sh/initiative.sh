pub use spell::Spell;

mod spell;

pub fn spells() -> Result<Vec<Spell>, String> {
    serde_json::from_str(include_str!("../../../data/srd_5e/src/5e-SRD-Spells.json"))
        .map_err(|e| format!("{}", e))
}
