name: Spell Check

# See: https://docs.github.com/en/free-pro-team@latest/actions/reference/events-that-trigger-workflows
on:
  push:
  pull_request:
  schedule:
    # Run every Tuesday at 8 AM UTC to catch new misspelling detections resulting from dictionary updates.
    - cron: "0 8 * * TUE"
  workflow_dispatch:
  repository_dispatch:

jobs:
  spellcheck:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repository
        uses: actions/checkout@v3

      - name: Spell check
        uses: codespell-project/actions-codespell@master
0 comments on commit f53e1fd
@mowjoejoejoejoe
 
Add heading textAdd bold text, <Ctrl+b>Add italic text, <Ctrl+i>
Add a quote, <Ctrl+Shift+.>Add code, <Ctrl+e>Add a link, <Ctrl+k>
Add a bulleted list, <Ctrl+Shift+8>Add a numbered list, <Ctrl+Shift+7>Add a task list, <Ctrl+Shift+l>
Directly mention a user or team
Reference an issue, pull request, or discussion
Add saved reply

