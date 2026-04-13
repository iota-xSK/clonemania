local grid = {}

function grid.new(n, m) -- create new grid of width n, height m
	local newgrid = {}

	for i=1,m do
		newgrid[i]={}
		for j=1,n do
			newgrid[i][j] = " "
		end
	end

	function newgrid:get(x,y)
		if x > n or x < 1 then
			error(string.format("x needs to be in range [1, %d], got %d", n, x))
		end

		if y > m or y < 1 then
			error(string.format("y needs to be in range [1, %d], got %d", m, y))
		end

		return self[y][x]
	end

	function newgrid:set(x,y,cell)
		if x > n or x < 1 then
			error(string.format("x needs to be in range [1, %d], got %d", n, x))
		end

		if y > m or y < 1 then
			error(string.format("y needs to be in range [1, %d], got %d", m, y))
		end

		self[y][x] = cell
	end
	return newgrid
end

return grid
