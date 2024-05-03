This game is a Dating Sim Game!

The goal of this game was to create a basic dating sim with 3 different silly targets, to assign these targets at the beginning with a quiz,
and to then play through the date scenario with different experiences depending on the target. If you made enough bad choices then the 
game would end.

The first part of the code is importing different things I needed for my project, like std::io and rand::seq:Slice Random.
Then i set up different structs and enums like the Map, Room, Choice, State, and Target. Then I wrote a function to take the quiz that woud 
assign you to your dating target depending on the way in which you answered the quiz. If there was a tie then the target was randomly
selected.

After the selection, I set up the date. First I decided to figure out how to handle the choices being made. If you target had a 
Bad Valence for a choice then it would take away from the romance bar, and if they had a Good valence then it would add to the romance bar.
I also included unique messages that would print out if you selected a choice that had a Bad or Good Valence, and set it up so that
these options would only print for your selected target. If they didn't have a strong feeling about a choice then it just went with
the neutral choice. After making the choice it then would move to the next room. 

In the main function I called the quiz first, then set up the Map of rooms with the different options of choices for each event. These 
choices stored the unique responses for each of my silly dating targets. Finally, I had a section that made sure the rooms and choices
were printed out and that errors were caught, and that once it got to the final room the game ended.
