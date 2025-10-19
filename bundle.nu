cargo br

let os: string = sys host | get name
let ext: string = match $os {
    Windows => ".exe",
    Darwin => "",
    _ => {
        panic "unimplemented"
    }
}
let filename = $"bevy-ai-chatbox($ext)"

mkdir dist
cp -r assets dist/
cp $"target/release/($filename)" dist/
# Clean generated files.
rm --force dist/config.ron
rm --force dist/dialog.ron

print "The packaged files have been successfully output to dist/"