local sprite = app.activeSprite
if not sprite then
    print("ERROR: No active sprite. Open a file via CLI: aseprite -b file.aseprite --script-param cols=N --script this_script.lua")
    return
end

local cols = tonumber(app.params["cols"])
if not cols or cols < 1 then
    print("ERROR: Missing or invalid 'cols' parameter. Pass --script-param cols=N")
    return
end

local total_frames = #sprite.frames
local num_rows = math.ceil(total_frames / cols)

local nonempty = {}
for _, cel in ipairs(sprite.cels) do
    local fn = cel.frameNumber
    if not nonempty[fn] and cel.image then
        for pixel in cel.image:pixels() do
            if app.pixelColor.rgbaA(pixel()) > 0 then
                nonempty[fn] = true
                break
            end
        end
    end
end

while #sprite.tags > 0 do
    sprite:deleteTag(sprite.tags[1])
end

local tags_created = 0
for row = 1, num_rows do
    local row_start = (row - 1) * cols + 1
    local row_end = math.min(row * cols, total_frames)

    local first_nonempty = nil
    local last_nonempty = nil
    for f = row_start, row_end do
        if nonempty[f] then
            if not first_nonempty then
                first_nonempty = f
            end
            last_nonempty = f
        end
    end

    if first_nonempty and last_nonempty then
        local tag = sprite:newTag(first_nonempty, last_nonempty)
        tag.name = "animation_" .. row
        tags_created = tags_created + 1
        print(string.format("  %s: frames %d-%d (%d frames)",
            tag.name, first_nonempty, last_nonempty, last_nonempty - first_nonempty + 1))
    else
        print(string.format("  Row %d: skipped (empty)", row))
    end
end

sprite:saveAs(sprite.filename)
print(string.format("\nDone: %d tags added to %s", tags_created, sprite.filename))
