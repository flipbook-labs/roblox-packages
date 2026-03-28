local cli = require("@scripts/batteries/cli")
local fs = require("@std/fs")
local path = require("@std/path")
local process = require("@std/process")
local system = require("@std/system")

local BIN_NAME = "roblox-packages"

local args = cli.parser()

args:add("pack", "option", {
	help = "Path to a release directory, or a target triple to resolve as target/<triple>/release",
	aliases = { "p" },
})
args:add("unpack", "option", {
	help = "Path to a directory containing nested release artifacts",
	aliases = { "u" },
})
args:add("help", "flag", {
	help = "Show this help message",
	aliases = { "h" },
})

local function formatPath(value)
	return path.format(value)
end

local function printHelp()
	local body = {
		"Usage:",
		"\tlute run release --pack <path>",
		"\tlute run release --unpack <path>",
		"",
		"Options:",
		"\t--pack, -p     Path to a release directory or target triple",
		"\t--unpack, -u   Path to a directory containing nested releases",
		"\t--help, -h     Show this help message",
	}
	return print(table.concat(body, "\n"))
end

local function removePathIfExists(targetPath)
	if not fs.exists(targetPath) then
		return
	end

	if fs.type(targetPath) == "dir" then
		fs.removedirectory(targetPath, { recursive = true })
	else
		fs.remove(targetPath)
	end
end

local function runCommand(commandArgs, options)
	local result = process.run(commandArgs, options)
	assert(result.ok, `Command failed ({result.exitcode}): {table.concat(commandArgs, " ")}`)
	return result
end

local function printDirectoryListing(directoryPath)
	print("")
	print("Releases dir:")

	local entries = fs.listdirectory(directoryPath)
	table.sort(entries, function(left, right)
		return left.name < right.name
	end)

	if #entries == 0 then
		print("  <empty>")
		return
	end

	for _, entry in entries do
		local suffix = if entry.type == "dir" then "/" else ""
		print(`  {entry.name}{suffix}`)
	end
end

local function resolvePackDirectory(inputPath)
	local cwd = process.cwd()
	local directPath = path.resolve(cwd, inputPath)
	if fs.exists(directPath) then
		assert(fs.type(directPath) == "dir", `Pack path '{inputPath}' is not a directory`)
		return directPath
	end

	local targetPath = path.resolve(cwd, path.join("target", inputPath, "release"))
	if fs.exists(targetPath) then
		assert(fs.type(targetPath) == "dir", `Resolved pack path '{formatPath(targetPath)}' is not a directory`)
		return targetPath
	end

	error(
		`Pack path '{inputPath}' does not exist. Pass a release directory or a target triple that resolves to target/{inputPath}/release`
	)
end

local function packRelease(inputPath)
	local cwd = process.cwd()
	local targetDir = resolvePackDirectory(inputPath)
	local binExt = if system.win32 then ".exe" else ""
	local binaryName = BIN_NAME .. binExt
	local binaryPath = path.join(targetDir, binaryName)
	assert(fs.exists(binaryPath), `Built binary '{formatPath(binaryPath)}' does not exist`)

	local stagingDir = path.join(cwd, "staging")
	local releaseZip = path.join(cwd, "release.zip")

	removePathIfExists(stagingDir)
	removePathIfExists(releaseZip)

	fs.createdirectory(stagingDir)
	fs.copy(binaryPath, path.join(stagingDir, binaryName))

	if not system.win32 then
		runCommand({ "chmod", "+x", binaryName }, {
			cwd = formatPath(stagingDir),
			stdio = "inherit",
		})
	end

	local archiveArgs = if system.win32
		then { "7z", "a", formatPath(releaseZip), binaryName }
		else { "zip", formatPath(releaseZip), binaryName }

	runCommand(archiveArgs, {
		cwd = formatPath(stagingDir),
		stdio = "inherit",
	})

	removePathIfExists(stagingDir)
	print(`Created {formatPath(releaseZip)}`)
end

local function unpackReleases(inputPath)
	local releasesDir = path.resolve(process.cwd(), inputPath)
	assert(fs.exists(releasesDir), `Releases directory '{inputPath}' does not exist`)
	assert(fs.type(releasesDir) == "dir", `Releases path '{inputPath}' is not a directory`)

	printDirectoryListing(releasesDir)
	print("")
	print("Searching for zipped releases...")

	local entries = fs.listdirectory(releasesDir)
	table.sort(entries, function(left, right)
		return left.name < right.name
	end)

	for _, entry in entries do
		if entry.type == "dir" then
			local nestedZip = path.join(releasesDir, entry.name, "release.zip")
			if fs.exists(nestedZip) and fs.type(nestedZip) ~= "dir" then
				local outputZip = path.join(releasesDir, entry.name .. ".zip")
				print(`Found zipped release '{entry.name}'`)
				removePathIfExists(outputZip)
				fs.copy(nestedZip, outputZip)
				fs.removedirectory(path.join(releasesDir, entry.name), { recursive = true })
			end
		end
	end

	printDirectoryListing(releasesDir)
end

local function main()
	local argv = {}
	if #process.args >= 2 then
		table.move(process.args, 2, #process.args, 1, argv)
	end
	args:parse(argv)

	if args:has("help") then
		printHelp()
		return
	end

	local forwarded = args:forwarded() or {}
	assert(#forwarded == 0, `Unexpected positional arguments: {table.concat(forwarded, " ")}`)

	local pack = args:get("pack")
	local unpack = args:get("unpack")

	if pack == nil and unpack == nil then
		printHelp()
		return
	end

	assert(not (pack ~= nil and unpack ~= nil), "Use either --pack or --unpack, not both")

	if pack ~= nil then
		packRelease(pack)
		return
	end

	assert(unpack ~= nil, "Missing unpack path")
	unpackReleases(unpack)
end

local ok, err = pcall(main)
if not ok then
	print(`Error: {tostring(err)}`)
	process.exit(1)
end
