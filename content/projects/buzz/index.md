---
title: Buzz
subtitle: Daily production base for Daily Bruin
date: 2019-05-23
image: buzz.png
role: Creator
stack: MongoDB, Express, React, Node
github: dailybruin/buzz
live: https://buzz.dailybruin.com
---

This was the first Daily Bruin project I created entirely on my own. It was common to come into the office with blue papers (design notes) strewn about every which way on all sorts of tables, filled with mark ups and notes. Aside from being a huge paper waster, design notes were also inefficient and required editors to physically be in the office to complete them. While this wasn't a problem during the school year, over the summer, designers had to hunt down editors through Slack to get the necessary information for that day's paper. Thus, enter Buzz.

Buzz was started and finished within the span of about a week, most of that time being spent debugging the Slack OAuth API. Technically, Buzz is a simple MERN app, with a heavy focus on the "R". Designing any one specific schema for a newspaper is notoriously challenging, as unstructured data is essentially the norm. For example, one of my - in my opinion - most reasonable assumptions for Buzz was that "word count" would be a number. Duh right?

Not duh. Within the very day Buzz was deployed, I got an error about something not being a number. You see, someone had entered "500~600" for the word count for a particular story. In hindsight, this was completely obvious: of course editors do not know exact word counts for stories in advance. Often, they have a reasonable range and to represent a range, you have to be a little more lenient than just numbers. 

But Buzz's flexibility extends far further. I made the decision fairly early on that the backend would be nothing more than a data store and message handler for the frontend. The frontend dictates the "schema" or structure of data saved. The backend itself just saves whatever the frontend passes to it. This way, if a section - say, Opinion - wants to add a completely new part of the paper, *no change needs to be made on the backend at all!* All it takes is changing the hard-coded form on the frontend and everything saves without problem.

Some of course may object to the lack of data integrity, especially in the long term. Well, the long term part is "solved" since Buzz deletes all data (except member lists) every 14 days. This is a domain-specific optimization that arose from using a free instance of a MongoLab database, as well as the fact that design notes are extremely ephemeral. Integrity in general is absolutely mandatory for more generic programs and indeed should be the default wherever possible. However, so far Buzz has faced - by far - the fewest bugs and the smallest downtime of any other internal tool at the Bruin. 

While Buzz is certaintly not any engineering marvel, it was a good example for me personally to see the effects that problem domain have on software design. As the oft-hated, overquoted maxim goes "choose the right tool for the right job."
