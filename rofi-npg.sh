#!/bin/sh

/home/ly/Documents/programming/rust/npg/target/release/npg list >> /dev/null

ACCOUNT_PATH="$(/home/ly/Documents/programming/rust/npg/target/release/npg list | fuzzel --dmenu )"
if [ -z "$ACCOUNT_PATH" ]; then
	return
fi

OPTION="$(printf "autotype\npassword\nusername\nemail" | fuzzel --dmenu)"
if [ -z "$OPTION" ]; then
	return
fi

if [ "$OPTION" = "autotype" ]; then
	PASSWORD="$(/home/ly/Documents/programming/rust/npg/target/release/npg show -a "$ACCOUNT_PATH" | head -n1)"
	USERNAME="$(/home/ly/Documents/programming/rust/npg/target/release/npg show -a "$ACCOUNT_PATH" | grep "username:" | awk '{print $2}')"
	echo type "$USERNAME" | doas dotool
	echo "$USERNAME"
	echo key tab | doas dotool
	echo type "$PASSWORD" | doas dotool
elif [ "$OPTION" = "password" ]; then
	PASSWORD="$(/home/ly/Documents/programming/rust/npg/target/release/npg show -a "$ACCOUNT_PATH" | head -n1)"
	echo type "$PASSWORD" | doas dotool
elif [ "$OPTION" = "username" ]; then
	USERNAME="$(/home/ly/Documents/programming/rust/npg/target/release/npg show -a "$ACCOUNT_PATH" | grep "username:" | awk '{print $2}')"
	echo type "$USERNAME" | doas dotool
elif [ "$OPTION" = "email" ]; then
	EMAIL="$(/home/ly/Documents/programming/rust/npg/target/release/npg show -a "$ACCOUNT_PATH" | grep "username:" | awk '{print $2}')"
	echo type "$EMAIL" | doas dotool
fi
