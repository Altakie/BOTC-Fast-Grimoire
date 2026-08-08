# BOTC Fast Grimoire — Storyteller Tool

A storyteller-only companion app for running Blood on the Clocktower games: tracks game state and resolves rule effects as the storyteller enters player actions.

## Language

**Auto mode**:
The engine's current (and only implemented) mode of operation — the storyteller supplies an input (e.g. who was executed, who a night ability targets) and the engine automatically resolves the downstream rule effects (e.g. triggering an ability, ending the game). It does not choose actions on the players' behalf, only resolves consequences of the input given.
_Avoid_: automatic mode, auto-resolve mode

**Manual mode**:
A planned mode (not yet implemented, tracked in `TODO.md`) that exposes every field of the game state for direct inspection and editing — collapsed-by-default structs and vectors, toggle buttons for booleans, validated input fields for strings/numbers, and alignment-colored dropdowns for enum members. This is distinct from the "manual pick" buttons shown in the `Clocktower Storyteller.dc.html` design mockup, which depict contextual manual overrides rather than a generic state editor. Toggling into Manual mode replaces the entire game screen's content area (all panes), rather than overlaying it. Edits made in Manual mode are silent — they do not write to the event log. Changing a seat's role autofills `alignment` with that role's default team; alignment remains separately editable afterward for deliberate mismatches (e.g. a Recluse registering as evil).
_Avoid_: manual pick, override mode

**Why trace**:
A planned (not yet implemented) explanation of the reasoning behind an auto-resolved outcome, shown in the mockup's decision card. Out of scope for this implementation pass.
