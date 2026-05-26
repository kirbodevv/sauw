xcrun devicectl list devices \
	| grep -i iPhone \
	| grep -v unavailable \
	| grep -E -o "[0-9a-fA-F]{8}(-[0-9a-fA-F]{4}){3}-[0-9a-fA-F]{12}" \
	| head -n 1
