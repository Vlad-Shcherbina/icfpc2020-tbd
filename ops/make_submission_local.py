'''\
Usage:
    python3 ops/make_submission_local.py <branch> <binary>

<branch>:
    value 'username_zzz' means 'submissions/username_zzz'
    special value 'main' means 'submission'
<binary>:
    value 'example_submission' means 'src/example_submission.rs'
    value 'username/z' means 'scratches/username/src/bin/z.rs'

***END_USAGE***
'''

import os
import sys
import time
import shutil
import platform
import subprocess
from pathlib import Path


def main():
    if platform.system() != 'Linux':
        print('This script needs to produce Linux binaries so it only runs on Linux')
        sys.exit(1)

    if len(sys.argv) != 3:
        print(__doc__.split('***END_USAGE***')[0])
        sys.exit(1)
    [branch, binary] = sys.argv[1:]

    if branch == 'main':
        raw_branch = 'submission'
    else:
        raw_branch = 'submissions/' + branch

    [package, _sep, binary] = binary.rpartition('/')
    if not package:
        package = 'tbd'

    project_root = (Path(__file__)/'..'/'..').resolve()

    subprocess.check_call(
        ['cargo', 'build', '-p', package, '--bin', binary, '--release'],
        cwd=project_root)

    cache = project_root / 'cache'
    assert cache.exists()
    sub_repo = cache / 'icfpc2020-tbd-submissions'
    sub_repo_deploy_key = project_root / 'ops' / 'submission_repo_deploy_key'
    os.chmod(sub_repo_deploy_key, 0o600)
    assert ' ' not in str(sub_repo_deploy_key), 'sorry :('
    ssh_env = dict(GIT_SSH_COMMAND='ssh -i ' + str(sub_repo_deploy_key))

    if not sub_repo.exists():
        subprocess.check_call([
            'git', 'clone',
            'git@github.com:Vlad-Shcherbina/icfpc2020-tbd-submissions.git',
            str(sub_repo),
            ], env=ssh_env)

        subprocess.check_call(['git', 'config', 'user.name', 'make_submission'], cwd=sub_repo)
        subprocess.check_call(['git', 'config', 'user.email', 'make@submission'], cwd=sub_repo)

    subprocess.check_call(['git', 'reset', '--hard'], cwd=sub_repo, env=ssh_env)
    subprocess.check_call(['git', 'clean', '-xdf'], cwd=sub_repo, env=ssh_env)
    subprocess.check_call(['git', 'pull', '--rebase'], cwd=sub_repo, env=ssh_env)

    return_code = subprocess.call(['git', 'checkout', raw_branch], cwd=sub_repo)
    if return_code:
        subprocess.check_call(['git', 'checkout', '-b', raw_branch], cwd=sub_repo)
        subprocess.check_call(['git', 'push', '--set-upstream', 'origin', raw_branch], cwd=sub_repo, env=ssh_env)

    time.sleep(0.1)  # to avoid interliving with the output from the above commands
    print('*' * 30)
    message = input('Describe this submission in one line:\n')

    # intentionally do it again, because user input takes time and we want
    # to minimize the risk of merge conflicts
    subprocess.check_call(['git', 'pull', '--rebase'], cwd=sub_repo, env=ssh_env)

    subprocess.check_call(['git', 'rm', '*'], cwd=sub_repo, env=ssh_env)

    (sub_repo / '.platform').write_text('bash')
    (sub_repo / 'build.sh').write_text('')
    (sub_repo / 'run.sh').write_text('''\
#!/bin/sh
RUST_BACKTRACE=1 ./a.out "$@"
''')

    shutil.copy(
        project_root / 'cache/target/release' / binary,
        sub_repo / 'a.out')

    subprocess.check_call(['git', 'add', '.'], cwd=sub_repo, env=ssh_env)
    subprocess.check_call(
        ['git', 'commit', '--allow-empty', '-m', branch + ': ' + message],
        cwd=sub_repo, env=ssh_env)
    subprocess.check_call(['git', 'push'], cwd=sub_repo, env=ssh_env)

    print('*' * 30)
    print('Submitted!')


if __name__ == '__main__':
    main()
