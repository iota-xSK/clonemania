local levellib = require("level")

function love.load()
	level = levellib.new(7, 5)
	level:fromstr({
		"  X  ",
		"#X   ",
		" X@@ ",
		"  @@ ",
		"   X ",
		"     ",
		"     ",
	})
end

function love.update(dt)
end

function love.keypressed( key, scancode, isrepeat )
	if key == "up" then
		level:move("up")
	elseif key == "down" then
		level:move("down")
	elseif key == "left" then
		level:move("left")
	elseif key == "right" then
		level:move("right")
	end
end

function love.draw()
	level:draw(0, 0, 50)
end
