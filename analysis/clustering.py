import numpy as np
from sklearn import cluster, metrics

import csv, sys


def main():
    n_clusters = int(sys.argv[1]) if len(sys.argv) > 1 else 3
    trunc_misses = int(sys.argv[2]) if len(sys.argv) > 2 else 5

    # alg = cluster.AffinityPropagation()
    alg = cluster.KMeans(n_clusters=n_clusters, init="random")
    # alg = cluster.DBSCAN(eps=7, min_samples=3)
    # alg = cluster.SpectralClustering(n_clusters=n_clusters)
    # alg = cluster.MeanShift(bandwidth=6.9)

    dump_groups(*make_groups(sys.stdin, alg), trunc_misses)


def dump_groups(groups, get_topics_in_common, trunc_misses=5):
    for group_name, trainees in groups.items():
        print_group(trainees, group_name, get_topics_in_common(trainees), trunc_misses)


def make_groups(stream, alg):
    reader = csv.reader(stream)
    names = []
    missed_topics = []

    keywords = reader.__next__()[1:]
    for row in reader:
        row = iter(row)
        names.append(row.__next__())
        missed_topics.append(
            [1 if cell == "x" else 0 for cell in row if cell == "x" or cell == ""]
        )

    features = np.array(missed_topics)

    clustering = alg.fit(features)
    labels = clustering.labels_

    groups = group(labels, names)

    def get_topics_in_common(trainees):
        d = {}

        for trainee in trainees:
            missed = missed_topics[names.index(trainee)]
            for i, was_missed in enumerate(missed):
                d.setdefault(i, []).append(was_missed)

        return [
            (keywords[i], list)
            for i, list in sorted(
                d.items(), key=lambda kv: (-1 * sum(kv[1]), keywords[kv[0]])
            )
        ]

    return groups, get_topics_in_common


def print_group(members, group_name, topic_misses=None, trunc_misses=5):
    print("## {}\n".format(group_name))

    for member in members:
        print("- {}".format(member))

    if topic_misses is not None:
        print("\n\n### Topics in common\n")

        if trunc_misses is not None:
            topic_misses = topic_misses[:trunc_misses]

        for topic, misses in topic_misses:
            print(
                "- **{}**: {} / {} missed (%{})".format(
                    topic,
                    sum(misses),
                    len(misses),
                    round((sum(misses) / len(misses)) * 100),
                )
            )

    print("\n")


def group(labels, names):
    groups = {label: [] for label in set(labels)}
    for i, label in enumerate(labels):
        groups[label].append(names[i])

    groups = {
        "Group {}".format(i + 1): names
        for i, names in enumerate([names for label, names in groups.items()])
    }

    return groups


if __name__ == "__main__":
    main()
