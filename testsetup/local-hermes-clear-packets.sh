# clears the packages in the channel as they frequently get stuck

while true
do
    hermes clear packets --chain osmosis-dev-1 --port transfer --channel channel-0
    hermes clear packets --chain celestia-dev-1 --port transfer --channel channel-0
    sleep 1
done
