# FibBot

A GitHub Action that calculates Fibonacci numbers from PR content

## Usage

To use this action, add the following to your workflow:

```yaml
name: FibBot Workflow

on:
  pull_request:
    types: [opened, synchronize]

jobs:
  fibbot:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v2

      - name: Run FibBot
        uses: ./fibbot
        with:
          enable_fib: 'true'
          max_threshold: '100'