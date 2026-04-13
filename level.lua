local gridlib = require("grid")
local level = {}

function level.new(n, m)
	local newlevel = {
		ground = gridlib.new(n, m),
		clones = {},
		boxes = gridlib.new(n, m)
	}

	function newlevel:fromstr(strtable)
		for rowc, row in ipairs(strtable) do
			for i=1,m do
				local thing = string.sub(row, i,i)

				if thing == "#" or thing == " " or thing == "." then
					self.ground:set(rowc, i, thing)
				elseif thing == "@" then
					local clone = {position = {rowc, i}, want_move = {0, 0}}
					table.insert(self.clones, clone)
					
				elseif thing == "X" then
					self.boxes:set(rowc, i, thing)
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

				if self.boxes:get(i, j) == "X" then
					local rect_startx = x+(j-1)*width
					local rect_starty = x+(i-1)*width

					love.graphics.setColor(0.3, 0.3, 0.0)
					love.graphics.rectangle("fill",rect_startx + width/10, rect_starty + width/10, width - 2*width/10, width - 2*width/10 )
				end
			end
		end

		for k, v in ipairs(self.clones) do
			local i, j = unpack(v.position)
			love.graphics.setColor(0.5, 0.1, 0.1)
			love.graphics.rectangle("fill", x+(j-1)*width, y+(i-1)*width+width/2, width, width/2, width/10)
			love.graphics.setColor(1, 1, 0)
			love.graphics.circle("fill", x+(j-1)*width + width / 2,y+(i-1)*width + width/4 , width/4)
		end

	end

	function newlevel:move(direction)
		local dxy = {
			left = {0, -1},
			right = {0, 1},
			up = {-1, 0},
			down = {1, 0}
		}
		for i=1,#self.clones do
			local x, y = unpack(self.clones[i].position)
			local dx, dy = unpack(dxy[direction])
			local mx = dx + x
			local my = dy + y

			if mx <= n and mx >= 1 and my <= m and my >= 1 then
				if self.ground:get(mx, my) ~= "#" then
					self.clones[i].want_move = {dx, dy}
					self.clones[i].position = {mx, my}
				end
			end
		end

		for i=1,#self.clones do
			local x, y = unpack(self.clones[i].position)
			if self.boxes:get(x, y) == "X" then
				local dx, dy = unpack(self.clones[i].want_move)
				local mx = x+dx
				local my = y+dy
				
				if mx <= n and mx >= 1 and my <= m and my >= 1 and 
					self.boxes:get(mx, my) == " " and self.ground:get(mx, my) ~= "#" then
					self.boxes:set(mx, my, "X")
					self.boxes:set(x, y, " ")
				else
					self.clones[i].position[1] = self.clones[i].position[1] - self.clones[i].want_move[1]
					self.clones[i].position[2] = self.clones[i].position[2] - self.clones[i].want_move[2]
				end

			end
		end

		for n=1,#self.clones do -- I have no clue why this works but it does
			for i=1,#self.clones do
				for j=1,#self.clones do
					if i ~= j then
						local x1, y1 = unpack(self.clones[i].position)
						local x2, y2 = unpack(self.clones[j].position)
						if x1 == x2 and y1 == y2 then
							self.clones[i].position[1] = self.clones[i].position[1] - self.clones[i].want_move[1]
							self.clones[i].position[2] = self.clones[i].position[2] - self.clones[i].want_move[2]
						end
					end
				end
			end
		end

		for i=1,#self.clones do
			self.clones[i].want_move = {0, 0}
		end
	end

	return newlevel
end

return level
