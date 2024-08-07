local lowerBound, upperBound = unpack(get("color_range"))

function randomizePlayerColor(player)
	player = player or source
	local r, g, b = math.random(lowerBound, upperBound), math.random(lowerBound, upperBound), math.random(lowerBound, upperBound)
	setPlayerNametagColor(player, r, g, b)
end
addEventHandler("onPlayerJoin", root, randomizePlayerColor)

local function setAllPlayerColors()
	for _, player in ipairs(getElementsByType("player")) do
		if eventName == "onResourceStop" then
			setPlayerNametagColor(player, false)
		else
			randomizePlayerColor(player)
		end
	end
end
addEventHandler("onResourceStart", resourceRoot, setAllPlayerColors)
addEventHandler("onGamemodeMapStart", root, setAllPlayerColors) -- mapmanager resets player colors to white when the map ends
addEventHandler("onResourceStop", resourceRoot, setAllPlayerColors)

addEventHandler('onPlayerChat', root,
	function(msg, type)
		if type == 0 then
			cancelEvent()
			local r, g, b = getPlayerColor(source)
			local name = getPlayerName(source)
			msg = msg:gsub('#%x%x%x%x%x%x', '')
			outputChatBox( name.. ': #FFFFFF' .. msg, root, r, g, b, true)
			outputServerLog( "CHAT: " .. name .. ": " .. msg )
		end
	end
)

getPlayerColor = getPlayerNametagColor
getPlayerColour = getPlayerNametagColor
