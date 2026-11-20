local gridlib = require("grid")
local level = {}
local util = {}

function util.file_exists(file)
	local f = io.open(file, "rb")
	if f then
		f:close()
	end
	return f ~= nil
end

function util.lines_from(file)
	if not util.file_exists(file) then
		return {}
	end
	local lines = {}
	for line in io.lines(file) do
		lines[#lines + 1] = line
	end
	return lines
end

function util.parse_number(str, idx)
	local n = 0
	for i = idx or 1, string.len(str) do
		if string.byte(str, i) >= 48 and string.byte(str, i) <= 57 then
			n = n * 10 + string.byte(str, i) - 48
		elseif string.byte(str, i) == 32 or string.byte(str, i) == 10 then
			return n, i
		else
			return nil
		end
	end
	return n
end

function level.deserialize(filename)
	local filelines = util.lines_from(filename)

	local n, idx = util.parse_number(filelines[1])
	if not n then
		print("couldn't parse n")
		return nil
	end

	local m, _ = util.parse_number(filelines[1], idx + 1)

	if not m then
		print("couldn't parse m")
		return nil
	end

	local lvl = level.new(n, m)
	local serialized = {}

	for i = 2, #filelines do
		serialized[i - 1] = filelines[i]
	end

	lvl:fromstr(serialized)
	return lvl
end

function level.new(n, m)
	local newlevel = {
		ground = gridlib.new(n, m),
		items = gridlib.new(n, m),
		undostack = {},
	}

	function newlevel:fromstr(strtable)
		for rowc, row in ipairs(strtable) do
			for i = 1, m do
				local thing = string.sub(row, i, i)

				if thing == "#" or thing == " " or thing == "." then
					self.ground:set(rowc, i, thing)
				elseif thing == "@" then
					self.items:set(rowc, i, thing)
				elseif thing == "X" then
					self.items:set(rowc, i, thing)
				end
			end
		end
	end

	function newlevel:draw(x, y, width)
		for i = 1, n do
			for j = 1, m do
				local groundthing = self.ground:get(i, j)

				local rect_startx = x + (j - 1) * width
				local rect_starty = x + (i - 1) * width

				if groundthing == "#" then
					love.graphics.setColor(0.1, 0.2, 0.3)
					love.graphics.rectangle("fill", rect_startx, rect_starty, width, width)
				elseif groundthing == " " then
					love.graphics.setColor(0, 0, 0)
					love.graphics.rectangle("fill", rect_startx, rect_starty, width, width)
				elseif groundthing == "." then
					love.graphics.setColor(0.1, 0.3, 0.3)
					love.graphics.circle("fill", rect_startx + width / 2, rect_starty + width / 2, width / 4)
				else
					love.graphics.setColor(0, 0, 1)
					love.graphics.rectangle("fill", rect_startx, rect_starty, width, width)
				end

				if self.items:get(i, j) == "X" then
					love.graphics.setColor(0.3, 0.3, 0.0)
					love.graphics.rectangle(
						"fill",
						rect_startx + width / 10,
						rect_starty + width / 10,
						width - 2 * width / 10,
						width - 2 * width / 10
					)
				elseif self.items:get(i, j) == "@" then
					love.graphics.setColor(0.5, 0.1, 0.1)
					love.graphics.rectangle(
						"fill",
						x + (j - 1) * width,
						y + (i - 1) * width + width / 2,
						width,
						width / 2,
						width / 10
					)
					love.graphics.setColor(1, 1, 0)
					love.graphics.circle(
						"fill",
						x + (j - 1) * width + width / 2,
						y + (i - 1) * width + width / 4,
						width / 4
					)
				end
			end
		end
	end

	function newlevel:is_solved()
		local dotcount = 0
		for i = 1, n do
			for j = 1, m do
				if self.ground:get(i, j) == "." and self.items:get(i, j) ~= "X" then
					dotcount = dotcount + 1
				end
			end
		end
		return dotcount ~= 0
	end

	function newlevel:move(direction)
		table.insert(self.undostack, self.items:deepclone())

		if direction == "up" then
			for i = 1, n do
				for j = 1, m do
					if self.items:get(i, j) == "@" and i - 1 > 0 then
						if
							self.items:get(i - 1, j) ~= "X"
							and self.items:get(i - 1, j) ~= "@"
							and self.ground:get(i - 1, j) ~= "#"
						then
							self.items:set(i, j, " ")
							self.items:set(i - 1, j, "@")
						end
					elseif self.items:get(i, j) == "X" then
						if i + 1 <= n and i - 1 > 0 then
							if
								self.items:get(i + 1, j) == "@"
								and self.items:get(i - 1, j) ~= "X"
								and self.ground:get(i - 1, j) ~= "#"
								and self.items:get(i - 1, j) ~= "@"
							then
								self.items:set(i, j, " ")
								self.items:set(i - 1, j, "X")
							end
						end
					end
				end
			end
		elseif direction == "down" then
			for i = n, 1, -1 do
				for j = 1, m do
					if i + 1 <= n then
						if self.items:get(i, j) == "@" then
							if
								self.items:get(i + 1, j) ~= "X"
								and self.items:get(i + 1, j) ~= "@"
								and self.ground:get(i + 1, j) ~= "#"
							then
								self.items:set(i, j, " ")
								self.items:set(i + 1, j, "@")
							end
						elseif self.items:get(i, j) == "X" then
							if i - 1 > 0 then
								if
									self.items:get(i - 1, j) == "@"
									and self.items:get(i + 1, j) ~= "X"
									and self.ground:get(i + 1, j) ~= "#"
									and self.items:get(i + 1, j) ~= "@"
								then
									self.items:set(i, j, " ")
									self.items:set(i + 1, j, "X")
								end
							end
						end
					end
				end
			end
		elseif direction == "left" then
			for i = 1, n do
				for j = 1, m do
					if self.items:get(i, j) == "@" and j - 1 > 0 then
						if
							self.items:get(i, j - 1) ~= "X"
							and self.items:get(i, j - 1) ~= "@"
							and self.ground:get(i, j - 1) ~= "#"
						then
							self.items:set(i, j, " ")
							self.items:set(i, j - 1, "@")
						end
					elseif self.items:get(i, j) == "X" then
						if j + 1 <= m and j - 1 > 0 then
							if
								self.items:get(i, j + 1) == "@"
								and self.items:get(i, j - 1) ~= "X"
								and self.ground:get(i, j - 1) ~= "#"
								and self.items:get(i, j - 1) ~= "@"
							then
								self.items:set(i, j, " ")
								self.items:set(i, j - 1, "X")
							end
						end
					end
				end
			end
		elseif direction == "right" then
			for i = 1, n do
				for j = m, 1, -1 do
					if self.items:get(i, j) == "@" and j + 1 <= m then
						if
							self.items:get(i, j + 1) ~= "X"
							and self.items:get(i, j + 1) ~= "@"
							and self.ground:get(i, j + 1) ~= "#"
						then
							self.items:set(i, j, " ")
							self.items:set(i, j + 1, "@")
						end
					elseif self.items:get(i, j) == "X" then
						if j + 1 <= m and j - 1 > 0 then
							if
								self.items:get(i, j - 1) == "@"
								and self.items:get(i, j + 1) ~= "X"
								and self.ground:get(i, j + 1) ~= "#"
								and self.items:get(i, j + 1) ~= "@"
							then
								self.items:set(i, j, " ")
								self.items:set(i, j + 1, "X")
							end
						end
					end
				end
			end
		end
	end

	function newlevel:undo()
		if #self.undostack >= 1 then
			self.items = self.undostack[#self.undostack]
			self.undostack[#self.undostack] = nil
		end
	end

	return newlevel
end

return level
