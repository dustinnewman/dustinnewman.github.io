---
title: "The Unreasonable Pain of Archiving Twitter"
date: 2022-06-11T18:34:06-07:00
draft: false
---

I am currently in the process of archiving a large number of tweets. I am keeping these tweets in the hopes that I can look back later and either laugh at the hubris or bang my head for not seeing the obvious writing on the wall. Naturally, I have an interest in making sure these tweets are viewable long into the future, perhaps years. Who knows what the world will look like in 10 years, much less 40. Will Markdown be popular? Will Twitter have faded from existence? Will even the mighty HTML slip into memory? This uncertainty makes archiving a tricky, even slightly neurotic task. The entire point is to future-proof right now. I want to gather all the important data right now, to keep forever, because it is very likely, almost guaranteed, 99% of it will no longer exist or be reachable. Since I want these tweets to be in the best possible format, that is the one that gives the highest chance of being easily readable in the future, I have gone through quite the dilemma with this archiving journey. These are roughly the steps and pitfalls I have encountered along the way.

As I cover each area, I will use a recent [tweet by Reuters](https://twitter.com/Reuters/status/1535478193183924224/) as an illustrative example.

![Screenshot of tweet by Reuters reading "U.S. CDC rescinds COVID-19 international air testing rules" with an image of a man walking in an airport past an orange sign that reads "COVID-19 Testing".](./1535478193183924224.png)

## Paper

Let's just get it out of the way right now. Yes, a hard (paper) copy is the best format in terms of longevity. You need no interpreters, no software, not even electricity: just your eyesight. However, seeing as I am archiving upwards of twenty thousand tweets, suffice it to say that I have neither the space nor the ink to print and save twenty thousand pages of tweets.

## Screenshots

One of the easiest archival methods is to take screenshots. It has certainly worked for many a cancelled celebrity and proven fairly durable. Screenshots are immediately visual, not prohibitively large in size, and intuitive. They preserve quite a bit of extra information as well, from the current design of Twitter to likes, retweets, and timestamps. You could even name the files by tweet ID to save that information meta-textually as well. However, because they are image formats, I am reliant on PNG files being readable long into the future. Granted, since we have so much data already saved as PNGs, there is a certain "safety in numbers" logic to saving tweets as PNGs. However, this is personally not a risk I want to take. Further, if I need to do any analysis or search of the tweets in the future (which is somewhat likely), an image format will make this impossible barring OCR.

## HTML

Boom, just go right to `File > Save As... > HTML` and you have an exact copy of what you see in your browser. Plus, it's plaintext, preserves all visible information about the tweet, and is in a common format unlikely to be dropped or forgotten. Let's do it!

![Screenshot of Twitter error message reading "Hmm... this page doesn't exist. Try searching for something else."](./html.png)

Oh. Yep, that's right. Twitter will only show the actual tweet contents if you are browsing from twitter.com. I wasn't quite motivated enough to figure out the nitty-gritty of the various ways they do this and doing so would mean I would need to write some kind of "unlocker" script to remove all these measures which is not only time-consuming, but somewhat tampers with the authenticity of the archives themselves, since I modified them, but we can see that preventing off-site browsing is at least somewhat intentional on the part of Twitter from this little snippet at the end of their page:

```html
<script nonce="abcdefg123456789">document.cookie = decodeURIComponent("gt=123456789; Max-Age=10800; Domain=.twitter.com; Path=/; Secure");</script>
```

(And yes, I grepped the HTML file already, there is very little information preserved in the file itself. It's not as though there is a full-text copy of the tweet in the file and Twitter merely includes a script to obfuscate this. Actually loading the tweet needs to pass some origin checks.)

## DOM Dump

But that was just the page source. With SPA apps, it is very common for the actual page source to be nothing but a JavaScript wrapper that loads the app itself. Once loaded, you can capture the DOM tree as it appears in your browser to get a view of exactly what was on your screen at that time. Sure, it might require some trickery to programmatically capture this DOM dump, but once you have it, it will be identical to what you saw.

![Screenshot of Twitter error message reading "Hmm... this page doesn't exist. Try searching for something else."](./html.png)

Oh.

It is worth noting that this DOM dump HTML does actually include the tweet as we can see here

```html
<span class="css-901oao css-16my406 r-poiln3 r-bcqeeo r-qvutc0">U.S. CDC rescinds COVID-19 international air testing rules </span>
```

as well as a lot of information in the meta tags in the head. However, this is still not ideal for archiving. I would either need to strip out all this DRM-esque script nonsense (thus raising the possibility I might modify the contents of these tweets accidentally or on purpose) or not be able to open the HTML file in the future.

### WebArchive

Okayyy, so just plain HTML is out of the picture. However, the Internet Archive stores tons of snapshots of tweets every day. Surely, they must have a way right?

![Screenshot of Internet Archive snapshot of the tweet. The tweet is full width and missing a lot of the styling of the original.](./wayback_machine.png)

While they do seem to be more successful than I was, this too has its own share of problems. The first being that we want to archive over twenty thousand tweets, a task which would not only result in my likely ban from the Internet Archive, but would also be reliant on the Archive both (a) staying up and live and (b) not removing/pruning large amounts of my archive. Seeing as the vast, vast majority of these tweets will only be of interest to me, this is not a risk I want to take. 

So, let's make our own archive then. After all, the WARC format is open-source and standardized, so there is broad support for keeping these files into the future. We run `File > Save As... WebArchive` and...

![Screenshot of original Reuters tweet.](./1535478193183924224.png)

Huzzah! Now that that's out of the way, let's just turn off our Wi-Fi connectivity and make sure that-

![Screenshot of error message.](./web_archive.png)

I almost didn't believe it, but, yes, for some reason, when the Internet Archive makes an archive of a tweet, it is okay. However, when I do it, I am required to have Wi-Fi connectivity and connect to Twitter's servers. We can clearly see that these two WebArchives (mine vs the Internet Archive's) are different even without shutting our Wi-Fi off. Theirs has different styling than the usual Twitter UI, not just night mode vs light mode, but the entire page seems to be a special, slimmed-down version of what most people see. Whereas my WebArchive was 6.7 MB of all that JavaScript goodness, the one I downloaded from the Wayback Machine was a measly 431 KB. To me, this indicates that either Internet Archive or Twitter is specifying the equivalent of `User-Agent: InternetArchive` somewhere and that changes the archive file from a bloated, network-dependent mess into a minimal, standalone file. Regardless, there is no way I can ask the Internet Archive to download some twenty thousand tweets, so we need to find another way.

### JSON

And we arrive. The last stop of our journey. For now, I have decided to archive all of these tweets in JSON format with the help of the Twitter API. Even with the free tier, I am allowed up to 500k tweets per month, more than enough for my needs. JSON allows us quite a bit of future-proofing. Even if JSON is lost and forgotten someday, it is still a plaintext format, meaning that, as long as we avoid some of the more crazy or nested structures, we should be okay and able to convert to a variety of different formats later. It is far easier to analyze/search than screenshots and more Twitter-proof than WebArchives since, as we saw, Twitter can call their own APIs and render our archives invalid any time they wish. With JSON, at least I have the raw data I am interested in. If needed, I figure I can always write a parser to take all the JSON files and plug them into a duplicate of the Twitter UI later. Even if the authenticity of the UI is in question, at least I will know the data itself will be from the source. For reference, the schema will look something like this

```json
{
    author_id: "",
    conversation_id: "",
    entities: {
        mentions: [],
        referenced_tweets: [],
    },
    text: "",
    id: "",
    created_at: "",
    public_metrics: {
        retweet_count: 0,
        reply_count: 0,
        like_count: 0,
        quote_count: 0,
    },
    source: "",
    in_reply_to_user_id: ""
}
```

Basically, we can support a fair bit of complexity from replies to retweets to retweets as replies, all encoded with tweet and user IDs rather than relying on HTML-facing metrics such as handle (which can change over time). This is also by far the slimmest, most minimal of the formats, which, given I am working with a 2 TB flash drive, is much appreciated.

If you came to this post from a search engine/social media site expecting a golden solution, I have nothing for you, sorry. Except complaints. I have many complaints.

