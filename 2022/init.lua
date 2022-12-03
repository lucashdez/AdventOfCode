local function size(arr)
	local result = 0
	for i in pairs(arr) do
		result = result + 1
	end
	return result
end

local function main()
	if size(arg) ~= 3 then
		local error_str = "Not enough arguments\n"
		error_str = error_str .. "Usage:\n\tlua init.lua [directory name]"
		print(error_str)
		return -1
	end
	os.execute("cargo init " .. arg[1])
	os.execute("copy .\\main.rs " .. arg[1] .. "\\src\\")
end

main()
