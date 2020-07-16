'''
Same as make_submission_local.py, but uses the build server.
'''

import sys
import platform
import subprocess
from pathlib import Path


def main():
    if platform.system() == 'Windows':
        print('This script uses rsync and ssh.')
        print("It can't be run on Windows, run it from WSL instead.")
        sys.exit(1)
    if len(sys.argv) != 3:
        print('Usage:')
        print('    make_submission_remote.py <branch> <binary>')
        print()
        print('See make_submission_local.py for details.')
        sys.exit(1)

    project_root = (Path(__file__)/'..'/'..').resolve()

    build_server_id = project_root / '.build-server-id'
    if not build_server_id.exists():
        print(build_server_id, 'not found')
        print('Obtain your build server ID from manpages (see comments in ops/rbuild.sh)')
        print('and put it in .build-server-id')
        sys.exit(1)
    build_server_id = build_server_id.read_text().strip()

    server = f'tbd-build-{build_server_id}@thoughtflare.memorici.de'
    subprocess.check_call([
        'rsync',
        '-avz',
        '-e', 'ssh -p21984',
        '--filter', ':- .gitignore',
        '.',
        server + ':icfpc2020-tbd',
        ], cwd=project_root)

    command = 'python3 icfpc2020-tbd/ops/make_submission_local.py ' + ' '.join(sys.argv[1:])
    subprocess.check_call(['ssh', '-p21984', server, command])


if __name__ == '__main__':
    main()
