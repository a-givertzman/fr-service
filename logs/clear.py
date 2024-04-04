import os
logs_dir = './logs/'

for subdir, dirs, files in os.walk(logs_dir):
    for file in files:
        if file.endswith('.log'):
            print(os.path.join(subdir, file))
            with open(os.path.join(subdir, file), mode="w", encoding="utf-8") as f:
                f.write('')