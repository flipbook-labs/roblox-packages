-- roblox-packages is a Rust crate, not a Loom package. This config exists only
-- to declare the dev dependencies agents read locally (the shared skills
-- library) and to carry a mirror of the release version. `version` is kept in
-- sync with Cargo.toml by Changewrite; see changewrite.toml.
return {
	package = {
		name = "roblox-packages",
		version = "0.5.0",
		dev_dependencies = {
			AgentSkills = {
				rev = "v0.4.0",
				sourceKind = "github",
				source = "https://github.com/flipbook-labs/agent-skills",
			},
		},
	},
}
