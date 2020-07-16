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
