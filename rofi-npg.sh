#!/bin/dash

ACCOUNT_PATH="$(/home/ly/.local/bin/npg list | fuzzel --dmenu )"
if [ -z "$ACCOUNT_PATH" ]; then
	return
fi

OPTION="$(echo "autotype\npassword\nusername\nemail" | fuzzel --dmenu)"
if [ -z "$OPTION" ]; then
	return
fi

if [ "$OPTION" = "autotype" ]; then
	PASSWORD="$(/home/ly/.local/bin/npg show -a "$ACCOUNT_PATH" | head -n1)"
	USERNAME="$(/home/ly/.local/bin/npg show -a "$ACCOUNT_PATH" | grep "username:" | awk '{print $2}')"
	doas ydotool type "$USERNAME"
	doas ydotool key 15:1 15:0
	doas ydotool type "$PASSWORD"
elif [ "$OPTION" = "password" ]; then
	PASSWORD="$(/home/ly/.local/bin/npg show -a "$ACCOUNT_PATH" | head -n1)"
	doas ydotool type "$PASSWORD"
elif [ "$OPTION" = "username" ]; then
	USERNAME="$(/home/ly/.local/bin/npg show -a "$ACCOUNT_PATH" | grep "username:" | awk '{print $2}')"
	doas ydotool type "$USERNAME"
elif [ "$OPTION" = "email" ]; then
	EMAIL="$(/home/ly/.local/bin/npg show -a "$ACCOUNT_PATH" | grep "username:" | awk '{print $2}')"
	doas ydotool type "$EMAIL"
fi
