import plotly.figure_factory as ff
import itertools as it
import random

pb_num = 2
update_everything = False


def main():
    if not update_everything:
        update(pb_num)
    else:
        for i in range(7):
            update(i)


def update(num):
    f = open("{}_solution.txt".format(num)).readlines()
    df = []
    colors = {}
    for i, line in enumerate(f):
        tmp = [map(lambda x: int(x), line.split())]*4
        for job, task, start, end in it.zip_longest(*tmp):
            key = 'Job {}'.format(job + 1)
            df.append(dict(Task='Machine {}'.format(i),
                           Start=start,
                           Finish=end,
                           Resource=key))
            if key not in colors:
                colors.update({key: 'rgb({}, {}, {})'.format(random.randint(0, 255),
                                                                                    random.randint(0, 255),
                                                                                    random.randint(0, 255))})

    fig = ff.create_gantt(df, colors=colors, index_col='Resource', show_colorbar=True, group_tasks=True)
    fig['layout']['xaxis'].update({'type': None})
    fig.show()
    return 0


main()
