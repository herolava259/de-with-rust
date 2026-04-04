## advanced setup 

- conditional jobs:
```yaml
if: contains(github.event.head_commit.message, 'deploy')
```

- caching deps:

```yml
- uses: actions/cache@3
  with:
    path: ~/.cache/pip
    key: ${{ runner.os }}-pip-${{ hashFiles('**/requirements.txt')}}
```

- matrix builds:

```yml
strategy:
  matrix:
    python-version: [3.9, 3.10, 3.11]
```

- artifacts (store outputs)

```yml
- name: Upload artifact
  uses: actions/upload-artifact@v3
  with:
    name: test-results
    path: results/
```

- scheduled-workflows:

```yml
on:
  schedule:
    - cron: "0 0 * * *"
```

- data pipeline validation job 

```yml
data-pipeline:
  name: Run Data Pipeline
  runs-on: ubuntu-latest
  needs: build-test

  steps:
    - uses: action/checkout@v3

    - name: Install Rust 
    - run: |
        cargo run -- data/sample.csv output.csv
    - name: Validate output
      run: |
        test -f output.csv
        wc -l output.csv
    # data validation check
    - name: Check data quality
      run: | 
        rows=$(wc -l < output.csv)
        if [ "$rows" -lt 10 ]; then
          echo "Dataset too small"
          exit 1
        fi
```

- add docker build:

```yml
docker:
  name: Build Docker Image
  runs-on: ubuntu-latest
  needs: build-test

  steps:
    - uses: actions/checkout@v3

    - name: Build Docker image
      run: docker build -t rust-data-pipeline .
    
    - name: Run container
      run: docker run rust-data-pipeline
```

- optional: deploy / artifact upload 

```yml
- name: Upload output
  uses: actions/upload-artifact@v3
  with: 
    name: processed-data
    path: output.csv
```