local levellib = require("level")

function love.load()
	level = levellib.deserialize("lvl3.txt")
end

function love.update(dt) end

function love.keypressed(key, scancode, isrepeat)
	if key == "up" then
		level:move("up")
	elseif key == "down" then
		level:move("down")
	elseif key == "left" then
		level:move("left")
	elseif key == "right" then
		level:move("right")
	elseif key == "u" then
		level:undo()
	end
end

function love.draw()
	level:draw(0, 0, 50)
end
