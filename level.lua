local gridlib = require("grid")
local level = {}

function level.new(n, m)
	local newlevel = {
		ground = gridlib.new(n, m),
		items = gridlib.new(n, m),
	}

	function newlevel:fromstr(strtable)
		for rowc, row in ipairs(strtable) do
			for i=1,m do
				local thing = string.sub(row, i,i)

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
		for i=1,n do
			for j=1,m do
				local groundthing = self.ground:get(i,j)

				local rect_startx = x+(j-1)*width
				local rect_starty = x+(i-1)*width

				if groundthing == "#" then
					love.graphics.setColor(0.1, 0.2, 0.3)
					love.graphics.rectangle("fill", rect_startx, rect_starty, width, width)
				elseif groundthing == " " then
					love.graphics.setColor(0, 0, 0)
					love.graphics.rectangle("fill", rect_startx, rect_starty, width, width)
				elseif groundthing == "." then
					love.graphics.setColor(0.1, 0.3, 0.3)
					love.graphics.circle("fill", rect_startx + width/2, rect_starty + width/2 , width/4)
				else
					love.graphics.setColor(0, 0, 1)
					love.graphics.rectangle("fill",rect_startx, rect_starty, width, width)
				end

				if self.items:get(i, j) == "X" then
					local rect_startx = x+(j-1)*width
					local rect_starty = x+(i-1)*width

					love.graphics.setColor(0.3, 0.3, 0.0)
					love.graphics.rectangle("fill",rect_startx + width/10, rect_starty + width/10, width - 2*width/10, width - 2*width/10 )
				elseif self.items:get(i, j) == "@" then
					love.graphics.setColor(0.5, 0.1, 0.1)
					love.graphics.rectangle("fill", x+(j-1)*width, y+(i-1)*width+width/2, width, width/2, width/10)
					love.graphics.setColor(1, 1, 0)
					love.graphics.circle("fill", x+(j-1)*width + width / 2,y+(i-1)*width + width/4 , width/4)
				end
			end
		end

	end

	function newlevel:move(direction)
		local dxy = {
			left = {0, -1},
			right = {0, 1},
			up = {-1, 0},
			down = {1, 0}
		}

		if direction == "up" then
			for i=1,n do
				for j=1,m do
					if self.items:get(i,j) == "@" and i-1>0 then
						if self.items:get(i-1,j) ~= "X" and self.items:get(i-1,j) ~= "@" and self.ground:get(i-1, j) ~= "#" then
							self.items:set(i,j, " ")
							self.items:set(i-1,j, "@")
						end
					elseif self.items:get(i,j) == "X" then
						if i+1 <= n and i-1 > 0 then
							if self.items:get(i+1,j) == "@" and self.items:get(i-1, j) ~= "X" and self.ground:get(i-1, j) ~= "#" then
								self.items:set(i,j, " ")
								self.items:set(i-1,j, "X")
							end
						end
					end
				end
			end
		elseif direction == "down" then
			for i=n,1,-1 do
				for j=1,m do
					if i+1 <= n then 
						if self.items:get(i,j) == "@" then
							if self.items:get(i+1,j) ~= "X" and self.items:get(i+1,j) ~= "@" and self.ground:get(i+1, j) ~= "#" then
								self.items:set(i,j, " ")
								self.items:set(i+1,j, "@")
							end
						elseif self.items:get(i,j) == "X" then
							if i-1 > 0 then
								if self.items:get(i-1,j) == "@" and self.items:get(i+1, j) ~= "X" and self.ground:get(i+1, j) ~= "#"  then
									self.items:set(i,j, " ")
									self.items:set(i+1,j, "X")
								end
							end
						end
					end
				end
			end
		elseif direction == "left" then
			for i=1,n do
				for j=1,m do
					if self.items:get(i,j) == "@" and j-1>0 then
						if self.items:get(i,j-1) ~= "X" and self.items:get(i,j-1) ~= "@" and self.ground:get(i,j-1) ~= "#" then
							self.items:set(i,j, " ")
							self.items:set(i,j-1, "@")
						end
					elseif self.items:get(i,j) == "X" then
						if j+1 <= m and j-1 > 0 then
							if self.items:get(i,j+1) == "@" and self.items:get(i, j-1) ~= "X" and self.ground:get(i, j-1) ~= "#" then
								self.items:set(i,j, " ")
								self.items:set(i,j-1, "X")
							end
						end
					end
				end
			end
		elseif direction == "right" then
			for i=1,n do
				for j=m,1,-1 do
					if self.items:get(i,j) == "@" and j+1<=m then
						if self.items:get(i,j+1) ~= "X" and self.items:get(i,j+1) ~= "@" and self.ground:get(i,j+1) ~= "#" then
							self.items:set(i,j, " ")
							self.items:set(i,j+1, "@")
						end
					elseif self.items:get(i,j) == "X" then
						if j+1 <= m and j-1 > 0 then
							if self.items:get(i,j-1) == "@" and self.items:get(i, j+1) ~= "X" and self.ground:get(i, j+1) ~= "#" then
								self.items:set(i,j, " ")
								self.items:set(i,j+1, "X")
							end
						end
					end
				end
			end
		end
	end

	return newlevel
end

return level
