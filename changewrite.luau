return {
	categories = { "Features", "Changes", "Fixes", "Dependencies", "Internal" },
	collapsed = { "Dependencies", "Internal" },
	version = {
		current = "0.6.0",
		mirror = {
			{ file = "Cargo.toml" },
			{ file = "Cargo.lock", pattern = 'name = "roblox%-packages"%s+version = "([^"]+)"' },
			{ file = "pkg.luau" },
		},
	},
	kind = {
		release = { text = "Release", color = "#94606f" },
		build = { text = "Build", color = "#5a7396" },
		ci = { text = "CI", color = "#75659a" },
		cli = { text = "CLI", color = "#0e7490" },
		contributing = { text = "Contributing", color = "#5f7f55" },
		docs = { text = "Docs", color = "#8a5a2b" },
		install = { text = "Install", color = "#5b5bd6" },
		packages = { text = "Packages", color = "#7a7444" },
		rotriever = { text = "Rotriever", color = "#bc4c00" },
		tooling = { text = "Tooling", color = "#4e8079" },
	},
}
