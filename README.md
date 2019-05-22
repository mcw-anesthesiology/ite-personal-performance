# ITE Personal Performance

Simple set of tools to extract discrete information from ACGME's ITE Personal
Performance Reports.

Currently only 2019 topics are supported.

## Score extractor

Converts a raw dump of ITE Personal Performance Score reports into
a CSV detailing the various scores each trainee.

Input is accepted from stdin, output CSV is sent to stdout.

### Usage

```bash
$ pdftotext -raw ITE_Score_Report.pdf - | ite-personal-scores
```

### Example output

```csv
Name,Scaled,Basic,Advanced,Basic Sciences # questions,Basic Sciences # correct,Basic Sciences 50%-ile,Basic Sciences 75%-ile,Basic Sciences 90%-ile,Clinical Sciences # questions,Clinical Sciences # correct,Clinical Sciences 50%-ile,Clinical Sciences 75%-ile,Clinical Sciences 90%-ile,Organ-based Basic & Clinical Sciences # questions,Organ-based Basic & Clinical Sciences # correct,Organ-based Basic & Clinical Sciences 50%-ile,Organ-based Basic & Clinical Sciences 75%-ile,Organ-based Basic & Clinical Sciences 90%-ile,Clinical Subspecialties # questions,Clinical Subspecialties # correct,Clinical Subspecialties 50%-ile,Clinical Subspecialties 75%-ile,Clinical Subspecialties 90%-ile,Special Problems or Issues in Anesthesiology # questions,Special Problems or Issues in Anesthesiology # correct,Special Problems or Issues in Anesthesiology 50%-ile,Special Problems or Issues in Anesthesiology 75%-ile,Special Problems or Issues in Anesthesiology 90%-ile
…
```

## Keyword extractor

Converts a raw text dump of of ITE Personal Performance keyword reports into a
CSV detailing which topics were missed by each trainee. An `x` in a given
column indicates that the topic was missed by the trainee.

Input is accepted from stdin, output CSV is sent to stdout.

### Usage

```bash
$ pdftotext -raw ITE_Keyword_Report.pdf - | ite-personal-keywords
```

### Example output

```csv
"Name","Topic 1","Topic 2","Topic 3"
"Trainee 1",x,,x
"Trainee 2",,x,x
```


## Clusterer

Creates study groups for trainees based on their missed topics. You'll likely
want to take a look at `analysis/clustering.py` and tweak the algorithm for
your particular use case, but by default it creates 3 study groups using
K-Means clustering, and displays the top 5 missed topics in common for each
group. Number of groups and missed topics can be customized using command line
arguments.

### Usage

The following will create 4 study groups and display the top 6 missed topics in
common for each group.

```bash
pdftotext -raw ITE_Keyword_Report.pdf - | ite-personal-keywords | python clustering.py 4 6
```

### Example output

Output is a simple markdown document listing the groups and the missed topics
in common for each group.


```markdown
## Group 1

- Trainee 1
- Trainee 3
- Trainee 4
- Trainee 7

### Topics in common

- Aging: Phys changes (A): 4 / 4 missed (%100)
- Ambulatory surg: Fast track criteria (A): 3 / 4 missed (%75)

## Group 2

- Trainee 2
- Trainee 5
- Trainee 6

### Topics in common

- Airway fire prevention (A): 3 / 3 missed (%100)
- Ambulatory surg: Fast track criteria (A): 3 / 3 missed (%100)
```

For ease of distribution, the document can be rendered as a Word DOCX (or
anything else) easily using pandoc:

```bash
pdftotext -raw ITE_Keyword_Report.pdf - | ite-personal-keywords | python clustering.py | pandoc -i -f markdown -o study-groups.docx
```
