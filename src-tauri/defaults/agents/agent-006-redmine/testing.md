## Test: review ticket
Review Redmine ticket 7209.
Expected: REDMINE_API: GET /issues/7209.json?include=journals,attachments then reply with Summary, Status & completion, Missing, Final thoughts.

## Test: search issues
Search Redmine for "monitoring".
Expected: REDMINE_API: GET /search.json?q=monitoring&issues=1&limit=100 then summarize results.

## Test: add comment
Add a comment "Status checked, no blockers" to ticket 7209.
Expected: First GET the issue, then REDMINE_API: PUT /issues/7209.json {"issue":{"notes":"Status checked, no blockers"}}
