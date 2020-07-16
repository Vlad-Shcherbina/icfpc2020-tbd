
# DON'T FORGET TO UPDATE README AND HELP IF YOU UPDATE THE SCRIPT!

# README:
# This script is the Arch Linux build server client.
#
# To use build server, send your .ssh/id_rsa.pub to @manpages on slack and he
# will reply to you with a number you have to put inside a gitignored
# .build-server-id file in the root of your work directory.
#
# After you have done that, you will be able to run scripts/rbuild.sh or
# scripts/rbuild.sh --debug to get build output (cache/) into your gitignored
# remote_target directory.
#
# NB! while rsynicing work directory, the following local directories are
# excluded: cache/ target/ remote_target.
#
# I also omitted shebang on puprpose, because I don't know if mac os has /usr/env.

if [ ! -f ops/rbuild.sh ] ; then
  echo "Use from the repository root!"
  exit
fi

if [[ $1 == "--help" || $1 == "-h" ]]; then
  echo "Send your ''~/.ssh/id_rsa.pub'' to @manpages on slack."
  echo "Run as ''scripts/rbuild.sh'' to get release version."
  echo "Run as ''scripts/rbuild.sh --debug'' to get both release and debug versions."
  exit
fi

if [ ! -d remote_target ] ; then
  mkdir remote_target
fi

command="cd icfpc2020-tbd && cargo build --release"
if [ ! -z $1 ] ; then
  command="${command} && cargo build"
fi

bid=`cat .build-server-id`
now=`date`

# Send the files over
rsync -Pave 'ssh -p21984' --exclude remote_target --exclude cache --exclude target ../icfpc2020-tbd tbd-build-${bid}@thoughtflare.memorici.de:

# Run cargo build --release and maybe cargo build
ssh -p21984 tbd-build-${bid}@thoughtflare.memorici.de ${command}

# Make the local directory
mkdir "remote_target/${now}"

# Get the cache that will contain target/release
rsync -Pave 'ssh -p21984' tbd-build-${bid}@thoughtflare.memorici.de:icfpc2020-tbd/cache/ "remote_target/${now}"
