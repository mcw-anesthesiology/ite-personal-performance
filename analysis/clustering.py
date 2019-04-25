import numpy as np

from sklearn import cluster, metrics

from pprint import pprint
import csv, sys


def main():
    reader = csv.reader(sys.stdin)
    names = []
    missed_topics = []

    header = reader.__next__()
    for row in reader:
        row = iter(row)
        names.append(row.__next__())
        missed_topics.append(
            [1 if cell == "x" else 0 for cell in row if cell == "x" or cell == ""]
        )

    threshold = None

    if threshold is not None:
        header_iter = iter(header)
        header_iter.__next__()

        topics_to_ignore = []

        for i, keyword in enumerate(header_iter):
            l = [row[i] for row in missed_topics]
            percent_missed = sum(l) / len(l)
            if percent_missed > threshold:
                topics_to_ignore.append(i)

        print(
            "Ignored topics: ",
            [header[i + 1] for i in topics_to_ignore],
            file=sys.stderr,
        )

        features = np.array(
            [
                [feature for i, feature in enumerate(row) if i not in topics_to_ignore]
                for row in missed_topics
            ]
        )
    else:
        features = np.array(missed_topics)

    alg = cluster.AffinityPropagation()
    # alg = cluster.KMeans(n_clusters=15)
    # alg = cluster.DBSCAN(eps=7, min_samples=3)
    # alg = cluster.SpectralClustering(n_clusters=15)
    # alg = cluster.MeanShift(bandwidth=6.9)

    clustering = alg.fit(features)
    labels = clustering.labels_

    print(clustering, file=sys.stderr)
    print(labels, file=sys.stderr)

    n_clusters_ = len(set(labels)) - (1 if -1 in labels else 0)
    n_noise_ = list(labels).count(-1)
    print("Estimated number of clusters: %d" % n_clusters_, file=sys.stderr)
    print("Estimated number of noise points: %d" % n_noise_, file=sys.stderr)

    groups = group(labels, names)

    ungrouped = [names[0] for label, names in groups.items() if len(names) == 1]
    groups = [names for label, names in groups.items() if len(names) > 1]

    pprint(groups, indent=4, stream=sys.stderr)

    print("Ungrouped trainees", file=sys.stderr)
    pprint(ungrouped, indent=4, stream=sys.stderr)

    print(
        "{} groups, {} groupless trainees".format(len(groups), len(ungrouped)),
        file=sys.stderr,
    )

    print(
        "Calinski-Harabaz index",
        metrics.calinski_harabaz_score(features, labels),
        file=sys.stderr,
    )
    print(
        "Davies Bouldin index",
        metrics.davies_bouldin_score(features, labels),
        file=sys.stderr,
    )

    for i, trainees in enumerate(groups):
        print_group(trainees, "Group {}".format(i + 1))

    print_group(ungrouped, "Ungrouped trainees")


def print_group(members, group_name):
    print("## {}\n".format(group_name))

    for member in members:
        print("- {}".format(member))

    print("\n")


def group(labels, names):
    groups = {label: [] for label in set(labels)}
    for i, label in enumerate(labels):
        groups[label].append(names[i])

    return groups


if __name__ == "__main__":
    main()
