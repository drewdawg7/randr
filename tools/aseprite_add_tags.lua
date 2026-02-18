local sprite = app.activeSprite
if not sprite then
    print("ERROR: No active sprite")
    return
end

local FRAME_SIZE = 32
local original_image = sprite.cels[1].image:clone()

local rows = {}
for _, slice in ipairs(sprite.slices) do
    local row = math.floor(slice.bounds.y / FRAME_SIZE)
    local col = math.floor(slice.bounds.x / FRAME_SIZE)
    if not rows[row] then rows[row] = {} end
    table.insert(rows[row], { col = col, bounds = slice.bounds })
end

local sorted_rows = {}
for row_idx, slices in pairs(rows) do
    table.sort(slices, function(a, b) return a.col < b.col end)
    table.insert(sorted_rows, { index = row_idx, slices = slices })
end
table.sort(sorted_rows, function(a, b) return a.index < b.index end)

local function has_content(image, x, y)
    for py = y, y + FRAME_SIZE - 1 do
        for px = x, x + FRAME_SIZE - 1 do
            if app.pixelColor.rgbaA(image:getPixel(px, py)) > 0 then
                return true
            end
        end
    end
    return false
end

local new_sprite = Sprite(FRAME_SIZE, FRAME_SIZE, original_image.colorMode)
local frame_num = 0

for i, row in ipairs(sorted_rows) do
    local tag_start = frame_num + 1
    for _, slice in ipairs(row.slices) do
        if not has_content(original_image, slice.bounds.x, slice.bounds.y) then
            break
        end
        frame_num = frame_num + 1
        if frame_num > 1 then
            new_sprite:newEmptyFrame()
        end
        local cel = new_sprite:newCel(new_sprite.layers[1], frame_num)
        for py = 0, FRAME_SIZE - 1 do
            for px = 0, FRAME_SIZE - 1 do
                cel.image:drawPixel(px, py,
                    original_image:getPixel(slice.bounds.x + px, slice.bounds.y + py))
            end
        end
    end
    if frame_num >= tag_start then
        local tag = new_sprite:newTag(tag_start, frame_num)
        tag.name = "animation_" .. i
        print(string.format("  %s: frames %d-%d (%d frames)",
            tag.name, tag_start, frame_num, frame_num - tag_start + 1))
    end
end

new_sprite:saveAs(sprite.filename)
sprite:close()
print(string.format("\nDone: %s -> %d frames", new_sprite.filename, frame_num))
