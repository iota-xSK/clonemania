local queue = {}

function queue.new()
	local newqueue = {
		head = -1,
		tail = 0,
		rawlist = {},
	}

	function newqueue:enqueue(item)
		self.tail = self.tail - 1
		self.rawlist[self.tail] = item
	end

	function newqueue:dequeue()
		if self.head < self.tail then
			return nil
		end

		local item = self.rawlist[self.head]
		self.head = self.head - 1
		self.item = nil
		return item
	end
	return newqueue
end

return queue
