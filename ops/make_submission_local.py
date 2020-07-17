'''\
Usage:
    python3 ops/make_submission_local.py <branch> <binary>

<branch>:
    value 'username_zzz' means 'submissions/username_zzz'
    special value 'main' means 'submission'
<binary>:
    value 'example_submission' means 'src/bin/example_submission.rs'
    value 'username/z' means 'scratches/username/src/bin/z.rs'

***END_USAGE***

This script makes a submission to their system.
The system is described in https://github.com/icfpcontest2020/dockerfiles
The submissions show up in https://icfpcontest2020.github.io/#/submissions

The script builds the specified submission binary locally, and then pushes it
to the specified branch in our submission repository (which is different from
the source repository).
'''

import os
import sys
import time
import shutil
import getpass
import platform
import subprocess
from pathlib import Path


def main():
    if platform.system() != 'Linux':
        print('This script needs to produce Linux binaries so it only runs on Linux.')
        print('You can use make_submission_remote.py instead.')
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
    key = sub_repo_deploy_key.read_text()
    sub_repo_deploy_key.write_text(key)  # fix crlf newlines
    os.chmod(sub_repo_deploy_key, 0o600)
    assert ' ' not in str(sub_repo_deploy_key), 'sorry :('
    ssh_env = dict(GIT_SSH_COMMAND='ssh -i ' + str(sub_repo_deploy_key))

    if not sub_repo.exists():
        subprocess.check_call([
            'git', 'clone',
            'git@github.com:Vlad-Shcherbina/icfpc2020-tbd-submissions.git',
            str(sub_repo),
            ], env=ssh_env)

        subprocess.check_call(['git', 'config', 'user.name', getpass.getuser()], cwd=sub_repo)
        subprocess.check_call(['git', 'config', 'user.email', 'make@submission'], cwd=sub_repo)

    subprocess.check_call(['git', 'reset', '--hard'], cwd=sub_repo, env=ssh_env)
    subprocess.check_call(['git', 'clean', '-xdf'], cwd=sub_repo, env=ssh_env)
    subprocess.check_call(['git', 'pull', '--rebase'], cwd=sub_repo, env=ssh_env)

    return_code = subprocess.call(['git', 'checkout', raw_branch], cwd=sub_repo)
    if return_code:
        subprocess.check_call(['git', 'checkout', '-b', raw_branch], cwd=sub_repo)
        subprocess.check_call(['git', 'push', '--set-upstream', 'origin', raw_branch], cwd=sub_repo, env=ssh_env)

    time.sleep(0.1)  # to avoid interleaving with the output from the above commands
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

    provenance = f'### Binary\ncargo build -p {package} --bin {binary} --release\n'
    provenance += '\n\n### Base commit\n'
    provenance += subprocess.check_output(
        ['git', 'log', '-1', "--pretty=%C(auto)%h\n%s\n%an\n%ad", '--abbrev-commit'],
        cwd=project_root, universal_newlines=True)
    provenance += '\n\n### Local changes on top of this base commit\n\n'
    provenance += subprocess.check_output(
        ['git', 'diff', 'HEAD', '--ignore-space-at-eol', '--ignore-cr-at-eol',
         '--diff-filter=d',  # ignore deleted files to avoid noise caused by rsync and gitignore interaction
         ],
        cwd=project_root, universal_newlines=True)
    (sub_repo / 'provenance.txt').write_text(provenance)

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
