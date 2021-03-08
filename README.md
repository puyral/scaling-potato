# scaling-potato
## overview
The goal is making resumes of users' contributions at the Wikimedia Foundation (Wikipedia, Wikidata, Wiktionary, ...)

We want to show how important a given user is, and what are the categories he contributes the most (and maybe what kind of user it is : article maker, typo-correcting bot…).

To do so we'll have to define different metrics to measure the relevance of a given user. The important parameters seem to be the quantity of contributions and their size… And we are considering also using metrics from the pages itself, like its categories or some sort of PageRank (with the hyperlink).

The information will mostly come from the Wikipedia API. Maybe some third party database may be needed, and eventually some page parsing.

To get the different domains a user contributes to, we can use the categories already given by Wikipedia. It should suffice but if it doesn't we can look at the different hyperlink in the pages.

To display the information: we want to be inspired by GitLab's. We think that the website will consist of a research bar, where you can enter the name of the user. You will get the principal categories, and the link to the most important articles of the users given a specific category. We think that we will add easily some graphs about for example the evolution of the number of contributions. It will depend also on the libraries that we will find, and the info that we could have about the users.

Some other data that we can consider are the number of visitors of a given pages, the number of hyperlink, the number of reference etc… Whether the information really is up to date can be also an interesting question.

## access
Can be access [scaling-potato.ml](scaling-potato.ml).
