import matplotlib.pyplot as plt

data = [(22, -2), (17, -5), (29, -4), (29, -5), (4, -3), (31, 0), (4, -1),
        (4, -4), (34, -2), (36, -3), (31, -4), (34, -1), (4, 0), (3, 0),
        (26, 0), (6, -3), (26, -3), (4, -2), (21, -4), (16, -2), (9, -3),
        (39, 0), (37, -2), (16, 0), (36, 0), (31, -3), (27, -2), (32, 0),
        (29, -1), (39, -2), (2, -5), (36, -5), (1, -4), (14, 0), (11, -5),
        (29, -2), (11, -1), (24, -5), (6, -4), (12, 0), (9, -4), (9, -2),
        (17, -2), (26, -5), (16, -5), (21, -5), (6, -1), (21, 0), (18, -5),
        (11, -4), (26, -1), (26, -2), (9, 0), (18, 0), (21, -2), (33, 0),
        (11, -3), (39, -1), (31, -1), (11, 0), (19, -5), (23, -1), (18, -2),
        (9, -1), (8, -5), (21, -1), (31, -2), (28, -2), (36, -2), (36, -4),
        (23, -3), (12, -2), (39, -5), (3, -5), (39, -3), (16, -3), (17, 0),
        (29, -3), (16, -1), (31, -5), (29, 0), (39, -4), (36, -1), (26, -4),
        (19, 0), (38, -2), (32, -3), (11, -2), (6, -2), (6, 0), (16, -4),
        (13, -2), (21, -3), (7, -5), (24, 0), (23, -4), (13, 0), (33, -3)]

x_val = [x[0] for x in data]
y_val = [y[1] for y in data]

plt.plot(x_val, y_val, 'or')
plt.show()
