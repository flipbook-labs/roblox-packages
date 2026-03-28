local path = require("@std/path")
local system = require("@std/system")
local process = require("@std/process")

local globalDefsPath = path.join(system.tmpdir(), "globalTypes.d.luau")

process.run({
	"curl",
	"-s",
	"-o",
	tostring(globalDefsPath),
	"-O",
	"https://raw.githubusercontent.com/JohnnyMorganz/luau-lsp/master/scripts/globalTypes.d.lua",
})

process.run({
	"luau-lsp",
	"analyze",
	`--defs={globalDefsPath}`,
	"--platform=standard",
	"--flag:LuauSolverV2=true",
	".lute",
}, {
	stdio = "inherit",
})
