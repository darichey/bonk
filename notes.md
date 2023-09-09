How do I want this to work?

## The data
The raw transaction data has to come from somewhere like my bank. Maybe eventually that can be pulled more easily with Plaid or something. I want to check that raw data in so I can have it forever without worrying about it going away.

So the raw data is checked in, but I want to "lift" it to a form that is easier to deal with in the application. Primarily, this will be getting everything from the different data sources into the same format, and constructing some in memory db to hold it for faster queries. Since the raw data is checked in, we can just do this process every time the application inits, and don't need to save anything else.