local levellib = require("level")

function love.load()
	level = levellib.deserialize("lvl1.txt")
end

function love.update(dt) end

local key_to_action = {
	up = function()
		level:move("up")
	end,
	down = function()
		level:move("down")
	end,
	left = function()
		level:move("left")
	end,
	right = function()
		level:move("right")
	end,
	u = function()
		level:undo()
	end,
}
function love.keypressed(key, scancode, isrepeat)
	if key_to_action[key] then
		key_to_action[key]()
	end
end

function love.draw()
	level:draw(0, 0, 50)
end
