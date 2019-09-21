#!/bin/bash

echo "Hello, do you want enable console log on file? (y,N)?"
read consoleLog

process=$(pgrep money-manager-server)
if [ ! -z $process ]; then
	echo "Killing process ${process}"
	kill -9 $process
fi

echo "Cloning..."

git fetch
git reset --hard origin/master
git checkout master
git pull
chmod +x run.sh

echo "Preparing..."

if [ ! -d "/var/log/money-manager" ];  then
	mkdir /var/log/money-manager
fi

echo "Compiling..."

cargo build

echo "Running..."

if [ ! -z $consoleLog ] && [ $consoleLog == 'y' ]; then
	echo "Console Log Enabled!"
	exec ./target/debug/money-manager-server &> console.log &
else
	echo "Console Log Disabled!"
	exec ./target/debug/money-manager-server &> /dev/null &
fi

echo "Ok!"
