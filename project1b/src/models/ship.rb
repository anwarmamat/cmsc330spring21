# ===========================================
# =====DON'T modify the following code=======
# ===========================================

class Ship
    attr_reader :start_position, :orientation, :size

    def initialize(start_position, orientation, size)
        @start_position = start_position
        @orientation = orientation
        @size = size
    end

    def to_s
        "Ship: #{@start_position}, #{orientation}, #{@size}"
    end
end
