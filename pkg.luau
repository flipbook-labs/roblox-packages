-- roblox-packages is a Rust crate, not a Loom package. This config exists only
-- to declare the dev dependencies agents read locally (the shared skills
-- library) and to carry a mirror of the release version. `version` is kept in
-- sync with Cargo.toml by Changewrite; see changewrite.luau.
return {
	package = {
		name = "roblox-packages",
		version = "0.6.0",
		dev_dependencies = {
			["agent-skills"] = {
				rev = "v0.9.0",
				sourceKind = "github",
				source = "https://github.com/flipbook-labs/agent-skills",
			},
		},
	},
}
