# Tube Status Project

This is a small server, using the twilio messaging service and some simple webscraping to provide daily updates on the london underground.

I wrote this to learn about rust, as such I apologise for the code in advance.

## Plan Currently: 
The server works, its a bit complex with the sql database, I think that it would be better to just have a json file with user info. I could also write a parser for this and not use any dependencies for v0.2
 - [ ] Replace SQL with json.
 - [ ] Make into single binary that can be run in a container easily.
 - [ ] This is a full rewrite I think. 
